//! Version-bump helper for the artifact graph proof-of-concept crate.
//!
//! Provides `bump_version()` — an atomic helper that every writer calls
//! immediately before committing a mutation. It increments the `version`
//! counter and sets `updated_at = time::now()` in a single UPSERT round-trip.
//!
//! Optimistic-lock enforcement is controlled by the `ORQA_OPTIMISTIC_LOCK`
//! environment variable:
//!
//! - `ORQA_OPTIMISTIC_LOCK=false` (default, MVP): version is bumped but NOT
//!   checked. Concurrent writes last-writer-wins.
//! - `ORQA_OPTIMISTIC_LOCK=true` (future, deferred): the caller must supply the
//!   `expected_version` they read before their write. If the stored version does
//!   not match, `bump_version` returns `BumpError::Conflict` which the HTTP layer
//!   maps to 409 Conflict.
//!
//! The 409 path exists and is tested even though it is unreachable in MVP.

use anyhow::Context;

use crate::GraphDb;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors that can arise from `bump_version`.
#[derive(Debug, thiserror::Error)]
pub enum BumpError {
    /// Optimistic-lock mismatch: stored version != expected_version.
    ///
    /// The HTTP layer maps this variant to HTTP 409 Conflict.
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
// bump_version
// ---------------------------------------------------------------------------

/// Read the current `version` stored for an artifact, returning `0` if absent.
///
/// Used by `bump_version` to fetch the pre-bump version when the flag is off
/// and no expected_version has been supplied by the caller.
async fn read_version(db: &GraphDb, safe_id: &str) -> Result<u64, BumpError> {
    let query = format!("SELECT version FROM artifact:`{safe_id}` LIMIT 1;");
    let mut response = db
        .db
        .query(&query)
        .await
        .context("reading version")
        .map_err(|e| BumpError::Other(e.to_string()))?;
    let rows: Vec<serde_json::Value> = response
        .take(0)
        .context("reading version rows")
        .map_err(|e| BumpError::Other(e.to_string()))?;
    let version = rows
        .first()
        .and_then(|r| r.get("version"))
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    Ok(version)
}

/// Atomically increment `version` and set `updated_at = time::now()` for an artifact.
///
/// Every writer (upsert, soft-delete, ingest) must call this helper immediately
/// before committing its mutation. The helper performs a single UPSERT that
/// contains both the version increment and the timestamp update so that no
/// partial state is observable.
///
/// # Optimistic-lock behaviour
///
/// When `ORQA_OPTIMISTIC_LOCK=false` (MVP default): bumps unconditionally.
/// `expected_version` is ignored — pass `None`.
///
/// When `ORQA_OPTIMISTIC_LOCK=true` (future): `expected_version` must be
/// `Some(v)` where `v` is the version the caller read before computing its
/// mutation. If the stored version has advanced past `v`, `BumpError::Conflict`
/// is returned and the caller should surface HTTP 409.
///
/// Returns the new version value on success.
pub async fn bump_version(
    db: &GraphDb,
    artifact_id: &str,
    expected_version: Option<u64>,
) -> Result<u64, BumpError> {
    let safe_id = artifact_id.replace('`', "");

    if optimistic_lock_enabled() {
        // Enforced path — check version before bumping.
        let stored = read_version(db, &safe_id).await?;
        let expected = expected_version.unwrap_or(stored);
        if stored != expected {
            return Err(BumpError::Conflict {
                artifact_id: artifact_id.to_owned(),
                expected,
                found: stored,
            });
        }
        let new_version = stored + 1;
        apply_bump(db, &safe_id, new_version).await?;
        Ok(new_version)
    } else {
        // Unenforced path (MVP default) — read current then bump.
        let stored = read_version(db, &safe_id).await?;
        let new_version = stored + 1;
        apply_bump(db, &safe_id, new_version).await?;
        Ok(new_version)
    }
}

/// Issue the UPSERT that writes `version = new_version` and `updated_at = time::now()`.
///
/// This is a minimal UPSERT touching only the two version-tracking fields so that
/// it composes cleanly with the caller's own field writes. The caller owns the
/// remaining fields.
async fn apply_bump(db: &GraphDb, safe_id: &str, new_version: u64) -> Result<(), BumpError> {
    let query = format!(
        "UPSERT artifact:`{safe_id}` SET \
            version = {new_version}, \
            updated_at = time::now();"
    );
    db.db
        .query(&query)
        .await
        .with_context(|| format!("applying version bump for {safe_id}"))
        .map_err(|e| BumpError::Other(e.to_string()))?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::initialize_schema;
    use tokio::sync::Mutex;

    // Serializes tests that mutate the process-wide ORQA_OPTIMISTIC_LOCK env var.
    // Without this guard, parallel test threads race on set/remove and produce
    // non-deterministic failures.
    // Uses `tokio::sync::Mutex` (async-aware) so the guard can be held across
    // `.await` without triggering `clippy::await_holding_lock`.
    static ENV_LOCK: Mutex<()> = Mutex::const_new(());

    /// Insert a minimal artifact so bump_version has a record to update.
    ///
    /// Does NOT set `version` explicitly; the schema DEFAULT 1 applies.
    async fn insert_artifact(db: &GraphDb, id: &str) {
        let safe = id.replace('`', "");
        let query = format!(
            "UPSERT artifact:`{safe}` SET \
                artifact_type = 'task', \
                title = 'Test', \
                path = '.orqa/test/{safe}.md', \
                frontmatter = {{}}, \
                updated_at = time::now();"
        );
        db.db.query(&query).await.expect("insert artifact");
    }

    /// Schema DEFAULT 1 means a freshly inserted record starts at version=1.
    /// The first bump_version call reads 1 and produces 2.
    #[tokio::test]
    async fn bump_version_increments_from_default() {
        let db = GraphDb::open_memory().await.unwrap();
        initialize_schema(&db).await.unwrap();
        insert_artifact(&db, "TASK-BV01").await;

        let new_version = bump_version(&db, "TASK-BV01", None)
            .await
            .expect("bump must succeed");
        assert_eq!(
            new_version, 2,
            "first explicit bump from DEFAULT 1 must produce version 2"
        );
    }

    #[tokio::test]
    async fn bump_version_increments_sequentially() {
        let db = GraphDb::open_memory().await.unwrap();
        initialize_schema(&db).await.unwrap();
        insert_artifact(&db, "TASK-BV02").await;

        // DEFAULT 1 → first bump → 2 → second bump → 3.
        bump_version(&db, "TASK-BV02", None)
            .await
            .expect("first bump");
        let v2 = bump_version(&db, "TASK-BV02", None)
            .await
            .expect("second bump");
        assert_eq!(v2, 3, "second bump from DEFAULT 1 must produce version 3");
    }

    #[tokio::test]
    async fn conflict_returned_when_flag_enabled_and_version_stale() {
        // Serialize env-var mutation with the other env-var test so parallel
        // cargo test threads cannot see each other's set/remove calls.
        let _guard = ENV_LOCK.lock().await;

        let prev = std::env::var("ORQA_OPTIMISTIC_LOCK").ok();
        std::env::set_var("ORQA_OPTIMISTIC_LOCK", "true");

        let db = GraphDb::open_memory().await.unwrap();
        initialize_schema(&db).await.unwrap();
        // DEFAULT 1 applies on insert.
        insert_artifact(&db, "TASK-BV03").await;

        // Advance stored version: DEFAULT 1 + first bump = 2.
        bump_version(&db, "TASK-BV03", None)
            .await
            .expect("first bump must succeed");

        // Attempt with a stale expected_version=0 while stored=2.
        let result = bump_version(&db, "TASK-BV03", Some(0)).await;

        // Restore the env var before asserting so failures don't poison state.
        match prev {
            Some(v) => std::env::set_var("ORQA_OPTIMISTIC_LOCK", v),
            None => std::env::remove_var("ORQA_OPTIMISTIC_LOCK"),
        }

        match result {
            Err(BumpError::Conflict {
                expected, found, ..
            }) => {
                assert_eq!(expected, 0, "expected version must be 0");
                assert_eq!(found, 2, "found version must be 2 (DEFAULT 1 + one bump)");
            }
            other => panic!("expected Conflict, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn no_conflict_when_flag_disabled() {
        // Serialize env-var mutation with the conflict test.
        let _guard = ENV_LOCK.lock().await;

        let prev = std::env::var("ORQA_OPTIMISTIC_LOCK").ok();
        std::env::set_var("ORQA_OPTIMISTIC_LOCK", "false");

        let db = GraphDb::open_memory().await.unwrap();
        initialize_schema(&db).await.unwrap();
        insert_artifact(&db, "TASK-BV04").await;

        // DEFAULT 1 + two bumps = 3.
        bump_version(&db, "TASK-BV04", None).await.unwrap();
        bump_version(&db, "TASK-BV04", None).await.unwrap();

        // Even with a stale expected_version, no conflict when flag is off.
        let result = bump_version(&db, "TASK-BV04", Some(0)).await;

        match prev {
            Some(v) => std::env::set_var("ORQA_OPTIMISTIC_LOCK", v),
            None => std::env::remove_var("ORQA_OPTIMISTIC_LOCK"),
        }

        assert!(
            result.is_ok(),
            "flag off must never return Conflict, got: {result:?}"
        );
    }
}
