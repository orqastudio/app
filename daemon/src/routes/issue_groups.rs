// Issue group routes: deduplicated error clusters with sparkline and ring-buffer.
//
// Issue groups deduplicate recurring events by fingerprint. Each group tracks the
// total count, a 24-bucket hourly sparkline, and the 50 most recent event IDs.
// These routes expose the full IssueGroupRepository contract over HTTP.
//
// Endpoints:
//   POST /issue-groups/upsert              — insert or update a group by fingerprint
//   GET  /issue-groups                     — list groups with sort and filter options
//   GET  /issue-groups/:fingerprint        — get a single group by fingerprint

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use orqa_storage::repo::issue_groups::{IssueGroup, SortBy, SortDir};
use orqa_storage::traits::IssueGroupRepository as _;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Request body for POST /issue-groups/upsert.
#[derive(Debug, Deserialize)]
pub struct UpsertIssueGroupRequest {
    pub fingerprint: String,
    pub title: String,
    pub component: String,
    pub level: String,
    pub timestamp_ms: i64,
    pub event_id: u64,
}

/// Query parameters for GET /issue-groups.
#[derive(Debug, Deserialize)]
pub struct ListIssueGroupsQuery {
    /// Sort column: "last_seen" (default), "count", "level", "component".
    pub sort_by: Option<String>,
    /// Sort direction: "desc" (default) or "asc".
    pub sort_dir: Option<String>,
    /// Optional exact-match filter on component.
    pub component: Option<String>,
    /// Optional exact-match filter on level.
    pub level: Option<String>,
    /// Maximum results (default 100, max 1000).
    pub limit: Option<u32>,
    /// Zero-based offset for pagination.
    pub offset: Option<u32>,
}

/// Parse a sort_by string to the enum variant (default: LastSeen).
fn parse_sort_by(s: &str) -> SortBy {
    match s {
        "count" => SortBy::Count,
        "level" => SortBy::Level,
        "component" => SortBy::Component,
        _ => SortBy::LastSeen,
    }
}

/// Parse a sort_dir string to the enum variant (default: Desc).
fn parse_sort_dir(s: &str) -> SortDir {
    match s {
        "asc" => SortDir::Asc,
        _ => SortDir::Desc,
    }
}

/// Response helper when the storage layer is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "issue group store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /issue-groups/upsert — insert or update an issue group.
///
/// On first occurrence creates a new row with count=1. On subsequent occurrences
/// increments count, rotates the sparkline bucket, and updates the ring buffer.
pub async fn upsert_issue_group(
    State(state): State<HealthState>,
    Json(req): Json<UpsertIssueGroupRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .issue_groups()
        .upsert(
            &req.fingerprint,
            &req.title,
            &req.component,
            &req.level,
            req.timestamp_ms,
            req.event_id,
        )
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "UPSERT_FAILED" })),
            )
        })
}

/// Handle GET /issue-groups — list groups with optional filtering and sorting.
///
/// Returns a paginated list of issue groups. Default ordering is by last_seen DESC.
pub async fn list_issue_groups(
    State(state): State<HealthState>,
    Query(query): Query<ListIssueGroupsQuery>,
) -> Result<Json<Vec<IssueGroup>>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    let sort_by = query
        .sort_by
        .as_deref()
        .map_or(SortBy::LastSeen, parse_sort_by);
    let sort_dir = query
        .sort_dir
        .as_deref()
        .map_or(SortDir::Desc, parse_sort_dir);
    let limit = query.limit.unwrap_or(100).min(1000);
    let offset = query.offset.unwrap_or(0);

    storage
        .issue_groups()
        .list(
            sort_by,
            sort_dir,
            query.component.as_deref(),
            query.level.as_deref(),
            limit,
            offset,
        )
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
            )
        })
}

/// Handle GET /issue-groups/:fingerprint — get a single issue group by fingerprint.
///
/// Returns 404 when the fingerprint is not in the database.
pub async fn get_issue_group(
    State(state): State<HealthState>,
    Path(fingerprint): Path<String>,
) -> Result<Json<IssueGroup>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    match storage.issue_groups().get(&fingerprint).await {
        Ok(Some(group)) => Ok(Json(group)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "issue group not found", "code": "NOT_FOUND" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        )),
    }
}
