use crate::error::OrqaError;
use crate::repo::project_repo;
use crate::state::AppState;
use crate::tools::runner::{RegisteredTool, ToolResult, ToolStatus};

/// Resolve the active project's filesystem path from the database.
fn active_project_path(state: &tauri::State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?.ok_or_else(|| {
        OrqaError::NotFound("no active project — open a project first".to_string())
    })?;

    Ok(project.path)
}

/// Get all registered tools from plugin manifests.
///
/// Reads `plugin-tools.json` from the active project root.
#[tauri::command]
pub fn get_registered_tools(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RegisteredTool>, OrqaError> {
    let project_path = active_project_path(&state)?;
    Ok(state
        .tools
        .runner
        .registered_tools(std::path::Path::new(&project_path)))
}

/// Run a plugin-registered tool by plugin name and tool key.
///
/// Spawns the tool as a child process, captures stdout/stderr, and caches
/// the result for status queries.
#[tauri::command]
pub fn run_tool(
    plugin_name: String,
    tool_key: String,
    state: tauri::State<'_, AppState>,
) -> Result<ToolResult, OrqaError> {
    let project_path = active_project_path(&state)?;
    let project_root = std::path::Path::new(&project_path);

    let tools = state.tools.runner.registered_tools(project_root);
    let tool = tools
        .iter()
        .find(|t| t.plugin == plugin_name && t.key == tool_key)
        .ok_or_else(|| {
            OrqaError::NotFound(format!(
                "tool not found: {plugin_name}:{tool_key}"
            ))
        })?;

    state
        .tools
        .runner
        .run(tool, project_root)
        .map_err(|e| OrqaError::Sidecar(format!("tool execution failed: {e}")))
}

/// Get the status of all registered tools (last run info).
#[tauri::command]
pub fn tool_status(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ToolStatus>, OrqaError> {
    let project_path = active_project_path(&state)?;
    Ok(state
        .tools
        .runner
        .statuses(std::path::Path::new(&project_path)))
}

#[cfg(test)]
mod tests {
    // Tool commands require tauri::State which can't be unit-tested directly.
    // Integration coverage comes from the ToolRunner unit tests and
    // end-to-end tests via the Tauri IPC layer.
}
