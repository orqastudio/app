use std::path::Path;

use tauri::State;

use crate::domain::enforcement::{EnforcementRule, ScanFinding};
use crate::domain::enforcement_engine::EnforcementEngine;
use crate::error::OrqaError;
use crate::repo::project_repo;
use crate::state::AppState;

/// List the enforcement rules currently loaded for the active project.
///
/// Returns the full list of parsed rules including their enforcement entries.
/// Rules without YAML frontmatter are included with empty `entries`.
#[tauri::command]
pub fn enforcement_rules_list(
    state: State<'_, AppState>,
) -> Result<Vec<EnforcementRule>, OrqaError> {
    let guard = state
        .enforcement
        .lock()
        .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;

    match guard.as_ref() {
        Some(engine) => Ok(engine.rules().to_vec()),
        None => Ok(Vec::new()),
    }
}

/// Reload the enforcement engine from the active project's `.claude/rules/` directory.
///
/// Returns the number of rules loaded. Use this when rule files have been edited
/// and you want the engine to pick up the changes without restarting the app.
#[tauri::command]
pub fn enforcement_rules_reload(state: State<'_, AppState>) -> Result<usize, OrqaError> {
    let project_path = resolve_active_project_path(&state)?;
    let rules_dir = Path::new(&project_path).join(".claude").join("rules");

    if !rules_dir.exists() {
        let mut guard = state
            .enforcement
            .lock()
            .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;
        *guard = None;
        return Ok(0);
    }

    let engine = EnforcementEngine::load(&rules_dir)?;
    let count = engine.rules().len();

    let mut guard = state
        .enforcement
        .lock()
        .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;
    *guard = Some(engine);

    tracing::debug!(
        "[enforcement] reloaded {count} rules from '{}'",
        rules_dir.display()
    );
    Ok(count)
}

/// Scan the active project's governance files for rule violations.
///
/// Runs all `event: scan` enforcement entries against the project files matching
/// each entry's `scope` glob. Returns a flat list of findings across all scan
/// entries and all matching files.
///
/// Returns an empty list if no project is active or the enforcement engine has
/// not been loaded yet.
#[tauri::command]
pub fn enforcement_scan_governance(
    state: State<'_, AppState>,
) -> Result<Vec<ScanFinding>, OrqaError> {
    let project_path = resolve_active_project_path(&state)?;

    let guard = state
        .enforcement
        .lock()
        .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;

    match guard.as_ref() {
        None => Ok(Vec::new()),
        Some(engine) => engine.scan(Path::new(&project_path)),
    }
}

/// Resolve the active project's path from the database.
fn resolve_active_project_path(state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("db lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_string()))?;

    Ok(project.path)
}
