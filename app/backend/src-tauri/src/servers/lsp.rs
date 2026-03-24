//! LSP server spawning — delegates to the `orqa-lsp-server` pre-built binary.
//!
//! Instead of embedding LSP protocol logic, this module spawns the standalone
//! `orqa-lsp-server` binary as a child process. Both the CLI (`orqa lsp`) and
//! the app share the same binary — one source of truth for protocol handling.

use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

use super::find_server_binary;

/// Spawn `orqa-lsp-server` over stdio for the given project root.
///
/// The child process inherits stdin/stdout so the caller can communicate
/// via the LSP JSON-RPC stdio transport. Blocks until the process exits.
///
/// # Errors
///
/// Returns an `io::Error` if the binary cannot be found or spawned,
/// or if the process exits with a non-zero status.
pub fn run(project_root: &Path, daemon_port: u16) -> Result<(), io::Error> {
    let binary = find_server_binary("orqa-lsp-server")?;

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
            "orqa-lsp-server exited with status {}",
            status.code().unwrap_or(-1)
        )))
    }
}
