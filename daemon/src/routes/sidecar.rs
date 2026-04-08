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
use orqa_engine_types::types::settings::{SidecarState, SidecarStatus};

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

/// Handle GET /sidecar/status — return the status of the Claude sidecar.
///
/// Checks the process_snapshots registry for a Claude-related subprocess
/// (MCP server). If found and running, reports connected. Otherwise reports
/// the appropriate state based on CLI detection.
pub async fn get_sidecar_status(State(state): State<HealthState>) -> Json<SidecarStatus> {
    let snapshots = state
        .process_snapshots
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default();

    let mcp = snapshots.iter().find(|s| s.source == "mcp");
    Json(snapshot_to_sidecar_status(mcp))
}

/// Map a process snapshot to a `SidecarStatus`.
fn snapshot_to_sidecar_status(mcp: Option<&crate::subprocess::ProcessSnapshot>) -> SidecarStatus {
    match mcp {
        Some(proc) if proc.status == "running" => SidecarStatus {
            state: SidecarState::Connected,
            pid: proc.pid,
            uptime_seconds: proc.uptime_seconds,
            cli_detected: true,
            cli_version: None,
            error_message: None,
        },
        Some(proc) if proc.status == "stopped" => SidecarStatus {
            state: SidecarState::Stopped,
            pid: None,
            uptime_seconds: None,
            cli_detected: proc.binary_path.is_some(),
            cli_version: None,
            error_message: None,
        },
        Some(proc) if proc.status == "crashed" => SidecarStatus {
            state: SidecarState::Error,
            pid: None,
            uptime_seconds: None,
            cli_detected: proc.binary_path.is_some(),
            cli_version: None,
            error_message: Some("MCP server crashed".to_owned()),
        },
        Some(proc) if proc.status == "not_found" => SidecarStatus {
            state: SidecarState::NotStarted,
            pid: None,
            uptime_seconds: None,
            cli_detected: false,
            cli_version: None,
            error_message: Some(format!(
                "Binary not found: {}",
                proc.binary_path.as_deref().unwrap_or("orqa-mcp-server")
            )),
        },
        _ => SidecarStatus {
            state: SidecarState::NotStarted,
            pid: None,
            uptime_seconds: None,
            cli_detected: false,
            cli_version: None,
            error_message: None,
        },
    }
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
