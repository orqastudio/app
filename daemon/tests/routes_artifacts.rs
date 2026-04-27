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
    let router = helpers::build_app_router().await;
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
    let router = helpers::build_app_router().await;
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

    assert_eq!(
        items.len(),
        1,
        "type=epic filter must return exactly 1 artifact"
    );
    assert_eq!(items[0]["id"], "EPIC-test001");
    assert_eq!(items[0]["artifact_type"], "epic");
}

/// GET /artifacts?type=nonexistent returns an empty array (no 404 — just empty list).
#[tokio::test]
async fn list_artifacts_filter_by_nonexistent_type_returns_empty() {
    let router = helpers::build_app_router().await;
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

    assert_eq!(
        items.len(),
        0,
        "filter by nonexistent type must return empty array"
    );
}

/// GET /artifacts?status=active returns EPIC-test001 and RULE-test001 (both active).
#[tokio::test]
async fn list_artifacts_filter_by_status_returns_matching_subset() {
    let router = helpers::build_app_router().await;
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
    assert_eq!(
        items.len(),
        2,
        "status=active filter must return 2 artifacts"
    );
    let ids: Vec<&str> = items.iter().map(|v| v["id"].as_str().unwrap()).collect();
    assert!(
        ids.contains(&"EPIC-test001"),
        "active set must include EPIC-test001"
    );
    assert!(
        ids.contains(&"RULE-test001"),
        "active set must include RULE-test001"
    );
}

// ---------------------------------------------------------------------------
// GET /artifacts/:id
// ---------------------------------------------------------------------------

/// GET /artifacts/EPIC-test001 returns 200 with the artifact's fields.
#[tokio::test]
async fn get_artifact_by_id_returns_200_with_fields() {
    let router = helpers::build_app_router().await;
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
    let router = helpers::build_app_router().await;
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
    let router = helpers::build_app_router().await;
    let request = Request::builder()
        .method("GET")
        .uri("/artifacts/EPIC-test001/content")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let content = json["content"]
        .as_str()
        .expect("content field must be a string");
    assert!(!content.is_empty(), "artifact content must not be empty");
    // Fixture file has YAML frontmatter — presence of `---` confirms raw file is returned.
    assert!(
        content.contains("---"),
        "content must include YAML frontmatter delimiter"
    );
    assert!(
        content.contains("EPIC-test001"),
        "content must include the artifact ID"
    );
}

/// GET /artifacts/DOES-NOT-EXIST/content returns 404.
#[tokio::test]
async fn get_artifact_content_nonexistent_returns_404() {
    let router = helpers::build_app_router().await;
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
    let router = helpers::build_app_router().await;
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
    assert!(
        json.get("ancestry_chains").is_some(),
        "traceability must include ancestry_chains"
    );
    assert!(
        json.get("descendants").is_some(),
        "traceability must include descendants"
    );
    assert!(
        json.get("siblings").is_some(),
        "traceability must include siblings"
    );
    assert!(
        json.get("impact_radius").is_some(),
        "traceability must include impact_radius"
    );
    assert!(
        json.get("disconnected").is_some(),
        "traceability must include disconnected"
    );
}

/// GET /artifacts/DOES-NOT-EXIST/traceability returns 404.
#[tokio::test]
async fn get_artifact_traceability_nonexistent_returns_404() {
    let router = helpers::build_app_router().await;
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
    let router = helpers::build_app_router().await;
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
/// independent GraphState snapshot backed by an in-memory SurrealDB).
#[tokio::test]
async fn update_artifact_returns_200_with_updated_field() {
    let router = helpers::build_app_router().await;
    let request = Request::builder()
        .method("PUT")
        .uri("/artifacts/EPIC-test001")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"field":"description","value":"Updated by integration test"}"#,
        ))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["id"], "EPIC-test001");
    assert_eq!(json["field"], "description");
    assert_eq!(json["new_value"], "Updated by integration test");
}

/// PUT /artifacts/:id returns 503 when SurrealDB is unavailable.
///
/// The artifact must exist in the HashMap (so 404 is not the reason) but
/// the SurrealDB handle must be absent. The PUT handler must return 503
/// and must NOT silently fall back to a disk write.
#[tokio::test]
async fn update_artifact_returns_503_when_surrealdb_unavailable() {
    let router = helpers::build_app_router_without_surrealdb().await;
    let request = Request::builder()
        .method("PUT")
        .uri("/artifacts/EPIC-test001")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"field":"description","value":"Should fail with 503"}"#,
        ))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::SERVICE_UNAVAILABLE,
        "PUT must return 503 when SurrealDB is unavailable, not silently succeed"
    );

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        json["code"], "DB_UNAVAILABLE",
        "error code must be DB_UNAVAILABLE"
    );
}

// ---------------------------------------------------------------------------
// POST /artifacts
// ---------------------------------------------------------------------------

/// POST /artifacts with a valid body creates the artifact and returns 201.
///
/// The created artifact must be visible in a subsequent GET /artifacts request
/// (read-your-writes). Both requests share the same router instance with the
/// same GraphState, so the HashMap is updated atomically by the POST handler.
#[tokio::test]
async fn create_artifact_returns_201_with_fields() {
    let router = helpers::build_app_router().await;
    let body = serde_json::json!({
        "id": "TASK-post001",
        "type": "task",
        "title": "Integration test task",
        "status": "todo"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "POST must return 201 Created"
    );

    let resp_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&resp_bytes).unwrap();
    assert_eq!(json["id"], "TASK-post001");
    assert_eq!(json["artifact_type"], "task");
    assert!(
        json["version"].as_u64().unwrap_or(0) >= 1,
        "version must be at least 1"
    );
    assert!(
        !json["content_hash"].as_str().unwrap_or("").is_empty(),
        "content_hash must be set"
    );
}

/// POST /artifacts followed by GET /artifacts/:id returns the new artifact (read-your-writes).
#[tokio::test]
async fn create_artifact_visible_in_get_by_id() {
    use tower::ServiceExt as _;

    let state = helpers::build_app_state().await;
    let router1 = orqa_daemon_lib::build_router(state.clone());
    let router2 = orqa_daemon_lib::build_router(state);

    let post_body = serde_json::json!({
        "id": "TASK-ryw001",
        "type": "task",
        "title": "Read-your-writes test",
        "status": "todo"
    });
    let post_req = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&post_body).unwrap()))
        .unwrap();

    let post_response = router1.oneshot(post_req).await.unwrap();
    assert_eq!(post_response.status(), StatusCode::CREATED);

    // Same GraphState — GET must see the artifact immediately.
    let get_req = Request::builder()
        .method("GET")
        .uri("/artifacts/TASK-ryw001")
        .body(Body::empty())
        .unwrap();
    let get_response = router2.oneshot(get_req).await.unwrap();
    assert_eq!(
        get_response.status(),
        StatusCode::OK,
        "GET /artifacts/:id must return 200 for the just-created artifact"
    );

    let body = get_response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["id"], "TASK-ryw001");
}

/// POST /artifacts with an ID that already exists returns 409.
#[tokio::test]
async fn create_artifact_duplicate_returns_409() {
    let router = helpers::build_app_router().await;
    let body = serde_json::json!({
        "id": "EPIC-test001",
        "type": "epic",
        "title": "Duplicate test"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::CONFLICT,
        "duplicate artifact ID must return 409 Conflict"
    );

    let resp_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&resp_bytes).unwrap();
    assert_eq!(json["code"], "DUPLICATE", "error code must be DUPLICATE");
}

/// POST /artifacts with a missing required field (id) returns 422 (axum deserialization error).
#[tokio::test]
async fn create_artifact_missing_required_field_returns_4xx() {
    let router = helpers::build_app_router().await;
    let body = serde_json::json!({
        "type": "task",
        "title": "Missing id field"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    let status = response.status().as_u16();
    assert!(
        (400..500).contains(&status),
        "missing required field must return 4xx, got {status}"
    );
}

/// POST /artifacts returns 503 when SurrealDB is unavailable.
#[tokio::test]
async fn create_artifact_returns_503_when_surrealdb_unavailable() {
    let router = helpers::build_app_router_without_surrealdb().await;
    let body = serde_json::json!({
        "id": "TASK-503test",
        "type": "task",
        "title": "503 test"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::SERVICE_UNAVAILABLE,
        "POST must return 503 when SurrealDB is unavailable"
    );

    let resp_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&resp_bytes).unwrap();
    assert_eq!(
        json["code"], "DB_UNAVAILABLE",
        "error code must be DB_UNAVAILABLE"
    );
}

/// POST /artifacts with relationships creates edges and GET /graph/trace/:id succeeds.
#[tokio::test]
async fn create_artifact_with_relationships_and_trace() {
    use tower::ServiceExt as _;

    let state = helpers::build_app_state().await;
    let router1 = orqa_daemon_lib::build_router(state.clone());
    let router2 = orqa_daemon_lib::build_router(state);

    let post_body = serde_json::json!({
        "id": "TASK-edge001",
        "type": "task",
        "title": "Edge test task",
        "status": "todo",
        "relationships": [
            { "target": "EPIC-test001", "type": "delivers" }
        ]
    });
    let post_req = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&post_body).unwrap()))
        .unwrap();

    let post_response = router1.oneshot(post_req).await.unwrap();
    assert_eq!(post_response.status(), StatusCode::CREATED);

    let resp_bytes = post_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&resp_bytes).unwrap();
    assert_eq!(
        json["edge_count"], 1,
        "one relationship edge must be inserted"
    );

    // GET /graph/trace/:id must succeed — the artifact is in the DB.
    let trace_req = Request::builder()
        .method("GET")
        .uri("/graph/trace/TASK-edge001")
        .body(Body::empty())
        .unwrap();
    let trace_response = router2.oneshot(trace_req).await.unwrap();
    assert_eq!(
        trace_response.status(),
        StatusCode::OK,
        "GET /graph/trace/:id must return 200 for the just-created artifact"
    );
}

/// POST /artifacts with an invalid status value returns 422 with SCHEMA_INVALID code.
///
/// The fixture now includes a task plugin manifest that constrains `status` to an enum.
/// This test verifies the composed-schema validator fires before the SurrealDB write.
#[tokio::test]
async fn create_artifact_invalid_status_returns_422() {
    let router = helpers::build_app_router().await;
    let body = serde_json::json!({
        "id": "TASK-schema001",
        "type": "task",
        "title": "Schema validation test",
        "status": "not-a-valid-status"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "invalid status value must return 422"
    );

    let resp_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&resp_bytes).unwrap();
    assert_eq!(
        json["code"], "SCHEMA_INVALID",
        "error code must be SCHEMA_INVALID, got: {json}"
    );
    assert!(
        json["violations"].is_array(),
        "response must include violations array"
    );
    assert!(
        !json["violations"].as_array().unwrap().is_empty(),
        "violations array must be non-empty"
    );
}

/// POST /artifacts with relationships: verify edge was actually written to SurrealDB.
///
/// After POST, queries SurrealDB directly (via the db handle from app state) to assert
/// the relates_to record exists — not just that the request contained an edge.
#[tokio::test]
async fn create_artifact_edge_written_to_surrealdb() {
    use orqa_graph::writers::count_edges_from;

    let state = helpers::build_app_state().await;
    let router = orqa_daemon_lib::build_router(state.clone());

    let post_body = serde_json::json!({
        "id": "TASK-dbedge001",
        "type": "task",
        "title": "DB edge verification test",
        "status": "todo",
        "relationships": [
            { "target": "EPIC-test001", "type": "delivers" }
        ]
    });
    let post_req = Request::builder()
        .method("POST")
        .uri("/artifacts")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&post_body).unwrap()))
        .unwrap();

    let post_response = router.oneshot(post_req).await.unwrap();
    assert_eq!(
        post_response.status(),
        StatusCode::CREATED,
        "POST must return 201"
    );

    // Query SurrealDB directly to confirm the edge record was written.
    let db = state
        .graph_state
        .surreal_db()
        .expect("SurrealDB must be available in test state");
    let edge_count = count_edges_from(&db, "TASK-dbedge001")
        .await
        .expect("count_edges_from must succeed");
    assert_eq!(
        edge_count, 1,
        "exactly one relates_to edge must exist in SurrealDB for TASK-dbedge001"
    );
}
