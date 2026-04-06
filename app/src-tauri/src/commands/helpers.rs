//! Shared helpers for Tauri command modules.

use crate::error::OrqaError;
use crate::state::AppState;

/// Resolve the active project's filesystem path from the database.
///
/// Reads the stored path from the database, then validates it is the true
/// project root by confirming a `.orqa/` directory is present. If it is not,
/// walks up the directory tree until `.orqa/` is found. This handles cases
/// where a subdirectory (e.g. `app/`) was stored instead of the project root.
/// Falls back to the stored path when no `.orqa/` ancestor can be found.
pub fn active_project_path(state: &tauri::State<'_, AppState>) -> Result<String, OrqaError> {
    let storage = state.db.get()?;
    let project = storage.projects().get_active()?.ok_or_else(|| {
        OrqaError::NotFound("no active project — open a project first".to_owned())
    })?;

    let path = project.path;

    // Validate that the stored path is the true project root by checking for
    // `.orqa/`. If the check passes, return immediately.
    let p = std::path::Path::new(&path);
    if p.join(".orqa").exists() {
        return Ok(path);
    }

    // Walk up the directory tree to find the nearest ancestor that contains
    // `.orqa/`. This recovers from cases where a subdirectory was stored.
    let mut current = p.to_path_buf();
    while let Some(parent) = current.parent() {
        if parent.join(".orqa").exists() {
            tracing::warn!(
                stored = %path,
                resolved = %parent.display(),
                "stored project path had no .orqa/ — resolved to parent"
            );
            return Ok(parent.to_string_lossy().to_string());
        }
        current = parent.to_path_buf();
    }

    // No `.orqa/` ancestor found — return the stored path as-is and let
    // callers surface the error when they try to use it.
    tracing::warn!(
        path = %path,
        "stored project path has no .orqa/ directory — using stored path as fallback"
    );
    Ok(path)
}
