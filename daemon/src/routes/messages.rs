// Message routes: create, list (via sessions), search, and tool-message creation.
//
// Messages are content blocks within a session. The list endpoint is already
// exposed at GET /sessions/:id/messages in sessions.rs. This module adds the
// missing operations: POST create, POST create_tool_message, POST search (FTS5),
// and mutation endpoints (update_content, update_stream_status).
//
// Endpoints:
//   POST /messages                       — create a standard message
//   POST /messages/tool                  — create a tool_use or tool_result message
//   POST /messages/search                — full-text search across a project (FTS5)
//   PUT  /messages/:id/content           — update message content (streaming accumulation)
//   PUT  /messages/:id/stream-status     — update message stream status

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use orqa_engine_types::types::message::{Message, MessageRole, StreamStatus};
use orqa_storage::repo::messages::NewToolMessage;
use orqa_storage::traits::MessageRepository as _;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Request body for POST /messages.
#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    /// Session this message belongs to.
    pub session_id: i64,
    /// Role: "user", "assistant", or "system".
    pub role: String,
    /// Text content of the message.
    pub content: Option<String>,
    /// Turn index within the session.
    pub turn_index: i32,
    /// Block index within the turn.
    pub block_index: i32,
}

/// Request body for POST /messages/tool.
#[derive(Debug, Deserialize)]
pub struct CreateToolMessageRequest {
    pub session_id: i64,
    pub role: String,
    pub content_type: String,
    pub content: Option<String>,
    pub tool_call_id: String,
    pub tool_name: String,
    pub tool_input: Option<String>,
    pub tool_is_error: bool,
    pub turn_index: i32,
    pub block_index: i32,
}

/// Request body for POST /messages/search.
#[derive(Debug, Deserialize)]
pub struct SearchMessagesRequest {
    /// Project to search within.
    pub project_id: i64,
    /// FTS5 search query string.
    pub query: String,
    /// Maximum results to return (default 50).
    pub limit: Option<i64>,
}

/// Request body for PUT /messages/:id/content.
#[derive(Debug, Deserialize)]
pub struct UpdateContentRequest {
    pub content: String,
}

/// Request body for PUT /messages/:id/stream-status.
#[derive(Debug, Deserialize)]
pub struct UpdateStreamStatusRequest {
    /// "pending", "complete", or "error".
    pub status: String,
}

/// Parse a message role string into a `MessageRole` variant.
fn parse_role(s: &str) -> Option<MessageRole> {
    match s {
        "user" => Some(MessageRole::User),
        "assistant" => Some(MessageRole::Assistant),
        "system" => Some(MessageRole::System),
        _ => None,
    }
}

/// Parse a stream status string into a `StreamStatus` variant.
fn parse_stream_status(s: &str) -> Option<StreamStatus> {
    match s {
        "pending" => Some(StreamStatus::Pending),
        "complete" => Some(StreamStatus::Complete),
        "error" => Some(StreamStatus::Error),
        _ => None,
    }
}

/// Response helper when the storage layer is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "message store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /messages — create a standard (non-tool) message.
pub async fn create_message(
    State(state): State<HealthState>,
    Json(req): Json<CreateMessageRequest>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let role = parse_role(&req.role).ok_or_else(|| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": format!("unknown role: {}", req.role), "code": "INVALID_ROLE" })),
        )
    })?;

    storage
        .messages()
        .create(
            req.session_id,
            role,
            req.content.as_deref(),
            req.turn_index,
            req.block_index,
        )
        .await
        .map(|m| (StatusCode::CREATED, Json(m)))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "CREATE_FAILED" })),
            )
        })
}

/// Handle POST /messages/tool — create a tool_use or tool_result message.
pub async fn create_tool_message(
    State(state): State<HealthState>,
    Json(req): Json<CreateToolMessageRequest>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    let msg = NewToolMessage {
        session_id: req.session_id,
        role: &req.role,
        content_type: &req.content_type,
        content: req.content.as_deref(),
        tool_call_id: &req.tool_call_id,
        tool_name: &req.tool_name,
        tool_input: req.tool_input.as_deref(),
        tool_is_error: req.tool_is_error,
        turn_index: req.turn_index,
        block_index: req.block_index,
    };

    storage
        .messages()
        .create_tool_message(&msg)
        .await
        .map(|m| (StatusCode::CREATED, Json(m)))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "CREATE_FAILED" })),
            )
        })
}

/// Handle POST /messages/search — full-text search across a project using FTS5.
///
/// Delegates to the messages FTS5 virtual table. Returns ranked results with
/// snippet context.
pub async fn search_messages(
    State(state): State<HealthState>,
    Json(req): Json<SearchMessagesRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let limit = req.limit.unwrap_or(50).min(500);

    storage
        .messages()
        .search(req.project_id, &req.query, limit)
        .await
        .map(|results| {
            let count = results.len();
            Json(serde_json::json!({
                "results": results,
                "count": count
            }))
        })
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "SEARCH_FAILED" })),
            )
        })
}

/// Handle PUT /messages/:id/content — update a message's text content.
///
/// Used during streaming accumulation to append or replace content as tokens arrive.
pub async fn update_message_content(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateContentRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .messages()
        .update_content(id, &req.content)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
            )
        })
}

/// Handle PUT /messages/:id/stream-status — update the streaming state of a message.
///
/// Transitions a message from "pending" to "complete" or "error" once the LLM
/// stream completes.
pub async fn update_message_stream_status(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStreamStatusRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let status = parse_stream_status(&req.status).ok_or_else(|| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": format!("unknown status: {}", req.status), "code": "INVALID_STATUS" })),
        )
    })?;

    storage
        .messages()
        .update_stream_status(id, status)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
            )
        })
}
