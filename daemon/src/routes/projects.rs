// Project management routes: list, open, settings, scan, and icon.
//
// Projects are tracked in the unified SQLite storage (orqa-storage). Project
// settings (project.json) are read and written via the orqa-project crate's
// FileProjectSettingsStore. The scan endpoint delegates to the project scanner
// to detect the technology stack.
//
// Endpoints:
//   GET  /projects           — list all known projects
//   GET  /projects/active    — get the most recently used project
//   POST /projects/open      — open/activate a project by path
//   GET  /projects/settings  — read project.json for the active project
//   PUT  /projects/settings  — write project.json for the active project
//   POST /projects/scan      — scan the project filesystem (stack + governance)
//   POST /projects/icon      — upload a project icon
//   GET  /projects/icon      — read the project icon

use axum::body::Bytes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use orqa_engine_types::traits::storage::ProjectSettingsStore as _;
use orqa_engine_types::types::project::{Project, ProjectSummary};
use orqa_project::scanner::scan_project;
use orqa_project::store::FileProjectSettingsStore;
use orqa_storage::traits::ProjectRepository as _;

use crate::graph_state::GraphState;
use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Request shapes
// ---------------------------------------------------------------------------

/// Request body for POST /projects/open.
#[derive(Debug, Deserialize)]
pub struct OpenProjectRequest {
    /// Absolute filesystem path to the project root.
    pub path: String,
    /// Display name for the project (used when creating a new record).
    pub name: Option<String>,
}

/// Request body for PUT /projects/settings.
#[derive(Debug, Deserialize)]
pub struct UpdateProjectSettingsRequest {
    /// The new settings value to write to project.json.
    pub settings: serde_json::Value,
}

/// Response helper when the storage is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "project store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Resolve the project root from GraphState.
fn project_root_from_graph(state: &GraphState) -> Option<std::path::PathBuf> {
    state.0.read().ok().map(|g| g.project_root.clone())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /projects — list all known projects from SQLite.
pub async fn list_projects(
    State(state): State<HealthState>,
) -> Result<Json<Vec<ProjectSummary>>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage.projects().list().await.map(Json).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
        )
    })
}

/// Handle GET /projects/active — return the most recently used project.
///
/// Returns 404 when no projects have been opened yet.
pub async fn get_active_project(
    State(state): State<HealthState>,
) -> Result<Json<Project>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    match storage.projects().get_active().await {
        Ok(Some(p)) => Ok(Json(p)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "no active project", "code": "NOT_FOUND" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        )),
    }
}

/// Handle POST /projects/open — open a project by path, creating a record if needed.
///
/// If the project is already known, touches its updated_at to surface it as
/// active. If new, inserts a record with the provided (or derived) name.
pub async fn open_project(
    State(state): State<HealthState>,
    Json(req): Json<OpenProjectRequest>,
) -> Result<Json<Project>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let path = req.path.clone();
    let name = req.name.clone();

    // Check if the project already exists by path.
    match storage.projects().get_by_path(&path).await {
        Ok(existing) => {
            // Touch to make it the most recently active project.
            let _ = storage.projects().touch_updated_at(existing.id).await;
            let refreshed = storage
                .projects()
                .get(existing.id)
                .await
                .unwrap_or(existing);
            Ok(Json(refreshed))
        }
        Err(orqa_storage::error::StorageError::NotFound(_)) => {
            // Derive a display name from the final path component if not provided.
            let display_name = name.unwrap_or_else(|| {
                std::path::Path::new(&path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("project")
                    .to_owned()
            });
            storage
                .projects()
                .create(&display_name, &path, None)
                .await
                .map(Json)
                .map_err(|e| {
                    (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        Json(
                            serde_json::json!({ "error": e.to_string(), "code": "CREATE_FAILED" }),
                        ),
                    )
                })
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        )),
    }
}

/// Handle GET /projects/settings — read project.json for the active project.
///
/// Returns the raw project.json value so callers receive the full current shape
/// without coupling to a specific Rust struct version.
pub async fn get_project_settings(
    State(state): State<HealthState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
        )
    })?;

    tokio::task::spawn_blocking(move || {
        let store = FileProjectSettingsStore::new();
        match store.load(&project_root) {
            Ok(Some(v)) => Ok(Json(v)),
            Ok(None) => Ok(Json(serde_json::Value::Object(serde_json::Map::new()))),
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "SETTINGS_LOAD_FAILED" })),
            )),
        }
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        )
    })?
}

/// Handle PUT /projects/settings — write project.json for the active project.
///
/// Overwrites project.json with the provided value. Merges are the caller's
/// responsibility — pass the full desired settings object.
pub async fn update_project_settings(
    State(state): State<HealthState>,
    Json(req): Json<UpdateProjectSettingsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
        )
    })?;
    let settings = req.settings.clone();

    tokio::task::spawn_blocking(move || {
        let store = FileProjectSettingsStore::new();
        store
            .save(&project_root, &settings)
            .map(|()| Json(settings))
            .map_err(|e| {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(serde_json::json!({ "error": e.to_string(), "code": "SETTINGS_SAVE_FAILED" })),
                )
            })
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        )
    })?
}

/// Handle POST /projects/scan — scan the project filesystem for stack and governance info.
///
/// Delegates to `orqa_project::scanner::scan_project`. Returns detected languages,
/// frameworks, and governance artifact counts.
pub async fn scan_project_handler(
    State(state): State<HealthState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
        )
    })?;

    tokio::task::spawn_blocking(move || {
        let path_str = project_root.to_string_lossy().to_string();
        scan_project(&path_str, &[])
            .map(|r| Json(serde_json::to_value(&r).unwrap_or(serde_json::Value::Null)))
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": e.to_string(), "code": "SCAN_FAILED" })),
                )
            })
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        )
    })?
}

/// Handle POST /projects/icon — store an uploaded project icon.
///
/// Accepts raw bytes (PNG/JPG) and writes them to `.state/project-icon` under
/// the project root. Returns 204 on success.
pub async fn upload_project_icon(
    State(state): State<HealthState>,
    body: Bytes,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
        )
    })?;

    tokio::task::spawn_blocking(move || {
        let state_dir = project_root.join(".state");
        std::fs::create_dir_all(&state_dir).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "IO_ERROR" })),
            )
        })?;
        let icon_path = state_dir.join("project-icon");
        std::fs::write(&icon_path, &body).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "IO_ERROR" })),
            )
        })?;
        Ok(StatusCode::NO_CONTENT)
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        )
    })?
}

/// Handle GET /projects/icon — read the project icon bytes.
///
/// Returns the raw bytes from `.state/project-icon`. Returns 404 when no
/// icon has been uploaded.
pub async fn get_project_icon(
    State(state): State<HealthState>,
) -> Result<Bytes, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
        )
    })?;

    tokio::task::spawn_blocking(move || {
        let icon_path = project_root.join(".state/project-icon");
        if !icon_path.exists() {
            return Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "no project icon set", "code": "NOT_FOUND" })),
            ));
        }
        std::fs::read(&icon_path).map(Bytes::from).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "IO_ERROR" })),
            )
        })
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        )
    })?
}

// ---------------------------------------------------------------------------
// Route tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use axum::routing::{get, post};
    use axum::Router;
    use http_body_util::BodyExt as _;
    use std::sync::Arc;
    use tower::ServiceExt as _;

    use crate::graph_state::GraphState;
    use orqa_storage::Storage;

    /// Absolute path to the minimal fixture project, matching the integration test
    /// fixture so project_root is a real directory for settings handlers.
    fn fixture_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/minimal-project")
    }

    /// Build a Router wiring the project routes to a fresh in-memory store.
    ///
    /// Graph state uses the fixture project so `project_root` is valid for
    /// the project-settings handlers that read/write project.json.
    async fn project_router() -> Router {
        let storage = Storage::open_in_memory()
            .await
            .map(Arc::new)
            .expect("in-memory storage");

        let graph_state = GraphState::build(&fixture_root())
            .await
            .unwrap_or_else(|_| GraphState::build_empty(&fixture_root()));

        let state = HealthState::for_test(graph_state, Some(Arc::clone(&storage)));

        let project_sub = Router::new()
            .route("/", get(list_projects))
            .route("/active", get(get_active_project))
            .route("/open", post(open_project))
            .route(
                "/settings",
                get(get_project_settings).put(update_project_settings),
            )
            .with_state(state);

        Router::new().nest("/projects", project_sub)
    }

    fn json_body(val: serde_json::Value) -> Body {
        Body::from(serde_json::to_vec(&val).unwrap())
    }

    async fn body_json(body: Body) -> serde_json::Value {
        let bytes = body.collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).expect("response must be valid JSON")
    }

    // ---- GET /projects --------------------------------------------------------

    #[tokio::test]
    async fn get_projects_returns_empty_list_initially() {
        // With no projects opened, GET /projects must return an empty array.
        let app = project_router().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/projects")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        let list = body.as_array().expect("response must be a JSON array");
        assert!(list.is_empty());
    }

    // ---- POST /projects/open --------------------------------------------------

    #[tokio::test]
    async fn post_projects_open_creates_project_entry() {
        // Opening a project by path must create a record and return it with name and path.
        let app = project_router().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/projects/open")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({
                        "path": "/workspace/my-project",
                        "name": "My Project"
                    })))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        assert_eq!(body["name"], "My Project");
        assert_eq!(body["path"], "/workspace/my-project");
        assert!(body["id"].as_i64().is_some());
    }

    #[tokio::test]
    async fn post_projects_open_twice_returns_same_project() {
        // Opening the same path twice must return the same record (idempotent).
        let app = project_router().await;
        let open = |app: Router| {
            app.oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/projects/open")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({
                        "path": "/workspace/same",
                        "name": "Same"
                    })))
                    .unwrap(),
            )
        };
        let first = body_json(open(app.clone()).await.unwrap().into_body()).await;
        let second = body_json(open(app).await.unwrap().into_body()).await;
        assert_eq!(first["id"], second["id"]);
    }

    // ---- GET /projects/active -------------------------------------------------

    #[tokio::test]
    async fn get_active_returns_404_when_no_projects() {
        // With no projects opened, GET /projects/active must return 404.
        let app = project_router().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/projects/active")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_active_returns_most_recently_opened_project() {
        // After opening a project, GET /projects/active must return it.
        let app = project_router().await;
        let open_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/projects/open")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({
                        "path": "/workspace/active-test"
                    })))
                    .unwrap(),
            )
            .await
            .unwrap();
        let opened = body_json(open_resp.into_body()).await;

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/projects/active")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let active = body_json(resp.into_body()).await;
        assert_eq!(active["id"], opened["id"]);
    }

    // ---- GET /projects/settings -----------------------------------------------

    #[tokio::test]
    async fn get_project_settings_returns_200() {
        // GET /projects/settings must return 200. The fixture project may or may
        // not have a project.json — either an object or null is valid, but the
        // status must be 200.
        let app = project_router().await;
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/projects/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    // ---- PUT /projects/settings -----------------------------------------------

    #[tokio::test]
    async fn put_project_settings_writes_and_persists() {
        // PUT /projects/settings must write the value and return it.
        // We use a temp dir as project root so the test doesn't pollute the fixture.
        let tmp = tempfile::TempDir::new().expect("tempdir");
        let tmp_root = tmp.path().to_path_buf();

        let storage = Storage::open_in_memory()
            .await
            .map(Arc::new)
            .expect("in-memory storage");

        let state = HealthState::for_test(
            GraphState::build_empty(&tmp_root),
            Some(Arc::clone(&storage)),
        );

        let project_sub = Router::new()
            .route(
                "/settings",
                get(get_project_settings).put(update_project_settings),
            )
            .with_state(state);

        let router = Router::new().nest("/projects", project_sub);

        let settings_value = serde_json::json!({
            "theme": "dark",
            "language": "en"
        });

        let put_resp = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/projects/settings")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "settings": settings_value })))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(put_resp.status(), StatusCode::OK);
        let put_body = body_json(put_resp.into_body()).await;
        assert_eq!(put_body["theme"], "dark");

        // Now GET must return the same value.
        let get_resp = router
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/projects/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(get_resp.status(), StatusCode::OK);
        let get_body = body_json(get_resp.into_body()).await;
        assert_eq!(get_body["theme"], "dark");
        assert_eq!(get_body["language"], "en");
    }
}
