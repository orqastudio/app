use std::path::Path;

use tauri::State;

use crate::domain::enforcement::EnforcementRule;
use crate::domain::enforcement_engine::EnforcementEngine;
use crate::error::OrqaError;
use crate::repo::enforcement_rules_repo;
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
        .engine
        .lock()
        .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;

    match guard.as_ref() {
        Some(engine) => Ok(engine.rules().to_vec()),
        None => Ok(Vec::new()),
    }
}

/// Reload the enforcement engine from the active project's `.orqa/rules/` directory.
///
/// Returns the number of rules loaded. Use this when rule files have been edited
/// and you want the engine to pick up the changes without restarting the app.
#[tauri::command]
pub fn enforcement_rules_reload(state: State<'_, AppState>) -> Result<usize, OrqaError> {
    let project_path = resolve_active_project_path(&state)?;
    let rules_dir = Path::new(&project_path).join(".orqa").join("rules");

    if !rules_dir.exists() {
        let mut guard = state
            .enforcement
            .engine
            .lock()
            .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;
        *guard = None;
        return Ok(0);
    }

    let rules = enforcement_rules_repo::load_rules(&rules_dir)?;
    let engine = EnforcementEngine::new(rules);
    let count = engine.rules().len();

    let mut guard = state
        .enforcement
        .engine
        .lock()
        .map_err(|e| OrqaError::Database(format!("enforcement lock poisoned: {e}")))?;
    *guard = Some(engine);

    tracing::debug!(
        "[enforcement] reloaded {count} rules from '{}'",
        rules_dir.display()
    );
    Ok(count)
}

/// Resolve the active project's path from the database.
fn resolve_active_project_path(state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("db lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_string()))?;

    Ok(project.path)
}
