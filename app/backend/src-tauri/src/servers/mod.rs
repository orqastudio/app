pub mod ipc_socket;
pub mod lsp;
pub mod mcp;

use std::io;
use std::path::PathBuf;

/// Locate a server binary by name.
///
/// Search order:
/// 1. Same directory as the running executable (co-located binaries from `cargo build`)
/// 2. `PATH` lookup
///
/// Returns the full path to the binary, or an `io::Error` if not found.
pub(crate) fn find_server_binary(name: &str) -> Result<PathBuf, io::Error> {
    // On Windows the binary name needs .exe
    let bin_name = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };

    // 1. Check next to the current executable (workspace target/debug or target/release)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join(&bin_name);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }

    // 2. Fall back to PATH
    // On Windows, Command::new will find .exe on PATH automatically,
    // but we still try to resolve it for a better error message.
    let path_dirs = std::env::var_os("PATH").unwrap_or_default();
    for dir in std::env::split_paths(&path_dirs) {
        let candidate = dir.join(&bin_name);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "server binary '{name}' not found — expected next to the running \
             executable or on PATH. Build with `cargo build` in the workspace root."
        ),
    ))
}
