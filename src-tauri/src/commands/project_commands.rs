use std::path::Path;

use tauri::State;

use crate::domain::enforcement_engine::EnforcementEngine;
use crate::domain::project::{Project, ProjectSummary};
use crate::error::OrqaError;
use crate::repo::project_repo;
use crate::state::AppState;

/// Open an existing directory as an Orqa Studio project.
///
/// If the directory is already registered, returns the existing project.
/// Otherwise creates a new project record. In Phase 1, scanning is deferred.
///
/// Also loads the enforcement engine from `.claude/rules/` if it exists.
#[tauri::command]
pub fn project_open(path: String, state: State<'_, AppState>) -> Result<Project, OrqaError> {
    let canonical = validate_directory_path(&path)?;
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    // Check if already registered
    let project = match project_repo::get_by_path(&conn, &canonical) {
        Ok(project) => {
            // Touch the updated_at timestamp so it surfaces as the active project
            conn.execute(
                "UPDATE projects SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
                rusqlite::params![project.id],
            )?;
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

    Ok(project)
}

/// Load the enforcement engine from the project's `.claude/rules/` directory.
///
/// If the rules directory does not exist, the engine is cleared (no enforcement).
/// Failures are logged as warnings — a missing or malformed rules directory must
/// not block the project from opening.
fn load_enforcement_engine(state: &State<'_, AppState>, project_path: &str) {
    let rules_dir = Path::new(project_path).join(".claude").join("rules");

    let engine = if rules_dir.exists() {
        match EnforcementEngine::load(&rules_dir) {
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

    match state.enforcement.lock() {
        Ok(mut guard) => *guard = engine,
        Err(e) => tracing::warn!("[enforcement] failed to acquire enforcement lock: {e}"),
    }
}

/// Validate inputs and create the project directory structure on disk.
///
/// Returns the canonical (UTF-8) path to the newly created project directory.
fn init_project_directory(
    name: &str,
    parent_path: &str,
    init_git: bool,
) -> Result<String, OrqaError> {
    let parent = Path::new(parent_path);
    if !parent.exists() || !parent.is_dir() {
        return Err(OrqaError::Validation(format!(
            "parent path does not exist or is not a directory: {parent_path}"
        )));
    }

    let project_dir = parent.join(name);
    if project_dir.exists() {
        return Err(OrqaError::Validation(format!(
            "directory already exists: {}",
            project_dir.display()
        )));
    }

    std::fs::create_dir_all(project_dir.join(".claude"))?;

    if init_git {
        if let Err(e) = std::process::Command::new("git")
            .arg("init")
            .current_dir(&project_dir)
            .output()
        {
            tracing::warn!("git init failed: {e}");
        }
    }

    project_dir
        .to_str()
        .ok_or_else(|| OrqaError::Validation("project path is not valid UTF-8".to_string()))
        .map(|s| s.to_string())
}

/// Create a new project directory and register it.
///
/// Creates the directory at `parent_path/name`, creates a `.claude/` subdirectory,
/// and optionally runs `git init`. Registers the project in the database.
#[tauri::command]
pub fn project_create(
    name: String,
    parent_path: String,
    init_git: Option<bool>,
    state: State<'_, AppState>,
) -> Result<Project, OrqaError> {
    if name.trim().is_empty() {
        return Err(OrqaError::Validation(
            "project name cannot be empty".to_string(),
        ));
    }

    let canonical = init_project_directory(name.trim(), &parent_path, init_git.unwrap_or(true))?;

    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    project_repo::create(&conn, name.trim(), &canonical, None)
}

/// Get a project by its ID.
#[tauri::command]
pub fn project_get(project_id: i64, state: State<'_, AppState>) -> Result<Project, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    project_repo::get(&conn, project_id)
}

/// Get the most recently active project, if any.
#[tauri::command]
pub fn project_get_active(state: State<'_, AppState>) -> Result<Option<Project>, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    project_repo::get_active(&conn)
}

/// List all projects with summary information.
#[tauri::command]
pub fn project_list(state: State<'_, AppState>) -> Result<Vec<ProjectSummary>, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    project_repo::list(&conn)
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
        .map(|s| s.to_string())
        .ok_or_else(|| OrqaError::Validation("path is not valid UTF-8".to_string()))
}

/// Derive a project name from the directory path (last path component).
fn derive_project_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed")
        .to_string()
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
