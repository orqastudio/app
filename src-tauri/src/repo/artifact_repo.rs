use rusqlite::{params, Connection};

use crate::domain::artifact::{Artifact, ArtifactSummary, ArtifactType, ComplianceStatus};
use crate::error::OrqaError;

/// Create a new artifact and return the full record.
pub fn create(
    conn: &Connection,
    project_id: i64,
    artifact_type: &ArtifactType,
    rel_path: &str,
    name: &str,
    content: &str,
    description: Option<&str>,
) -> Result<Artifact, OrqaError> {
    let type_str = serialize_artifact_type(artifact_type);

    conn.execute(
        "INSERT INTO artifacts (project_id, artifact_type, rel_path, name, description, \
         compliance_status) \
         VALUES (?1, ?2, ?3, ?4, ?5, 'unknown')",
        params![project_id, type_str, rel_path, name, description],
    )?;

    let id = conn.last_insert_rowid();

    // Insert into FTS index
    conn.execute(
        "INSERT INTO artifacts_fts(rowid, name, content) VALUES (?1, ?2, ?3)",
        params![id, name, content],
    )?;

    get(conn, id)
}

/// Get an artifact by its primary key.
pub fn get(conn: &Connection, id: i64) -> Result<Artifact, OrqaError> {
    conn.query_row(
        "SELECT id, project_id, artifact_type, rel_path, name, description, \
                file_hash, file_size, file_modified_at, compliance_status, \
                relationships, metadata, created_at, updated_at \
         FROM artifacts WHERE id = ?1",
        params![id],
        |row| {
            let type_str: String = row.get(2)?;
            let status_str: String = row.get(9)?;
            let rels_json: Option<String> = row.get(10)?;
            let meta_json: Option<String> = row.get(11)?;

            Ok(Artifact {
                id: row.get(0)?,
                project_id: row.get(1)?,
                artifact_type: parse_artifact_type(&type_str),
                rel_path: row.get(3)?,
                name: row.get(4)?,
                description: row.get(5)?,
                content: String::new(),
                file_hash: row.get(6)?,
                file_size: row.get(7)?,
                file_modified_at: row.get(8)?,
                compliance_status: parse_compliance_status(&status_str),
                relationships: rels_json.and_then(|j| serde_json::from_str(&j).ok()),
                metadata: meta_json.and_then(|j| serde_json::from_str(&j).ok()),
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => OrqaError::NotFound(format!("artifact {id}")),
        other => OrqaError::Database(other.to_string()),
    })
}

/// Get an artifact by project ID and relative path.
pub fn get_by_path(
    conn: &Connection,
    project_id: i64,
    rel_path: &str,
) -> Result<Artifact, OrqaError> {
    conn.query_row(
        "SELECT id, project_id, artifact_type, rel_path, name, description, \
                file_hash, file_size, file_modified_at, compliance_status, \
                relationships, metadata, created_at, updated_at \
         FROM artifacts WHERE project_id = ?1 AND rel_path = ?2",
        params![project_id, rel_path],
        |row| {
            let type_str: String = row.get(2)?;
            let status_str: String = row.get(9)?;
            let rels_json: Option<String> = row.get(10)?;
            let meta_json: Option<String> = row.get(11)?;

            Ok(Artifact {
                id: row.get(0)?,
                project_id: row.get(1)?,
                artifact_type: parse_artifact_type(&type_str),
                rel_path: row.get(3)?,
                name: row.get(4)?,
                description: row.get(5)?,
                content: String::new(),
                file_hash: row.get(6)?,
                file_size: row.get(7)?,
                file_modified_at: row.get(8)?,
                compliance_status: parse_compliance_status(&status_str),
                relationships: rels_json.and_then(|j| serde_json::from_str(&j).ok()),
                metadata: meta_json.and_then(|j| serde_json::from_str(&j).ok()),
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => OrqaError::NotFound(format!(
            "artifact with path {rel_path} in project {project_id}"
        )),
        other => OrqaError::Database(other.to_string()),
    })
}

/// List artifacts for a project, optionally filtered by type.
pub fn list(
    conn: &Connection,
    project_id: i64,
    type_filter: Option<&ArtifactType>,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let sql = if type_filter.is_some() {
        "SELECT id, artifact_type, rel_path, name, description, \
                compliance_status, file_modified_at \
         FROM artifacts WHERE project_id = ?1 AND artifact_type = ?2 \
         ORDER BY name ASC"
    } else {
        "SELECT id, artifact_type, rel_path, name, description, \
                compliance_status, file_modified_at \
         FROM artifacts WHERE project_id = ?1 \
         ORDER BY artifact_type ASC, name ASC"
    };

    let mut stmt = conn.prepare(sql)?;

    let rows = if let Some(filter) = type_filter {
        let type_str = serialize_artifact_type(filter);
        stmt.query_map(params![project_id, type_str], map_summary)?
    } else {
        stmt.query_map(params![project_id], map_summary)?
    };

    let mut artifacts = Vec::new();
    for row in rows {
        artifacts.push(row?);
    }
    Ok(artifacts)
}

/// Update file metadata for an artifact (after a scan).
pub fn update(
    conn: &Connection,
    id: i64,
    file_hash: &str,
    file_size: i64,
    file_modified_at: &str,
) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE artifacts SET file_hash = ?1, file_size = ?2, file_modified_at = ?3, \
         last_scanned_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), \
         updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ?4",
        params![file_hash, file_size, file_modified_at, id],
    )?;

    if rows == 0 {
        return Err(OrqaError::NotFound(format!("artifact {id}")));
    }
    Ok(())
}

/// Update the FTS index content for an artifact after its body text changes.
pub fn update_fts_content(conn: &Connection, id: i64, content: &str) -> Result<(), OrqaError> {
    conn.execute(
        "UPDATE artifacts_fts SET content = ?1 WHERE rowid = ?2",
        params![content, id],
    )?;
    Ok(())
}

/// Delete an artifact by id.
pub fn delete(conn: &Connection, id: i64) -> Result<(), OrqaError> {
    // Remove from contentless FTS index first
    conn.execute("DELETE FROM artifacts_fts WHERE rowid = ?1", params![id])?;

    let rows = conn.execute("DELETE FROM artifacts WHERE id = ?1", params![id])?;
    if rows == 0 {
        return Err(OrqaError::NotFound(format!("artifact {id}")));
    }
    Ok(())
}

fn serialize_artifact_type(t: &ArtifactType) -> &'static str {
    match t {
        ArtifactType::Agent => "agent",
        ArtifactType::Rule => "rule",
        ArtifactType::Skill => "skill",
        ArtifactType::Hook => "hook",
        ArtifactType::Doc => "doc",
    }
}

fn parse_artifact_type(s: &str) -> ArtifactType {
    match s {
        "agent" => ArtifactType::Agent,
        "rule" => ArtifactType::Rule,
        "skill" => ArtifactType::Skill,
        "hook" => ArtifactType::Hook,
        "doc" => ArtifactType::Doc,
        _ => ArtifactType::Doc,
    }
}

fn parse_compliance_status(s: &str) -> ComplianceStatus {
    match s {
        "compliant" => ComplianceStatus::Compliant,
        "non_compliant" => ComplianceStatus::NonCompliant,
        "unknown" => ComplianceStatus::Unknown,
        "error" => ComplianceStatus::Error,
        _ => ComplianceStatus::Unknown,
    }
}

fn map_summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<ArtifactSummary> {
    let type_str: String = row.get(1)?;
    let status_str: String = row.get(5)?;
    Ok(ArtifactSummary {
        id: row.get(0)?,
        artifact_type: parse_artifact_type(&type_str),
        rel_path: row.get(2)?,
        name: row.get(3)?,
        description: row.get(4)?,
        compliance_status: parse_compliance_status(&status_str),
        file_modified_at: row.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;
    use crate::repo::project_repo;

    fn setup() -> Connection {
        let conn = init_memory_db().expect("db init");
        project_repo::create(&conn, "test", "/test", None).expect("create project");
        conn
    }

    #[test]
    fn create_and_get_artifact() {
        let conn = setup();
        let artifact = create(
            &conn,
            1,
            &ArtifactType::Rule,
            ".claude/rules/no-stubs.md",
            "no-stubs",
            "# No Stubs",
            Some("No stubs or placeholders"),
        )
        .expect("create artifact");

        assert_eq!(artifact.project_id, 1);
        assert_eq!(artifact.artifact_type, ArtifactType::Rule);
        assert_eq!(artifact.name, "no-stubs");
        assert_eq!(artifact.compliance_status, ComplianceStatus::Unknown);

        let fetched = get(&conn, artifact.id).expect("get artifact");
        assert_eq!(fetched.name, "no-stubs");
    }

    #[test]
    fn get_nonexistent_artifact() {
        let conn = setup();
        let result = get(&conn, 999);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn get_by_path_works() {
        let conn = setup();
        create(
            &conn,
            1,
            &ArtifactType::Agent,
            ".claude/agents/backend-engineer.md",
            "backend-engineer",
            "# Backend Engineer",
            None,
        )
        .expect("create");

        let record =
            get_by_path(&conn, 1, ".claude/agents/backend-engineer.md").expect("get_by_path");
        assert_eq!(record.name, "backend-engineer");
    }

    #[test]
    fn get_by_path_not_found() {
        let conn = setup();
        let result = get_by_path(&conn, 1, "nonexistent.md");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn list_all_artifacts() {
        let conn = setup();
        create(&conn, 1, &ArtifactType::Rule, "r1.md", "rule1", "c", None).expect("create");
        create(&conn, 1, &ArtifactType::Agent, "a1.md", "agent1", "c", None).expect("create");
        create(&conn, 1, &ArtifactType::Rule, "r2.md", "rule2", "c", None).expect("create");

        let all = list(&conn, 1, None).expect("list all");
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn list_filtered_by_type() {
        let conn = setup();
        create(&conn, 1, &ArtifactType::Rule, "r1.md", "rule1", "c", None).expect("create");
        create(&conn, 1, &ArtifactType::Agent, "a1.md", "agent1", "c", None).expect("create");
        create(&conn, 1, &ArtifactType::Rule, "r2.md", "rule2", "c", None).expect("create");

        let rules = list(&conn, 1, Some(&ArtifactType::Rule)).expect("list rules");
        assert_eq!(rules.len(), 2);

        let agents = list(&conn, 1, Some(&ArtifactType::Agent)).expect("list agents");
        assert_eq!(agents.len(), 1);
    }

    #[test]
    fn update_file_metadata() {
        let conn = setup();
        let artifact =
            create(&conn, 1, &ArtifactType::Rule, "r.md", "rule", "c", None).expect("create");

        update(
            &conn,
            artifact.id,
            "sha256:abc123",
            1024,
            "2026-03-03T12:00:00Z",
        )
        .expect("update");

        let fetched = get(&conn, artifact.id).expect("get");
        assert_eq!(fetched.file_hash.as_deref(), Some("sha256:abc123"));
        assert_eq!(fetched.file_size, Some(1024));
    }

    #[test]
    fn update_not_found() {
        let conn = setup();
        let result = update(&conn, 999, "hash", 0, "now");
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn delete_artifact() {
        let conn = setup();
        let artifact =
            create(&conn, 1, &ArtifactType::Rule, "r.md", "rule", "c", None).expect("create");
        delete(&conn, artifact.id).expect("delete");

        let result = get(&conn, artifact.id);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn delete_not_found() {
        let conn = setup();
        let result = delete(&conn, 999);
        assert!(matches!(result, Err(OrqaError::NotFound(_))));
    }

    #[test]
    fn unique_path_constraint() {
        let conn = setup();
        create(&conn, 1, &ArtifactType::Rule, "same.md", "first", "c", None).expect("create");
        let result = create(
            &conn,
            1,
            &ArtifactType::Rule,
            "same.md",
            "second",
            "c",
            None,
        );
        assert!(result.is_err(), "duplicate path should fail");
    }

    #[test]
    fn cascade_on_project_delete() {
        let conn = setup();
        create(&conn, 1, &ArtifactType::Rule, "r.md", "rule", "c", None).expect("create");

        conn.execute("DELETE FROM projects WHERE id = 1", [])
            .expect("delete project");

        let all = list(&conn, 1, None).expect("list");
        assert!(all.is_empty(), "artifacts should be cascaded");
    }
}
