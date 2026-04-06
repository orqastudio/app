// DevTools launch and status routes.
//
// The OrqaDev devtools window is a separate Tauri application binary
// (`orqa-devtools`). These routes let the app trigger devtools launch
// and check whether it is already running.
//
// Endpoints:
//   POST /devtools/launch  — launch the OrqaDev window as a child process
//   GET  /devtools/status  — check if OrqaDev is currently running

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use orqa_engine::ports::resolve_daemon_port;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /devtools/launch — launch the OrqaDev devtools window.
///
/// Searches for the `orqa-devtools` binary alongside the current daemon
/// executable and spawns it. Returns 202 Accepted when the process is
/// successfully spawned, or 404 when the binary is not found.
pub async fn launch_devtools(
    State(state): State<HealthState>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    // Check if already running to avoid duplicate launches.
    let already_running = state
        .process_snapshots
        .lock()
        .map(|g| {
            g.iter()
                .any(|s| s.name.contains("devtools") && s.status == "running")
        })
        .unwrap_or(false);

    if already_running {
        return Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "status": "already_running" })),
        ));
    }

    // Find the binary next to the current executable.
    let binary =
        crate::subprocess::SubprocessManager::find_binary("orqa-devtools").ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "orqa-devtools binary not found",
                    "code": "BINARY_NOT_FOUND"
                })),
            )
        })?;

    let port_arg = resolve_daemon_port().to_string();
    let spawned = std::process::Command::new(&binary)
        .args(["--daemon-port", &port_arg])
        .spawn()
        .is_ok();

    if spawned {
        Ok((
            StatusCode::ACCEPTED,
            Json(serde_json::json!({ "status": "launched" })),
        ))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "failed to spawn orqa-devtools",
                "code": "SPAWN_FAILED"
            })),
        ))
    }
}

/// Handle GET /devtools/status — check if the OrqaDev window is running.
///
/// Checks the process_snapshots registry for a devtools entry. Falls back
/// to a binary probe when not tracked as a snapshot.
pub async fn get_devtools_status(State(state): State<HealthState>) -> Json<serde_json::Value> {
    let running = state
        .process_snapshots
        .lock()
        .map(|g| {
            g.iter()
                .any(|s| s.name.contains("devtools") && s.status == "running")
        })
        .unwrap_or(false);

    let binary_available =
        crate::subprocess::SubprocessManager::find_binary("orqa-devtools").is_some();

    Json(serde_json::json!({
        "running": running,
        "binary_available": binary_available
    }))
}
