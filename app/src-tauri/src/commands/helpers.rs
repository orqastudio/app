//! Shared helpers for Tauri command modules.

use crate::error::OrqaError;
use crate::repo::project_repo;
use crate::state::AppState;

/// Resolve the active project's filesystem path from the database.
pub fn active_project_path(state: &tauri::State<'_, AppState>) -> Result<String, OrqaError> {
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
