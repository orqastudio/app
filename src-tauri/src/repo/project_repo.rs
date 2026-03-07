use rusqlite::{params, Connection, OptionalExtension};

use crate::domain::project::{DetectedStack, Project, ProjectSummary};
use crate::error::OrqaError;

/// Create a new project and return the full `Project` record.
pub fn create(
    conn: &Connection,
    name: &str,
    path: &str,
    description: Option<&str>,
) -> Result<Project, OrqaError> {
    conn.execute(
        "INSERT INTO projects (name, path, description) VALUES (?1, ?2, ?3)",
        params![name, path, description],
    )?;

    let id = conn.last_insert_rowid();
    get(conn, id)
}

/// Get a project by its primary key.
pub fn get(conn: &Connection, id: i64) -> Result<Project, OrqaError> {
    conn.query_row(
        "SELECT id, name, path, description, detected_stack, created_at, updated_at \
         FROM projects WHERE id = ?1",
        params![id],
        |row| {
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
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => OrqaError::NotFound(format!("project {id}")),
        other => OrqaError::Database(other.to_string()),
    })
}

/// Get a project by its filesystem path.
pub fn get_by_path(conn: &Connection, path: &str) -> Result<Project, OrqaError> {
    conn.query_row(
        "SELECT id, name, path, description, detected_stack, created_at, updated_at \
         FROM projects WHERE path = ?1",
        params![path],
        |row| {
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
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            OrqaError::NotFound(format!("project with path {path}"))
        }
        other => OrqaError::Database(other.to_string()),
    })
}

/// Get the most recently updated project, if any.
pub fn get_active(conn: &Connection) -> Result<Option<Project>, OrqaError> {
    conn.query_row(
        "SELECT id, name, path, description, detected_stack, created_at, updated_at \
         FROM projects ORDER BY updated_at DESC, id DESC LIMIT 1",
        [],
        |row| {
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
        },
    )
    .optional()
    .map_err(|e| OrqaError::Database(e.to_string()))
}

/// List all projects with summary info (session + artifact counts).
pub fn list(conn: &Connection) -> Result<Vec<ProjectSummary>, OrqaError> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.name, p.path, p.detected_stack, p.updated_at, \
                (SELECT COUNT(*) FROM sessions s WHERE s.project_id = p.id) AS session_count, \
                (SELECT COUNT(*) FROM artifacts a WHERE a.project_id = p.id) AS artifact_count \
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

    let mut projects = Vec::new();
    for row in rows {
        projects.push(row?);
    }
    Ok(projects)
}

/// Touch the `updated_at` timestamp for a project, surfacing it as the most recently active.
pub fn touch_updated_at(conn: &Connection, id: i64) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE projects SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?1",
        params![id],
    )?;
    if rows == 0 {
        return Err(OrqaError::NotFound(format!("project {id}")));
    }
    Ok(())
}

/// Update the detected stack for a project (serialized as JSON).
pub fn update_detected_stack(
    conn: &Connection,
    id: i64,
    stack: &DetectedStack,
) -> Result<(), OrqaError> {
    let stack_json = serde_json::to_string(stack)?;
    let rows = conn.execute(
        "UPDATE projects SET detected_stack = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?2",
        params![stack_json, id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("project {id}")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;

    #[test]
    fn create_and_get_project() {
        let conn = init_memory_db().expect("db init");
        let project = create(&conn, "forge", "/home/user/forge", Some("A desktop app"))
            .expect("create should succeed");

        assert_eq!(project.name, "forge");
        assert_eq!(project.path, "/home/user/forge");
        assert_eq!(project.description.as_deref(), Some("A desktop app"));
        assert!(project.detected_stack.is_none());

        let fetched = get(&conn, project.id).expect("get should succeed");
        assert_eq!(fetched.name, "forge");
    }

    #[test]
    fn get_nonexistent_returns_not_found() {
        let conn = init_memory_db().expect("db init");
        let result = get(&conn, 999);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn get_by_path_works() {
        let conn = init_memory_db().expect("db init");
        create(&conn, "forge", "/home/user/forge", None).expect("create");

        let project = get_by_path(&conn, "/home/user/forge").expect("get_by_path");
        assert_eq!(project.name, "forge");
    }

    #[test]
    fn get_by_path_not_found() {
        let conn = init_memory_db().expect("db init");
        let result = get_by_path(&conn, "/no/such/path");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn get_active_empty_db() {
        let conn = init_memory_db().expect("db init");
        let result = get_active(&conn).expect("get_active");
        assert!(result.is_none());
    }

    #[test]
    fn get_active_returns_most_recent() {
        let conn = init_memory_db().expect("db init");
        create(&conn, "old", "/old", None).expect("create");
        create(&conn, "new", "/new", None).expect("create");

        let active = get_active(&conn)
            .expect("get_active")
            .expect("should have a project");
        assert_eq!(active.name, "new");
    }

    #[test]
    fn list_projects_with_counts() {
        let conn = init_memory_db().expect("db init");
        let p = create(&conn, "test", "/test", None).expect("create");

        // Add a session to verify counts
        conn.execute(
            "INSERT INTO sessions (project_id, model) VALUES (?1, 'auto')",
            params![p.id],
        )
        .expect("insert session");

        let projects = list(&conn).expect("list");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].session_count, 1);
        assert_eq!(projects[0].artifact_count, 0);
    }

    #[test]
    fn update_detected_stack_works() {
        let conn = init_memory_db().expect("db init");
        let p = create(&conn, "test", "/test", None).expect("create");

        let stack = DetectedStack {
            languages: vec!["rust".to_string()],
            frameworks: vec!["tauri".to_string()],
            package_manager: Some("cargo".to_string()),
            has_claude_config: true,
            has_design_tokens: false,
        };

        update_detected_stack(&conn, p.id, &stack).expect("update");

        let fetched = get(&conn, p.id).expect("get");
        let ds = fetched.detected_stack.expect("should have stack");
        assert_eq!(ds.languages, vec!["rust"]);
        assert!(ds.has_claude_config);
    }

    #[test]
    fn update_detected_stack_not_found() {
        let conn = init_memory_db().expect("db init");
        let stack = DetectedStack {
            languages: vec![],
            frameworks: vec![],
            package_manager: None,
            has_claude_config: false,
            has_design_tokens: false,
        };

        let result = update_detected_stack(&conn, 999, &stack);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn duplicate_path_fails() {
        let conn = init_memory_db().expect("db init");
        create(&conn, "first", "/same/path", None).expect("create");
        let result = create(&conn, "second", "/same/path", None);
        assert!(result.is_err());
    }
}
