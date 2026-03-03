use std::path::Path;

use tauri::State;

use crate::domain::artifact::{Artifact, ArtifactSummary, ArtifactType};
use crate::error::ForgeError;
use crate::repo::{artifact_repo, project_repo};
use crate::state::AppState;

/// List artifacts for a project, optionally filtered by type.
#[tauri::command]
pub fn artifact_list(
    project_id: i64,
    artifact_type: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactSummary>, ForgeError> {
    let type_filter = artifact_type
        .as_deref()
        .map(parse_artifact_type)
        .transpose()?;

    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

    artifact_repo::list(&conn, project_id, type_filter.as_ref())
}

/// Get an artifact by its ID.
#[tauri::command]
pub fn artifact_get(artifact_id: i64, state: State<'_, AppState>) -> Result<Artifact, ForgeError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

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
) -> Result<Artifact, ForgeError> {
    if rel_path.trim().is_empty() {
        return Err(ForgeError::Validation(
            "relative path cannot be empty".to_string(),
        ));
    }

    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

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
) -> Result<Artifact, ForgeError> {
    if name.trim().is_empty() {
        return Err(ForgeError::Validation(
            "artifact name cannot be empty".to_string(),
        ));
    }

    let parsed_type = parse_artifact_type(&artifact_type)?;
    let rel_path = derive_rel_path(&parsed_type, name.trim());

    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

    // Get the project to resolve the full filesystem path
    let project = project_repo::get(&conn, project_id)?;
    let full_path = Path::new(&project.path).join(&rel_path);

    // Ensure the parent directory exists
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write file to disk
    std::fs::write(&full_path, &content)?;

    // Calculate file metadata
    let file_size = content.len() as i64;
    let file_hash = format!("sha256:{:x}", simple_hash(&content));

    // Index in database
    let mut artifact = artifact_repo::create(
        &conn,
        project_id,
        &parsed_type,
        &rel_path,
        name.trim(),
        &content,
        None,
    )?;

    // Update file metadata
    artifact_repo::update(
        &conn,
        artifact.id,
        &file_hash,
        file_size,
        &artifact.created_at,
    )?;

    // Re-fetch to get updated metadata
    artifact = artifact_repo::get(&conn, artifact.id)?;
    artifact.content = content;

    Ok(artifact)
}

/// Update an artifact's content, writing to disk and updating the database.
#[tauri::command]
pub fn artifact_update(
    artifact_id: i64,
    content: String,
    state: State<'_, AppState>,
) -> Result<Artifact, ForgeError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

    // Get the artifact to find its path
    let artifact = artifact_repo::get(&conn, artifact_id)?;
    let project = project_repo::get(&conn, artifact.project_id)?;
    let full_path = Path::new(&project.path).join(&artifact.rel_path);

    // Write updated content to disk
    std::fs::write(&full_path, &content)?;

    // Update file metadata in DB
    let file_size = content.len() as i64;
    let file_hash = format!("sha256:{:x}", simple_hash(&content));
    let now = chrono_now_iso();

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
pub fn artifact_delete(artifact_id: i64, state: State<'_, AppState>) -> Result<(), ForgeError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| ForgeError::Database(format!("lock poisoned: {e}")))?;

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

/// Parse a string into an `ArtifactType`, returning a validation error for unknown types.
fn parse_artifact_type(s: &str) -> Result<ArtifactType, ForgeError> {
    match s {
        "agent" => Ok(ArtifactType::Agent),
        "rule" => Ok(ArtifactType::Rule),
        "skill" => Ok(ArtifactType::Skill),
        "hook" => Ok(ArtifactType::Hook),
        "doc" => Ok(ArtifactType::Doc),
        other => Err(ForgeError::Validation(format!(
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

/// Generate a basic ISO 8601 timestamp (approximation without chrono crate).
fn chrono_now_iso() -> String {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Basic UTC timestamp calculation
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    // Calculate year/month/day from days since epoch (1970-01-01)
    let (year, month, day) = days_to_ymd(days);

    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Simplified calendar calculation
    let mut y = 1970;
    let mut remaining = days;

    loop {
        let days_in_year = if is_leap_year(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }

    let days_in_months: [u64; 12] = if is_leap_year(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut m = 0;
    for (i, &days_in_month) in days_in_months.iter().enumerate() {
        if remaining < days_in_month {
            m = i as u64 + 1;
            break;
        }
        remaining -= days_in_month;
    }

    (y, m, remaining + 1)
}

/// Check if a year is a leap year.
fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
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
        assert!(matches!(result, Err(ForgeError::Validation(_))));
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
        assert!(matches!(result, Err(ForgeError::NotFound(_))));
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
        assert!(matches!(result, Err(ForgeError::NotFound(_))));
    }

    #[test]
    fn empty_name_validation() {
        let name = "   ";
        assert!(name.trim().is_empty());
    }

    #[test]
    fn chrono_now_iso_format() {
        let ts = chrono_now_iso();
        // Should match pattern like "2026-03-03T12:00:00Z"
        assert!(ts.ends_with('Z'));
        assert_eq!(ts.len(), 20);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
    }

    #[test]
    fn days_to_ymd_epoch() {
        let (y, m, d) = days_to_ymd(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn days_to_ymd_known_date() {
        // 2024-01-01 is 19723 days from epoch
        let (y, m, d) = days_to_ymd(19723);
        assert_eq!((y, m, d), (2024, 1, 1));
    }

    #[test]
    fn is_leap_year_checks() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2023));
    }
}
