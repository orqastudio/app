//! SurrealQL graph queries for the artifact graph.
//!
//! Each function executes a SurrealQL query and returns deserialized results.
//! These correspond to the graph metrics in `engine/graph/src/metrics.rs`
//! but expressed as database queries instead of in-memory traversals.

use anyhow::Result;
use surrealdb::types::{RecordId, SurrealValue};

use crate::GraphDb;

/// A single step in a traceability path.
#[derive(Debug, Clone, SurrealValue)]
pub struct TraceStep {
    /// Artifact record ID.
    pub id: RecordId,
    /// Artifact type.
    pub artifact_type: String,
    /// Title of the artifact.
    pub title: String,
}

/// Result of a count-by-group query.
#[derive(Debug, Clone, SurrealValue)]
pub struct GroupCount {
    /// The group key (type or status value).
    pub group: Option<String>,
    /// Number of artifacts in this group.
    pub count: i64,
}

/// An orphan artifact (no incoming or outgoing edges).
#[derive(Debug, Clone, SurrealValue)]
pub struct OrphanArtifact {
    /// The artifact's record ID.
    pub id: RecordId,
    /// The artifact's type.
    pub artifact_type: String,
    /// Title.
    pub title: String,
}

/// Traceability: find paths from an artifact to any pillar/vision artifact.
///
/// Uses SurrealDB's recursive graph traversal with `->relates_to->artifact`
/// at depths 1..15. Returns all reachable pillar/vision artifacts.
///
/// NOTE: SurrealDB 3.x graph traversal returns the set of reachable nodes,
/// not individual paths. For full path enumeration, the in-memory DFS in
/// `engine/graph/src/metrics.rs` is more appropriate. This query answers
/// "can this artifact reach a pillar?" efficiently.
pub async fn trace_to_pillars(
    db: &GraphDb,
    artifact_id: &str,
    root_types: &[&str],
) -> Result<Vec<TraceStep>> {
    let safe_id = artifact_id.replace('`', "");
    let types_list: Vec<String> = root_types.iter().map(|t| format!("'{t}'")).collect();
    let types_in = types_list.join(", ");

    let query = format!(
        "SELECT id, artifact_type, title \
         FROM artifact:`{safe_id}`->relates_to->artifact.{{1..15}} \
         WHERE artifact_type IN [{types_in}];"
    );

    let mut response = db.db.query(&query).await?;
    let results: Vec<TraceStep> = response.take(0)?;
    Ok(results)
}

/// Descendants: find all artifacts reachable via incoming edges.
///
/// Walks `<-relates_to<-artifact` recursively up to 10 hops to find all
/// artifacts that eventually point to the given artifact.
pub async fn trace_descendants(db: &GraphDb, artifact_id: &str) -> Result<Vec<TraceStep>> {
    let safe_id = artifact_id.replace('`', "");

    let query = format!(
        "SELECT id, artifact_type, title \
         FROM artifact:`{safe_id}`<-relates_to<-artifact.{{1..10}};"
    );

    let mut response = db.db.query(&query).await?;
    let results: Vec<TraceStep> = response.take(0)?;
    Ok(results)
}

/// Siblings: find artifacts that share a target via the same relationship type.
///
/// Given artifact A that relates to target T via type R, find all other
/// artifacts B that also relate to T via R.
pub async fn find_siblings(db: &GraphDb, artifact_id: &str) -> Result<Vec<TraceStep>> {
    let safe_id = artifact_id.replace('`', "");

    let query = format!(
        "LET $my_targets = (SELECT out, relationship_type \
             FROM relates_to WHERE in = artifact:`{safe_id}`); \
         SELECT DISTINCT in.id AS id, in.artifact_type AS artifact_type, in.title AS title \
             FROM relates_to \
             WHERE out IN $my_targets.out \
             AND relationship_type IN $my_targets.relationship_type \
             AND in != artifact:`{safe_id}`;"
    );

    let mut response = db.db.query(&query).await?;
    // The sibling results are in statement index 1 (the SELECT after LET).
    let results: Vec<TraceStep> = response.take(1).unwrap_or_default();
    Ok(results)
}

/// Orphans: find all artifacts with zero incoming AND zero outgoing edges.
///
/// Excludes artifacts with terminal statuses (archived, surpassed, completed).
pub async fn find_orphans(db: &GraphDb) -> Result<Vec<OrphanArtifact>> {
    let query = "\
        SELECT id, artifact_type, title \
        FROM artifact \
        WHERE count(->relates_to) = 0 \
            AND count(<-relates_to) = 0 \
            AND status NOT IN ['archived', 'surpassed', 'completed'];";

    let mut response = db.db.query(query).await?;
    let results: Vec<OrphanArtifact> = response.take(0)?;
    Ok(results)
}

/// Average degree: compute mean of (outgoing + incoming edge count) across all artifacts.
pub async fn avg_degree(db: &GraphDb) -> Result<f64> {
    // Collect each artifact's degree, then compute the mean over the array.
    let query = "\
        LET $degrees = (SELECT VALUE (count(->relates_to) + count(<-relates_to)) FROM artifact); \
        RETURN math::mean($degrees);";

    let mut response = db.db.query(query).await?;
    // Statement 0 is the LET, statement 1 is the RETURN.
    let result: Option<f64> = response.take(1)?;
    Ok(result.unwrap_or(0.0))
}

/// Count artifacts grouped by `artifact_type`.
pub async fn count_by_type(db: &GraphDb) -> Result<Vec<GroupCount>> {
    let query = "\
        SELECT artifact_type AS group, count() AS count \
        FROM artifact \
        GROUP BY artifact_type \
        ORDER BY count DESC;";

    let mut response = db.db.query(query).await?;
    let results: Vec<GroupCount> = response.take(0)?;
    Ok(results)
}

/// Count artifacts grouped by `status`.
pub async fn count_by_status(db: &GraphDb) -> Result<Vec<GroupCount>> {
    let query = "\
        SELECT status AS group, count() AS count \
        FROM artifact \
        GROUP BY status \
        ORDER BY count DESC;";

    let mut response = db.db.query(query).await?;
    let results: Vec<GroupCount> = response.take(0)?;
    Ok(results)
}

// NOTE: Connected components (weakly connected components via BFS) cannot be
// expressed as a single SurrealQL query. SurrealDB's graph traversal finds
// all reachable nodes from a starting point, but computing ALL components
// requires iterating over unvisited nodes — a procedural algorithm that
// doesn't map to declarative SurrealQL. The in-memory BFS in
// `engine/graph/src/metrics.rs::compute_components` is the right approach.

/// Total artifact count.
pub async fn total_artifacts(db: &GraphDb) -> Result<usize> {
    let query = "SELECT count() AS total FROM artifact GROUP ALL;";
    let mut response = db.db.query(query).await?;

    #[derive(Clone, SurrealValue)]
    struct CountResult {
        total: i64,
    }

    let result: Option<CountResult> = response.take(0)?;
    Ok(result.map_or(0, |r| r.total as usize))
}

/// Total edge count.
pub async fn total_edges(db: &GraphDb) -> Result<usize> {
    let query = "SELECT count() AS total FROM relates_to GROUP ALL;";
    let mut response = db.db.query(query).await?;

    #[derive(Clone, SurrealValue)]
    struct CountResult {
        total: i64,
    }

    let result: Option<CountResult> = response.take(0)?;
    Ok(result.map_or(0, |r| r.total as usize))
}
