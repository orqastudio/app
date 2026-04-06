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
// (the daemon health port, default 10100) plus an offset of 1. This gives a
// default LSP port of 10101. The daemon health port is the single env var that
// controls the port range for all daemon-adjacent services.
//
// If the binary is not found the daemon degrades gracefully and logs a warning.

use std::path::Path;

use tracing::{info, warn};

use orqa_engine::ports::resolve_lsp_port;

use crate::correlation::current_correlation_id;
use crate::subprocess::SubprocessManager;

/// Binary name for the LSP server.
const LSP_BINARY: &str = "orqa-lsp-server";

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

    info!(subsystem = "lsp", lsp_port, "spawning LSP server");

    let args = vec![
        "--tcp".to_owned(),
        lsp_port.to_string(),
        "--daemon-port".to_owned(),
        daemon_port.to_string(),
    ];

    let mut manager = SubprocessManager::new("lsp-server", LSP_BINARY, args);

    // Forward the active correlation ID so the LSP server can include it in
    // its own logs, enabling end-to-end tracing from daemon to LSP.
    if let Some(trace_id) = current_correlation_id() {
        manager.set_env("ORQA_TRACE_ID", trace_id);
    }

    match manager.start(project_root) {
        Ok(()) => {
            info!(
                subsystem = "lsp",
                status = ?manager.status(),
                lsp_port,
                "LSP server startup complete"
            );
        }
        Err(e) => {
            warn!(
                subsystem = "lsp",
                error = %e,
                "failed to spawn LSP server — daemon continues without LSP"
            );
        }
    }

    manager
}
