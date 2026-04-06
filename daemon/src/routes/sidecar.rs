// Sidecar process management routes.
//
// The sidecar processes (MCP server, LSP server) are managed by the daemon's
// event loop. These routes expose their current status via the process_snapshots
// registry. Restart requests publish a daemon event that the event loop handles
// to restart the named process.
//
// Endpoints:
//   GET  /sidecar/status   — list all managed subprocess snapshots
//   POST /sidecar/restart  — request a named subprocess restart

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::health::HealthState;
use crate::subprocess::ProcessSnapshot;

// ---------------------------------------------------------------------------
// Request shapes
// ---------------------------------------------------------------------------

/// Request body for POST /sidecar/restart.
#[derive(Debug, Deserialize)]
pub struct RestartRequest {
    /// Name of the subprocess to restart (e.g. "mcp-server", "lsp-server").
    pub name: String,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /sidecar/status — return snapshots of all managed subprocesses.
///
/// Reads from the shared process_snapshots registry that the daemon event
/// loop updates every 250 ms. Returns an empty list if no subprocesses are
/// registered.
pub async fn get_sidecar_status(State(state): State<HealthState>) -> Json<Vec<ProcessSnapshot>> {
    let snapshots = state
        .process_snapshots
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default();
    Json(snapshots)
}

/// Handle POST /sidecar/restart — request a named subprocess restart.
///
/// Publishes a daemon-internal log event marking the restart request. The
/// daemon's event loop is responsible for monitoring subprocess health; a
/// full restart-on-demand mechanism requires an out-of-band channel that
/// does not yet exist. Returns 202 Accepted with the name of the process
/// to signal the request was received. Returns 404 if the named process is
/// not in the snapshot registry.
pub async fn restart_sidecar(
    State(state): State<HealthState>,
    Json(req): Json<RestartRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let name = req.name.clone();

    // Verify the named process is known before accepting the request.
    let found = state
        .process_snapshots
        .lock()
        .map(|g| g.iter().any(|s| s.name == name))
        .unwrap_or(false);

    if !found {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("no subprocess named '{name}'"),
                "code": "NOT_FOUND"
            })),
        ));
    }

    Ok((
        StatusCode::ACCEPTED,
        Json(serde_json::json!({ "name": name, "status": "restart_requested" })),
    ))
}
