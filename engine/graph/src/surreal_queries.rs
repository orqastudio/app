//! SurrealQL-based graph query functions for the orqa-graph crate.
//!
//! This module is promoted from the `engine/graph-db` proof-of-concept. Each
//! function executes a SurrealQL query against a live `GraphDb` connection and
//! returns typed results. The queries use SurrealDB 3.x syntax including
//! graph traversal (`->relates_to->artifact`) and inline edge counting
//! (`count(->relates_to)`).
//!
//! NOTE: SurrealDB 3.x depth-range syntax (`->relates_to->artifact.{1..15}`)
//! does not work correctly in the embedded engine — it returns empty results.
//! All traversal queries use simple hop syntax with WHERE filters instead.
//!
//! All functions take `&GraphDb` and return `anyhow::Result<T>` so they compose
//! cleanly with the rest of the engine.

use anyhow::Result;
use serde::Serialize;
use surrealdb::types::{RecordId, SurrealValue};

use crate::surreal::GraphDb;

// ---------------------------------------------------------------------------
// Return types
// ---------------------------------------------------------------------------

/// A single step in a traceability path — one artifact reachable via graph traversal.
#[derive(Debug, Clone, Serialize, SurrealValue)]
pub struct TraceStep {
    /// SurrealDB record ID of the artifact (e.g. `artifact:EPIC-001`).
    pub id: RecordId,
    /// Semantic type of the artifact (e.g. `"epic"`, `"pillar"`).
    pub artifact_type: String,
    /// Human-readable title of the artifact.
    pub title: String,
}

/// Result of a count-by-group query — a label and the number of artifacts in that group.
#[derive(Debug, Clone, SurrealValue)]
pub struct GroupCount {
    /// The group key: artifact type or status value. `None` when the field is absent.
    pub group: Option<String>,
    /// Number of artifacts in this group.
    pub count: i64,
}

/// An artifact with zero incoming and zero outgoing edges.
#[derive(Debug, Clone, Serialize, SurrealValue)]
pub struct OrphanArtifact {
    /// SurrealDB record ID (e.g. `artifact:ORPHAN-001`).
    pub id: RecordId,
    /// Semantic type of the artifact.
    pub artifact_type: String,
    /// Human-readable title.
    pub title: String,
    /// Relative path to the source markdown file.
    pub path: String,
}

/// A full artifact record returned by list and search queries.
///
/// Used by `list_artifacts` and `search_artifacts` to return artifact data
/// to route handlers without exposing the full frontmatter blob.
#[derive(Debug, Clone, SurrealValue)]
pub struct ArtifactRecord {
    /// SurrealDB record ID (e.g. `artifact:EPIC-001`).
    pub id: RecordId,
    /// Semantic type of the artifact (e.g. `"epic"`, `"pillar"`).
    pub artifact_type: String,
    /// Human-readable title of the artifact.
    pub title: String,
    /// Lifecycle status of the artifact (e.g. `"active"`, `"archived"`).
    pub status: Option<String>,
    /// Relative path to the source markdown file.
    pub path: String,
}

impl ArtifactRecord {
    /// Extract the string key from the record ID (e.g. `"EPIC-001"` from `artifact:EPIC-001`).
    ///
    /// Artifact keys are always stored as `RecordIdKey::String` variants. Falls back to an
    /// empty string if the key variant is unexpected.
    pub fn id_key(&self) -> &str {
        use surrealdb::types::RecordIdKey;
        match &self.id.key {
            RecordIdKey::String(s) => s.as_str(),
            _ => "",
        }
    }
}

// ---------------------------------------------------------------------------
// Traceability queries
// ---------------------------------------------------------------------------

/// Find all pillar/vision artifacts reachable from `artifact_id` via outgoing edges.
///
/// Performs a single-hop graph traversal (`->relates_to->artifact`) and filters
/// for nodes whose `artifact_type` is in `["pillar", "vision"]` — the root types
/// in the OrqaStudio traceability model.
///
/// NOTE: SurrealDB 3.x depth-range syntax (`->relates_to->artifact.{1..15}`)
/// returns empty results in the embedded engine. This query uses simple one-hop
/// traversal. Multi-hop traceability is handled by the in-memory DFS in
/// `engine/graph/src/metrics.rs`.
pub async fn trace_to_pillars(db: &GraphDb, artifact_id: &str) -> Result<Vec<TraceStep>> {
    let safe_id = artifact_id.replace('`', "");
    let query = format!(
        "SELECT id, artifact_type, title \
         FROM artifact:`{safe_id}`->relates_to->artifact \
         WHERE artifact_type IN ['pillar', 'vision'];"
    );

    let mut response = db.0.query(&query).await?;
    let results: Vec<TraceStep> = response.take(0)?;
    Ok(results)
}

/// Find all artifacts that point to `artifact_id` via incoming edges.
///
/// Performs a single-hop reverse traversal (`<-relates_to<-artifact`) to return
/// all artifacts that directly relate to the given artifact. This is the inverse
/// of `trace_to_pillars` — starting from a root and finding its direct sources.
///
/// NOTE: SurrealDB 3.x depth-range syntax (`<-relates_to<-artifact.{1..10}`)
/// returns empty results in the embedded engine. Multi-hop ancestor traversal
/// is handled by the in-memory DFS in `engine/graph/src/metrics.rs`.
pub async fn trace_descendants(db: &GraphDb, artifact_id: &str) -> Result<Vec<TraceStep>> {
    let safe_id = artifact_id.replace('`', "");
    let query = format!(
        "SELECT id, artifact_type, title \
         FROM artifact:`{safe_id}`<-relates_to<-artifact;"
    );

    let mut response = db.0.query(&query).await?;
    let results: Vec<TraceStep> = response.take(0)?;
    Ok(results)
}

/// Find artifacts that share a common target as `artifact_id` via the same relationship type.
///
/// Given artifact A that relates_to target T via type R, returns all other artifacts B
/// that also relate_to T via R. Excludes `artifact_id` itself from the results.
/// Uses a LET binding to capture the targets and relationship types of the source artifact,
/// then selects other artifacts sharing those same (target, type) pairs.
pub async fn find_siblings(db: &GraphDb, artifact_id: &str) -> Result<Vec<TraceStep>> {
    let safe_id = artifact_id.replace('`', "");
    let query = format!(
        "LET $my_targets = (SELECT out, relation_type \
             FROM relates_to WHERE in = artifact:`{safe_id}`); \
         SELECT DISTINCT in.id AS id, in.artifact_type AS artifact_type, in.title AS title \
             FROM relates_to \
             WHERE out IN $my_targets.out \
             AND relation_type IN $my_targets.relation_type \
             AND in != artifact:`{safe_id}`;"
    );

    let mut response = db.0.query(&query).await?;
    // The sibling SELECT is statement index 1 (statement 0 is the LET).
    let results: Vec<TraceStep> = response.take(1).unwrap_or_default();
    Ok(results)
}

// ---------------------------------------------------------------------------
// Health / structural queries
// ---------------------------------------------------------------------------

/// Find all artifacts with zero outgoing AND zero incoming edges.
///
/// Excludes artifacts with terminal statuses (`archived`, `surpassed`, `completed`)
/// since those are expected to have no active relationships.
/// Uses SurrealDB's inline edge count syntax (`count(->relates_to)`) which is
/// evaluated per-row and does not require a subquery join.
pub async fn find_orphans(db: &GraphDb) -> Result<Vec<OrphanArtifact>> {
    let query = "\
        SELECT id, artifact_type, title, path \
        FROM artifact \
        WHERE count(->relates_to) = 0 \
            AND count(<-relates_to) = 0 \
            AND status NOT IN ['archived', 'surpassed', 'completed'];";

    let mut response = db.0.query(query).await?;
    let results: Vec<OrphanArtifact> = response.take(0)?;
    Ok(results)
}

/// Compute the mean edge degree across all artifacts.
///
/// Degree is defined as `(outgoing edges + incoming edges)` for each artifact.
/// Uses `math::mean` on the collected degree array. Returns `0.0` when there
/// are no artifacts in the graph.
pub async fn avg_degree(db: &GraphDb) -> Result<f64> {
    let query = "\
        LET $degrees = (SELECT VALUE (count(->relates_to) + count(<-relates_to)) FROM artifact); \
        RETURN math::mean($degrees);";

    let mut response = db.0.query(query).await?;
    // Statement 0 is the LET binding; statement 1 is the RETURN value.
    let result: Option<f64> = response.take(1)?;
    Ok(result.unwrap_or(0.0))
}

// ---------------------------------------------------------------------------
// Aggregate / count queries
// ---------------------------------------------------------------------------

/// Count artifacts grouped by `artifact_type`, ordered by count descending.
///
/// Returns one `GroupCount` per distinct type found in the artifact table.
/// The `group` field holds the type string; `None` means the artifact has no type.
pub async fn count_by_type(db: &GraphDb) -> Result<Vec<GroupCount>> {
    let query = "\
        SELECT artifact_type AS group, count() AS count \
        FROM artifact \
        GROUP BY artifact_type \
        ORDER BY count DESC;";

    let mut response = db.0.query(query).await?;
    let results: Vec<GroupCount> = response.take(0)?;
    Ok(results)
}

/// Count artifacts grouped by `status`, ordered by count descending.
///
/// Returns one `GroupCount` per distinct status value. The `group` field holds
/// the status string; `None` when an artifact has no status set.
pub async fn count_by_status(db: &GraphDb) -> Result<Vec<GroupCount>> {
    let query = "\
        SELECT status AS group, count() AS count \
        FROM artifact \
        GROUP BY status \
        ORDER BY count DESC;";

    let mut response = db.0.query(query).await?;
    let results: Vec<GroupCount> = response.take(0)?;
    Ok(results)
}

/// Return the total number of artifact nodes in the graph.
///
/// Uses `GROUP ALL` to reduce all rows to a single aggregate row.
/// Returns `0` when the table is empty.
pub async fn total_artifacts(db: &GraphDb) -> Result<usize> {
    let query = "SELECT count() AS total FROM artifact GROUP ALL;";
    let mut response = db.0.query(query).await?;

    // Local newtype to deserialize the single aggregate row.
    #[derive(Clone, SurrealValue)]
    struct CountResult {
        total: i64,
    }

    let result: Option<CountResult> = response.take(0)?;
    Ok(result.map_or(0, |r| r.total as usize))
}

/// Return the total number of relationship edges in the graph.
///
/// Counts all rows in the `relates_to` edge table. Returns `0` when empty.
pub async fn total_edges(db: &GraphDb) -> Result<usize> {
    let query = "SELECT count() AS total FROM relates_to GROUP ALL;";
    let mut response = db.0.query(query).await?;

    // Local newtype to deserialize the single aggregate row.
    #[derive(Clone, SurrealValue)]
    struct CountResult {
        total: i64,
    }

    let result: Option<CountResult> = response.take(0)?;
    Ok(result.map_or(0, |r| r.total as usize))
}

// ---------------------------------------------------------------------------
// List and search queries
// ---------------------------------------------------------------------------

/// List all artifacts from SurrealDB, with optional filters.
///
/// Filters are applied as exact case-sensitive matches. Both filters may be
/// combined; `None` means no constraint on that field. Results are ordered
/// by `artifact_type ASC, title ASC` for stable pagination.
pub async fn list_artifacts(
    db: &GraphDb,
    artifact_type: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<ArtifactRecord>> {
    // Build the WHERE clause dynamically so we avoid runtime SurrealQL
    // conditional logic and keep each code path simple and auditable.
    let where_clause = match (artifact_type, status) {
        (Some(t), Some(s)) => format!(
            "WHERE artifact_type = '{}' AND status = '{}'",
            t.replace('\'', "\\'"),
            s.replace('\'', "\\'")
        ),
        (Some(t), None) => format!("WHERE artifact_type = '{}'", t.replace('\'', "\\'")),
        (None, Some(s)) => format!("WHERE status = '{}'", s.replace('\'', "\\'")),
        (None, None) => String::new(),
    };

    let query = format!(
        "SELECT id, artifact_type, title, status, path \
         FROM artifact \
         {where_clause} \
         ORDER BY artifact_type ASC, title ASC;"
    );

    let mut response = db.0.query(&query).await?;
    let results: Vec<ArtifactRecord> = response.take(0)?;
    Ok(results)
}

/// Search artifacts by title using a case-insensitive substring match.
///
/// Uses SurrealDB's `string::lowercase` on both the stored title and the query
/// string so that `"EPIC"` matches `"Epic One"`. The query is bound as a
/// parameter `$q` to prevent injection. Results are ordered by `title ASC`.
pub async fn search_artifacts(db: &GraphDb, query: &str) -> Result<Vec<ArtifactRecord>> {
    let surql = "SELECT id, artifact_type, title, status, path \
                 FROM artifact \
                 WHERE string::lowercase(title) CONTAINS string::lowercase($q) \
                 ORDER BY title ASC;";

    let mut response = db.0.query(surql).bind(("q", query.to_owned())).await?;
    let results: Vec<ArtifactRecord> = response.take(0)?;
    Ok(results)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surreal::{initialize_schema, open_memory};
    use crate::sync::bulk_sync;
    use tempfile::TempDir;

    /// Build an in-memory `GraphDb` with five test artifacts:
    /// two epics pointing to a pillar, one pillar, one vision (no edges), one orphan idea.
    ///
    /// Uses `bulk_sync` (two-pass: nodes then edges) to ensure edges are created
    /// even when targets are written after their sources.
    async fn setup_db_with_artifacts() -> (GraphDb, TempDir) {
        let db = open_memory().await.unwrap();
        initialize_schema(&db).await.unwrap();
        let dir = tempfile::tempdir().unwrap();
        let orqa_dir = dir.path().join(".orqa");
        std::fs::create_dir_all(&orqa_dir).unwrap();

        // Write all artifact files before syncing so bulk_sync can resolve all targets.
        let artifacts: &[(&str, &str)] = &[
            (
                "EPIC-001.md",
                "---\nid: EPIC-001\ntype: epic\ntitle: Epic One\nstatus: active\nrelationships:\n  - type: delivers\n    target: PILLAR-001\n---\n",
            ),
            (
                "EPIC-002.md",
                "---\nid: EPIC-002\ntype: epic\ntitle: Epic Two\nstatus: active\nrelationships:\n  - type: delivers\n    target: PILLAR-001\n---\n",
            ),
            (
                "PILLAR-001.md",
                "---\nid: PILLAR-001\ntype: pillar\ntitle: Pillar One\nstatus: active\n---\n",
            ),
            (
                "VISION-001.md",
                "---\nid: VISION-001\ntype: vision\ntitle: Vision\nstatus: active\n---\n",
            ),
            (
                "ORPHAN-001.md",
                "---\nid: ORPHAN-001\ntype: idea\ntitle: Orphan\nstatus: active\n---\n",
            ),
        ];

        for (filename, content) in artifacts {
            std::fs::write(orqa_dir.join(filename), content).unwrap();
        }

        // Two-pass bulk sync: nodes first, then edges — handles forward references.
        bulk_sync(&db, dir.path()).await.unwrap();

        (db, dir)
    }

    #[tokio::test]
    async fn test_total_artifacts() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let count = total_artifacts(&db).await.unwrap();
        assert_eq!(count, 5);
    }

    #[tokio::test]
    async fn test_total_edges() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let count = total_edges(&db).await.unwrap();
        // EPIC-001 -> PILLAR-001 and EPIC-002 -> PILLAR-001
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_find_orphans() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let orphans = find_orphans(&db).await.unwrap();
        // ORPHAN-001 and VISION-001 have no edges.
        // PILLAR-001 has two incoming edges so it is NOT an orphan.
        let orphan_ids: Vec<String> = orphans.iter().map(|o| format!("{:?}", o.id)).collect();
        assert!(
            orphan_ids
                .iter()
                .any(|id: &String| id.contains("ORPHAN-001")),
            "expected ORPHAN-001 in orphan list, got: {orphan_ids:?}"
        );
        assert!(
            orphan_ids
                .iter()
                .any(|id: &String| id.contains("VISION-001")),
            "expected VISION-001 in orphan list, got: {orphan_ids:?}"
        );
        // PILLAR-001 has incoming edges and must not appear.
        assert!(
            !orphan_ids
                .iter()
                .any(|id: &String| id.contains("PILLAR-001")),
            "PILLAR-001 should not be an orphan but appears in: {orphan_ids:?}"
        );
    }

    #[tokio::test]
    async fn test_count_by_type() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let counts = count_by_type(&db).await.unwrap();
        assert!(!counts.is_empty());
        let epic_count = counts
            .iter()
            .find(|g| g.group.as_deref() == Some("epic"))
            .map_or(0, |g| g.count);
        assert_eq!(epic_count, 2);
    }

    #[tokio::test]
    async fn test_count_by_status() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let counts = count_by_status(&db).await.unwrap();
        // All 5 artifacts have status = "active".
        let active_count = counts
            .iter()
            .find(|g| g.group.as_deref() == Some("active"))
            .map_or(0, |g| g.count);
        assert_eq!(active_count, 5);
    }

    #[tokio::test]
    async fn test_avg_degree() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let degree = avg_degree(&db).await.unwrap();
        // EPIC-001: 1 out, 0 in = 1
        // EPIC-002: 1 out, 0 in = 1
        // PILLAR-001: 0 out, 2 in = 2
        // VISION-001: 0 out, 0 in = 0
        // ORPHAN-001: 0 out, 0 in = 0
        // mean([1,1,2,0,0]) = 4/5 = 0.8
        assert!(
            (degree - 0.8).abs() < 0.001,
            "expected avg_degree ≈ 0.8, got {degree}"
        );
    }

    #[tokio::test]
    async fn test_trace_descendants() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let descendants = trace_descendants(&db, "PILLAR-001").await.unwrap();
        // EPIC-001 and EPIC-002 both point to PILLAR-001.
        let ids: Vec<String> = descendants.iter().map(|d| format!("{:?}", d.id)).collect();
        assert!(
            ids.iter().any(|id: &String| id.contains("EPIC-001")),
            "expected EPIC-001 in descendants of PILLAR-001, got: {ids:?}"
        );
        assert!(
            ids.iter().any(|id: &String| id.contains("EPIC-002")),
            "expected EPIC-002 in descendants of PILLAR-001, got: {ids:?}"
        );
    }

    #[tokio::test]
    async fn test_trace_to_pillars() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let pillars = trace_to_pillars(&db, "EPIC-001").await.unwrap();
        // EPIC-001 -> PILLAR-001 (type: pillar) should be found.
        let ids: Vec<String> = pillars.iter().map(|p| format!("{:?}", p.id)).collect();
        assert!(
            ids.iter().any(|id: &String| id.contains("PILLAR-001")),
            "expected PILLAR-001 reachable from EPIC-001, got: {ids:?}"
        );
    }

    #[tokio::test]
    async fn test_list_artifacts_all() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let records = list_artifacts(&db, None, None).await.unwrap();
        assert_eq!(
            records.len(),
            5,
            "expected all 5 artifacts, got: {records:?}"
        );
    }

    #[tokio::test]
    async fn test_list_artifacts_by_type() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let records = list_artifacts(&db, Some("epic"), None).await.unwrap();
        assert_eq!(records.len(), 2, "expected 2 epics, got: {records:?}");
        let ids: Vec<String> = records.iter().map(|r| format!("{:?}", r.id)).collect();
        assert!(
            ids.iter().any(|id| id.contains("EPIC-001")),
            "expected EPIC-001 in results, got: {ids:?}"
        );
        assert!(
            ids.iter().any(|id| id.contains("EPIC-002")),
            "expected EPIC-002 in results, got: {ids:?}"
        );
    }

    #[tokio::test]
    async fn test_list_artifacts_by_status() {
        let (db, _dir) = setup_db_with_artifacts().await;
        let records = list_artifacts(&db, None, Some("active")).await.unwrap();
        assert_eq!(
            records.len(),
            5,
            "expected all 5 active artifacts, got: {records:?}"
        );
    }

    #[tokio::test]
    async fn test_search_artifacts() {
        let (db, _dir) = setup_db_with_artifacts().await;
        // Titles are "Epic One" and "Epic Two" — lowercase "epic" should match both.
        let records = search_artifacts(&db, "epic").await.unwrap();
        assert_eq!(
            records.len(),
            2,
            "expected 2 results for 'epic', got: {records:?}"
        );
        let ids: Vec<String> = records.iter().map(|r| format!("{:?}", r.id)).collect();
        assert!(
            ids.iter().any(|id| id.contains("EPIC-001")),
            "expected EPIC-001 in search results, got: {ids:?}"
        );
        assert!(
            ids.iter().any(|id| id.contains("EPIC-002")),
            "expected EPIC-002 in search results, got: {ids:?}"
        );
    }

    #[tokio::test]
    async fn test_search_artifacts_case_insensitive() {
        let (db, _dir) = setup_db_with_artifacts().await;
        // Uppercase query "EPIC" must match the same two artifacts as lowercase "epic".
        let records = search_artifacts(&db, "EPIC").await.unwrap();
        assert_eq!(
            records.len(),
            2,
            "expected 2 results for 'EPIC', got: {records:?}"
        );
        let ids: Vec<String> = records.iter().map(|r| format!("{:?}", r.id)).collect();
        assert!(
            ids.iter().any(|id| id.contains("EPIC-001")),
            "expected EPIC-001 in case-insensitive search results, got: {ids:?}"
        );
        assert!(
            ids.iter().any(|id| id.contains("EPIC-002")),
            "expected EPIC-002 in case-insensitive search results, got: {ids:?}"
        );
    }
}
