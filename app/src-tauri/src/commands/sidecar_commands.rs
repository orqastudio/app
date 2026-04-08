// Tauri IPC commands for claude subprocess status.
//
// The claude subprocess is managed entirely by the daemon. The app is a thin
// client that queries and controls it via HTTP.
//
// Endpoints used:
//   GET  /sidecar/status  — query claude subprocess status
//   POST /sidecar/restart — restart the claude subprocess

use tauri::State;

use orqa_engine_types::types::settings::SidecarStatus;

use crate::error::OrqaError;
use crate::state::AppState;

/// Query the current status of the claude subprocess managed by the daemon.
#[tauri::command]
pub async fn sidecar_status(state: State<'_, AppState>) -> Result<SidecarStatus, OrqaError> {
    match state.daemon.client.get_sidecar_status().await {
        Ok(status) => {
            tracing::debug!(state = ?status.state, "sidecar_status: ok");
            Ok(status)
        }
        Err(e) => {
            tracing::warn!(error = %e, "sidecar_status: failed");
            Err(e)
        }
    }
}

/// Restart the claude subprocess via the daemon.
#[tauri::command]
pub async fn sidecar_restart(state: State<'_, AppState>) -> Result<SidecarStatus, OrqaError> {
    tracing::info!("[sidecar] sidecar_restart: delegating to daemon");
    state.daemon.client.restart_sidecar().await
}

#[cfg(test)]
mod tests {
    // Sidecar commands require daemon connectivity. Covered by integration tests.
}
