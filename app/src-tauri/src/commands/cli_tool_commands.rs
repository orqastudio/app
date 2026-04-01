use std::time::Instant;

use super::helpers::active_project_path;
use crate::cli_tools::runner::{CliToolResult, CliToolStatus, RegisteredCliTool};
use crate::error::OrqaError;
use crate::state::AppState;

/// Get all registered CLI tools from plugin manifests.
///
/// Reads `plugin-cli-tools.json` from the active project root.
#[tauri::command]
pub fn get_registered_cli_tools(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RegisteredCliTool>, OrqaError> {
    let project_path = active_project_path(&state)?;
    Ok(state
        .cli_tools
        .runner
        .registered_cli_tools(std::path::Path::new(&project_path)))
}

/// Run a plugin-registered CLI tool by plugin name and tool key.
///
/// Spawns the tool as a child process, captures stdout/stderr, and caches
/// the result for status queries.
#[tauri::command]
pub fn run_cli_tool(
    plugin_name: String,
    tool_key: String,
    state: tauri::State<'_, AppState>,
) -> Result<CliToolResult, OrqaError> {
    tracing::info!(
        plugin = %plugin_name,
        tool_key = %tool_key,
        "[cli_tool] run_cli_tool entry"
    );
    let start = Instant::now();

    let project_path = active_project_path(&state)?;
    let project_root = std::path::Path::new(&project_path);

    let tools = state.cli_tools.runner.registered_cli_tools(project_root);
    let tool = tools
        .iter()
        .find(|t| t.plugin == plugin_name && t.key == tool_key)
        .ok_or_else(|| {
            OrqaError::NotFound(format!("CLI tool not found: {plugin_name}:{tool_key}"))
        })?;

    let result = state
        .cli_tools
        .runner
        .run(tool, project_root)
        .map_err(|e| OrqaError::Sidecar(format!("CLI tool execution failed: {e}")))?;

    tracing::info!(
        plugin = %plugin_name,
        tool_key = %tool_key,
        exit_code = result.exit_code,
        elapsed_ms = start.elapsed().as_millis() as u64,
        "[cli_tool] run_cli_tool complete"
    );

    Ok(result)
}

/// Get the status of all registered CLI tools (last run info).
#[tauri::command]
pub fn cli_tool_status(state: tauri::State<'_, AppState>) -> Result<Vec<CliToolStatus>, OrqaError> {
    let project_path = active_project_path(&state)?;
    Ok(state
        .cli_tools
        .runner
        .statuses(std::path::Path::new(&project_path)))
}

#[cfg(test)]
mod tests {
    // CLI tool commands require tauri::State which can't be unit-tested directly.
    // Integration coverage comes from the CliToolRunner unit tests and
    // end-to-end tests via the Tauri IPC layer.
}
