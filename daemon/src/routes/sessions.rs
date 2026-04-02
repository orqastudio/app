// Session routes: full CRUD for agent sessions and their messages.
//
// Sessions are stored in the daemon SQLite database (.state/daemon.db).
// All handlers require HealthState so they can access the DaemonStore.
// Database operations run via spawn_blocking to keep the tokio runtime free.
//
// Endpoints:
//   POST   /sessions               — create a new session
//   GET    /sessions               — list sessions (query: project_id, status)
//   GET    /sessions/:id           — get a single session
//   PUT    /sessions/:id           — update session (title, status)
//   DELETE /sessions/:id           — delete a session
//   POST   /sessions/:id/end       — end an active session
//   GET    /sessions/:id/messages  — list messages in a session

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::health::HealthState;
use crate::store::{
    message_list, session_create, session_delete, session_get, session_list,
    session_update_status, session_update_title, Message, Session, SessionSummary,
};

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Query parameters for GET /sessions.
#[derive(Debug, Deserialize)]
pub struct ListSessionsQuery {
    /// Filter by project ID.
    pub project_id: Option<i64>,
    /// Filter by status (active, completed, abandoned, error).
    pub status: Option<String>,
}

/// Request body for POST /sessions.
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    /// Project to associate this session with.
    pub project_id: i64,
    /// LLM model identifier (default: "auto").
    pub model: Option<String>,
    /// Initial system prompt for the session.
    pub system_prompt: Option<String>,
}

/// Request body for PUT /sessions/:id.
#[derive(Debug, Deserialize)]
pub struct UpdateSessionRequest {
    /// New title for the session. Null to leave unchanged.
    pub title: Option<String>,
    /// New status for the session. Null to leave unchanged.
    pub status: Option<String>,
}

/// Response when the daemon store is unavailable.
fn store_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "session store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle POST /sessions — create a new session for a project.
pub async fn create_session(
    State(state): State<HealthState>,
    Json(req): Json<CreateSessionRequest>,
) -> Result<(StatusCode, Json<Session>), (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;
    let model = req.model.unwrap_or_else(|| "auto".to_owned());
    let system_prompt = req.system_prompt.clone();
    let project_id = req.project_id;

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;
        session_create(&conn, project_id, &model, system_prompt.as_deref())
            .map(|s| (StatusCode::CREATED, Json(s)))
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e, "code": "CREATE_FAILED" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle GET /sessions — list sessions with optional project_id and status filters.
pub async fn list_sessions(
    State(state): State<HealthState>,
    Query(query): Query<ListSessionsQuery>,
) -> Result<Json<Vec<SessionSummary>>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;
    let project_id = query.project_id;
    let status = query.status.clone();

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;

        session_list(&conn, project_id, status.as_deref())
            .map(Json).map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e, "code": "LIST_FAILED" })),
        ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle GET /sessions/:id — get a single session by ID.
pub async fn get_session(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<Json<Session>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;
        session_get(&conn, id)
            .map(Json)
            .map_err(|e| {
                let (status, code) = if e.contains("not found") {
                    (StatusCode::NOT_FOUND, "NOT_FOUND")
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, "DB_ERROR")
                };
                (status, Json(serde_json::json!({ "error": e, "code": code })))
            })
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle PUT /sessions/:id — update a session's title and/or status.
pub async fn update_session(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateSessionRequest>,
) -> Result<Json<Session>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;

        if let Some(title) = req.title {
            session_update_title(&conn, id, &title).map_err(|e| (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": e, "code": "UPDATE_FAILED" })),
            ))?;
        }
        if let Some(status) = req.status {
            session_update_status(&conn, id, &status).map_err(|e| (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": e, "code": "UPDATE_FAILED" })),
            ))?;
        }

        session_get(&conn, id)
            .map(Json)
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e, "code": "DB_ERROR" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle DELETE /sessions/:id — permanently delete a session and its messages.
pub async fn delete_session(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;
        session_delete(&conn, id).map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e, "code": "DELETE_FAILED" })),
        ))?;
        Ok(StatusCode::NO_CONTENT)
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle POST /sessions/:id/end — mark a session as completed.
pub async fn end_session(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<Json<Session>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;
        session_update_status(&conn, id, "completed").map_err(|e| (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": e, "code": "UPDATE_FAILED" })),
        ))?;
        session_get(&conn, id)
            .map(Json)
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e, "code": "DB_ERROR" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle GET /sessions/:id/messages — list all messages in a session.
pub async fn list_session_messages(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Message>>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;
        message_list(&conn, id)
            .map(Json)
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e, "code": "LIST_FAILED" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

