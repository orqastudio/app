// Smoke tests for the orqa-daemon HTTP router.
#![allow(missing_docs)]
//
// These tests verify that the three core daemon endpoints are reachable and
// return meaningful data when built from the minimal fixture project. They use
// `tower::ServiceExt::oneshot` to dispatch requests directly to the router
// without binding a real port.
//
// What these tests verify:
//   - GET /health returns 200 with a JSON body that has "status": "ok"
//   - GET /artifacts returns 200 with a non-empty list (fixture has 3 artifacts)
//   - GET /graph/stats returns 200 with node_count > 0 (fixture has 3 nodes)
//
// These are BEHAVIOR tests. If someone breaks the routing wiring, the engine
// graph construction, or the fixture files, at least one of these will fail.

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

/// GET /health must return 200 with a JSON body containing `"status": "ok"`.
///
/// This verifies that the health route is mounted and the handler can access
/// the graph state without panicking.
#[tokio::test]
async fn health_returns_200_and_ok_status() {
    let app = helpers::build_test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes)
        .expect("health response must be valid JSON");

    assert_eq!(
        body["status"],
        serde_json::json!("ok"),
        "health response must include status: ok"
    );
}

/// GET /artifacts must return 200 with a JSON array containing at least one artifact.
///
/// The fixture project has three artifacts: EPIC-test001, TASK-test001, RULE-test001.
/// An empty list here means graph construction failed or the fixtures are broken.
#[tokio::test]
async fn artifacts_returns_non_empty_list() {
    let app = helpers::build_test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/artifacts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes)
        .expect("artifacts response must be valid JSON");

    let artifacts = body.as_array().expect("artifacts response must be a JSON array");
    assert!(
        !artifacts.is_empty(),
        "artifacts list must not be empty — fixture project has 3 artifacts"
    );
}

/// GET /graph/stats must return 200 with `node_count` > 0.
///
/// This verifies that the graph stats endpoint is mounted and the engine's
/// `graph_stats` function sees the real fixture nodes. A node_count of 0 means
/// graph construction silently returned an empty graph — a real defect.
#[tokio::test]
async fn graph_stats_returns_positive_node_count() {
    let app = helpers::build_test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/graph/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes)
        .expect("graph/stats response must be valid JSON");

    let node_count = body["node_count"]
        .as_u64()
        .expect("node_count must be a non-negative integer");

    assert!(
        node_count > 0,
        "node_count must be > 0 — fixture project has 3 artifacts (epic, task, rule)"
    );
}
