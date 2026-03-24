//! MCP server spawning — delegates to the `orqa-mcp-server` pre-built binary.
//!
//! Instead of embedding the MCP protocol logic, this module spawns the standalone
//! `orqa-mcp-server` binary as a child process. Both the CLI (`orqa mcp`) and the
//! app share the same binary — one source of truth for protocol handling.

use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

use super::find_server_binary;

/// Spawn `orqa-mcp-server` over stdio for the given project root.
///
/// The child process inherits stdin/stdout so the caller can communicate
/// via the JSON-RPC stdio transport. Blocks until the process exits.
///
/// # Errors
///
/// Returns an `io::Error` if the binary cannot be found or spawned,
/// or if the process exits with a non-zero status.
pub fn run(project_root: &Path, daemon_port: u16) -> Result<(), io::Error> {
    let binary = find_server_binary("orqa-mcp-server")?;

    let status: ExitStatus = Command::new(&binary)
        .arg(project_root.as_os_str())
        .arg("--daemon-port")
        .arg(daemon_port.to_string())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "orqa-mcp-server exited with status {}",
            status.code().unwrap_or(-1)
        )))
    }
}
