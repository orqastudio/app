use crate::domain::settings::SidecarStatus;
use crate::error::ForgeError;
use crate::state::AppState;

/// Query the current status of the sidecar process.
///
/// Returns the live status from `SidecarManager`, including PID, uptime,
/// and connection state.
#[tauri::command]
pub fn sidecar_status(state: tauri::State<'_, AppState>) -> Result<SidecarStatus, ForgeError> {
    Ok(state.sidecar.status())
}

/// Restart the sidecar process.
///
/// Kills any existing sidecar process and spawns a new one. The sidecar
/// command is currently a test echo sidecar; this will be configurable
/// in a future phase.
#[tauri::command]
pub fn sidecar_restart(state: tauri::State<'_, AppState>) -> Result<SidecarStatus, ForgeError> {
    state.sidecar.restart("node", &["test-sidecar/echo.cjs"])
}

#[cfg(test)]
mod tests {
    #[test]
    fn sidecar_status_not_started_without_tauri() {
        // Direct unit test of the manager — cannot test tauri::State in isolation
        let manager = crate::sidecar::manager::SidecarManager::new();
        let status = manager.status();
        assert_eq!(
            status.state,
            crate::domain::settings::SidecarState::NotStarted
        );
        assert!(status.pid.is_none());
        assert!(status.uptime_seconds.is_none());
        assert!(!status.cli_detected);
        assert!(status.cli_version.is_none());
        assert!(status.error_message.is_none());
    }
}
