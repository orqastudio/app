//! Artifact sync module — incremental and bulk sync of `.orqa/` markdown files into SurrealDB.
//!
//! This module provides content-hash-based deduplication so that unchanged files are not
//! re-parsed or re-upserted. Each file is hashed with SHA-256; if the stored hash matches
//! the on-disk hash, the file is skipped. Otherwise the artifact node and its relationship
//! edges are upserted atomically.
//!
//! Four public entry points:
//! - `sync_file` — sync a single markdown file
//! - `delete_artifact` — remove an artifact by ID and its outgoing edges
//! - `delete_artifact_by_path` — remove an artifact by relative file path (used by file watcher)
//! - `bulk_sync` — walk an entire `.orqa/` tree and sync all `.md` files

use std::path::Path;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};

use crate::build::extract_frontmatter;
use crate::surreal::GraphDb;
use crate::writers::bump_version;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Outcome of a single file sync operation.
pub enum SyncResult {
    /// File content hash matched the stored hash — no write was performed.
    Unchanged,
    /// Artifact node and edges were written to SurrealDB.
    Upserted {
        /// Artifact ID derived from the `id` frontmatter field.
        id: String,
        /// Number of relationship edges written.
        edge_count: usize,
    },
    /// File was intentionally skipped for a stated reason (e.g. no frontmatter, no `id` field).
    Skipped {
        /// Human-readable reason the file was not synced.
        reason: String,
    },
}

/// Summary of a `bulk_sync` run.
pub struct BulkSyncSummary {
    /// Total `.md` files found (including skipped and errored).
    pub files_scanned: usize,
    /// Files whose content differed from the stored hash and were written.
    pub upserted: usize,
    /// Files whose content matched the stored hash and were left untouched.
    pub unchanged: usize,
    /// Files that produced an error or were skipped during sync.
    pub errors: usize,
}

// ---------------------------------------------------------------------------
// Internal: field extraction
// ---------------------------------------------------------------------------

/// All parsed fields for one artifact, ready to write to SurrealDB.
struct ParsedArtifact {
    id: String,
    title: String,
    description: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    artifact_type: String,
    created: Option<String>,
    updated: Option<String>,
    frontmatter_json: String,
    body: String,
    relationships: Vec<RawRelationship>,
}

/// A relationship entry extracted from YAML frontmatter.
#[derive(Clone)]
struct RawRelationship {
    target: String,
    rel_type: String,
}

/// Parse a markdown file's bytes into a `ParsedArtifact`.
///
/// Returns `Err` if the bytes are not valid UTF-8.
/// Returns `Ok(None)` if the file has no frontmatter or no `id` field (i.e. not an artifact).
fn parse_artifact_bytes(bytes: &[u8], path: &Path) -> Result<Option<(ParsedArtifact, String)>> {
    // body is returned alongside ParsedArtifact as part of the tuple
    let content = String::from_utf8_lossy(bytes).into_owned();
    let (fm_text, body) = extract_frontmatter(&content);

    let Some(fm_text) = fm_text else {
        return Ok(None);
    };

    let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&fm_text) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(path = %path.display(), error = %e, "YAML parse error — skipping");
            return Ok(None);
        }
    };

    let Some(id) = yaml_str(&yaml_value, "id").filter(|s| !s.trim().is_empty()) else {
        return Ok(None);
    };
    let id = id.trim().to_owned();

    let title = yaml_str(&yaml_value, "title").unwrap_or_else(|| id.clone());
    let description = yaml_str(&yaml_value, "description");
    let status = yaml_str(&yaml_value, "status");
    let priority = yaml_str(&yaml_value, "priority");
    let artifact_type = yaml_str(&yaml_value, "type")
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| infer_type_from_id_prefix(&id));
    let created = yaml_str(&yaml_value, "created");
    let updated = yaml_str(&yaml_value, "updated");
    let frontmatter_json = serde_json::to_string(&yaml_value).unwrap_or_else(|_| "{}".to_owned());
    let relationships = parse_relationships(&yaml_value);

    Ok(Some((
        ParsedArtifact {
            id,
            title,
            description,
            status,
            priority,
            artifact_type,
            created,
            updated,
            frontmatter_json,
            body: body.clone(),
            relationships,
        },
        body,
    )))
}

// ---------------------------------------------------------------------------
// Internal: SurrealDB write helpers
// ---------------------------------------------------------------------------

/// Check whether an artifact's stored content_hash in SurrealDB matches `hash`.
///
/// Returns `true` if the stored hash matches (file is unchanged), `false` otherwise.
async fn is_unchanged(db: &GraphDb, rel_path: &str, hash: &str) -> Result<bool> {
    let path_escaped = escape_surql_string(rel_path);
    let query = format!("SELECT content_hash FROM artifact WHERE path = '{path_escaped}' LIMIT 1;");
    let mut response = db.0.query(&query).await.context("querying artifact hash")?;
    let rows: Vec<serde_json::Value> = response.take(0).context("reading hash result")?;

    Ok(rows
        .first()
        .and_then(|r| r.get("content_hash"))
        .and_then(|v| v.as_str())
        == Some(hash))
}

/// Upsert a parsed artifact node into SurrealDB, replacing any existing record.
///
/// After writing the core fields, calls `bump_version` to atomically increment
/// the `version` counter and set `updated_at = time::now()`.
async fn upsert_artifact_node(
    db: &GraphDb,
    artifact: &ParsedArtifact,
    rel_path: &str,
    hash: &str,
) -> Result<()> {
    let safe_id = sanitize_record_id(&artifact.id);
    let upsert = format!(
        "UPSERT artifact:`{safe_id}` SET \
            artifact_type = '{type_sql}', \
            title = '{title_sql}', \
            description = {desc_sql}, \
            status = {status_sql}, \
            priority = {priority_sql}, \
            path = '{path_sql}', \
            body = '{body_sql}', \
            frontmatter = {fm_json}, \
            content_hash = '{hash_sql}', \
            source_plugin = NONE, \
            created = {created_sql}, \
            updated = {updated_sql};",
        type_sql = escape_surql_string(&artifact.artifact_type),
        title_sql = escape_surql_string(&artifact.title),
        desc_sql = option_to_surql(artifact.description.as_deref()),
        status_sql = option_to_surql(artifact.status.as_deref()),
        priority_sql = option_to_surql(artifact.priority.as_deref()),
        path_sql = escape_surql_string(rel_path),
        body_sql = escape_surql_string(&artifact.body),
        fm_json = &artifact.frontmatter_json,
        hash_sql = escape_surql_string(hash),
        created_sql = option_to_surql(artifact.created.as_deref()),
        updated_sql = option_to_surql(artifact.updated.as_deref()),
    );
    db.0.query(&upsert)
        .await
        .with_context(|| format!("upserting artifact {}", artifact.id))?;

    // Bump version and updated_at atomically after the field write.
    bump_version(db, &artifact.id, None)
        .await
        .map_err(|e| anyhow::anyhow!("version bump failed for {}: {e}", artifact.id))?;

    Ok(())
}

/// Delete all outgoing relationship edges for `artifact_id` then re-insert from `relationships`.
///
/// Returns the number of edges successfully written.
async fn replace_edges(
    db: &GraphDb,
    artifact_id: &str,
    relationships: &[RawRelationship],
) -> Result<usize> {
    let safe_id = sanitize_record_id(artifact_id);

    // Remove stale edges before re-inserting to prevent duplicates.
    db.0.query(format!(
        "DELETE relates_to WHERE in = artifact:`{safe_id}`;"
    ))
    .await
    .context("deleting existing edges")?;

    let mut edge_count = 0;
    for rel in relationships {
        let target_safe = sanitize_record_id(&rel.target);
        let rel_type_sql = escape_surql_string(&rel.rel_type);
        let edge_query = format!(
            "RELATE artifact:`{safe_id}`->relates_to->artifact:`{target_safe}` \
                SET relation_type = '{rel_type_sql}';"
        );
        match db.0.query(&edge_query).await {
            Err(e) => {
                tracing::warn!(
                    from = artifact_id, to = %rel.target,
                    error = %e, "edge query transport error"
                );
            }
            Ok(response) => match response.check() {
                Ok(_) => edge_count += 1,
                Err(e) => {
                    tracing::warn!(
                        from = artifact_id, to = %rel.target,
                        error = %e, "edge creation failed (target may not exist yet)"
                    );
                }
            },
        }
    }
    Ok(edge_count)
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Sync a single markdown file into SurrealDB.
///
/// Reads the file, computes a SHA-256 content hash, and compares it to the hash
/// stored for the artifact's `path` key. If the hashes match the file is unchanged
/// and no write is performed. Otherwise the artifact node and all its relationship
/// edges are upserted and `SyncResult::Upserted` is returned.
///
/// `path` must be an absolute path to the `.md` file.
/// `project_root` is used to compute the relative path stored in SurrealDB.
pub async fn sync_file(db: &GraphDb, path: &Path, project_root: &Path) -> Result<SyncResult> {
    let rel_path = path
        .strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let hash = hex::encode(Sha256::digest(&bytes));

    if is_unchanged(db, &rel_path, &hash).await? {
        return Ok(SyncResult::Unchanged);
    }

    let Some((artifact, _body)) = parse_artifact_bytes(&bytes, path)? else {
        return Ok(SyncResult::Skipped {
            reason: format!("no usable frontmatter or id in {}", path.display()),
        });
    };

    upsert_artifact_node(db, &artifact, &rel_path, &hash).await?;
    let edge_count = replace_edges(db, &artifact.id, &artifact.relationships).await?;

    Ok(SyncResult::Upserted {
        id: artifact.id,
        edge_count,
    })
}

/// Delete an artifact and all its outgoing relationship edges from SurrealDB.
///
/// The artifact is identified by its `id` frontmatter value (e.g. `EPIC-358d42a4`).
/// Both the `relates_to` edges where this artifact is the source and the artifact node
/// itself are deleted.
pub async fn delete_artifact(db: &GraphDb, artifact_id: &str) -> Result<()> {
    let safe_id = sanitize_record_id(artifact_id);
    db.0.query(format!(
        "DELETE relates_to WHERE in = artifact:`{safe_id}`;"
    ))
    .await
    .context("deleting artifact edges")?;
    db.0.query(format!("DELETE artifact:`{safe_id}`;"))
        .await
        .context("deleting artifact node")?;
    Ok(())
}

/// Delete an artifact (and its outgoing edges) identified by its relative file path.
///
/// Used by the file watcher when a `.md` file under `.orqa/` is removed from disk.
/// Finds the artifact record by its `path` field, deletes all outgoing `relates_to`
/// edges where that artifact is the source, then deletes the artifact node itself.
/// No-ops silently if no artifact with the given path exists in the database.
///
/// `rel_path` must use forward slashes (normalise Windows paths before calling).
pub async fn delete_artifact_by_path(db: &GraphDb, rel_path: &str) -> Result<()> {
    let path_escaped = escape_surql_string(rel_path);
    // Two-step: first delete outgoing edges, then delete the node.
    // `SELECT VALUE id` returns a plain array of record IDs which we use as the
    // edge `in` filter — SurrealDB evaluates `in IN [...]` correctly.
    db.0.query(format!(
        "LET $victims = (SELECT VALUE id FROM artifact WHERE path = '{path_escaped}'); \
         DELETE relates_to WHERE in IN $victims; \
         DELETE artifact WHERE path = '{path_escaped}';"
    ))
    .await
    .context("deleting artifact by path")?;
    Ok(())
}

/// Walk the `{project_root}/.orqa/` directory tree and sync all `.md` files into SurrealDB.
///
/// Uses a two-pass approach to handle forward references:
/// - Pass 1: upsert all artifact nodes (with content_hash dedup)
/// - Pass 2: re-create all edges (now all targets exist)
///
/// This ensures edges are always correct even when artifacts reference targets
/// that appear later in the directory walk order.
#[allow(clippy::too_many_lines)]
pub async fn bulk_sync(db: &GraphDb, project_root: &Path) -> Result<BulkSyncSummary> {
    let orqa_dir = project_root.join(".orqa");
    let mut summary = BulkSyncSummary {
        files_scanned: 0,
        upserted: 0,
        unchanged: 0,
        errors: 0,
    };

    // Collect all .md paths first to enable the two-pass strategy.
    let md_paths: Vec<_> = walkdir::WalkDir::new(&orqa_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| {
            let p = e.path();
            p.is_file()
                && p.extension().and_then(|x| x.to_str()) == Some("md")
                && !p
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.eq_ignore_ascii_case("README.md"))
        })
        .map(walkdir::DirEntry::into_path)
        .collect();

    // Pass 1: upsert artifact nodes.
    // Collect (artifact_id, relationships) for every parseable file for the edge pass.
    let mut edge_work: Vec<(String, Vec<RawRelationship>)> = Vec::new();

    for path in &md_paths {
        summary.files_scanned += 1;

        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!(path = %path.display(), error = %e, "failed to read file");
                summary.errors += 1;
                continue;
            }
        };

        let hash = hex::encode(Sha256::digest(&bytes));
        let rel_path = path
            .strip_prefix(project_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        match parse_artifact_bytes(&bytes, path) {
            Ok(Some((artifact, _))) => {
                let id = artifact.id.clone();
                let relationships = artifact.relationships.clone();

                match is_unchanged(db, &rel_path, &hash).await {
                    Ok(true) => summary.unchanged += 1,
                    _ => {
                        match upsert_artifact_node(db, &artifact, &rel_path, &hash).await {
                            Ok(()) => summary.upserted += 1,
                            Err(e) => {
                                tracing::warn!(id = %id, error = %e, "node upsert failed");
                                summary.errors += 1;
                                continue; // skip edge_work for this artifact
                            }
                        }
                    }
                }

                edge_work.push((id, relationships));
            }
            Ok(None) => {
                // No frontmatter or no id — not an artifact file.
                summary.errors += 1;
            }
            Err(e) => {
                tracing::warn!(path = %path.display(), error = %e, "parse error");
                summary.errors += 1;
            }
        }
    }

    // Pass 2: sync edges. All nodes now exist so RELATE will succeed.
    for (artifact_id, relationships) in &edge_work {
        if let Err(e) = replace_edges(db, artifact_id, relationships).await {
            tracing::warn!(id = %artifact_id, error = %e, "edge sync failed");
        }
    }

    Ok(summary)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Extract a string value for `key` from a YAML mapping value.
fn yaml_str(yaml: &serde_yaml::Value, key: &str) -> Option<String> {
    yaml.get(key).and_then(|v| v.as_str()).map(str::to_owned)
}

/// Infer an artifact type from the ID prefix when no `type:` frontmatter field is present.
///
/// For example, `EPIC-001` → `"epic"`, `TASK-abc` → `"task"`. Falls back to `"unknown"` for
/// unrecognised prefixes.
fn infer_type_from_id_prefix(id: &str) -> String {
    let prefix = id.split('-').next().unwrap_or("").to_lowercase();
    match prefix.as_str() {
        "epic" | "task" | "milestone" | "rule" | "lesson" | "decision" | "review" | "agent"
        | "persona" | "principle" | "pillar" | "vision" | "discovery" | "wireframe"
        | "knowledge" | "doc" | "plan" | "idea" | "res" | "know" => prefix,
        _ => "unknown".to_owned(),
    }
}

/// Parse the `relationships:` array from YAML frontmatter.
///
/// Each entry requires a `target` field. The `type` field defaults to `"unknown"` if absent.
fn parse_relationships(yaml_value: &serde_yaml::Value) -> Vec<RawRelationship> {
    let Some(seq) = yaml_value
        .get("relationships")
        .and_then(|v| v.as_sequence())
    else {
        return Vec::new();
    };

    seq.iter()
        .filter_map(|item| {
            let target = item.get("target")?.as_str()?.trim().to_owned();
            if target.is_empty() {
                return None;
            }
            let rel_type = item
                .get("type")
                .and_then(|v| v.as_str())
                .map_or_else(|| "unknown".to_owned(), |s| s.trim().to_owned());
            Some(RawRelationship { target, rel_type })
        })
        .collect()
}

/// Sanitize an artifact ID for use as a SurrealDB backtick-delimited record ID.
///
/// Removes backtick characters that would break the SurrealQL `artifact:\`id\`` syntax.
fn sanitize_record_id(id: &str) -> String {
    id.replace('`', "")
}

/// Escape a string for safe embedding in a SurrealQL single-quoted string literal.
fn escape_surql_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "\\'")
}

/// Convert an `Option<&str>` to a SurrealQL expression.
///
/// `Some(s)` becomes `'escaped_value'`; `None` becomes the SurrealQL `NONE` keyword.
fn option_to_surql(opt: Option<&str>) -> String {
    match opt {
        Some(s) => format!("'{}'", escape_surql_string(s)),
        None => "NONE".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surreal::{initialize_schema, open_memory};

    /// Create an in-memory GraphDb with the schema applied, ready for testing.
    async fn make_db() -> GraphDb {
        let db = open_memory().await.unwrap();
        initialize_schema(&db).await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_sync_file_creates_artifact() {
        let db = make_db().await;
        let dir = tempfile::tempdir().unwrap();
        let content =
            "---\nid: TEST-001\ntype: epic\ntitle: Test Epic\nstatus: active\n---\n\nBody here.";
        let orqa_dir = dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();
        let path = orqa_dir.join("TEST-001.md");
        std::fs::write(&path, content).unwrap();

        let result = sync_file(&db, &path, dir.path()).await.unwrap();
        assert!(matches!(result, SyncResult::Upserted { .. }));

        // Verify the artifact was written to SurrealDB.
        let rows: Vec<serde_json::Value> =
            db.0.query("SELECT * FROM artifact WHERE path CONTAINS 'TEST-001'")
                .await
                .unwrap()
                .take(0)
                .unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[tokio::test]
    async fn test_sync_file_unchanged() {
        let db = make_db().await;
        let dir = tempfile::tempdir().unwrap();
        let content = "---\nid: TEST-002\ntype: epic\ntitle: Test\nstatus: active\n---\nBody.";
        let orqa_dir = dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();
        let path = orqa_dir.join("TEST-002.md");
        std::fs::write(&path, content).unwrap();

        // First sync — should upsert.
        sync_file(&db, &path, dir.path()).await.unwrap();
        // Second sync with identical content — should be unchanged.
        let result = sync_file(&db, &path, dir.path()).await.unwrap();
        assert!(matches!(result, SyncResult::Unchanged));
    }

    #[tokio::test]
    async fn test_sync_file_reupserts_after_change() {
        let db = make_db().await;
        let dir = tempfile::tempdir().unwrap();
        let orqa_dir = dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();
        let path = orqa_dir.join("TEST-003.md");

        std::fs::write(
            &path,
            "---\nid: TEST-003\ntype: epic\ntitle: V1\nstatus: active\n---\nV1.",
        )
        .unwrap();
        sync_file(&db, &path, dir.path()).await.unwrap();

        // Overwrite with changed content — hash will differ, triggering re-upsert.
        std::fs::write(
            &path,
            "---\nid: TEST-003\ntype: epic\ntitle: V2\nstatus: active\n---\nV2.",
        )
        .unwrap();
        let result = sync_file(&db, &path, dir.path()).await.unwrap();
        assert!(matches!(result, SyncResult::Upserted { .. }));
    }

    #[tokio::test]
    async fn test_delete_artifact() {
        let db = make_db().await;
        let dir = tempfile::tempdir().unwrap();
        let orqa_dir = dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();
        let path = orqa_dir.join("TEST-004.md");
        std::fs::write(
            &path,
            "---\nid: TEST-004\ntype: epic\ntitle: Del\nstatus: active\n---\n",
        )
        .unwrap();

        sync_file(&db, &path, dir.path()).await.unwrap();
        delete_artifact(&db, "TEST-004").await.unwrap();

        let rows: Vec<serde_json::Value> =
            db.0.query("SELECT * FROM artifact WHERE path CONTAINS 'TEST-004'")
                .await
                .unwrap()
                .take(0)
                .unwrap();
        assert_eq!(rows.len(), 0);
    }

    #[tokio::test]
    async fn test_bulk_sync() {
        let db = make_db().await;
        let dir = tempfile::tempdir().unwrap();
        let orqa_dir = dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();

        for i in 1..=3 {
            let content =
                format!("---\nid: BULK-00{i}\ntype: epic\ntitle: Bulk {i}\nstatus: active\n---\n");
            std::fs::write(orqa_dir.join(format!("BULK-00{i}.md")), content).unwrap();
        }

        let summary = bulk_sync(&db, dir.path()).await.unwrap();
        assert_eq!(summary.files_scanned, 3);
        assert_eq!(summary.upserted, 3);
        assert_eq!(summary.errors, 0);
    }
}
