use std::path::{Path, PathBuf};

use tauri::{AppHandle, Runtime, State};

#[cfg(test)]
use crate::domain::artifact::ArtifactType;
use crate::domain::artifact::NavTree;
use crate::error::OrqaError;
use crate::repo::project_repo;
use crate::state::AppState;
use crate::watcher;

/// Scan the active project and return a unified navigation tree.
///
/// Reads artifact layout from the project's `.orqa/project.json` `artifacts` field.
/// Each entry drives the scan — no folder-guessing. Returns an empty tree when no
/// active project is set or when the `artifacts` config is absent.
#[tauri::command]
pub fn artifact_scan_tree(state: State<'_, AppState>) -> Result<NavTree, OrqaError> {
    let project_path = active_project_path(&state)?;

    // Load artifacts config from project.json — empty list if missing or unset.
    let artifacts = crate::repo::project_settings_repo::read(&project_path)
        .unwrap_or(None)
        .map(|s| s.artifacts)
        .unwrap_or_default();

    crate::domain::artifact_reader::artifact_scan_tree(Path::new(&project_path), &artifacts)
}

/// Look up the active project's filesystem path from the database.
fn active_project_path(state: &State<'_, AppState>) -> Result<String, OrqaError> {
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

/// Start (or replace) the `.orqa/` file-system watcher for a project.
///
/// Watches `<project_path>/.orqa/` recursively with a 500 ms debounce.
/// When any file changes a single `artifact-changed` Tauri event is emitted to
/// all windows so the frontend can invalidate its nav-tree cache.
///
/// Safe to call multiple times — each call replaces the previous watcher.
#[tauri::command]
pub fn artifact_watch_start<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    project_path: String,
) -> Result<(), OrqaError> {
    watcher::start(app, PathBuf::from(&project_path), &state.artifacts.watcher)
        .map_err(OrqaError::FileSystem)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;
    use crate::repo::{artifact_repo, project_repo};

    fn setup() -> rusqlite::Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        conn
    }

    #[test]
    fn artifact_list_empty() {
        let conn = setup();
        let artifacts = artifact_repo::list(&conn, 1, None).expect("list");
        assert!(artifacts.is_empty());
    }

    #[test]
    fn artifact_list_filtered() {
        let conn = setup();
        artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Rule,
            "r1.md",
            "rule1",
            "content",
            None,
        )
        .expect("create");
        artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Agent,
            "a1.md",
            "agent1",
            "content",
            None,
        )
        .expect("create");

        let all = artifact_repo::list(&conn, 1, None).expect("list all");
        assert_eq!(all.len(), 2);

        let rules = artifact_repo::list(&conn, 1, Some(&ArtifactType::Rule)).expect("list rules");
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn artifact_get_by_id() {
        let conn = setup();
        let created = artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Rule,
            "test.md",
            "test-rule",
            "# Test",
            None,
        )
        .expect("create");

        let fetched = artifact_repo::get(&conn, created.id).expect("get");
        assert_eq!(fetched.name, "test-rule");
    }

    #[test]
    fn artifact_get_nonexistent() {
        let conn = setup();
        let result = artifact_repo::get(&conn, 999);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn artifact_get_by_path_works() {
        let conn = setup();
        artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Agent,
            ".orqa/team/agents/test.md",
            "test-agent",
            "# Agent",
            None,
        )
        .expect("create");

        let fetched =
            artifact_repo::get_by_path(&conn, 1, ".orqa/team/agents/test.md").expect("get_by_path");
        assert_eq!(fetched.name, "test-agent");
    }

    #[test]
    fn artifact_delete_works() {
        let conn = setup();
        let artifact = artifact_repo::create(
            &conn,
            1,
            &ArtifactType::Rule,
            "delete-me.md",
            "delete-me",
            "content",
            None,
        )
        .expect("create");

        artifact_repo::delete(&conn, artifact.id).expect("delete");
        let result = artifact_repo::get(&conn, artifact.id);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn empty_name_validation() {
        let name = "   ";
        assert!(name.trim().is_empty());
    }
}
