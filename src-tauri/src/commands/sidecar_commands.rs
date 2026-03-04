use crate::domain::settings::SidecarStatus;
use crate::error::OrqaError;
use crate::state::AppState;

/// The sidecar is a Node.js process.
const SIDECAR_COMMAND: &str = "node";

/// Determine the sidecar script arguments.
///
/// Prefers the real Agent SDK sidecar (`sidecar/dist/sidecar.js`) if built.
/// Falls back to the test echo sidecar (`src-tauri/test-sidecar/echo.cjs`).
/// Path resolution tries CWD as project root first (cargo tauri dev),
/// then CWD as src-tauri/ (cargo run).
fn sidecar_args() -> Vec<String> {
    // 1. Real Agent SDK sidecar (preferred)
    let real_sidecar = std::path::Path::new("sidecar/dist/sidecar.js");
    if real_sidecar.exists() {
        return vec![real_sidecar.to_string_lossy().to_string()];
    }

    // 2. Real sidecar from src-tauri/ CWD
    let real_alt = std::path::Path::new("../sidecar/dist/sidecar.js");
    if real_alt.exists() {
        return vec![real_alt.to_string_lossy().to_string()];
    }

    // 3. Test echo sidecar (fallback for development without Agent SDK)
    let echo_path = std::path::Path::new("src-tauri/test-sidecar/echo.cjs");
    if echo_path.exists() {
        return vec![echo_path.to_string_lossy().to_string()];
    }

    // 4. Test echo from src-tauri/ CWD
    let echo_alt = std::path::Path::new("test-sidecar/echo.cjs");
    if echo_alt.exists() {
        return vec![echo_alt.to_string_lossy().to_string()];
    }

    // Last resort
    vec!["sidecar/dist/sidecar.js".to_string()]
}

/// Ensure the sidecar is running, spawning it if necessary.
///
/// Returns Ok(()) if the sidecar is already connected or was successfully spawned.
pub fn ensure_sidecar_running(state: &AppState) -> Result<(), OrqaError> {
    if state.sidecar.is_connected() {
        return Ok(());
    }

    let args = sidecar_args();
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    state
        .sidecar
        .spawn(SIDECAR_COMMAND, &arg_refs)
        .map_err(|e| {
            OrqaError::Sidecar(format!(
                "failed to auto-start sidecar: {e}. \
                 Ensure Node.js is installed and in PATH."
            ))
        })
}

/// Query the current status of the sidecar process.
///
/// Returns the live status from `SidecarManager`, including PID, uptime,
/// and connection state.
#[tauri::command]
pub fn sidecar_status(state: tauri::State<'_, AppState>) -> Result<SidecarStatus, OrqaError> {
    Ok(state.sidecar.status())
}

/// Restart the sidecar process.
///
/// Kills any existing sidecar process and spawns a new one.
#[tauri::command]
pub fn sidecar_restart(state: tauri::State<'_, AppState>) -> Result<SidecarStatus, OrqaError> {
    let args = sidecar_args();
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    state.sidecar.restart(SIDECAR_COMMAND, &arg_refs)
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
