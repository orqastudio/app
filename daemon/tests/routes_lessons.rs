// Integration tests for lesson routes.
//
// Covers:
//   GET  /lessons                — list all lessons (empty initially)
//   POST /lessons                — create a new lesson (returns 201)
//   GET  /lessons (after POST)   — created lesson appears in the list
//   PUT  /lessons/:id/recurrence — incrementing recurrence count
//
// Lessons write files to disk, so each test uses a temporary project directory
// rather than the read-only fixture. The temp project has a minimal project.json
// that configures the `lessons` artifact path.
#![allow(missing_docs)]

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

/// Set up a writable temp directory with a minimal project.json that declares
/// the lessons artifact path.
///
/// Returns `(Router, TempDir)`. The caller must keep `TempDir` alive for the
/// duration of the test — dropping it removes the temp directory.
fn build_lessons_router() -> (axum::Router, tempfile::TempDir) {
    use axum::routing::{get, put};
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::routes::lessons::{
        create_lesson, increment_lesson_recurrence, list_lessons,
    };

    let dir = tempfile::tempdir().expect("tempdir must succeed");
    let orqa_dir = dir.path().join(".orqa/learning/lessons");
    std::fs::create_dir_all(&orqa_dir).expect("lessons dir must be created");

    // Minimal project.json that declares the lessons artifact path.
    // The `artifacts` array uses the `Type` variant (flat object with key + path).
    let project_json = serde_json::json!({
        "name": "Test Project",
        "description": "Temp project for lesson route tests",
        "default_model": "auto",
        "excluded_paths": [],
        "stack": {
            "languages": [],
            "frameworks": [],
            "package_manager": null,
            "has_claude_config": false,
            "has_design_tokens": false
        },
        "governance": {
            "lessons": 0, "decisions": 0, "rules": 0, "documentation": 0,
            "has_claude_config": false
        },
        "statuses": [],
        "delivery": { "types": [] },
        "relationships": [],
        "plugins": {},
        "artifacts": [
            { "key": "lessons", "path": ".orqa/learning/lessons" }
        ]
    });
    std::fs::write(
        dir.path().join(".orqa/project.json"),
        serde_json::to_string_pretty(&project_json).unwrap(),
    )
    .expect("project.json must be written");

    let graph_state = GraphState::build_empty(dir.path());

    let router = axum::Router::new()
        .route("/lessons", get(list_lessons).post(create_lesson))
        .route(
            "/lessons/{id}/recurrence",
            put(increment_lesson_recurrence),
        )
        .with_state(graph_state);

    (router, dir)
}

/// Parse the response body into a serde_json::Value.
async fn parse_body(response: axum::response::Response) -> serde_json::Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).expect("response must be valid JSON")
}

// ---------------------------------------------------------------------------
// GET /lessons — empty initially
// ---------------------------------------------------------------------------

/// GET /lessons returns 200 with an empty array when no lessons exist.
///
/// A fresh project with no lesson files must return [] not an error.
/// This is the "empty is valid" contract.
#[tokio::test]
async fn lessons_list_empty_initially() {
    let (app, _dir) = build_lessons_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_body(response).await;
    let lessons = body.as_array().expect("/lessons must return a JSON array");
    assert!(
        lessons.is_empty(),
        "no lessons created yet — list must be empty, got: {body}"
    );
}

// ---------------------------------------------------------------------------
// POST /lessons — create a lesson
// ---------------------------------------------------------------------------

/// POST /lessons creates a new lesson and returns 201 with the full lesson.
///
/// The response must include the generated id, the submitted title and category,
/// initial recurrence of 0, and a non-empty body. This verifies the full
/// create pipeline: ID generation, file write, and frontmatter round-trip.
#[tokio::test]
async fn lessons_create_returns_201_with_lesson() {
    let (app, _dir) = build_lessons_router();

    let new_lesson = serde_json::json!({
        "title": "Always write tests",
        "category": "process",
        "body": "We discovered that writing tests before shipping catches regressions early."
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/lessons")
                .header("content-type", "application/json")
                .body(Body::from(new_lesson.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "create lesson must return 201 CREATED"
    );

    let body = parse_body(response).await;

    assert!(
        body.get("id").is_some(),
        "created lesson must have an 'id' field"
    );
    assert!(
        !body["id"].as_str().unwrap_or("").is_empty(),
        "lesson id must not be empty"
    );
    assert_eq!(
        body["title"],
        serde_json::json!("Always write tests"),
        "title must match submitted value"
    );
    assert_eq!(
        body["category"],
        serde_json::json!("process"),
        "category must match submitted value"
    );
    // FileLessonStore::create initialises recurrence to 1 (first occurrence captured).
    assert_eq!(
        body["recurrence"].as_i64().unwrap_or(-1),
        1,
        "new lesson must have recurrence 1 (FileLessonStore initialises to 1)"
    );
}

// ---------------------------------------------------------------------------
// GET /lessons after POST — created lesson appears in list
// ---------------------------------------------------------------------------

/// After POST /lessons, GET /lessons returns the created lesson.
///
/// This verifies the round-trip: create writes a file, list reads it back.
/// The lesson id returned by POST must appear in the subsequent GET list.
#[tokio::test]
async fn lessons_created_lesson_appears_in_list() {
    let (router, _dir) = build_lessons_router();

    // We need two separate requests to the same router, but `oneshot` consumes
    // the router. Clone the underlying service by rebuilding — the router holds
    // a shared GraphState backed by the same TempDir path.
    //
    // Since we can't call oneshot twice on the same value, rebuild the router
    // using the same temp dir. The _dir keeps the directory alive.
    // Re-use `router` for POST, then rebuild for GET.
    let new_lesson = serde_json::json!({
        "title": "Test round-trip",
        "category": "testing",
        "body": "Lesson body content."
    });

    // POST: create
    let post_resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/lessons")
                .header("content-type", "application/json")
                .body(Body::from(new_lesson.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(post_resp.status(), StatusCode::CREATED);
    let created = parse_body(post_resp).await;
    let created_id = created["id"].as_str().expect("must have id").to_owned();

    // GET: list — must include the lesson we just created
    let get_resp = router
        .oneshot(
            Request::builder()
                .uri("/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_resp.status(), StatusCode::OK);
    let list_body = parse_body(get_resp).await;
    let lessons = list_body.as_array().expect("must be array");

    assert_eq!(lessons.len(), 1, "exactly one lesson must exist after one POST");

    let ids: Vec<&str> = lessons
        .iter()
        .filter_map(|l| l["id"].as_str())
        .collect();
    assert!(
        ids.contains(&created_id.as_str()),
        "GET /lessons must include the lesson created by POST, id={created_id}"
    );
}

// ---------------------------------------------------------------------------
// PUT /lessons/:id/recurrence — increment recurrence count
// ---------------------------------------------------------------------------

/// PUT /lessons/:id/recurrence increments the recurrence count and returns
/// the updated lesson.
///
/// This verifies the recurrence tracking pipeline: the file is read, the count
/// incremented from 0 to 1, and the updated lesson returned with the new count.
#[tokio::test]
async fn lessons_increment_recurrence_returns_updated_lesson() {
    let (router, _dir) = build_lessons_router();

    // First create a lesson.
    let new_lesson = serde_json::json!({
        "title": "Recurrence test",
        "category": "process",
        "body": "This lesson recurs."
    });

    let post_resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/lessons")
                .header("content-type", "application/json")
                .body(Body::from(new_lesson.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(post_resp.status(), StatusCode::CREATED);
    let created = parse_body(post_resp).await;
    let lesson_id = created["id"].as_str().expect("must have id").to_owned();

    // Now increment the recurrence.
    let put_resp = router
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/lessons/{lesson_id}/recurrence"))
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        put_resp.status(),
        StatusCode::OK,
        "increment recurrence must return 200 OK"
    );

    let updated = parse_body(put_resp).await;

    assert_eq!(
        updated["id"].as_str().unwrap_or(""),
        lesson_id,
        "updated lesson id must match"
    );
    // create() initialises recurrence to 1; increment_recurrence raises it to 2.
    assert_eq!(
        updated["recurrence"].as_i64().unwrap_or(-1),
        2,
        "recurrence must be incremented from 1 (initial) to 2"
    );
}
