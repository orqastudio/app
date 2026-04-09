// Projects repository for orqa-storage.
//
// Provides async CRUD and query operations over the `projects` table. Projects
// are the top-level container for sessions, messages, and governance artifacts.
// All SQL is expressed as raw statements via SeaORM's ConnectionTrait so that
// repository internals stay isolated from any specific SeaORM query builder API.

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

use orqa_engine_types::types::project::{DetectedStack, Project, ProjectSummary};

use crate::error::StorageError;
use crate::traits::ProjectRepository;

/// Async repository handle for the `projects` table.
///
/// Holds a shared `Arc<DatabaseConnection>` obtained from `Storage::projects()`.
pub struct ProjectRepo {
    pub(crate) db: Arc<DatabaseConnection>,
}

/// Map a SeaORM `QueryResult` row to a `Project` domain value.
///
/// Column positions must match the SELECT order used in every query in this module.
fn map_project(row: &sea_orm::QueryResult) -> Result<Project, StorageError> {
    let detected_stack_json: Option<String> = row
        .try_get("", "detected_stack")
        .map_err(|e| StorageError::Database(e.to_string()))?;
    Ok(Project {
        id: row
            .try_get("", "id")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        name: row
            .try_get("", "name")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        path: row
            .try_get("", "path")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        description: row
            .try_get("", "description")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        detected_stack: detected_stack_json
            .and_then(|s| serde_json::from_str::<DetectedStack>(&s).ok()),
        created_at: row
            .try_get("", "created_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
        updated_at: row
            .try_get("", "updated_at")
            .map_err(|e| StorageError::Database(e.to_string()))?,
    })
}

/// Fetch a project by its integer primary key.
///
/// Used internally after INSERT to return the newly created row.
async fn fetch_by_id(db: &DatabaseConnection, id: i64) -> Result<Project, StorageError> {
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "SELECT id, name, path, description, detected_stack, created_at, updated_at \
             FROM projects WHERE id = ?",
            [id.into()],
        ))
        .await
        .map_err(|e| StorageError::Database(e.to_string()))?
        .ok_or_else(|| StorageError::NotFound(format!("project {id}")))?;
    map_project(&row)
}

#[async_trait::async_trait]
impl ProjectRepository for ProjectRepo {
    /// Create a new project record and return the full row.
    async fn create(
        &self,
        name: &str,
        path: &str,
        description: Option<&str>,
    ) -> Result<Project, StorageError> {
        let desc_val: sea_orm::Value = match description {
            Some(d) => d.into(),
            None => sea_orm::Value::String(None),
        };
        self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO projects (name, path, description) VALUES (?, ?, ?)",
                [name.into(), path.into(), desc_val],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        let row = self
            .db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, name, path, description, detected_stack, created_at, updated_at \
                 FROM projects ORDER BY id DESC LIMIT 1"
                    .to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| {
                StorageError::NotFound("projects table is empty after insert".to_owned())
            })?;
        map_project(&row)
    }

    /// Get a project by its primary key.
    async fn get(&self, id: i64) -> Result<Project, StorageError> {
        fetch_by_id(&self.db, id).await
    }

    /// Get a project by its filesystem path.
    async fn get_by_path(&self, path: &str) -> Result<Project, StorageError> {
        let row = self
            .db
            .query_one_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "SELECT id, name, path, description, detected_stack, created_at, updated_at \
                 FROM projects WHERE path = ?",
                [path.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?
            .ok_or_else(|| StorageError::NotFound(format!("project with path {path}")))?;
        map_project(&row)
    }

    /// Get the most recently updated project, or `None` if no projects exist.
    async fn get_active(&self) -> Result<Option<Project>, StorageError> {
        let row = self
            .db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, name, path, description, detected_stack, created_at, updated_at \
                 FROM projects ORDER BY updated_at DESC, id DESC LIMIT 1"
                    .to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        match row {
            Some(r) => map_project(&r).map(Some),
            None => Ok(None),
        }
    }

    /// List all projects with summary info (session count, artifact count).
    ///
    /// `artifact_count` is always 0 — artifacts are file-based (`.orqa/`) not
    /// stored in SQLite. The field is kept for frontend API compatibility.
    async fn list(&self) -> Result<Vec<ProjectSummary>, StorageError> {
        let rows = self.db
            .query_all_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT p.id, p.name, p.path, p.detected_stack, p.updated_at, \
                        (SELECT COUNT(*) FROM sessions s WHERE s.project_id = p.id) AS session_count, \
                        0 AS artifact_count \
                 FROM projects p \
                 ORDER BY p.updated_at DESC".to_owned(),
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        rows.iter()
            .map(|row| {
                let detected_stack_json: Option<String> = row
                    .try_get("", "detected_stack")
                    .map_err(|e| StorageError::Database(e.to_string()))?;
                Ok(ProjectSummary {
                    id: row
                        .try_get("", "id")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    name: row
                        .try_get("", "name")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    path: row
                        .try_get("", "path")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    detected_stack: detected_stack_json
                        .and_then(|s| serde_json::from_str::<DetectedStack>(&s).ok()),
                    updated_at: row
                        .try_get("", "updated_at")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    session_count: row
                        .try_get("", "session_count")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                    artifact_count: row
                        .try_get("", "artifact_count")
                        .map_err(|e| StorageError::Database(e.to_string()))?,
                })
            })
            .collect()
    }

    /// Touch the `updated_at` timestamp, surfacing the project as most recently active.
    async fn touch_updated_at(&self, id: i64) -> Result<(), StorageError> {
        let result = self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE projects SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?",
                [id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("project {id}")));
        }
        Ok(())
    }

    /// Update the detected technology stack (stored as JSON).
    async fn update_detected_stack(
        &self,
        id: i64,
        stack: &DetectedStack,
    ) -> Result<(), StorageError> {
        let stack_json = serde_json::to_string(stack)?;
        let result = self.db
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "UPDATE projects SET detected_stack = ?, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?",
                [stack_json.into(), id.into()],
            ))
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("project {id}")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ProjectRepository;
    use crate::Storage;

    async fn open() -> Storage {
        Storage::open_in_memory().await.expect("in-memory storage")
    }

    #[tokio::test]
    async fn create_and_get_project() {
        let storage = open().await;
        let project = storage
            .projects()
            .create("forge", "/home/user/forge", Some("A desktop app"))
            .await
            .expect("create");

        assert_eq!(project.name, "forge");
        assert_eq!(project.path, "/home/user/forge");
        assert_eq!(project.description.as_deref(), Some("A desktop app"));
        assert!(project.detected_stack.is_none());

        let fetched = storage.projects().get(project.id).await.expect("get");
        assert_eq!(fetched.name, "forge");
    }

    #[tokio::test]
    async fn get_nonexistent_returns_not_found() {
        let storage = open().await;
        let result = storage.projects().get(999).await;
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[tokio::test]
    async fn get_by_path_works() {
        let storage = open().await;
        storage
            .projects()
            .create("forge", "/home/user/forge", None)
            .await
            .expect("create");
        let project = storage
            .projects()
            .get_by_path("/home/user/forge")
            .await
            .expect("get_by_path");
        assert_eq!(project.name, "forge");
    }

    #[tokio::test]
    async fn get_active_empty_db() {
        let storage = open().await;
        let result = storage.projects().get_active().await.expect("get_active");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn list_projects_with_counts() {
        let storage = open().await;
        let p = storage
            .projects()
            .create("test", "/test", None)
            .await
            .expect("create");

        storage
            .db()
            .execute_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO sessions (project_id, model) VALUES (?, 'auto')",
                [p.id.into()],
            ))
            .await
            .expect("insert session");

        let projects = storage.projects().list().await.expect("list");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].session_count, 1);
        assert_eq!(projects[0].artifact_count, 0);
    }

    #[tokio::test]
    async fn duplicate_path_fails() {
        let storage = open().await;
        storage
            .projects()
            .create("first", "/same/path", None)
            .await
            .expect("create");
        let result = storage
            .projects()
            .create("second", "/same/path", None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn update_detected_stack_works() {
        let storage = open().await;
        let p = storage
            .projects()
            .create("test", "/test", None)
            .await
            .expect("create");

        let stack = DetectedStack {
            languages: vec!["rust".to_owned()],
            frameworks: vec!["tauri".to_owned()],
            package_manager: Some("cargo".to_owned()),
            has_claude_config: true,
            has_design_tokens: false,
        };
        storage
            .projects()
            .update_detected_stack(p.id, &stack)
            .await
            .expect("update");

        let fetched = storage.projects().get(p.id).await.expect("get");
        let ds = fetched.detected_stack.expect("should have stack");
        assert_eq!(ds.languages, vec!["rust"]);
        assert!(ds.has_claude_config);
    }
}
