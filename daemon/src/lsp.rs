// LSP server lifecycle management for the OrqaStudio daemon.
//
// The `orqa-lsp-server` binary supports both stdio transport (for direct
// editor process spawning) and TCP transport (for editors that can connect
// to a persistent language server socket). The daemon starts it in TCP mode
// so that:
//
//   1. A single long-running LSP process serves all connected editors.
//   2. The daemon can monitor its health (TCP port liveness, process status).
//   3. Editors connect to a well-known port rather than managing their own
//      subprocess lifecycle.
//
// The default TCP port for the LSP server is derived from `ORQA_PORT_BASE`
// (the daemon health port, default 9120) plus an offset of 57. This gives a
// default LSP port of 9177. The daemon health port is the single env var that
// controls the port range for all daemon-adjacent services.
//
// If the binary is not found the daemon degrades gracefully and logs a warning.

use std::path::Path;

use tracing::{info, warn};

use crate::subprocess::SubprocessManager;

/// Binary name for the LSP server.
const LSP_BINARY: &str = "orqa-lsp-server";

/// Port offset added to ORQA_PORT_BASE for the LSP TCP listener.
const LSP_PORT_OFFSET: u16 = 57;

/// Default daemon health port (matches health.rs DEFAULT_PORT).
const DEFAULT_DAEMON_PORT: u16 = 9120;

/// Resolve the TCP port for the LSP server.
///
/// Reads `ORQA_PORT_BASE` (the daemon health port, default 9120) from the
/// environment and adds `LSP_PORT_OFFSET` (57). This gives a default LSP port
/// of 9177. Falls back to `DEFAULT_DAEMON_PORT + LSP_PORT_OFFSET` when the
/// variable is absent or unparseable.
pub fn resolve_lsp_port() -> u16 {
    let base: u16 = std::env::var("ORQA_PORT_BASE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_DAEMON_PORT);
    base + LSP_PORT_OFFSET
}

/// Build a `SubprocessManager` for the LSP server and attempt to start it.
///
/// The LSP server is started in TCP mode so a single process can serve all
/// editors simultaneously. Arguments passed:
///   - `project_root` — positional project path
///   - `--tcp <lsp_port>` — listen on TCP instead of stdio
///   - `--daemon-port <daemon_port>` — connect back to the daemon for graph
///     validation
///
/// If the binary is not found on disk or on PATH, the function logs a warning
/// and returns the manager in `BinaryNotFound` state — the daemon continues
/// operating without LSP.
pub fn start_lsp(project_root: &Path, daemon_port: u16) -> SubprocessManager {
    let lsp_port = resolve_lsp_port();

    let args = vec![
        "--tcp".to_string(),
        lsp_port.to_string(),
        "--daemon-port".to_string(),
        daemon_port.to_string(),
    ];

    let mut manager = SubprocessManager::new("lsp-server", LSP_BINARY, args);

    match manager.start(project_root) {
        Ok(()) => {
            info!(
                status = ?manager.status(),
                lsp_port,
                "LSP server startup complete"
            );
        }
        Err(e) => {
            warn!(
                error = %e,
                "failed to spawn LSP server — daemon continues without LSP"
            );
        }
    }

    manager
}
