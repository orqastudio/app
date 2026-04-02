//! Local IPC socket server for CLI <-> App communication.
//!
//! Listens on a TCP port on localhost. The port is written to a well-known
//! lock file so the CLI can discover it. Handles MCP and LSP protocol
//! messages from CLI proxy clients by spawning the pre-built binaries.
//!
//! The CLI runs `orqa mcp` or `orqa lsp` which connects to this socket
//! and bridges stdin/stdout <-> TCP.

use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

use orqa_engine_types::ports::resolve_daemon_port;

use super::find_server_binary;

/// Well-known file where the IPC port is stored.
/// CLI reads this to discover the running app instance.
const IPC_PORT_FILENAME: &str = "ipc.port";

/// Get the path to the IPC port file.
fn port_file_path() -> PathBuf {
    dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.orqastudio.app")
        .join(IPC_PORT_FILENAME)
}

/// Start the IPC socket server in a background thread.
///
/// Binds to a random available port on localhost, writes the port
/// to the well-known port file, and spawns a thread to accept
/// connections.
///
/// Each connection is handled in its own thread. The first line
/// from the client determines the protocol:
/// - `MCP <project-path>` -> spawn `orqa-mcp-server` on this connection
/// - `LSP <project-path>` -> spawn `orqa-lsp-server` on this connection
pub fn start(project_root: Option<PathBuf>) {
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(e) => {
            tracing::warn!("[ipc] failed to bind IPC socket: {e}");
            return;
        }
    };

    let port = listener.local_addr().map(|a| a.port()).unwrap_or(0);
    if port == 0 {
        tracing::warn!("[ipc] failed to get IPC port");
        return;
    }

    // Write port to the well-known file
    let port_file = port_file_path();
    if let Some(parent) = port_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(&port_file, port.to_string()) {
        tracing::warn!("[ipc] failed to write port file: {e}");
        return;
    }

    tracing::info!("[ipc] listening on 127.0.0.1:{port}");

    let default_root = project_root
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let root = default_root.clone();
                    thread::spawn(move || {
                        if let Err(e) = handle_connection(stream, &root) {
                            tracing::warn!("[ipc] connection error: {e}");
                        }
                    });
                }
                Err(e) => {
                    tracing::warn!("[ipc] accept error: {e}");
                }
            }
        }

        // Cleanup port file on exit
        let _ = std::fs::remove_file(port_file_path());
    });
}

/// Handle a single IPC connection.
///
/// Reads the first line to determine the protocol, then spawns the
/// appropriate server binary with stdin/stdout piped to the TCP stream.
fn handle_connection(
    stream: TcpStream,
    default_root: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;

    // Read protocol header: "MCP [path]" or "LSP [path]"
    let mut header = String::new();
    reader.read_line(&mut header)?;
    let header = header.trim();

    let parts: Vec<&str> = header.splitn(2, ' ').collect();
    let protocol = parts.first().copied().unwrap_or("");
    let project_root = parts
        .get(1)
        .map_or_else(|| default_root.to_path_buf(), |p| PathBuf::from(*p));

    let daemon_port = resolve_daemon_port();

    match protocol {
        "MCP" => {
            tracing::info!("[ipc] MCP session for {}", project_root.display());
            spawn_server_bridge(
                "orqa-mcp-server",
                &project_root,
                daemon_port,
                reader,
                writer,
            )?;
        }
        "LSP" => {
            tracing::info!("[ipc] LSP session for {}", project_root.display());
            spawn_server_bridge(
                "orqa-lsp-server",
                &project_root,
                daemon_port,
                reader,
                writer,
            )?;
        }
        _ => {
            writeln!(writer, "Unknown protocol: {protocol}. Expected MCP or LSP.")?;
        }
    }

    Ok(())
}

/// Spawn a server binary with its stdin/stdout bridged to the TCP connection.
///
/// The client's TCP stream becomes the binary's communication channel:
/// - Reads from TCP -> writes to binary stdin
/// - Reads from binary stdout -> writes to TCP
fn spawn_server_bridge(
    binary_name: &str,
    project_root: &Path,
    daemon_port: u16,
    mut tcp_reader: BufReader<TcpStream>,
    mut tcp_writer: TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let binary = find_server_binary(binary_name)?;

    let mut child = Command::new(&binary)
        .arg(project_root.as_os_str())
        .arg("--daemon-port")
        .arg(daemon_port.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("failed to spawn {binary_name}: {e}"))?;

    let mut child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("failed to capture child stdin"))?;
    let mut child_stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("failed to capture child stdout"))?;

    // Bridge: binary stdout -> TCP writer (runs in a separate thread)
    let stdout_handle = thread::spawn(move || {
        let _ = io::copy(&mut child_stdout, &mut tcp_writer);
    });

    // Bridge: TCP reader -> binary stdin (runs on this thread)
    let _ = io::copy(&mut tcp_reader, &mut child_stdin);

    // When the TCP reader closes, close the child's stdin so it exits cleanly.
    drop(child_stdin);

    // Wait for the stdout bridge to finish
    let _ = stdout_handle.join();

    // Wait for the child process to exit
    let status = child.wait()?;
    if !status.success() {
        tracing::warn!(
            "[ipc] {binary_name} exited with status {}",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}

/// Read the IPC port from the well-known port file.
/// Returns None if the app is not running.
pub fn read_port() -> Option<u16> {
    let port_file = port_file_path();
    let content = std::fs::read_to_string(&port_file).ok()?;
    content.trim().parse().ok()
}

/// Clean up the port file (called on app shutdown).
pub fn cleanup() {
    let _ = std::fs::remove_file(port_file_path());
}
