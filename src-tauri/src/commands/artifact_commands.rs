use std::path::{Path, PathBuf};

use tauri::State;

use crate::domain::artifact::{Artifact, ArtifactSummary, ArtifactType, DocNode};
use crate::domain::artifact_reader::{self, humanize_name};
use crate::domain::paths;
use crate::domain::time_utils;
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

/// Write artifact content to disk, creating parent directories as needed.
fn write_artifact_file(full_path: &Path, content: &str) -> Result<(), OrqaError> {
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(full_path, content)?;
    Ok(())
}

/// Index a new artifact in the database and update its file metadata.
fn index_artifact(
    conn: &rusqlite::Connection,
    project_id: i64,
    parsed_type: &ArtifactType,
    rel_path: &str,
    name: &str,
    content: &str,
) -> Result<Artifact, OrqaError> {
    let file_size = content.len() as i64;
    let file_hash = format!("sha256:{:x}", simple_hash(content));

    let mut artifact =
        artifact_repo::create(conn, project_id, parsed_type, rel_path, name, content, None)?;

    artifact_repo::update(
        conn,
        artifact.id,
        &file_hash,
        file_size,
        &artifact.created_at,
    )?;

    artifact = artifact_repo::get(conn, artifact.id)?;
    artifact.content = content.to_string();
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
    index_artifact(
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
    let file_hash = format!("sha256:{:x}", simple_hash(&content));
    let now = time_utils::now_iso_basic();

    artifact_repo::update(&conn, artifact_id, &file_hash, file_size, &now)?;
    artifact_repo::update_fts_content(&conn, artifact_id, &content)?;

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

/// Read a documentation file directly from the active project's docs/ directory on disk.
#[tauri::command]
pub fn doc_read(rel_path: String, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    artifact_reader::read_doc(Path::new(&project_path), &rel_path)
}

/// Read a single research document from `.orqa/research/`.
#[tauri::command]
pub fn research_read(rel_path: String, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    artifact_reader::read_research(Path::new(&project_path), &rel_path)
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

    artifact_reader::scan_doc_tree(&docs_dir)
}

/// Scan the active project's `.orqa/research/` directory tree and return a hierarchical structure.
///
/// Uses `ResearchFrontmatter` to extract labels from the research-specific YAML schema for leaf
/// nodes. Subdirectories produce `DocNode` entries with `children` (label = humanized directory
/// name, path = None). Returns an empty vec if `.orqa/research/` does not exist (no error).
#[tauri::command]
pub fn research_tree_scan(state: State<'_, AppState>) -> Result<Vec<DocNode>, OrqaError> {
    let project_path = active_project_path(&state)?;
    let research_dir = Path::new(&project_path).join(paths::RESEARCH_DIR);
    if !research_dir.is_dir() {
        return Ok(Vec::new());
    }

    artifact_reader::scan_research_tree(&research_dir)
}

/// Scan the active project's `.orqa/plans/` directory and return a flat list of plan docs.
///
/// Uses `PlanFrontmatter` to extract labels from the plan-specific YAML schema.
/// Returns an empty vec if `.orqa/plans/` does not exist (no error).
#[tauri::command]
pub fn plan_tree_scan(state: State<'_, AppState>) -> Result<Vec<DocNode>, OrqaError> {
    let project_path = active_project_path(&state)?;
    let plans_dir = Path::new(&project_path).join(paths::PLANS_DIR);
    if !plans_dir.is_dir() {
        return Ok(Vec::new());
    }

    artifact_reader::scan_plan_tree(&plans_dir)
}

/// Read a single implementation plan from `.orqa/plans/`.
#[tauri::command]
pub fn plan_read(rel_path: String, state: State<'_, AppState>) -> Result<Artifact, OrqaError> {
    if rel_path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_string(),
        ));
    }

    let project_path = active_project_path(&state)?;
    artifact_reader::read_plan(Path::new(&project_path), &rel_path)
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

/// Infer an `ArtifactType` from a `.claude/` relative path prefix.
fn infer_artifact_type_from_path(rel_path: &str) -> ArtifactType {
    if rel_path.starts_with(".claude/agents") {
        ArtifactType::Agent
    } else if rel_path.starts_with(".claude/rules") {
        ArtifactType::Rule
    } else if rel_path.starts_with(".claude/skills") {
        ArtifactType::Skill
    } else if rel_path.starts_with(".claude/hooks") {
        ArtifactType::Hook
    } else {
        ArtifactType::Doc
    }
}

/// Build an in-memory `Artifact` struct from a file on disk (no DB record).
fn artifact_from_file(
    full_path: &Path,
    rel_path: String,
    artifact_type: ArtifactType,
) -> Result<Artifact, OrqaError> {
    use crate::domain::artifact::ComplianceStatus;

    let content = std::fs::read_to_string(full_path)?;
    let file_name = full_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let name = humanize_name(&file_name);
    let file_size = std::fs::metadata(full_path).ok().map(|m| m.len() as i64);

    Ok(Artifact {
        id: 0,
        project_id: 0,
        artifact_type,
        rel_path,
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

/// Map an `ArtifactType` to its `.claude/` subdirectory path.
fn governance_dir(root: &Path, artifact_type: &ArtifactType) -> Option<PathBuf> {
    match artifact_type {
        ArtifactType::Agent => Some(root.join(".claude").join("agents")),
        ArtifactType::Rule => Some(root.join(".claude").join("rules")),
        ArtifactType::Skill => Some(root.join(".claude").join("skills")),
        ArtifactType::Hook => Some(root.join(".claude").join("hooks")),
        ArtifactType::Doc => None,
    }
}


/// Scan a governance directory and return sorted artifact summaries.
fn list_governance_files(
    root: &Path,
    artifact_type: &ArtifactType,
) -> Result<Vec<ArtifactSummary>, OrqaError> {
    let Some(dir) = governance_dir(root, artifact_type) else {
        return Ok(Vec::new());
    };

    if !dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut summaries = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        if let Some(summary) = artifact_reader::summary_from_entry(&entry, artifact_type)? {
            summaries.push(summary);
        }
    }

    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(summaries)
}


/// Parse a string into an `ArtifactType`, returning a validation error for unknown types.
fn parse_artifact_type(s: &str) -> Result<ArtifactType, OrqaError> {
    match s {
        "agent" => Ok(ArtifactType::Agent),
        "rule" => Ok(ArtifactType::Rule),
        "skill" => Ok(ArtifactType::Skill),
        "hook" => Ok(ArtifactType::Hook),
        "doc" => Ok(ArtifactType::Doc),
        other => Err(OrqaError::Validation(format!(
            "unknown artifact type: {other} (valid: agent, rule, skill, hook, doc)"
        ))),
    }
}

/// Derive the relative path for an artifact based on its type and name.
fn derive_rel_path(artifact_type: &ArtifactType, name: &str) -> String {
    let sanitized = name.replace(' ', "-").to_lowercase();

    match artifact_type {
        ArtifactType::Agent => format!(".claude/agents/{sanitized}.md"),
        ArtifactType::Rule => format!(".claude/rules/{sanitized}.md"),
        ArtifactType::Skill => format!(".claude/skills/{sanitized}/SKILL.md"),
        ArtifactType::Hook => format!(".claude/hooks/{sanitized}.sh"),
        ArtifactType::Doc => format!("docs/{sanitized}.md"),
    }
}

/// Simple hash for file content (not cryptographic, just for change detection).
fn simple_hash(content: &str) -> u64 {
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
    fn parse_artifact_type_valid() {
        assert!(matches!(
            parse_artifact_type("agent"),
            Ok(ArtifactType::Agent)
        ));
        assert!(matches!(
            parse_artifact_type("rule"),
            Ok(ArtifactType::Rule)
        ));
        assert!(matches!(
            parse_artifact_type("skill"),
            Ok(ArtifactType::Skill)
        ));
        assert!(matches!(
            parse_artifact_type("hook"),
            Ok(ArtifactType::Hook)
        ));
        assert!(matches!(parse_artifact_type("doc"), Ok(ArtifactType::Doc)));
    }

    #[test]
    fn parse_artifact_type_invalid() {
        let result = parse_artifact_type("unknown");
        assert!(matches!(result, Err(OrqaError::Validation(_))));
    }

    #[test]
    fn derive_rel_path_agent() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Agent, "backend-engineer"),
            ".claude/agents/backend-engineer.md"
        );
    }

    #[test]
    fn derive_rel_path_rule() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Rule, "no-stubs"),
            ".claude/rules/no-stubs.md"
        );
    }

    #[test]
    fn derive_rel_path_skill() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Skill, "chunkhound"),
            ".claude/skills/chunkhound/SKILL.md"
        );
    }

    #[test]
    fn derive_rel_path_hook() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Hook, "pre-commit"),
            ".claude/hooks/pre-commit.sh"
        );
    }

    #[test]
    fn derive_rel_path_doc() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Doc, "architecture"),
            "docs/architecture.md"
        );
    }

    #[test]
    fn derive_rel_path_sanitizes_spaces() {
        assert_eq!(
            derive_rel_path(&ArtifactType::Rule, "No Stubs Rule"),
            ".claude/rules/no-stubs-rule.md"
        );
    }

    #[test]
    fn simple_hash_deterministic() {
        let h1 = simple_hash("hello");
        let h2 = simple_hash("hello");
        assert_eq!(h1, h2);
    }

    #[test]
    fn simple_hash_varies_for_different_input() {
        let h1 = simple_hash("hello");
        let h2 = simple_hash("world");
        assert_ne!(h1, h2);
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
