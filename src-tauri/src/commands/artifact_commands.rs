use std::path::Path;

use tauri::State;

use crate::domain::artifact::{
    derive_rel_path, infer_artifact_type_from_path, parse_artifact_type, Artifact, ArtifactSummary,
    ArtifactType, DocNode,
};
use crate::domain::artifact_fs::{
    artifact_from_file, list_governance_files, now_iso, scan_directory, write_artifact_file,
};
use crate::error::OrqaError;
use crate::repo::{artifact_repo, project_repo};
use crate::state::AppState;

/// List artifacts for a project, optionally filtered by type.
#[tauri::command]
pub fn artifact_list(
    project_id: i64,
    artifact_type: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let type_filter = artifact_type
        .as_deref()
        .map(parse_artifact_type)
        .transpose()?;

    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    artifact_repo::list(&conn, project_id, type_filter.as_ref())
}

/// Get an artifact by its ID.
#[tauri::command]
pub fn artifact_get(artifact_id: i64, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let mut artifact = artifact_repo::get(&conn, artifact_id)?;

    // Load content from disk if the project path is available
    if let Ok(project) = project_repo::get(&conn, artifact.project_id) {
        let full_path = Path::new(&project.path).join(&artifact.rel_path);
        if full_path.exists() {
            artifact.content = std::fs::read_to_string(&full_path).unwrap_or_default();
        }
    }

    Ok(artifact)
}

/// Get an artifact by project ID and relative path.
#[tauri::command]
pub fn artifact_get_by_path(
    project_id: i64,
    rel_path: String,
    state: State<'_, AppState>,
) -> Result<Artifact, OrqaError> {
    if rel_path.trim().is_empty() {
        return Err(OrqaError::Validation(
            "relative path cannot be empty".to_string(),
        ));
    }

    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let mut artifact = artifact_repo::get_by_path(&conn, project_id, &rel_path)?;

    // Load content from disk
    if let Ok(project) = project_repo::get(&conn, artifact.project_id) {
        let full_path = Path::new(&project.path).join(&artifact.rel_path);
        if full_path.exists() {
            artifact.content = std::fs::read_to_string(&full_path).unwrap_or_default();
        }
    }

    Ok(artifact)
}

/// Create a new artifact, writing the file to disk and indexing in the database.
#[tauri::command]
pub fn artifact_create(
    project_id: i64,
    artifact_type: String,
    name: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<Artifact, OrqaError> {
    if name.trim().is_empty() {
        return Err(OrqaError::Validation(
            "artifact name cannot be empty".to_string(),
        ));
    }

    let parsed_type = parse_artifact_type(&artifact_type)?;
    let rel_path = derive_rel_path(&parsed_type, name.trim());

    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get(&conn, project_id)?;
    let full_path = Path::new(&project.path).join(&rel_path);

    write_artifact_file(&full_path, &content)?;
    artifact_repo::index_artifact(
        &conn,
        project_id,
        &parsed_type,
        &rel_path,
        name.trim(),
        &content,
    )
}

/// Update an artifact's content, writing to disk and updating the database.
#[tauri::command]
pub fn artifact_update(
    artifact_id: i64,
    content: String,
    state: State<'_, AppState>,
) -> Result<Artifact, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    // Get the artifact to find its path
    let artifact = artifact_repo::get(&conn, artifact_id)?;
    let project = project_repo::get(&conn, artifact.project_id)?;
    let full_path = Path::new(&project.path).join(&artifact.rel_path);

    // Write updated content to disk
    std::fs::write(&full_path, &content)?;

    // Update file metadata in DB
    let file_size = content.len() as i64;
    let file_hash = format!("sha256:{:x}", compute_hash(&content));
    let now = now_iso();

    artifact_repo::update(&conn, artifact_id, &file_hash, file_size, &now)?;

    // Update FTS index content
    conn.execute(
        "UPDATE artifacts_fts SET content = ?1 WHERE rowid = ?2",
        rusqlite::params![content, artifact_id],
    )?;

    // Re-fetch and return with content
    let mut updated = artifact_repo::get(&conn, artifact_id)?;
    updated.content = content;

    Ok(updated)
}

/// Delete an artifact, removing the file from disk and the database record.
#[tauri::command]
pub fn artifact_delete(artifact_id: i64, state: State<'_, AppState>) -> Result<(), OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    // Get the artifact to find its path
    let artifact = artifact_repo::get(&conn, artifact_id)?;
    let project = project_repo::get(&conn, artifact.project_id)?;
    let full_path = Path::new(&project.path).join(&artifact.rel_path);

    // Delete file from disk (ignore if already gone)
    if full_path.exists() {
        std::fs::remove_file(&full_path)?;
    }

    // Delete from database (handles FTS cleanup)
    artifact_repo::delete(&conn, artifact_id)
}

/// Read a documentation file directly from the active project's docs/ directory on disk.
#[tauri::command]
pub fn doc_read(rel_path: String, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    use crate::domain::artifact::ComplianceStatus;

    // Security: prevent path traversal
    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let docs_path = Path::new(&project_path)
        .join("docs")
        .join(format!("{}.md", rel_path));

    if !docs_path.exists() {
        return Err(OrqaError::NotFound(format!("doc not found: {}", rel_path)));
    }

    let content = std::fs::read_to_string(&docs_path)?;

    // Extract a display name from the path (last segment, title-cased)
    let name = rel_path
        .split('/')
        .next_back()
        .unwrap_or(&rel_path)
        .replace('-', " ");

    let metadata = std::fs::metadata(&docs_path).ok();
    let file_size = metadata.as_ref().map(|m| m.len() as i64);

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type: ArtifactType::Doc,
        rel_path: format!("docs/{}.md", rel_path),
        name,
        description: None,
        content,
        file_hash: None,
        file_size,
        file_modified_at: None,
        compliance_status: ComplianceStatus::Unknown,
        relationships: None,
        metadata: None,
        created_at: String::new(),
        updated_at: String::new(),
    })
}

/// Scan the active project's `docs/` directory tree and return a hierarchical structure.
///
/// Returns an empty vec if `docs/` does not exist (no error).
#[tauri::command]
pub fn doc_tree_scan(state: State<'_, AppState>) -> Result<Vec<DocNode>, OrqaError> {
    let project_path = active_project_path(&state)?;
    let docs_dir = Path::new(&project_path).join("docs");
    if !docs_dir.is_dir() {
        return Ok(Vec::new());
    }

    scan_directory(&docs_dir, &docs_dir)
}

/// List governance artifacts (agents, rules, skills, hooks) by scanning disk.
///
/// Returns file-based summaries — the database is not consulted.
#[tauri::command]
pub fn governance_list(
    artifact_type: String,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let parsed = parse_artifact_type(&artifact_type)?;
    if parsed == ArtifactType::Doc {
        return Err(OrqaError::Validation(
            "use doc_tree_scan for docs".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let root = Path::new(&project_path);
    list_governance_files(root, &parsed)
}

/// Read a single governance artifact from disk by its relative path.
#[tauri::command]
pub fn governance_read(
    rel_path: String,
    state: State<'_, AppState>,
) -> Result<Artifact, OrqaError> {
    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let full_path = Path::new(&project_path).join(&rel_path);

    if !full_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "artifact not found: {}",
            rel_path
        )));
    }

    let artifact_type = infer_artifact_type_from_path(&rel_path);
    artifact_from_file(&full_path, rel_path, artifact_type)
}

/// Look up the active project's filesystem path from the database.
fn active_project_path(state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?.ok_or_else(|| {
        OrqaError::NotFound("no active project — open a project first".to_string())
    })?;

    Ok(project.path)
}

/// Simple non-cryptographic hash used locally for change detection in `artifact_update`.
fn compute_hash(content: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in content.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(u64::from(byte));
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;
    use crate::repo::project_repo;

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
            ".claude/agents/test.md",
            "test-agent",
            "# Agent",
            None,
        )
        .expect("create");

        let fetched =
            artifact_repo::get_by_path(&conn, 1, ".claude/agents/test.md").expect("get_by_path");
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
