// Lesson routes: list, create, and update lesson recurrence.
//
// Lessons are markdown files in .orqa/learning/lessons/. All file I/O
// delegates to orqa_lesson::store::FileLessonStore, which is constructed
// from project settings at request time.
//
// Endpoints:
//   GET  /lessons               — list all lessons
//   POST /lessons               — create a new lesson
//   PUT  /lessons/:id/recurrence — increment recurrence count

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use orqa_engine_types::paths::ProjectPaths;
use orqa_engine_types::types::lesson::NewLesson;
use orqa_lesson::store::FileLessonStore;
use orqa_lesson::Lesson;

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a `FileLessonStore` for the given project root.
///
/// Loads project paths from project.json. Returns an error string if the
/// settings cannot be loaded.
fn make_lesson_store(project_root: &std::path::Path) -> Result<FileLessonStore, String> {
    let paths = ProjectPaths::load(project_root)
        .map_err(|e| format!("failed to load project paths: {e}"))?;
    Ok(FileLessonStore::new(paths))
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /lessons — return all lessons sorted by ID.
///
/// Returns an empty list if no lessons directory is configured or no lessons
/// have been created yet.
pub async fn list_lessons(
    State(state): State<GraphState>,
) -> Json<Vec<Lesson>> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Json(Vec::new());
        };
        guard.project_root.clone()
    };

    let Ok(store) = make_lesson_store(&project_root) else {
        return Json(Vec::new());
    };

    Json(store.list().unwrap_or_default())
}

/// Handle POST /lessons — create a new lesson file.
///
/// Generates the next sequential IMPL-NNN ID, writes the markdown file,
/// and returns the fully-populated lesson. Returns 201 on success.
pub async fn create_lesson(
    State(state): State<GraphState>,
    Json(req): Json<NewLesson>,
) -> Result<(StatusCode, Json<Lesson>), (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let store = make_lesson_store(&project_root).map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e, "code": "STORE_ERROR" })),
    ))?;

    let lesson = store.create(&req).map_err(|e| (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(serde_json::json!({ "error": e.to_string(), "code": "CREATE_FAILED" })),
    ))?;

    Ok((StatusCode::CREATED, Json(lesson)))
}

/// Handle PUT /lessons/:id/recurrence — increment the recurrence count.
///
/// Reads the lesson file, increments the count, updates the `updated` date,
/// and writes it back. Returns the updated lesson.
pub async fn increment_lesson_recurrence(
    State(state): State<GraphState>,
    Path(id): Path<String>,
) -> Result<Json<Lesson>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let store = make_lesson_store(&project_root).map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e, "code": "STORE_ERROR" })),
    ))?;

    store.increment_recurrence(&id).map(Json).map_err(|e| {
        let (status, code) = match &e {
            orqa_lesson::store::LessonStoreError::NotFound(_) => {
                (StatusCode::NOT_FOUND, "NOT_FOUND")
            }
            _ => (StatusCode::UNPROCESSABLE_ENTITY, "UPDATE_FAILED"),
        };
        (status, Json(serde_json::json!({ "error": e.to_string(), "code": code })))
    })
}
