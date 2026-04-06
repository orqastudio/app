// Tauri IPC commands for plugin management.
//
// All plugin operations are delegated to the daemon via HTTP. The daemon
// manages the plugin filesystem, registry, and install/uninstall operations.
// The app is a thin client.
//
// Endpoints used:
//   GET    /plugins                  — list installed plugins
//   GET    /plugins/registry         — list registry catalog
//   GET    /plugins/updates          — check for updates
//   POST   /plugins/install/local    — install from local path
//   POST   /plugins/install/github   — install from GitHub
//   DELETE /plugins/:name            — uninstall
//   GET    /plugins/:name/path       — get plugin filesystem path
//   GET    /plugins/:name            — get plugin manifest

use tauri::State;

use crate::error::OrqaError;
use crate::state::AppState;

/// List all installed plugins discovered from the plugins/ directory.
#[tauri::command]
pub async fn plugin_list_installed(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, OrqaError> {
    state.daemon.client.list_plugins().await
}

/// Fetch the plugin registry catalog.
#[tauri::command]
pub async fn plugin_registry_list(
    _source: Option<String>,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, OrqaError> {
    state.daemon.client.list_plugin_registry().await
}

/// Install a plugin from a local path.
#[tauri::command]
pub async fn plugin_install_local(
    path: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, OrqaError> {
    tracing::info!(subsystem = "plugin", source = "local", path = %path, "plugin_install_local: delegating to daemon");
    state.daemon.client.install_plugin_local(&path).await
}

/// Install a plugin from a GitHub release archive.
#[tauri::command]
pub async fn plugin_install_github(
    repo: String,
    version: Option<String>,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, OrqaError> {
    tracing::info!(subsystem = "plugin", source = "github", repo = %repo, "plugin_install_github: delegating to daemon");
    state
        .daemon
        .client
        .install_plugin_github(&repo, version.as_deref())
        .await
}

/// Uninstall a plugin by name.
#[tauri::command]
pub async fn plugin_uninstall(name: String, state: State<'_, AppState>) -> Result<(), OrqaError> {
    tracing::info!(subsystem = "plugin", plugin_name = %name, "plugin_uninstall: delegating to daemon");
    state.daemon.client.uninstall_plugin(&name).await
}

/// Check for available plugin updates.
#[tauri::command]
pub async fn plugin_check_updates(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, OrqaError> {
    state.daemon.client.check_plugin_updates().await
}

/// Get the filesystem path for an installed plugin.
///
/// Used by the frontend to load plugin view bundles at runtime.
#[tauri::command]
pub async fn plugin_get_path(
    name: String,
    state: State<'_, AppState>,
) -> Result<String, OrqaError> {
    let response = state.daemon.client.get_plugin_path(&name).await?;
    response
        .get("path")
        .and_then(|v| v.as_str())
        .map(str::to_owned)
        .ok_or_else(|| OrqaError::Sidecar("daemon response missing 'path' field".to_owned()))
}

/// Read the plugin manifest for a specific installed plugin.
///
/// Returns the raw JSON from `orqa-plugin.json` without type-specific parsing
/// to preserve all fields the frontend needs.
#[tauri::command]
pub async fn plugin_get_manifest(
    name: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, OrqaError> {
    state.daemon.client.get_plugin(&name).await
}

#[cfg(test)]
mod tests {
    // Plugin commands require daemon connectivity. Covered by integration tests.
}
