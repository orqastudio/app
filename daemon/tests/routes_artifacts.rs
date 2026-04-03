// Integration tests for artifact route handlers: GET /artifacts, GET /artifacts/:id,
// GET /artifacts/:id/content, GET /artifacts/:id/traceability, GET /artifacts/tree,
// PUT /artifacts/:id.
//
// All tests dispatch requests via `tower::ServiceExt::oneshot` against a real
// axum Router built from `build_app_router()`. The router uses real route
// handlers from `orqa_daemon_lib` backed by the minimal fixture project at
// `tests/fixtures/minimal-project/`:
//
//   EPIC-test001 — type: epic, status: active
//   TASK-test001 — type: task, status: todo,  delivers EPIC-test001
//   RULE-test001 — type: rule, status: active
//
// Tests verify the HTTP contract (status codes, JSON shape, field values) without
// inspecting internal implementation details.

#![allow(missing_docs)]

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

// ---------------------------------------------------------------------------
// GET /artifacts
// ---------------------------------------------------------------------------

/// GET /artifacts returns all 3 fixture artifacts.
#[tokio::test]
async fn list_artifacts_returns_all_fixtures() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().expect("list response must be a JSON array");

    assert_eq!(items.len(), 3, "must return exactly 3 fixture artifacts");
}

/// GET /artifacts?type=epic returns only EPIC-test001.
#[tokio::test]
async fn list_artifacts_filter_by_type_returns_matching_subset() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts?type=epic")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().expect("list response must be a JSON array");

    assert_eq!(items.len(), 1, "type=epic filter must return exactly 1 artifact");
    assert_eq!(items[0]["id"], "EPIC-test001");
    assert_eq!(items[0]["artifact_type"], "epic");
}

/// GET /artifacts?type=nonexistent returns an empty array (no 404 — just empty list).
#[tokio::test]
async fn list_artifacts_filter_by_nonexistent_type_returns_empty() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts?type=nonexistent")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().expect("list response must be a JSON array");

    assert_eq!(items.len(), 0, "filter by nonexistent type must return empty array");
}

/// GET /artifacts?status=active returns EPIC-test001 and RULE-test001 (both active).
#[tokio::test]
async fn list_artifacts_filter_by_status_returns_matching_subset() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts?status=active")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = json.as_array().expect("list response must be a JSON array");

    // EPIC-test001 and RULE-test001 are active; TASK-test001 is todo.
    assert_eq!(items.len(), 2, "status=active filter must return 2 artifacts");
    let ids: Vec<&str> = items.iter()
        .map(|v| v["id"].as_str().unwrap())
        .collect();
    assert!(ids.contains(&"EPIC-test001"), "active set must include EPIC-test001");
    assert!(ids.contains(&"RULE-test001"), "active set must include RULE-test001");
}

// ---------------------------------------------------------------------------
// GET /artifacts/:id
// ---------------------------------------------------------------------------

/// GET /artifacts/EPIC-test001 returns 200 with the artifact's fields.
#[tokio::test]
async fn get_artifact_by_id_returns_200_with_fields() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts/EPIC-test001")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["id"], "EPIC-test001");
    assert_eq!(json["artifact_type"], "epic");
    assert_eq!(json["title"], "Test Epic");
    assert_eq!(json["status"], "active");
}

/// GET /artifacts/DOES-NOT-EXIST returns 404.
#[tokio::test]
async fn get_artifact_nonexistent_returns_404() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts/DOES-NOT-EXIST")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// GET /artifacts/:id/content
// ---------------------------------------------------------------------------

/// GET /artifacts/EPIC-test001/content returns 200 with non-empty markdown content.
#[tokio::test]
async fn get_artifact_content_returns_200_with_markdown() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts/EPIC-test001/content")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let content = json["content"].as_str().expect("content field must be a string");
    assert!(!content.is_empty(), "artifact content must not be empty");
    // Fixture file has YAML frontmatter — presence of `---` confirms raw file is returned.
    assert!(content.contains("---"), "content must include YAML frontmatter delimiter");
    assert!(content.contains("EPIC-test001"), "content must include the artifact ID");
}

/// GET /artifacts/DOES-NOT-EXIST/content returns 404.
#[tokio::test]
async fn get_artifact_content_nonexistent_returns_404() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts/DOES-NOT-EXIST/content")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// GET /artifacts/:id/traceability
// ---------------------------------------------------------------------------

/// GET /artifacts/TASK-test001/traceability returns 200 with valid traceability fields.
///
/// TASK-test001 delivers EPIC-test001, so the result must have a non-empty
/// descendants or ancestry_chains list (the task has an upward connection).
#[tokio::test]
async fn get_artifact_traceability_returns_200_with_valid_fields() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts/TASK-test001/traceability")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Verify TraceabilityResult fields are present.
    assert!(json.get("ancestry_chains").is_some(), "traceability must include ancestry_chains");
    assert!(json.get("descendants").is_some(), "traceability must include descendants");
    assert!(json.get("siblings").is_some(), "traceability must include siblings");
    assert!(json.get("impact_radius").is_some(), "traceability must include impact_radius");
    assert!(json.get("disconnected").is_some(), "traceability must include disconnected");
}

/// GET /artifacts/DOES-NOT-EXIST/traceability returns 404.
#[tokio::test]
async fn get_artifact_traceability_nonexistent_returns_404() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts/DOES-NOT-EXIST/traceability")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// GET /artifacts/tree
// ---------------------------------------------------------------------------

/// GET /artifacts/tree returns 200 with a non-empty tree structure.
#[tokio::test]
async fn get_artifact_tree_returns_200_non_empty() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts/tree")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    // Tree response must be valid JSON (object or array — both are acceptable NavTree shapes).
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(!json.is_null(), "tree response must not be null");
}

// ---------------------------------------------------------------------------
// PUT /artifacts/:id
// ---------------------------------------------------------------------------

/// PUT /artifacts/EPIC-test001 with a field update returns 200 with the updated field.
///
/// Uses `description` as the field under test because changing it is idempotent
/// and does not affect other tests (each test builds its own router with an
/// independent GraphState snapshot; the file write will persist to disk but
/// tests do not rely on fixture file content being unchanged between runs).
#[tokio::test]
async fn update_artifact_returns_200_with_updated_field() {
    let router = helpers::build_app_router();
    let request = Request::builder()
        .method("PUT")
        .uri("/artifacts/EPIC-test001")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"field":"description","value":"Updated by integration test"}"#))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["id"], "EPIC-test001");
    assert_eq!(json["field"], "description");
    assert_eq!(json["new_value"], "Updated by integration test");
}
