// Session routes: full CRUD for agent sessions and their messages.
//
// Sessions are stored in the unified SQLite database (.state/orqa.db) via the
// engine/storage crate. All handlers use `HealthState::storage` to access the
// sessions and messages repos.
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

use orqa_engine_types::types::message::Message;
use orqa_engine_types::types::session::{Session, SessionStatus, SessionSummary};
use orqa_storage::traits::{MessageRepository as _, SessionRepository as _};

use crate::health::HealthState;

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

/// Parse a session status string into a `SessionStatus` variant.
///
/// Returns `None` if the string is unrecognised, so the caller can decide
/// whether to treat it as an error or fall back to no filter.
fn parse_session_status(s: &str) -> Option<SessionStatus> {
    match s {
        "active" => Some(SessionStatus::Active),
        "completed" => Some(SessionStatus::Completed),
        "abandoned" => Some(SessionStatus::Abandoned),
        "error" => Some(SessionStatus::Error),
        _ => None,
    }
}

/// Response when the storage layer is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
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
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let model = req.model.unwrap_or_else(|| "auto".to_owned());
    let system_prompt = req.system_prompt.clone();
    let project_id = req.project_id;

    storage
        .sessions()
        .create(project_id, &model, system_prompt.as_deref())
        .await
        .map(|s| (StatusCode::CREATED, Json(s)))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "CREATE_FAILED" })),
            )
        })
}

/// Handle GET /sessions — list sessions with optional project_id and status filters.
pub async fn list_sessions(
    State(state): State<HealthState>,
    Query(query): Query<ListSessionsQuery>,
) -> Result<Json<Vec<SessionSummary>>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let project_id = query.project_id;
    let status = query.status.clone();

    let status_filter = status.as_deref().and_then(parse_session_status);
    let result = if let Some(pid) = project_id {
        storage.sessions().list(pid, status_filter, 1000, 0).await
    } else {
        storage.sessions().list_all(status_filter).await
    };

    result.map(Json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
        )
    })
}

/// Handle GET /sessions/:id — get a single session by ID.
pub async fn get_session(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<Json<Session>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage.sessions().get(id).await.map(Json).map_err(|e| {
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

/// Handle PUT /sessions/:id — update a session's title and/or status.
pub async fn update_session(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateSessionRequest>,
) -> Result<Json<Session>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    if let Some(title) = req.title {
        storage
            .sessions()
            .update_title(id, &title)
            .await
            .map_err(|e| {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
                )
            })?;
    }
    if let Some(status_str) = req.status {
        let status = parse_session_status(&status_str).ok_or_else(|| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": format!("unknown status: {status_str}"), "code": "INVALID_STATUS" })),
            )
        })?;
        storage
            .sessions()
            .update_status(id, status)
            .await
            .map_err(|e| {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
                )
            })?;
    }

    storage.sessions().get(id).await.map(Json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        )
    })
}

/// Handle DELETE /sessions/:id — permanently delete a session and its messages.
pub async fn delete_session(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage.sessions().delete(id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DELETE_FAILED" })),
        )
    })?;
    Ok(StatusCode::NO_CONTENT)
}

/// Handle POST /sessions/:id/end — mark a session as completed.
pub async fn end_session(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<Json<Session>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage.sessions().end_session(id).await.map_err(|e| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
        )
    })?;
    storage.sessions().get(id).await.map(Json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        )
    })
}

/// Handle GET /sessions/:id/messages — list all messages in a session.
pub async fn list_session_messages(
    State(state): State<HealthState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Message>>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .messages()
        .list(id, 10_000, 0)
        .await
        .map(Json)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
            )
        })
}

// ---------------------------------------------------------------------------
// Route tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_storage::traits::ProjectRepository as _;

    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use axum::routing::{get, post};
    use axum::Router;
    use http_body_util::BodyExt as _;
    use std::sync::Arc;
    use tower::ServiceExt as _;

    use orqa_storage::Storage;

    use crate::graph_state::GraphState;

    /// Build an axum Router wiring the session routes to a fresh in-memory store.
    ///
    /// Returns (Router, project_id) so tests can create sessions without
    /// needing to hit a /projects route.
    async fn session_router() -> (Router, i64) {
        let storage = Storage::open_in_memory().await.expect("in-memory storage");

        // Create a seed project so we have a valid project_id for sessions.
        let project_id = storage
            .projects()
            .create("test-project", "/test/project", None)
            .await
            .expect("project_create")
            .id;

        let storage = Arc::new(storage);

        let state = HealthState::for_test(
            GraphState::build_empty(std::path::Path::new("/tmp/test")),
            Some(Arc::clone(&storage)),
        );

        // Use {id} capture syntax (axum 0.8+) for the path parameters.
        let session_sub = Router::new()
            .route("/", post(create_session).get(list_sessions))
            .route(
                "/{id}",
                get(get_session).put(update_session).delete(delete_session),
            )
            .route("/{id}/end", post(end_session))
            .route("/{id}/messages", get(list_session_messages))
            .with_state(state);

        let router = Router::new().nest("/sessions", session_sub);

        (router, project_id)
    }

    fn json_body(val: serde_json::Value) -> Body {
        Body::from(serde_json::to_vec(&val).unwrap())
    }

    async fn body_json(body: Body) -> serde_json::Value {
        let bytes = body.collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).expect("response must be valid JSON")
    }

    // ---- POST /sessions -------------------------------------------------------

    #[tokio::test]
    async fn post_sessions_creates_session_and_returns_201() {
        // POST /sessions must return 201 with id and status="active".
        let (app, project_id) = session_router().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/sessions")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({
                        "project_id": project_id,
                        "model": "auto"
                    })))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::CREATED);
        let body = body_json(resp.into_body()).await;
        assert!(
            body["id"].as_i64().is_some(),
            "response must include integer id"
        );
        assert_eq!(body["status"], "active");
    }

    #[tokio::test]
    async fn post_sessions_response_has_project_id() {
        // The created session must carry the project_id we supplied.
        let (app, project_id) = session_router().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/sessions")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "project_id": project_id })))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = body_json(resp.into_body()).await;
        assert_eq!(body["project_id"], project_id);
    }

    // ---- GET /sessions --------------------------------------------------------

    #[tokio::test]
    async fn get_sessions_returns_200_with_list() {
        // GET /sessions must return 200 with a JSON array (empty or populated).
        let (app, project_id) = session_router().await;
        // Create one session first.
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/sessions")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "project_id": project_id })))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_resp.status(), StatusCode::CREATED);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/sessions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        let list = body.as_array().expect("response must be a JSON array");
        assert_eq!(list.len(), 1);
    }

    // ---- GET /sessions/:id ----------------------------------------------------

    #[tokio::test]
    async fn get_session_returns_created_session() {
        // GET /sessions/:id must return the same session we just created.
        let (app, project_id) = session_router().await;
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/sessions")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "project_id": project_id })))
                    .unwrap(),
            )
            .await
            .unwrap();
        let created = body_json(create_resp.into_body()).await;
        let id = created["id"].as_i64().unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/sessions/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        assert_eq!(body["id"], id);
        assert_eq!(body["project_id"], project_id);
    }

    #[tokio::test]
    async fn get_session_returns_404_for_nonexistent() {
        // Requesting a session that doesn't exist must return 404.
        let (app, _) = session_router().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/sessions/9999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    // ---- PUT /sessions/:id ----------------------------------------------------

    #[tokio::test]
    async fn put_session_updates_title() {
        // After PUT with a new title, the title field must reflect the change.
        let (app, project_id) = session_router().await;
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/sessions")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "project_id": project_id })))
                    .unwrap(),
            )
            .await
            .unwrap();
        let created = body_json(create_resp.into_body()).await;
        let id = created["id"].as_i64().unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri(format!("/sessions/{id}"))
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "title": "My Session" })))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        assert_eq!(body["title"], "My Session");
    }

    // ---- DELETE /sessions/:id -------------------------------------------------

    #[tokio::test]
    async fn delete_session_returns_204() {
        // DELETE /sessions/:id must return 204 No Content.
        let (app, project_id) = session_router().await;
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/sessions")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "project_id": project_id })))
                    .unwrap(),
            )
            .await
            .unwrap();
        let created = body_json(create_resp.into_body()).await;
        let id = created["id"].as_i64().unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri(format!("/sessions/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn after_delete_get_session_returns_404() {
        // After DELETE, GET /sessions/:id must return 404.
        let (app, project_id) = session_router().await;
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/sessions")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "project_id": project_id })))
                    .unwrap(),
            )
            .await
            .unwrap();
        let created = body_json(create_resp.into_body()).await;
        let id = created["id"].as_i64().unwrap();

        app.clone()
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri(format!("/sessions/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/sessions/{id}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    // ---- GET /sessions/:id/messages -------------------------------------------

    #[tokio::test]
    async fn get_session_messages_on_new_session_returns_empty_array() {
        // A freshly created session must return an empty messages array.
        let (app, project_id) = session_router().await;
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/sessions")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "project_id": project_id })))
                    .unwrap(),
            )
            .await
            .unwrap();
        let created = body_json(create_resp.into_body()).await;
        let id = created["id"].as_i64().unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/sessions/{id}/messages"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        let messages = body.as_array().expect("messages must be a JSON array");
        assert!(messages.is_empty(), "new session must have no messages");
    }
}
