// Tauri IPC commands for project management.
//
// Projects are persisted in the project-scoped SQLite database. The daemon is
// notified about the active project root via the settings API so it can load
// rules, workflows, and artifacts. The app is a thin client for project registration.

use std::path::Path;
use std::sync::Arc;

use tauri::State;

use crate::db::open_project_storage;
use crate::domain::project::{Project, ProjectSummary};
use crate::error::OrqaError;
use crate::state::AppState;

/// Open an existing directory as an OrqaStudio project.
///
/// Resolves the canonical project root (directory containing `.orqa/`),
/// opens project-scoped storage at that root, registers or touches the project
/// record, and stores the storage in `AppState`. Returns the `Project` record.
#[tauri::command]
pub fn project_open(path: String, state: State<'_, AppState>) -> Result<Project, OrqaError> {
    tracing::info!(subsystem = "project", path = %path, "project_open: entry");
    let raw_canonical = validate_directory_path(&path)?;

    // Walk up to find the true project root (directory containing .orqa/).
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

    // Open (or create) project-scoped storage at the project root.
    let storage = open_project_storage(Path::new(&canonical))?;

    // Register or touch the project record.
    let projects = storage.projects();
    let project = match projects.get_by_path(&canonical) {
        Ok(project) => {
            projects.touch_updated_at(project.id)?;
            projects.get(project.id)?
        }
        Err(orqa_storage::StorageError::NotFound(_)) => {
            let name = derive_project_name(&canonical);
            projects.create(&name, &canonical, None)?
        }
        Err(e) => return Err(OrqaError::Database(e.to_string())),
    };

    // Swap the active storage in AppState.
    state.db.set(Arc::clone(&storage))?;

    tracing::info!(subsystem = "project", project_id = project.id, path = %canonical, "project_open: exit");
    Ok(project)
}

/// Get the most recently active project, if any.
#[tauri::command]
pub fn project_get_active(state: State<'_, AppState>) -> Result<Option<Project>, OrqaError> {
    let storage = state.db.get()?;
    Ok(storage.projects().get_active()?)
}

/// List all projects with summary information.
#[tauri::command]
pub fn project_list(state: State<'_, AppState>) -> Result<Vec<ProjectSummary>, OrqaError> {
    let storage = state.db.get()?;
    Ok(storage.projects().list()?)
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
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        let project = storage
            .projects()
            .create("test", "/test/path", Some("desc"))
            .expect("create");

        let fetched = storage.projects().get(project.id).expect("get");
        assert_eq!(fetched.name, "test");
        assert_eq!(fetched.path, "/test/path");
        assert_eq!(fetched.description.as_deref(), Some("desc"));
    }

    #[test]
    fn project_get_active_empty_db() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        let result = storage.projects().get_active().expect("get_active");
        assert!(result.is_none());
    }

    #[test]
    fn project_get_active_returns_most_recent() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        storage
            .projects()
            .create("old", "/old", None)
            .expect("create");
        storage
            .projects()
            .create("new", "/new", None)
            .expect("create");

        let active = storage
            .projects()
            .get_active()
            .expect("get_active")
            .expect("should have project");
        assert_eq!(active.name, "new");
    }

    #[test]
    fn project_list_returns_all() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        storage
            .projects()
            .create("p1", "/p1", None)
            .expect("create");
        storage
            .projects()
            .create("p2", "/p2", None)
            .expect("create");

        let projects = storage.projects().list().expect("list");
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn project_open_existing_returns_project() {
        let storage = orqa_storage::Storage::open_in_memory().expect("db init");
        let original = storage
            .projects()
            .create("test", "/tmp", None)
            .expect("create");

        let fetched = storage.projects().get_by_path("/tmp").expect("get_by_path");
        assert_eq!(fetched.id, original.id);
        assert_eq!(fetched.name, "test");
    }
}
