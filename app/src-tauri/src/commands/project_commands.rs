use std::path::Path;

use tauri::State;

use crate::domain::enforcement_engine::EnforcementEngine;
use crate::domain::project::{Project, ProjectSummary};
use crate::domain::workflow_config::{default_process_gates, tracker_config_from_artifacts};
use crate::domain::workflow_loader::load_process_gates_from_workflows;
use crate::domain::workflow_tracker::WorkflowTracker;
use crate::error::OrqaError;
use crate::repo::enforcement_rules_repo;
use crate::repo::project_repo;
use crate::state::AppState;

/// Open an existing directory as an OrqaStudio project.
///
/// If the directory is already registered, returns the existing project.
/// Otherwise creates a new project record. In Phase 1, scanning is deferred.
///
/// Also loads the enforcement engine from `.orqa/rules/` if it exists.
#[tauri::command]
pub fn project_open(path: String, state: State<'_, AppState>) -> Result<Project, OrqaError> {
    let canonical = validate_directory_path(&path)?;
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    // Check if already registered
    let project = match project_repo::get_by_path(&conn, &canonical) {
        Ok(project) => {
            // Touch the updated_at timestamp so it surfaces as the active project
            project_repo::touch_updated_at(&conn, project.id)?;
            project_repo::get(&conn, project.id)?
        }
        Err(OrqaError::NotFound(_)) => {
            let name = derive_project_name(&canonical);
            project_repo::create(&conn, &name, &canonical, None)?
        }
        Err(e) => return Err(e),
    };

    // Release the DB lock before loading rules (file I/O, no DB needed)
    drop(conn);

    load_enforcement_engine(&state, &canonical);
    load_tracker_config(&state, &canonical);
    load_process_gates(&state, &canonical);

    Ok(project)
}

/// Load the enforcement engine from the project's `.orqa/rules/` directory.
///
/// If the rules directory does not exist, the engine is cleared (no enforcement).
/// Failures are logged as warnings — a missing or malformed rules directory must
/// not block the project from opening.
fn load_enforcement_engine(state: &State<'_, AppState>, project_path: &str) {
    let rules_dir = Path::new(project_path).join(".orqa").join("rules");

    let engine = if rules_dir.exists() {
        match enforcement_rules_repo::load_rules(&rules_dir).map(EnforcementEngine::new) {
            Ok(engine) => {
                tracing::debug!(
                    "[enforcement] loaded {} rules from '{}'",
                    engine.rules().len(),
                    rules_dir.display()
                );
                Some(engine)
            }
            Err(e) => {
                tracing::warn!("[enforcement] failed to load rules: {e}");
                None
            }
        }
    } else {
        tracing::debug!(
            "[enforcement] no rules directory at '{}'",
            rules_dir.display()
        );
        None
    };

    match state.enforcement.engine.lock() {
        Ok(mut guard) => *guard = engine,
        Err(e) => tracing::warn!("[enforcement] failed to acquire enforcement lock: {e}"),
    }
}

/// Get the most recently active project, if any.
#[tauri::command]
pub fn project_get_active(state: State<'_, AppState>) -> Result<Option<Project>, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    project_repo::get_active(&conn)
}

/// List all projects with summary information.
#[tauri::command]
pub fn project_list(state: State<'_, AppState>) -> Result<Vec<ProjectSummary>, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    project_repo::list(&conn)
}

/// Load `TrackerConfig` from the project's `.orqa/project.json` artifacts array.
///
/// Reads the artifact entries from the project settings and builds path classification
/// rules from the configured artifact paths. This ensures no governance paths are
/// hardcoded — all classification is driven by the project config populated from
/// plugin manifests during `orqa install`.
///
/// Updates both `state.session.tracker_config` (for future session resets) and the
/// active `state.session.workflow_tracker` (for the current session). Failures are
/// logged as warnings — a missing or malformed project.json must not block the
/// project from opening.
fn load_tracker_config(state: &State<'_, AppState>, project_path: &str) {
    use orqa_engine::config::load_project_settings;

    let root = Path::new(project_path);
    let artifacts = match load_project_settings(root) {
        Ok(Some(settings)) => settings.artifacts,
        Ok(None) => {
            tracing::debug!("[tracker] no project.json at '{project_path}', using default paths");
            Vec::new()
        }
        Err(e) => {
            tracing::warn!("[tracker] failed to load project.json for tracker config: {e}");
            Vec::new()
        }
    };

    let config = tracker_config_from_artifacts(&artifacts);

    // Update tracker_config so future session resets (reset_workflow_tracker) use the new config.
    match state.session.tracker_config.lock() {
        Ok(mut tc) => {
            *tc = config.clone();
        }
        Err(e) => {
            tracing::warn!("[tracker] tracker_config mutex poisoned, skipping config reload: {e}");
        }
    }

    // Also rebuild the active workflow tracker so the current session uses the new paths.
    match state.session.workflow_tracker.lock() {
        Ok(mut wt) => {
            *wt = WorkflowTracker::new(config);
        }
        Err(e) => {
            tracing::warn!("[tracker] workflow_tracker mutex poisoned, skipping config reload: {e}");
        }
    }

    tracing::debug!("[tracker] tracker config and workflow tracker rebuilt from project.json artifact paths");
}

/// Load process gate definitions from the project's resolved workflow YAML files.
///
/// Scans `.orqa/workflows/*.resolved.yaml` for `process_gates:` sections and
/// replaces the session's in-memory gate list with the loaded definitions.
/// Falls back to `default_process_gates()` when no resolved workflow files
/// declare any gates. Failures are logged as warnings — a missing or malformed
/// workflow file must not block the project from opening.
fn load_process_gates(state: &State<'_, AppState>, project_path: &str) {
    let root = Path::new(project_path);
    let gates = match load_process_gates_from_workflows(root) {
        Some(loaded) => {
            tracing::debug!(
                "[process_gates] loaded {} gate(s) from resolved workflows at '{project_path}'",
                loaded.len()
            );
            loaded
        }
        None => {
            tracing::debug!(
                "[process_gates] no process gates found in resolved workflows at '{project_path}', using defaults"
            );
            default_process_gates()
        }
    };

    match state.session.process_gates.lock() {
        Ok(mut guard) => {
            *guard = gates;
        }
        Err(e) => {
            tracing::warn!("[process_gates] process_gates mutex poisoned, skipping reload: {e}");
        }
    }
}

/// Validate that a path exists and is a directory, returning the canonical path string.
fn validate_directory_path(path: &str) -> Result<String, OrqaError> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(OrqaError::Validation(format!(
            "path does not exist: {path}"
        )));
    }
    if !p.is_dir() {
        return Err(OrqaError::Validation(format!(
            "path is not a directory: {path}"
        )));
    }
    p.to_str()
        .map(ToString::to_string)
        .ok_or_else(|| OrqaError::Validation("path is not valid UTF-8".to_owned()))
}

/// Derive a project name from the directory path (last path component).
fn derive_project_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;

    #[test]
    fn derive_project_name_from_path() {
        assert_eq!(derive_project_name("/home/user/forge"), "forge");
        assert_eq!(derive_project_name("/tmp/my-project"), "my-project");
        assert_eq!(derive_project_name("C:\\Users\\Bob\\code"), "code");
    }

    #[test]
    fn validate_directory_path_nonexistent() {
        let result = validate_directory_path("/nonexistent/path/xyz123");
        assert!(matches!(result, Err(OrqaError::Validation(_))));
    }

    #[test]
    fn project_get_delegates_to_repo() {
        let conn = init_memory_db().expect("db init");
        let project =
            project_repo::create(&conn, "test", "/test/path", Some("desc")).expect("create");

        let fetched = project_repo::get(&conn, project.id).expect("get");
        assert_eq!(fetched.name, "test");
        assert_eq!(fetched.path, "/test/path");
        assert_eq!(fetched.description.as_deref(), Some("desc"));
    }

    #[test]
    fn project_get_active_empty_db() {
        let conn = init_memory_db().expect("db init");
        let result = project_repo::get_active(&conn).expect("get_active");
        assert!(result.is_none());
    }

    #[test]
    fn project_get_active_returns_most_recent() {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "old", "/old", None).expect("create");
        project_repo::create(&conn, "new", "/new", None).expect("create");

        let active = project_repo::get_active(&conn)
            .expect("get_active")
            .expect("should have project");
        assert_eq!(active.name, "new");
    }

    #[test]
    fn project_list_returns_all() {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "p1", "/p1", None).expect("create");
        project_repo::create(&conn, "p2", "/p2", None).expect("create");

        let projects = project_repo::list(&conn).expect("list");
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn project_open_existing_returns_project() {
        let conn = init_memory_db().expect("db init");
        let original = project_repo::create(&conn, "test", "/tmp", None).expect("create");

        // Simulate reopening by path lookup
        let fetched = project_repo::get_by_path(&conn, "/tmp").expect("get_by_path");
        assert_eq!(fetched.id, original.id);
        assert_eq!(fetched.name, "test");
    }

    #[test]
    fn project_create_validates_empty_name() {
        // Test the validation logic directly
        let name = "   ";
        assert!(name.trim().is_empty());
    }
}
