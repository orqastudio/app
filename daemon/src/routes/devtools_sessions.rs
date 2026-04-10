// Devtools session and event routes: session lifecycle and event query for OrqaDev.
//
// Devtools sessions group the log events captured during a single run of OrqaDev.
// These routes expose the full DevtoolsRepository trait over HTTP so the devtools
// frontend can list sessions, query events, rename sessions, and clean up old data.
//
// Endpoints:
//   POST /devtools-sessions                         — create a new devtools session
//   GET  /devtools-sessions                         — list all sessions
//   GET  /devtools-sessions/:id                     — get metadata for one session
//   PUT  /devtools-sessions/:id/label               — rename a session
//   POST /devtools-sessions/:id/end                 — mark a session as ended
//   DELETE /devtools-sessions/:id                   — delete a session and its events
//   POST /devtools-sessions/mark-orphaned           — mark open sessions as interrupted
//   POST /devtools-sessions/purge                   — delete sessions older than retention window
//   POST /devtools-sessions/:id/events              — insert a batch of events for a session
//   POST /devtools-sessions/:id/events/query        — paginated event query with filters

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use orqa_engine_types::types::event::LogEvent;
use orqa_storage::repo::devtools::DevtoolsEventQuery;
use orqa_storage::traits::DevtoolsRepository as _;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Request body for POST /devtools-sessions.
#[derive(Debug, Deserialize)]
pub struct CreateDevtoolsSessionRequest {
    pub session_id: String,
    pub started_at: i64,
}

/// Query parameters for GET /devtools-sessions.
#[derive(Debug, Deserialize)]
pub struct ListDevtoolsSessionsQuery {
    /// The current active session ID — used to set the `is_current` flag.
    pub current_session_id: String,
}

/// Request body for PUT /devtools-sessions/:id/label.
#[derive(Debug, Deserialize)]
pub struct RenameSessionRequest {
    pub label: String,
}

/// Request body for POST /devtools-sessions/:id/end.
#[derive(Debug, Deserialize)]
pub struct EndDevtoolsSessionRequest {
    pub ended_at: i64,
}

/// Request body for POST /devtools-sessions/purge.
#[derive(Debug, Deserialize)]
pub struct PurgeSessionsRequest {
    pub retention_days: u32,
}

/// Response helper when the storage layer is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "devtools store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /devtools-sessions — create a new devtools session.
pub async fn create_devtools_session(
    State(state): State<HealthState>,
    Json(req): Json<CreateDevtoolsSessionRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .create_session(&req.session_id, req.started_at)
        .await
        .map(|()| StatusCode::CREATED)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "CREATE_FAILED" })),
            )
        })
}

/// Handle GET /devtools-sessions — list all devtools sessions ordered by started_at DESC.
///
/// The `current_session_id` query param is used to mark the active session.
pub async fn list_devtools_sessions(
    State(state): State<HealthState>,
    Query(query): Query<ListDevtoolsSessionsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .list_sessions(&query.current_session_id)
        .await
        .map(|sessions| Json(serde_json::json!({ "sessions": sessions })))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
            )
        })
}

/// Handle GET /devtools-sessions/:id — get metadata for a specific session.
pub async fn get_devtools_session(
    State(state): State<HealthState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .get_session(&id)
        .await
        .map(|info| {
            Json(serde_json::json!({
                "id": info.id,
                "started_at": info.started_at,
                "label": info.label,
                "event_count": info.event_count,
            }))
        })
        .map_err(|e| {
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

/// Handle PUT /devtools-sessions/:id/label — rename a session.
pub async fn rename_devtools_session(
    State(state): State<HealthState>,
    Path(id): Path<String>,
    Json(req): Json<RenameSessionRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .rename_session(&id, &req.label)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
            )
        })
}

/// Handle POST /devtools-sessions/:id/end — mark a session as ended.
pub async fn end_devtools_session(
    State(state): State<HealthState>,
    Path(id): Path<String>,
    Json(req): Json<EndDevtoolsSessionRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .end_session(&id, req.ended_at)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
            )
        })
}

/// Handle DELETE /devtools-sessions/:id — delete a session and cascade its events.
pub async fn delete_devtools_session(
    State(state): State<HealthState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .delete_session(&id)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "DELETE_FAILED" })),
            )
        })
}

/// Handle POST /devtools-sessions/mark-orphaned — mark all open sessions as interrupted.
///
/// Called on devtools startup to clean up sessions from a prior crash.
pub async fn mark_orphaned_sessions(
    State(state): State<HealthState>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .mark_orphaned_sessions_interrupted()
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
            )
        })
}

/// Handle POST /devtools-sessions/purge — delete sessions older than a retention window.
///
/// Returns the number of sessions deleted.
pub async fn purge_devtools_sessions(
    State(state): State<HealthState>,
    Json(req): Json<PurgeSessionsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .purge_old_sessions(req.retention_days)
        .await
        .map(|n| Json(serde_json::json!({ "deleted": n })))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "PURGE_FAILED" })),
            )
        })
}

/// Handle POST /devtools-sessions/:id/events — insert a batch of log events for a session.
pub async fn insert_devtools_events(
    State(state): State<HealthState>,
    Path(id): Path<String>,
    Json(events): Json<Vec<LogEvent>>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .devtools()
        .insert_events(&id, events)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "INSERT_FAILED" })),
            )
        })
}

/// Handle POST /devtools-sessions/:id/events/query — paginated and filtered event query.
///
/// Accepts filter and pagination parameters in the body. The `session_id` from the
/// path is merged into the query so the body `session_id` field is optional.
pub async fn query_devtools_events(
    State(state): State<HealthState>,
    Path(id): Path<String>,
    Json(mut query): Json<DevtoolsEventQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    // Override session_id from path to prevent mismatches.
    query.session_id = id;

    storage
        .devtools()
        .query_events(&query)
        .await
        .map(|resp| {
            Json(serde_json::json!({
                "events": resp.events,
                "total": resp.total,
            }))
        })
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "QUERY_FAILED" })),
            )
        })
}
