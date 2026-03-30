// MCP server lifecycle management for the OrqaStudio daemon.
//
// The `orqa-mcp-server` binary supports both stdio transport (for direct LLM
// client spawning) and TCP transport (for daemon-managed persistent operation).
// The daemon starts it in TCP mode so that:
//
//   1. A single long-running MCP process serves all connected LLM clients.
//   2. The daemon can monitor its health (TCP port liveness, process status).
//   3. LLM clients connect to a well-known port rather than managing their own
//      subprocess lifecycle.
//
// The default TCP port for the MCP server is derived from `ORQA_PORT_BASE`
// (the daemon health port, default 10100) plus an offset of 2. This gives a
// default MCP port of 10102. The daemon health port is the single env var that
// controls the port range for all daemon-adjacent services.
//
// Port allocation:
//   Daemon health: ORQA_PORT_BASE + 0  (default 10100)
//   LSP server:    ORQA_PORT_BASE + 1  (default 10101)
//   MCP server:    ORQA_PORT_BASE + 2  (default 10102)
//
// If the binary is not found the daemon degrades gracefully and logs a warning.

use std::path::Path;

use tracing::{info, warn};

use crate::subprocess::SubprocessManager;

/// Binary name for the MCP server.
const MCP_BINARY: &str = "orqa-mcp-server";

/// Port offset added to ORQA_PORT_BASE for the MCP TCP listener.
const MCP_PORT_OFFSET: u16 = 2;

/// Default daemon health port (matches health.rs DEFAULT_PORT).
const DEFAULT_DAEMON_PORT: u16 = 10100;

/// Resolve the TCP port for the MCP server.
///
/// Reads `ORQA_PORT_BASE` (the daemon health port, default 10100) from the
/// environment and adds `MCP_PORT_OFFSET` (2). This gives a default MCP port
/// of 10102. Falls back to `DEFAULT_DAEMON_PORT + MCP_PORT_OFFSET` when the
/// variable is absent or unparseable.
pub fn resolve_mcp_port() -> u16 {
    let base: u16 = std::env::var("ORQA_PORT_BASE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_DAEMON_PORT);
    base + MCP_PORT_OFFSET
}

/// Build a `SubprocessManager` for the MCP server and attempt to start it.
///
/// The MCP server is started in TCP mode so a single process can serve all
/// LLM clients simultaneously. Arguments passed:
///   - `project_root` — positional project path
///   - `--tcp <mcp_port>` — listen on TCP instead of stdio
///   - `--daemon-port <daemon_port>` — connect back to the daemon for graph
///     operations
///
/// If the binary is not found on disk or on PATH, the function logs a warning
/// and returns the manager in `BinaryNotFound` state — the daemon continues
/// operating without MCP. LLM clients can still spawn `orqa-mcp-server`
/// directly in stdio mode as a fallback.
pub fn start_mcp(project_root: &Path, daemon_port: u16) -> SubprocessManager {
    let mcp_port = resolve_mcp_port();

    let args = vec![
        "--tcp".to_owned(),
        mcp_port.to_string(),
        "--daemon-port".to_owned(),
        daemon_port.to_string(),
    ];

    let mut manager = SubprocessManager::new("mcp-server", MCP_BINARY, args);

    match manager.start(project_root) {
        Ok(()) => {
            info!(
                status = ?manager.status(),
                mcp_port,
                "MCP server startup complete"
            );
        }
        Err(e) => {
            warn!(
                error = %e,
                "failed to spawn MCP server — daemon continues without MCP"
            );
        }
    }

    manager
}
