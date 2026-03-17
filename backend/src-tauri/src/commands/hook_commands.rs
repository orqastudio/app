use crate::error::OrqaError;
use crate::hooks::manager::{self, HookGenerationResult, RegisteredHook};
use crate::repo::project_repo;
use crate::state::AppState;

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

/// Get all registered hooks from plugin manifests.
///
/// Reads `plugin-hooks.json` from the active project root.
#[tauri::command]
pub fn get_registered_hooks(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RegisteredHook>, OrqaError> {
    let project_path = active_project_path(&state)?;
    Ok(manager::read_hook_registry(std::path::Path::new(
        &project_path,
    )))
}

/// Regenerate git hook dispatcher scripts from plugin registrations.
///
/// Reads all hook registrations, groups by event, and writes thin
/// dispatcher scripts to `.githooks/`. Existing non-generated hooks
/// are preserved as `{event}.legacy`.
#[tauri::command]
pub fn generate_hook_dispatchers(
    state: tauri::State<'_, AppState>,
) -> Result<HookGenerationResult, OrqaError> {
    let project_path = active_project_path(&state)?;
    manager::generate_dispatchers(std::path::Path::new(&project_path))
        .map_err(|e| OrqaError::FileSystem(format!("hook generation failed: {e}")))
}
