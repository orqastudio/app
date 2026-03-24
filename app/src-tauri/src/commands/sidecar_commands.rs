use crate::domain::settings::SidecarStatus;
use crate::error::OrqaError;
use crate::state::AppState;

/// The sidecar is a Node.js process.
const SIDECAR_COMMAND: &str = "node";

/// Config file written by the frontend/SDK when a sidecar plugin is registered.
const SIDECAR_CONFIG_FILENAME: &str = "sidecar-config.json";

/// Sidecar configuration loaded from `sidecar-config.json`.
#[derive(serde::Deserialize, Debug)]
struct SidecarConfig {
    /// Runtime command (e.g. "node").
    runtime: Option<String>,
    /// Resolved path to the sidecar entrypoint.
    entrypoint: String,
    /// Additional CLI arguments.
    args: Option<Vec<String>>,
}

/// Read sidecar configuration from `sidecar-config.json` in the project root.
///
/// Searches CWD (app root during `cargo tauri dev`) and one level up
/// (CWD is `src-tauri/` during `cargo run`).
fn read_sidecar_config() -> Option<SidecarConfig> {
    let candidates = [
        std::path::PathBuf::from(SIDECAR_CONFIG_FILENAME),
        std::path::PathBuf::from("../").join(SIDECAR_CONFIG_FILENAME),
    ];

    for path in &candidates {
        if path.exists() {
            if let Ok(contents) = std::fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str::<SidecarConfig>(&contents) {
                    return Some(config);
                }
            }
        }
    }
    None
}

/// Determine the sidecar runtime command and script arguments.
///
/// Reads `sidecar-config.json` (written by the plugin system) if present.
/// Falls back to the test echo sidecar when no config exists.
fn resolve_sidecar() -> (String, Vec<String>) {
    // 1. Config-driven: read from sidecar-config.json (written by plugin registry)
    if let Some(config) = read_sidecar_config() {
        let runtime = config
            .runtime
            .unwrap_or_else(|| SIDECAR_COMMAND.to_string());
        let mut args = vec![config.entrypoint];
        if let Some(extra) = config.args {
            args.extend(extra);
        }
        return (runtime, args);
    }

    // 2. Test echo sidecar fallback (CWD is app root)
    let echo_path = std::path::Path::new("src-tauri/test-sidecar/echo.cjs");
    if echo_path.exists() {
        return (
            SIDECAR_COMMAND.to_string(),
            vec![echo_path.to_string_lossy().to_string()],
        );
    }

    // 3. Test echo from src-tauri/ CWD
    let echo_alt = std::path::Path::new("test-sidecar/echo.cjs");
    if echo_alt.exists() {
        return (
            SIDECAR_COMMAND.to_string(),
            vec![echo_alt.to_string_lossy().to_string()],
        );
    }

    // Last resort — will fail at spawn time with a clear error
    (
        SIDECAR_COMMAND.to_string(),
        vec!["sidecar-not-configured".to_string()],
    )
}

/// Ensure the sidecar is running, spawning it if necessary.
///
/// Returns Ok(()) if the sidecar is already connected or was successfully spawned.
pub fn ensure_sidecar_running(state: &AppState) -> Result<(), OrqaError> {
    if state.sidecar.manager.is_connected() {
        return Ok(());
    }

    let (command, args) = resolve_sidecar();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    state
        .sidecar
        .manager
        .spawn(&command, &arg_refs)
        .map_err(|e| {
            OrqaError::Sidecar(format!(
                "failed to auto-start sidecar: {e}. \
                 Ensure Node.js is installed and in PATH, \
                 or check sidecar-config.json."
            ))
        })
}

/// Query the current status of the sidecar process.
///
/// Returns the live status from `SidecarManager`, including PID, uptime,
/// and connection state.
#[tauri::command]
pub fn sidecar_status(state: tauri::State<'_, AppState>) -> Result<SidecarStatus, OrqaError> {
    Ok(state.sidecar.manager.status())
}

/// Restart the sidecar process.
///
/// Kills any existing sidecar process and spawns a new one.
#[tauri::command]
pub fn sidecar_restart(state: tauri::State<'_, AppState>) -> Result<SidecarStatus, OrqaError> {
    let (command, args) = resolve_sidecar();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    state.sidecar.manager.restart(&command, &arg_refs)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn resolve_sidecar_returns_fallback_when_no_config() {
        // Without sidecar-config.json in CWD, should return a fallback
        let (command, args) = resolve_sidecar();
        assert_eq!(command, "node");
        assert!(!args.is_empty());
    }

    #[test]
    fn sidecar_config_deserialization() {
        let json = r#"{"runtime": "node", "entrypoint": "plugins/claude-integration/sidecar/dist/sidecar.js"}"#;
        let config: SidecarConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.runtime, Some("node".to_string()));
        assert_eq!(
            config.entrypoint,
            "plugins/claude-integration/sidecar/dist/sidecar.js"
        );
        assert!(config.args.is_none());
    }

    #[test]
    fn sidecar_config_deserialization_with_args() {
        let json = r#"{"entrypoint": "dist/sidecar.js", "args": ["--verbose"]}"#;
        let config: SidecarConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.runtime, None);
        assert_eq!(config.entrypoint, "dist/sidecar.js");
        assert_eq!(config.args, Some(vec!["--verbose".to_string()]));
    }
}
