// Tauri IPC commands for project management.
//
// Projects are persisted in the daemon's database. The app is a thin HTTP client
// for project registration — it never opens the database directly. All storage
// operations go through libs/db which proxies to the daemon REST API.

use std::path::Path;

use tauri::State;

use crate::domain::project::{Project, ProjectSummary};
use crate::error::OrqaError;
use crate::state::AppState;

/// Open an existing directory as an OrqaStudio project.
///
/// Resolves the canonical project root (directory containing `.orqa/`),
/// registers or touches the project record in the daemon, and returns the
/// `Project` record. No database is opened in the app process.
#[tauri::command]
pub async fn project_open(path: String, state: State<'_, AppState>) -> Result<Project, OrqaError> {
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

    let projects = state.db.client.projects();

    // Register or touch the project record in the daemon.
    let project = match projects.get_by_path(&canonical).await {
        Ok(project) => {
            projects.touch_updated_at(project.id).await?;
            projects.get(project.id).await?
        }
        Err(orqa_db::DbError::Http { status: 404, .. }) => {
            let name = derive_project_name(&canonical);
            projects.create(&name, &canonical, None).await?
        }
        Err(e) => return Err(OrqaError::Database(e.to_string())),
    };

    tracing::info!(subsystem = "project", project_id = project.id, path = %canonical, "project_open: exit");
    Ok(project)
}

/// Get the most recently active project, if any.
#[tauri::command]
pub async fn project_get_active(state: State<'_, AppState>) -> Result<Option<Project>, OrqaError> {
    Ok(state.db.client.projects().get_active().await?)
}

/// List all projects with summary information.
#[tauri::command]
pub async fn project_list(state: State<'_, AppState>) -> Result<Vec<ProjectSummary>, OrqaError> {
    Ok(state.db.client.projects().list().await?)
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
}
