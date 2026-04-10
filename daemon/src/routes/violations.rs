// Enforcement violation recording routes: persist and retrieve recorded violations.
//
// Violations are governance rule failures recorded to SQLite for trend analysis and
// audit history. This is distinct from the live scan in enforcement.rs — recorded
// violations accumulate over time and survive daemon restarts.
//
// Endpoints:
//   POST /violations              — record a new enforcement violation
//   GET  /violations              — list violations for a project (newest first)

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use orqa_engine_types::types::enforcement::EnforcementViolation;
use orqa_storage::traits::ViolationRepository as _;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Request body for POST /violations.
#[derive(Debug, Deserialize)]
pub struct RecordViolationRequest {
    pub project_id: i64,
    pub rule_name: String,
    pub action: String,
    pub tool_name: String,
    pub detail: Option<String>,
}

/// Query parameters for GET /violations.
#[derive(Debug, Deserialize)]
pub struct ListViolationsQuery {
    pub project_id: i64,
    /// Maximum number of violations to return. Returns all rows when absent.
    pub limit: Option<u32>,
}

/// Response helper when the storage layer is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "violation store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /violations — record a new enforcement violation to SQLite.
///
/// Returns 204 on success. The violation is timestamped by the storage layer.
pub async fn record_violation(
    State(state): State<HealthState>,
    Json(req): Json<RecordViolationRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .violations()
        .record(
            req.project_id,
            &req.rule_name,
            &req.action,
            &req.tool_name,
            req.detail.as_deref(),
        )
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "RECORD_FAILED" })),
            )
        })
}

/// Handle GET /violations — list recorded violations for a project, newest first.
///
/// `limit` caps the result count; omit to return all rows.
pub async fn list_violations(
    State(state): State<HealthState>,
    Query(query): Query<ListViolationsQuery>,
) -> Result<Json<Vec<EnforcementViolation>>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .violations()
        .list_for_project(query.project_id, query.limit)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
            )
        })
}
