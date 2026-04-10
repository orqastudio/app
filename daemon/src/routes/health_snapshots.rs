// Health snapshot routes: persist and retrieve point-in-time artifact graph health metrics.
//
// Health snapshots record graph health metrics for trend analysis in the governance
// dashboard. Each snapshot captures counts, connectivity ratios, and outlier
// statistics for a project at a specific moment.
//
// Endpoints:
//   POST /health-snapshots                — store a new snapshot for a project
//   GET  /health-snapshots/:id            — get a single snapshot by ID
//   GET  /health-snapshots                — get the most recent N snapshots for a project

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use orqa_engine_types::types::health::{HealthSnapshot, NewHealthSnapshot};
use orqa_storage::traits::HealthRepository as _;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Query parameters for GET /health-snapshots.
#[derive(Debug, Deserialize)]
pub struct ListSnapshotsQuery {
    /// Project to query snapshots for.
    pub project_id: i64,
    /// Maximum number of snapshots to return (default 20, max 100).
    pub limit: Option<i64>,
}

/// Response helper when the storage layer is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "health snapshot store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /health-snapshots — persist a new health snapshot for a project.
///
/// Returns the inserted snapshot with its assigned ID and timestamp.
pub async fn create_health_snapshot(
    State(state): State<HealthState>,
    Json(req): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<HealthSnapshot>), (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    let project_id = req["project_id"].as_i64().ok_or_else(|| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": "project_id is required", "code": "MISSING_FIELD" })),
        )
    })?;

    let snapshot: NewHealthSnapshot = serde_json::from_value(req).map_err(|e| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": e.to_string(), "code": "INVALID_BODY" })),
        )
    })?;

    storage
        .health()
        .create(project_id, &snapshot)
        .await
        .map(|s| (StatusCode::CREATED, Json(s)))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "CREATE_FAILED" })),
            )
        })
}

/// Handle GET /health-snapshots/:id — get a single snapshot by its row ID.
pub async fn get_health_snapshot(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<Json<HealthSnapshot>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage.health().get(id).await.map(Json).map_err(|e| {
        let (status, code) =
            if e.to_string().contains("NotFound") || e.to_string().contains("not found") {
                (StatusCode::NOT_FOUND, "NOT_FOUND")
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "DB_ERROR")
            };
        (
            status,
            Json(serde_json::json!({ "error": e.to_string(), "code": code })),
        )
    })
}

/// Handle GET /health-snapshots — return the most recent snapshots for a project.
///
/// Returns snapshots ordered newest first, capped at `limit` (default 20, max 100).
pub async fn list_health_snapshots(
    State(state): State<HealthState>,
    Query(query): Query<ListSnapshotsQuery>,
) -> Result<Json<Vec<HealthSnapshot>>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let limit = query.limit.unwrap_or(20).min(100);

    storage
        .health()
        .get_recent(query.project_id, limit)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
            )
        })
}
