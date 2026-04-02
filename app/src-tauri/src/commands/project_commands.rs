// Tauri IPC commands for project management.
//
// Projects are persisted in the local SQLite database. The daemon is notified
// about the active project root via the settings API so it can load rules,
// workflows, and artifacts. The app is a thin client for project registration.

use std::path::Path;

use tauri::State;

use crate::domain::project::{Project, ProjectSummary};
use crate::error::OrqaError;
use crate::repo::project_repo;
use crate::state::AppState;

/// Open an existing directory as an OrqaStudio project.
///
/// If the directory is already registered, returns the existing project.
/// Otherwise creates a new project record. Walks up the directory tree
/// to find the root containing `.orqa/` so subdirectory paths work correctly.
#[tauri::command]
pub fn project_open(path: String, state: State<'_, AppState>) -> Result<Project, OrqaError> {
    tracing::info!(subsystem = "project", path = %path, "project_open: entry");
    let raw_canonical = validate_directory_path(&path)?;

    // Walk up to find the true project root (directory containing .orqa/).
    // This handles cases where the app opens from a subdirectory like app/.
    let canonical = {
        let p = Path::new(&raw_canonical);
        if p.join(".orqa").exists() {
            raw_canonical.clone()
        } else {
            let mut current = p.to_path_buf();
            let mut found = None;
            while let Some(parent) = current.parent() {
                if parent.join(".orqa").exists() {
                    found = Some(parent.to_string_lossy().to_string());
                    break;
                }
                current = parent.to_path_buf();
            }
            found.unwrap_or(raw_canonical)
        }
    };

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

    tracing::info!(subsystem = "project", project_id = project.id, path = %canonical, "project_open: exit");
    Ok(project)
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
}
