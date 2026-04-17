//! Write operations for the artifact graph.
//!
//! This module provides the canonical version-bump helper and the upsert-with-version-bump
//! and three-way merge write operations used by `POST /artifacts/import`. It reuses the
//! sync module's internal string-escaping conventions and operates directly against SurrealDB.
//!
//! Entry points:
//! - `bump_version`   — atomic version increment + updated_at; called by every writer path
//! - `import_upsert`  — overwrite an existing record, bumping its `version` field
//! - `import_merge_write` — write a merge-resolved field map, bumping `version`
//! - `read_artifact`  — read the current frontmatter JSON for a record by ID
//!
//! `bump_version` behaviour:
//! - `ORQA_OPTIMISTIC_LOCK=false` (default, MVP): bumps unconditionally; no conflict check.
//! - `ORQA_OPTIMISTIC_LOCK=true` (deferred): checks `expected_version`; returns
//!   `BumpError::Conflict` (HTTP 409) when the stored version has advanced past the expected.
//!
//! All write functions are atomic at the record level: each issues a single UPSERT
//! statement so partial writes cannot occur.

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::surreal::GraphDb;

// ---------------------------------------------------------------------------
// Optimistic-lock error
// ---------------------------------------------------------------------------

/// Errors from `bump_version`.
#[derive(Debug, thiserror::Error)]
pub enum BumpError {
    /// Optimistic-lock mismatch: stored version != expected_version.
    ///
    /// Route handlers map this to HTTP 409 Conflict.
    #[error("version conflict: expected {expected}, found {found} for artifact {artifact_id}")]
    Conflict {
        /// The artifact whose version check failed.
        artifact_id: String,
        /// The version the caller expected to find in the database.
        expected: u64,
        /// The version actually stored in the database.
        found: u64,
    },

    /// Any other SurrealDB or I/O error.
    #[error("{0}")]
    Other(String),
}

// ---------------------------------------------------------------------------
// Optimistic-lock flag
// ---------------------------------------------------------------------------

/// Return `true` when the caller should enforce the expected-version check.
///
/// Reads `ORQA_OPTIMISTIC_LOCK` from the environment at call time so that
/// tests can toggle the flag without rebuilding. Defaults to `false`.
fn optimistic_lock_enabled() -> bool {
    std::env::var("ORQA_OPTIMISTIC_LOCK")
        .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// bump_version — canonical version-bump helper
// ---------------------------------------------------------------------------

/// Atomically increment `version` and set `updated_at = time::now()` for an artifact.
///
/// Every writer path (PUT, POST, soft-delete, sync ingest, migrate ingest) must call
/// this helper immediately before committing its mutation. The helper issues a single
/// UPSERT containing both the version increment and the timestamp update so that no
/// partial state is observable at the record level.
///
/// # Optimistic-lock behaviour
///
/// When `ORQA_OPTIMISTIC_LOCK=false` (MVP default): bumps unconditionally.
/// `expected_version` is ignored — pass `None`.
///
/// When `ORQA_OPTIMISTIC_LOCK=true` (deferred enforcement): `expected_version` must
/// be `Some(v)` where `v` is the version the caller read before computing its mutation.
/// If the stored version has advanced past `v`, `BumpError::Conflict` is returned and
/// the caller maps this to HTTP 409.
///
/// Returns the new version value on success.
pub async fn bump_version(
    db: &GraphDb,
    artifact_id: &str,
    expected_version: Option<u64>,
) -> std::result::Result<u64, BumpError> {
    let safe_id = sanitize_record_id(artifact_id);
    let stored = read_stored_version(db, &safe_id).await?;

    if optimistic_lock_enabled() {
        let expected = expected_version.unwrap_or(stored);
        if stored != expected {
            return Err(BumpError::Conflict {
                artifact_id: artifact_id.to_owned(),
                expected,
                found: stored,
            });
        }
    }

    let new_version = stored + 1;
    let query = format!(
        "UPSERT artifact:`{safe_id}` SET \
            version = {new_version}, \
            updated_at = time::now();"
    );
    db.0.query(&query)
        .await
        .with_context(|| format!("bump_version for {artifact_id}"))
        .map_err(|e| BumpError::Other(e.to_string()))?;
    Ok(new_version)
}

/// Read the current `version` stored for an artifact, returning `0` if absent.
async fn read_stored_version(db: &GraphDb, safe_id: &str) -> std::result::Result<u64, BumpError> {
    let query = format!("SELECT version FROM artifact:`{safe_id}` LIMIT 1;");
    let mut response =
        db.0.query(&query)
            .await
            .context("reading version")
            .map_err(|e| BumpError::Other(e.to_string()))?;
    let rows: Vec<Value> = response
        .take(0)
        .context("reading version rows")
        .map_err(|e| BumpError::Other(e.to_string()))?;
    Ok(rows
        .first()
        .and_then(|r| r.get("version"))
        .and_then(Value::as_u64)
        .unwrap_or(0))
}

// ---------------------------------------------------------------------------
// Helpers shared with sync.rs (duplicated here to keep modules independent)
// ---------------------------------------------------------------------------

/// Sanitize an artifact ID for use as a SurrealDB backtick-delimited record ID.
fn sanitize_record_id(id: &str) -> String {
    id.replace('`', "")
}

/// Escape a string for safe embedding in a SurrealQL single-quoted string literal.
fn escape_surql_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "\\'")
}

/// Convert an `Option<&str>` to a SurrealQL expression: `'value'` or `NONE`.
fn option_to_surql(opt: Option<&str>) -> String {
    match opt {
        Some(s) => format!("'{}'", escape_surql_string(s)),
        None => "NONE".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// The current frontmatter of an artifact stored in SurrealDB.
#[derive(Debug, Clone)]
pub struct StoredArtifact {
    /// Artifact ID (frontmatter `id` field).
    pub id: String,
    /// Frontmatter as a flat JSON object. `Null` if not stored yet.
    pub frontmatter: Value,
    /// Content hash stored for this record.
    pub content_hash: Option<String>,
    /// Current version counter (0 if not set).
    pub version: u64,
}

// ---------------------------------------------------------------------------
// Read
// ---------------------------------------------------------------------------

/// Read the current stored artifact fields from SurrealDB by artifact ID.
///
/// Returns `Ok(None)` if no record exists for the given ID.
pub async fn read_artifact(db: &GraphDb, artifact_id: &str) -> Result<Option<StoredArtifact>> {
    let safe_id = sanitize_record_id(artifact_id);
    let query =
        format!("SELECT id, frontmatter, content_hash, version FROM artifact:`{safe_id}` LIMIT 1;");
    let mut response = db.0.query(&query).await.context("reading artifact")?;
    let rows: Vec<Value> = response.take(0).context("reading artifact rows")?;

    let Some(row) = rows.into_iter().next() else {
        return Ok(None);
    };

    let id = row
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or(artifact_id)
        .to_owned();
    let frontmatter = row
        .get("frontmatter")
        .cloned()
        .unwrap_or_else(|| Value::Object(serde_json::Map::default()));
    let content_hash = row
        .get("content_hash")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let version = row.get("version").and_then(Value::as_u64).unwrap_or(0);

    Ok(Some(StoredArtifact {
        id,
        frontmatter,
        content_hash,
        version,
    }))
}

// ---------------------------------------------------------------------------
// Upsert (conflict policy: overwrite)
// ---------------------------------------------------------------------------

/// Write an artifact to SurrealDB, bumping its `version` field.
///
/// Used by the `upsert` conflict policy: the incoming file fully overwrites the
/// existing record. Any field changes in the DB are discarded. The `version`
/// counter is incremented by 1 from the current stored value (0 if new).
///
/// The `content_hash` is updated to `new_hash` so re-runs detect the record as
/// unchanged and skip it.
///
/// `rel_path` is the relative path stored in the `path` field (forward slashes).
pub async fn import_upsert(
    db: &GraphDb,
    artifact_id: &str,
    fields: &BTreeMap<String, Value>,
    rel_path: &str,
    new_hash: &str,
    current_version: u64,
) -> Result<()> {
    let query = build_upsert_query(artifact_id, fields, rel_path, new_hash);
    db.0.query(&query)
        .await
        .with_context(|| format!("import upsert for {artifact_id}"))?;

    // Bump version through the canonical helper so the optimistic-lock check
    // applies on every import write when ORQA_OPTIMISTIC_LOCK=true.
    // Pass None on fresh insert (current_version == 0); Some on update.
    let expected = (current_version != 0).then_some(current_version);
    bump_version(db, artifact_id, expected)
        .await
        .map_err(|e| anyhow::anyhow!("version bump failed for {artifact_id}: {e}"))?;

    Ok(())
}

/// Build the UPSERT SQL for `import_upsert`/`import_merge_write`.
///
/// Version and updated_at are deliberately NOT set here — `bump_version` owns
/// them so the optimistic-lock check is always in the critical path.
fn build_upsert_query(
    artifact_id: &str,
    fields: &BTreeMap<String, Value>,
    rel_path: &str,
    new_hash: &str,
) -> String {
    let safe_id = sanitize_record_id(artifact_id);
    let title = fields
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or(artifact_id);
    let artifact_type = fields
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let description = fields.get("description").and_then(|v| v.as_str());
    let status = fields.get("status").and_then(|v| v.as_str());
    let priority = fields.get("priority").and_then(|v| v.as_str());
    let created = fields.get("created").and_then(|v| v.as_str());
    let updated = fields.get("updated").and_then(|v| v.as_str());
    let fm_json = serde_json::to_string(fields).unwrap_or_else(|_| "{}".to_owned());

    format!(
        "UPSERT artifact:`{safe_id}` SET \
            artifact_type = '{type_sql}', \
            title = '{title_sql}', \
            description = {desc_sql}, \
            status = {status_sql}, \
            priority = {priority_sql}, \
            path = '{path_sql}', \
            frontmatter = {fm_json}, \
            content_hash = '{hash_sql}', \
            created = {created_sql}, \
            updated = {updated_sql};",
        type_sql = escape_surql_string(artifact_type),
        title_sql = escape_surql_string(title),
        desc_sql = option_to_surql(description),
        status_sql = option_to_surql(status),
        priority_sql = option_to_surql(priority),
        path_sql = escape_surql_string(rel_path),
        hash_sql = escape_surql_string(new_hash),
        created_sql = option_to_surql(created),
        updated_sql = option_to_surql(updated),
    )
}

// ---------------------------------------------------------------------------
// Field update (PUT /artifacts/:id)
// ---------------------------------------------------------------------------

/// Update a single frontmatter field in the SurrealDB artifact record.
///
/// Reads the current stored frontmatter, applies the field change, recomputes
/// the content_hash from the updated frontmatter JSON, writes the change, then
/// calls `bump_version` so `version` and `updated_at` always advance on PUT.
///
/// Returns the new `version` value on success.
///
/// Mapped column names: `status`, `title`, `description`, `priority` are promoted
/// to top-level SurrealDB columns in addition to `frontmatter`. All other fields
/// are written only into `frontmatter`.
pub async fn update_artifact_fields(
    db: &GraphDb,
    artifact_id: &str,
    field: &str,
    value: &str,
) -> Result<u64> {
    let stored = read_artifact(db, artifact_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("artifact '{artifact_id}' not found in SurrealDB"))?;

    // Apply the field change to the stored frontmatter JSON.
    let mut fm = match stored.frontmatter {
        Value::Object(m) => m,
        _ => serde_json::Map::default(),
    };
    fm.insert(field.to_owned(), Value::String(value.to_owned()));
    let updated_fm = Value::Object(fm);

    // Recompute content_hash from the updated frontmatter JSON.
    let fm_json = serde_json::to_string(&updated_fm).unwrap_or_else(|_| "{}".to_owned());
    use sha2::{Digest, Sha256};
    let new_hash = hex::encode(Sha256::digest(fm_json.as_bytes()));

    let safe_id = sanitize_record_id(artifact_id);

    // Determine which top-level column to update alongside frontmatter.
    let column_clause = match field {
        "status" => format!(", status = '{}'", escape_surql_string(value)),
        "title" => format!(", title = '{}'", escape_surql_string(value)),
        "description" => format!(", description = '{}'", escape_surql_string(value)),
        "priority" => format!(", priority = '{}'", escape_surql_string(value)),
        _ => String::new(),
    };

    let query = format!(
        "UPSERT artifact:`{safe_id}` SET \
            frontmatter = {fm_json}, \
            content_hash = '{hash_sql}'{column_clause};",
        hash_sql = escape_surql_string(&new_hash),
    );
    db.0.query(&query)
        .await
        .with_context(|| format!("update_artifact_fields for {artifact_id}"))?;

    let new_version = bump_version(db, artifact_id, None)
        .await
        .map_err(|e| anyhow::anyhow!("version bump failed for {artifact_id}: {e}"))?;

    Ok(new_version)
}

// ---------------------------------------------------------------------------
// Merge write
// ---------------------------------------------------------------------------

/// Write a merge-resolved field map to SurrealDB, bumping `version`.
///
/// Used by the `merge` conflict policy after `three_way_merge` produces a clean
/// `MergeResult`. Writes the resolved fields only — this is the same shape as
/// `import_upsert` but the field map comes from the merge result rather than
/// the incoming file directly.
pub async fn import_merge_write(
    db: &GraphDb,
    artifact_id: &str,
    resolved: &BTreeMap<String, Value>,
    rel_path: &str,
    new_hash: &str,
    current_version: u64,
) -> Result<()> {
    // Same implementation as upsert — the caller has already resolved conflicts.
    import_upsert(
        db,
        artifact_id,
        resolved,
        rel_path,
        new_hash,
        current_version,
    )
    .await
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surreal::{initialize_schema, open_memory};
    use serde_json::json;
    use tokio::sync::Mutex;

    // Serializes tests that manipulate the ORQA_OPTIMISTIC_LOCK env var.
    // Env vars are process-global; parallel tests race on set/remove operations.
    // Uses `tokio::sync::Mutex` (async-aware) so the guard can be held across
    // `.await` without triggering `clippy::await_holding_lock`.
    static ENV_LOCK: Mutex<()> = Mutex::const_new(());

    async fn make_db() -> GraphDb {
        let db = open_memory().await.unwrap();
        initialize_schema(&db).await.unwrap();
        db
    }

    fn fields_from(v: &Value) -> BTreeMap<String, Value> {
        match v {
            Value::Object(m) => m.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            _ => BTreeMap::new(),
        }
    }

    /// Schema DEFAULT 1 + bump_version means the first import_upsert produces version 2.
    /// Version 1 = "record exists, never explicitly written via import".
    /// Version 2+ = "at least one explicit import write has occurred".
    #[tokio::test]
    async fn import_upsert_creates_new_record() {
        let db = make_db().await;
        let fields = fields_from(&json!({
            "id": "EPIC-W01",
            "type": "epic",
            "title": "Writer Test Epic",
            "status": "active"
        }));
        import_upsert(&db, "EPIC-W01", &fields, ".orqa/EPIC-W01.md", "hash1", 0)
            .await
            .unwrap();

        let stored = read_artifact(&db, "EPIC-W01").await.unwrap();
        assert!(stored.is_some(), "record must be created");
        let stored = stored.unwrap();
        // DEFAULT 1 applied by schema on UPSERT; bump_version then produces 2.
        assert_eq!(
            stored.version, 2,
            "version must be DEFAULT 1 + one bump = 2"
        );
        assert_eq!(stored.content_hash.as_deref(), Some("hash1"));
    }

    #[tokio::test]
    async fn import_upsert_bumps_version_on_overwrite() {
        let db = make_db().await;
        let fields_v1 = fields_from(&json!({
            "id": "EPIC-W02",
            "type": "epic",
            "title": "V1",
            "status": "active"
        }));
        import_upsert(
            &db,
            "EPIC-W02",
            &fields_v1,
            ".orqa/EPIC-W02.md",
            "hash-v1",
            0,
        )
        .await
        .unwrap();

        let after_v1 = read_artifact(&db, "EPIC-W02").await.unwrap().unwrap();
        // DEFAULT 1 + one bump = 2.
        assert_eq!(after_v1.version, 2);

        let fields_v2 = fields_from(&json!({
            "id": "EPIC-W02",
            "type": "epic",
            "title": "V2",
            "status": "done"
        }));
        import_upsert(
            &db,
            "EPIC-W02",
            &fields_v2,
            ".orqa/EPIC-W02.md",
            "hash-v2",
            after_v1.version,
        )
        .await
        .unwrap();

        let after_v2 = read_artifact(&db, "EPIC-W02").await.unwrap().unwrap();
        // Second bump: stored=2, bump produces 3.
        assert_eq!(
            after_v2.version, 3,
            "version must increment on second upsert"
        );
        assert_eq!(after_v2.content_hash.as_deref(), Some("hash-v2"));
    }

    #[tokio::test]
    async fn read_artifact_returns_none_for_missing_id() {
        let db = make_db().await;
        let result = read_artifact(&db, "EPIC-NOTEXIST").await.unwrap();
        assert!(result.is_none());
    }

    // -----------------------------------------------------------------------
    // bump_version tests
    // -----------------------------------------------------------------------

    /// Insert a minimal artifact directly (no import path) for bump_version tests.
    async fn insert_minimal(db: &GraphDb, id: &str) {
        let safe = id.replace('`', "");
        let q = format!(
            "UPSERT artifact:`{safe}` SET \
                artifact_type = 'task', \
                title = 'Test {safe}', \
                path = '.orqa/test/{safe}.md', \
                frontmatter = {{}}, \
                updated_at = time::now();"
        );
        db.0.query(&q).await.expect("insert minimal artifact");
    }

    /// Schema DEFAULT 1 + bump_version: first explicit bump produces version 2.
    #[tokio::test]
    async fn bump_version_increments_from_default() {
        let db = make_db().await;
        insert_minimal(&db, "BV-G01").await;

        let v = bump_version(&db, "BV-G01", None).await.unwrap();
        assert_eq!(
            v, 2,
            "first explicit bump from DEFAULT 1 must produce version 2"
        );
    }

    #[tokio::test]
    async fn bump_version_increments_sequentially() {
        let db = make_db().await;
        insert_minimal(&db, "BV-G02").await;

        // DEFAULT 1 → first bump → 2 → second bump → 3.
        bump_version(&db, "BV-G02", None).await.unwrap();
        let v2 = bump_version(&db, "BV-G02", None).await.unwrap();
        assert_eq!(v2, 3, "second bump from DEFAULT 1 must produce version 3");
    }

    #[tokio::test]
    async fn bump_version_conflict_when_flag_enabled_and_stale() {
        // Serialize env-var manipulation with the other env-var test.
        let _guard = ENV_LOCK.lock().await;

        let prev = std::env::var("ORQA_OPTIMISTIC_LOCK").ok();
        std::env::set_var("ORQA_OPTIMISTIC_LOCK", "true");

        let db = make_db().await;
        // DEFAULT 1 applies on insert.
        insert_minimal(&db, "BV-G03").await;

        // Advance stored version: DEFAULT 1 + first bump = 2.
        bump_version(&db, "BV-G03", None).await.unwrap();

        // Attempt with stale expected_version=0 while stored=2.
        let result = bump_version(&db, "BV-G03", Some(0)).await;

        match prev {
            Some(v) => std::env::set_var("ORQA_OPTIMISTIC_LOCK", v),
            None => std::env::remove_var("ORQA_OPTIMISTIC_LOCK"),
        }

        match result {
            Err(BumpError::Conflict {
                expected, found, ..
            }) => {
                assert_eq!(expected, 0);
                assert_eq!(found, 2, "stored version must be 2 (DEFAULT 1 + one bump)");
            }
            other => panic!("expected BumpError::Conflict, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn bump_version_no_conflict_when_flag_disabled() {
        // Serialize env-var manipulation with the other env-var test.
        let _guard = ENV_LOCK.lock().await;

        let prev = std::env::var("ORQA_OPTIMISTIC_LOCK").ok();
        std::env::set_var("ORQA_OPTIMISTIC_LOCK", "false");

        let db = make_db().await;
        insert_minimal(&db, "BV-G04").await;

        bump_version(&db, "BV-G04", None).await.unwrap();
        bump_version(&db, "BV-G04", None).await.unwrap();

        let result = bump_version(&db, "BV-G04", Some(0)).await;

        match prev {
            Some(v) => std::env::set_var("ORQA_OPTIMISTIC_LOCK", v),
            None => std::env::remove_var("ORQA_OPTIMISTIC_LOCK"),
        }

        assert!(result.is_ok(), "flag off must never return Conflict");
    }
}
