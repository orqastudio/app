// MCP server lifecycle management for the OrqaStudio daemon.
//
// The `orqa-mcp-server` binary uses stdio (JSON-RPC over stdin/stdout) as its
// transport. This means each LLM client that wants MCP access spawns its own
// `orqa-mcp-server` instance rather than connecting to a shared server. As a
// result the daemon does not pre-spawn a persistent MCP instance — instead it
// exposes this module so that:
//
//   1. The daemon can optionally launch a single MCP instance for clients
//      that request it (e.g., via the health endpoint or tray menu).
//   2. The lifecycle pattern is in place for future work (e.g., TCP-mode MCP,
//      or a daemon-mediated MCP proxy).
//
// For now `start_mcp` attempts to spawn the binary, logs the outcome, and
// returns the manager. If the binary is not found, the daemon degrades
// gracefully and continues without MCP.

use std::path::Path;

use tracing::{info, warn};

use crate::subprocess::SubprocessManager;

/// Binary name for the MCP server.
const MCP_BINARY: &str = "orqa-mcp-server";

/// Build a `SubprocessManager` for the MCP server and attempt to start it.
///
/// The MCP server is spawned with `project_root` as its first positional
/// argument. The `--daemon-port` flag is passed so the MCP server can call
/// back into the daemon for graph-level operations.
///
/// If the binary is not found on disk or on PATH, the function logs a warning
/// and returns the manager in `BinaryNotFound` state — the daemon continues
/// operating without MCP.
pub fn start_mcp(project_root: &Path, daemon_port: u16) -> SubprocessManager {
    let args = vec!["--daemon-port".to_string(), daemon_port.to_string()];

    let mut manager = SubprocessManager::new("mcp-server", MCP_BINARY, args);

    match manager.start(project_root) {
        Ok(()) => {
            info!(
                status = ?manager.status(),
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
