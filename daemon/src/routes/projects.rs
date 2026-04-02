// Project management routes: list, open, settings, scan, and icon.
//
// Projects are tracked in the daemon SQLite database. Project settings
// (project.json) are read and written via the orqa-project crate's
// FileProjectSettingsStore. The scan endpoint delegates to the project
// scanner to detect the technology stack.
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
use orqa_project::scanner::scan_project;
use orqa_project::store::FileProjectSettingsStore;

use crate::graph_state::GraphState;
use crate::health::HealthState;
use crate::store::{project_create, project_get_active, project_get_by_path, project_list, project_touch, Project};

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

/// Response helper when the daemon store is unavailable.
fn store_unavailable() -> (StatusCode, Json<serde_json::Value>) {
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
) -> Result<Json<Vec<Project>>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;
        project_list(&conn).map(Json).map_err(|e| (
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

/// Handle GET /projects/active — return the most recently used project.
///
/// Returns 404 when no projects have been opened yet.
pub async fn get_active_project(
    State(state): State<HealthState>,
) -> Result<Json<Project>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;

        match project_get_active(&conn) {
            Ok(Some(p)) => Ok(Json(p)),
            Ok(None) => Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "no active project", "code": "NOT_FOUND" })),
            )),
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e, "code": "DB_ERROR" })),
            )),
        }
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle POST /projects/open — open a project by path, creating a record if needed.
///
/// If the project is already known, touches its updated_at to surface it as
/// active. If new, inserts a record with the provided (or derived) name.
pub async fn open_project(
    State(state): State<HealthState>,
    Json(req): Json<OpenProjectRequest>,
) -> Result<Json<Project>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;
    let path = req.path.clone();
    let name = req.name.clone();

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;

        // Check if the project already exists by path.
        match project_get_by_path(&conn, &path) {
            Ok(Some(existing)) => {
                // Touch to make it the most recently active project.
                let _ = project_touch(&conn, existing.id);
                let refreshed = crate::store::project_get(&conn, existing.id)
                    .unwrap_or(existing);
                Ok(Json(refreshed))
            }
            Ok(None) => {
                // Derive a display name from the final path component if not provided.
                let display_name = name.unwrap_or_else(|| {
                    std::path::Path::new(&path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("project")
                        .to_owned()
                });
                project_create(&conn, &display_name, &path, None)
                    .map(Json)
                    .map_err(|e| (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        Json(serde_json::json!({ "error": e, "code": "CREATE_FAILED" })),
                    ))
            }
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e, "code": "DB_ERROR" })),
            )),
        }
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle GET /projects/settings — read project.json for the active project.
///
/// Returns the raw project.json value so callers receive the full current shape
/// without coupling to a specific Rust struct version.
pub async fn get_project_settings(
    State(state): State<HealthState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
    ))?;

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
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle PUT /projects/settings — write project.json for the active project.
///
/// Overwrites project.json with the provided value. Merges are the caller's
/// responsibility — pass the full desired settings object.
pub async fn update_project_settings(
    State(state): State<HealthState>,
    Json(req): Json<UpdateProjectSettingsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
    ))?;
    let settings = req.settings.clone();

    tokio::task::spawn_blocking(move || {
        let store = FileProjectSettingsStore::new();
        store.save(&project_root, &settings)
            .map(|()| Json(settings))
            .map_err(|e| (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": e.to_string(), "code": "SETTINGS_SAVE_FAILED" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle POST /projects/scan — scan the project filesystem for stack and governance info.
///
/// Delegates to `orqa_project::scanner::scan_project`. Returns detected languages,
/// frameworks, and governance artifact counts.
pub async fn scan_project_handler(
    State(state): State<HealthState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
    ))?;

    tokio::task::spawn_blocking(move || {
        let path_str = project_root.to_string_lossy().to_string();
        scan_project(&path_str, &[])
            .map(|r| Json(serde_json::to_value(&r).unwrap_or(serde_json::Value::Null)))
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "SCAN_FAILED" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle POST /projects/icon — store an uploaded project icon.
///
/// Accepts raw bytes (PNG/JPG) and writes them to `.state/project-icon` under
/// the project root. Returns 204 on success.
pub async fn upload_project_icon(
    State(state): State<HealthState>,
    body: Bytes,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
    ))?;

    tokio::task::spawn_blocking(move || {
        let state_dir = project_root.join(".state");
        std::fs::create_dir_all(&state_dir).map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "IO_ERROR" })),
        ))?;
        let icon_path = state_dir.join("project-icon");
        std::fs::write(&icon_path, &body).map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "IO_ERROR" })),
        ))?;
        Ok(StatusCode::NO_CONTENT)
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle GET /projects/icon — read the project icon bytes.
///
/// Returns the raw bytes from `.state/project-icon`. Returns 404 when no
/// icon has been uploaded.
pub async fn get_project_icon(
    State(state): State<HealthState>,
) -> Result<Bytes, (StatusCode, Json<serde_json::Value>)> {
    let project_root = project_root_from_graph(&state.graph_state).ok_or_else(|| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": "project root unavailable", "code": "STATE_ERROR" })),
    ))?;

    tokio::task::spawn_blocking(move || {
        let icon_path = project_root.join(".state/project-icon");
        if !icon_path.exists() {
            return Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "no project icon set", "code": "NOT_FOUND" })),
            ));
        }
        std::fs::read(&icon_path)
            .map(Bytes::from)
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "IO_ERROR" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}
