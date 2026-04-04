// Projects repository for orqa-storage.
//
// Provides CRUD and query operations over the `projects` table. Projects are the
// top-level container for sessions, messages, and governance artifacts. All
// SQL is ported directly from app/src-tauri/src/repo/project_repo.rs.

use rusqlite::{OptionalExtension, params};

use orqa_engine_types::types::project::{DetectedStack, Project, ProjectSummary};

use crate::Storage;
use crate::error::StorageError;

/// Zero-cost repository handle for the `projects` table.
///
/// Borrows `Storage` for its lifetime. Obtain via `Storage::projects()`.
pub struct ProjectRepo<'a> {
    pub(crate) storage: &'a Storage,
}

impl ProjectRepo<'_> {
    /// Create a new project record and return the full row.
    pub fn create(
        &self,
        name: &str,
        path: &str,
        description: Option<&str>,
    ) -> Result<Project, StorageError> {
        let conn = self.storage.conn()?;
        conn.execute(
            "INSERT INTO projects (name, path, description) VALUES (?1, ?2, ?3)",
            params![name, path, description],
        )?;
        let id = conn.last_insert_rowid();
        Self::get_conn(&conn, id)
    }

    /// Get a project by its primary key.
    pub fn get(&self, id: i64) -> Result<Project, StorageError> {
        let conn = self.storage.conn()?;
        Self::get_conn(&conn, id)
    }

    /// Get a project by its filesystem path.
    pub fn get_by_path(&self, path: &str) -> Result<Project, StorageError> {
        let conn = self.storage.conn()?;
        conn.query_row(
            "SELECT id, name, path, description, detected_stack, created_at, updated_at \
             FROM projects WHERE path = ?1",
            params![path],
            map_project,
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                StorageError::NotFound(format!("project with path {path}"))
            }
            other => StorageError::Database(other.to_string()),
        })
    }

    /// Get the most recently updated project, or `None` if no projects exist.
    pub fn get_active(&self) -> Result<Option<Project>, StorageError> {
        let conn = self.storage.conn()?;
        conn.query_row(
            "SELECT id, name, path, description, detected_stack, created_at, updated_at \
             FROM projects ORDER BY updated_at DESC, id DESC LIMIT 1",
            [],
            map_project,
        )
        .optional()
        .map_err(|e| StorageError::Database(e.to_string()))
    }

    /// List all projects with summary info (session count).
    ///
    /// `artifact_count` is always 0 — artifacts are file-based (`.orqa/`) not
    /// stored in SQLite. The field is kept for frontend API compatibility.
    pub fn list(&self) -> Result<Vec<ProjectSummary>, StorageError> {
        let conn = self.storage.conn()?;
        let mut stmt = conn.prepare(
            "SELECT p.id, p.name, p.path, p.detected_stack, p.updated_at, \
                    (SELECT COUNT(*) FROM sessions s WHERE s.project_id = p.id) AS session_count, \
                    0 AS artifact_count \
             FROM projects p \
             ORDER BY p.updated_at DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(ProjectSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                detected_stack: row
                    .get::<_, Option<String>>(3)?
                    .and_then(|s| serde_json::from_str::<DetectedStack>(&s).ok()),
                updated_at: row.get(4)?,
                session_count: row.get(5)?,
                artifact_count: row.get(6)?,
            })
        })?;

        rows.map(|row| row.map_err(|e| StorageError::Database(e.to_string())))
            .collect()
    }

    /// Touch the `updated_at` timestamp for a project, surfacing it as most recently active.
    pub fn touch_updated_at(&self, id: i64) -> Result<(), StorageError> {
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE projects SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
            params![id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("project {id}")));
        }
        Ok(())
    }

    /// Update the detected technology stack for a project (serialized as JSON).
    pub fn update_detected_stack(
        &self,
        id: i64,
        stack: &DetectedStack,
    ) -> Result<(), StorageError> {
        let stack_json = serde_json::to_string(stack)?;
        let conn = self.storage.conn()?;
        let rows = conn.execute(
            "UPDATE projects \
             SET detected_stack = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
             WHERE id = ?2",
            params![stack_json, id],
        )?;
        if rows == 0 {
            return Err(StorageError::NotFound(format!("project {id}")));
        }
        Ok(())
    }

    /// Internal helper: fetch a project by id from an existing open connection.
    fn get_conn(
        conn: &rusqlite::Connection,
        id: i64,
    ) -> Result<Project, StorageError> {
        conn.query_row(
            "SELECT id, name, path, description, detected_stack, created_at, updated_at \
             FROM projects WHERE id = ?1",
            params![id],
            map_project,
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                StorageError::NotFound(format!("project {id}"))
            }
            other => StorageError::Database(other.to_string()),
        })
    }
}

/// Map a SQLite row to a `Project`.
fn map_project(row: &rusqlite::Row<'_>) -> rusqlite::Result<Project> {
    Ok(Project {
        id: row.get(0)?,
        name: row.get(1)?,
        path: row.get(2)?,
        description: row.get(3)?,
        detected_stack: row
            .get::<_, Option<String>>(4)?
            .and_then(|s| serde_json::from_str::<DetectedStack>(&s).ok()),
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Storage;

    fn open_test_storage() -> Storage {
        Storage::open_in_memory().expect("in-memory storage")
    }

    #[test]
    fn create_and_get_project() {
        let storage = open_test_storage();
        let repo = storage.projects();
        let project = repo
            .create("forge", "/home/user/forge", Some("A desktop app"))
            .expect("create");

        assert_eq!(project.name, "forge");
        assert_eq!(project.path, "/home/user/forge");
        assert_eq!(project.description.as_deref(), Some("A desktop app"));
        assert!(project.detected_stack.is_none());

        let fetched = repo.get(project.id).expect("get");
        assert_eq!(fetched.name, "forge");
    }

    #[test]
    fn get_nonexistent_returns_not_found() {
        let storage = open_test_storage();
        let repo = storage.projects();
        let result = repo.get(999);
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[test]
    fn get_by_path_works() {
        let storage = open_test_storage();
        let repo = storage.projects();
        repo.create("forge", "/home/user/forge", None).expect("create");
        let project = repo.get_by_path("/home/user/forge").expect("get_by_path");
        assert_eq!(project.name, "forge");
    }

    #[test]
    fn get_active_empty_db() {
        let storage = open_test_storage();
        let result = storage.projects().get_active().expect("get_active");
        assert!(result.is_none());
    }

    #[test]
    fn list_projects_with_counts() {
        let storage = open_test_storage();
        let p = storage
            .projects()
            .create("test", "/test", None)
            .expect("create");

        let conn = storage.conn().expect("conn");
        conn.execute(
            "INSERT INTO sessions (project_id, model) VALUES (?1, 'auto')",
            params![p.id],
        )
        .expect("insert session");

        let projects = storage.projects().list().expect("list");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].session_count, 1);
        assert_eq!(projects[0].artifact_count, 0);
    }

    #[test]
    fn duplicate_path_fails() {
        let storage = open_test_storage();
        let repo = storage.projects();
        repo.create("first", "/same/path", None).expect("create");
        let result = repo.create("second", "/same/path", None);
        assert!(result.is_err());
    }

    #[test]
    fn update_detected_stack_works() {
        let storage = open_test_storage();
        let repo = storage.projects();
        let p = repo.create("test", "/test", None).expect("create");

        let stack = DetectedStack {
            languages: vec!["rust".to_owned()],
            frameworks: vec!["tauri".to_owned()],
            package_manager: Some("cargo".to_owned()),
            has_claude_config: true,
            has_design_tokens: false,
        };
        repo.update_detected_stack(p.id, &stack).expect("update");

        let fetched = repo.get(p.id).expect("get");
        let ds = fetched.detected_stack.expect("should have stack");
        assert_eq!(ds.languages, vec!["rust"]);
        assert!(ds.has_claude_config);
    }
}
