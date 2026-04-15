// Graph analytics routes: statistics and health metrics.
//
// All handlers operate on the cached GraphState. No disk I/O is performed by
// these handlers — the graph is built once at startup and updated by the
// file watcher.
//
// Endpoints:
//   GET  /graph/stats             — summary statistics (nodes, edges, orphans, broken refs)
//   GET  /graph/health            — extended health metrics (components, density, etc.)
//   GET  /graph/health/snapshots  — historical snapshots (not implemented — returns empty)
//   POST /graph/health/snapshots  — store a new snapshot (not implemented — returns 501)
//   GET  /graph/parity            — compare HashMap count vs SurrealDB count

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use orqa_graph::surreal_queries::total_artifacts;
use orqa_validation::graph::graph_stats;
use orqa_validation::metrics::compute_health;
use orqa_validation::types::GraphHealth;
use orqa_validation::GraphStats;
use orqa_validation::PipelineCategories;

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Response shapes
// ---------------------------------------------------------------------------

/// Response body for GET /graph/stats.
#[derive(Debug, Serialize)]
pub struct GraphStatsResponse {
    /// Number of artifact nodes in the graph.
    pub node_count: usize,
    /// Total number of directed relationship edges.
    pub edge_count: usize,
    /// Artifacts with no outgoing or incoming references.
    pub orphan_count: usize,
    /// Number of references to non-existent artifact IDs.
    pub broken_refs: usize,
}

/// Request body for POST /graph/health/snapshots.
///
/// Fields are captured now and will be persisted by the A4 snapshot store.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct HealthSnapshotRequest {
    pub error_count: u32,
    pub warning_count: u32,
    pub node_count: usize,
    pub edge_count: usize,
    pub orphan_count: usize,
    pub health_score: f64,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /graph/stats — return summary statistics about the artifact graph.
///
/// All statistics are computed against the cached graph — no disk I/O.
pub async fn get_graph_stats(State(state): State<GraphState>) -> Json<GraphStatsResponse> {
    let Ok(guard) = state.0.read() else {
        return Json(GraphStatsResponse {
            node_count: 0,
            edge_count: 0,
            orphan_count: 0,
            broken_refs: 0,
        });
    };

    let stats: GraphStats = graph_stats(&guard.graph);

    Json(GraphStatsResponse {
        node_count: stats.node_count,
        edge_count: stats.edge_count,
        orphan_count: stats.orphan_count,
        broken_refs: stats.broken_ref_count,
    })
}

/// Handle GET /graph/health — compute extended health metrics from the cached graph.
///
/// `compute_health` performs graph-theoretic analysis: connected components,
/// orphan percentage, average degree, density, and pillar traceability.
pub async fn get_graph_health(State(state): State<GraphState>) -> Json<GraphHealth> {
    let Ok(guard) = state.0.read() else {
        return Json(GraphHealth::default());
    };

    let owned = guard.owned_pipeline_categories();
    let (d, l, es, et, rt) = owned.as_str_vecs();
    let health = compute_health(
        &guard.graph,
        &PipelineCategories {
            delivery: &d,
            learning: &l,
            excluded_statuses: &es,
            excluded_types: &et,
            root_types: &rt,
        },
    );
    Json(health)
}

/// Handle GET /graph/health/snapshots — return historical health snapshots.
///
/// Historical snapshot storage is implemented in a later task (A4). For now this
/// returns an empty array so clients can query without errors.
pub async fn list_health_snapshots() -> Json<Vec<serde_json::Value>> {
    Json(Vec::new())
}

/// Handle POST /graph/health/snapshots — store a new health snapshot.
///
/// Snapshot persistence is implemented in a later task (A4). Returns 501 until
/// the SQLite snapshot store is wired up.
pub async fn create_health_snapshot(Json(_req): Json<HealthSnapshotRequest>) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

// ---------------------------------------------------------------------------
// Parity validation
// ---------------------------------------------------------------------------

/// Response body for GET /graph/parity.
#[derive(Debug, Serialize)]
pub struct GraphParityResponse {
    /// Artifact count from the in-memory HashMap graph.
    pub hashmap_count: usize,
    /// Artifact count from SurrealDB (materialized view).
    /// `None` when SurrealDB is unavailable (failed to initialize).
    pub surreal_count: Option<usize>,
    /// `true` when both counts are equal (or SurrealDB is unavailable).
    ///
    /// `false` means the materialized view is out of sync — a manual
    /// `bulk_sync` or daemon restart may be required.
    pub in_sync: bool,
    /// `true` when SurrealDB was successfully initialized on startup.
    pub surreal_available: bool,
}

/// Handle GET /graph/parity — compare the in-memory HashMap against SurrealDB.
///
/// Returns the artifact count from both sources and a boolean `in_sync` flag.
/// A mismatch (`in_sync: false`) signals that the SurrealDB materialized view
/// has drifted from the file-backed HashMap — typically caused by a crash or
/// mid-flight sync error. Clients may use this to display a data-integrity
/// warning or trigger a manual resync.
///
/// When SurrealDB is unavailable, `surreal_count` is `null` and `in_sync` is
/// `true` (no contradiction can be detected without the second source).
pub async fn get_graph_parity(State(state): State<GraphState>) -> Json<GraphParityResponse> {
    let (hashmap_count, surreal_db) = match state.0.read() {
        Ok(guard) => (guard.graph.nodes.len(), guard.db.clone()),
        Err(_) => {
            return Json(GraphParityResponse {
                hashmap_count: 0,
                surreal_count: None,
                in_sync: true,
                surreal_available: false,
            });
        }
    };

    let Some(db) = surreal_db else {
        return Json(GraphParityResponse {
            hashmap_count,
            surreal_count: None,
            in_sync: true,
            surreal_available: false,
        });
    };

    match total_artifacts(&db).await {
        Ok(surreal_count) => Json(GraphParityResponse {
            in_sync: hashmap_count == surreal_count,
            hashmap_count,
            surreal_count: Some(surreal_count),
            surreal_available: true,
        }),
        Err(_) => Json(GraphParityResponse {
            hashmap_count,
            surreal_count: None,
            in_sync: true,
            surreal_available: true, // DB exists but query failed
        }),
    }
}
