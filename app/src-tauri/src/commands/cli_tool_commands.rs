// Tauri IPC commands for CLI tool management.
//
// All CLI tool operations are delegated to the daemon via HTTP. The daemon
// owns the CLI tool registry and executes tools in-process. The app is a
// thin client.
//
// Endpoints used:
//   GET  /cli-tools                        — list registered CLI tools
//   GET  /cli-tools/status                 — get last-run status for all tools
//   POST /cli-tools/:plugin/:key/run       — execute a CLI tool

use tauri::State;

use crate::daemon_client::{CliToolResult, CliToolStatus, RegisteredCliTool};
use crate::error::OrqaError;
use crate::state::AppState;

/// Get all registered CLI tools from plugin manifests.
#[tauri::command]
pub async fn get_registered_cli_tools(
    state: State<'_, AppState>,
) -> Result<Vec<RegisteredCliTool>, OrqaError> {
    state.daemon.client.list_cli_tools().await
}

/// Run a plugin-registered CLI tool by plugin name and tool key.
///
/// Delegates execution to the daemon, which spawns the tool as a child process,
/// captures output, and returns the result.
#[tauri::command]
pub async fn run_cli_tool(
    plugin_name: String,
    tool_key: String,
    state: State<'_, AppState>,
) -> Result<CliToolResult, OrqaError> {
    tracing::info!(
        plugin = %plugin_name,
        tool_key = %tool_key,
        "[cli_tool] run_cli_tool: delegating to daemon"
    );
    state
        .daemon
        .client
        .run_cli_tool(&plugin_name, &tool_key)
        .await
}

/// Get the status of all registered CLI tools (last run info).
#[tauri::command]
pub async fn cli_tool_status(state: State<'_, AppState>) -> Result<Vec<CliToolStatus>, OrqaError> {
    state.daemon.client.get_cli_tool_status().await
}

#[cfg(test)]
mod tests {
    // CLI tool commands require daemon connectivity. Covered by integration tests.
}
