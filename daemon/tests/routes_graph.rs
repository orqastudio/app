// Integration tests for graph route handlers: GET /graph/stats, GET /graph/health,
// POST /reload.
//
// All tests dispatch requests via `tower::ServiceExt::oneshot` against a real
// axum Router built from `build_app_router()`. The router uses real route
// handlers from `orqa_daemon_lib` backed by the minimal fixture project at
// `tests/fixtures/minimal-project/` (3 nodes: EPIC-test001, TASK-test001,
// RULE-test001; 1 edge: TASK→EPIC via `delivers`).
//
// Tests verify the HTTP contract (status codes, JSON field presence and values)
// without inspecting internal implementation details.

#![allow(missing_docs)]

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

// ---------------------------------------------------------------------------
// GET /graph/stats
// ---------------------------------------------------------------------------

/// GET /graph/stats returns 200 with node_count matching the fixture (3 nodes).
#[tokio::test]
async fn graph_stats_node_count_matches_fixtures() {
    let router = helpers::build_app_router().await;
    let request = Request::builder()
        .method("GET")
        .uri("/graph/stats")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Fixture has exactly 3 artifacts: EPIC-test001, TASK-test001, RULE-test001.
    assert_eq!(
        json["node_count"], 3,
        "node_count must equal number of fixture artifacts"
    );
}

/// GET /graph/stats returns edge_count > 0 because the fixture has a task→epic relationship.
#[tokio::test]
async fn graph_stats_edge_count_nonzero() {
    let router = helpers::build_app_router().await;
    let request = Request::builder()
        .method("GET")
        .uri("/graph/stats")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // TASK-test001 delivers EPIC-test001 — at least 1 directed edge must be present.
    let edge_count = json["edge_count"]
        .as_u64()
        .expect("edge_count must be a number");
    assert!(
        edge_count > 0,
        "edge_count must be > 0 (fixture has task→epic edge)"
    );
}

/// GET /graph/stats returns 200 with all required fields present.
#[tokio::test]
async fn graph_stats_has_required_fields() {
    let router = helpers::build_app_router().await;
    let request = Request::builder()
        .method("GET")
        .uri("/graph/stats")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(
        json.get("node_count").is_some(),
        "stats must include node_count"
    );
    assert!(
        json.get("edge_count").is_some(),
        "stats must include edge_count"
    );
    assert!(
        json.get("orphan_count").is_some(),
        "stats must include orphan_count"
    );
    assert!(
        json.get("broken_refs").is_some(),
        "stats must include broken_refs"
    );
}

// ---------------------------------------------------------------------------
// GET /graph/health
// ---------------------------------------------------------------------------

/// GET /graph/health returns 200 with all required GraphHealth fields.
#[tokio::test]
async fn graph_health_returns_200_with_valid_fields() {
    let router = helpers::build_app_router().await;
    let request = Request::builder()
        .method("GET")
        .uri("/graph/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Verify required GraphHealth fields are present.
    assert!(
        json.get("total_nodes").is_some(),
        "health must include total_nodes"
    );
    assert!(
        json.get("total_edges").is_some(),
        "health must include total_edges"
    );
    assert!(
        json.get("delivery_connectivity").is_some(),
        "health must include delivery_connectivity"
    );
    assert!(
        json.get("learning_connectivity").is_some(),
        "health must include learning_connectivity"
    );
    assert!(
        json.get("broken_ref_count").is_some(),
        "health must include broken_ref_count"
    );
    assert!(
        json.get("outlier_count").is_some(),
        "health must include outlier_count"
    );
}

/// GET /graph/health total_nodes matches the fixture count (3 nodes).
#[tokio::test]
async fn graph_health_total_nodes_matches_fixtures() {
    let router = helpers::build_app_router().await;
    let request = Request::builder()
        .method("GET")
        .uri("/graph/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json["total_nodes"], 3,
        "health total_nodes must match fixture artifact count"
    );
}

// ---------------------------------------------------------------------------
// POST /reload
// ---------------------------------------------------------------------------

/// POST /reload returns 200 with status "reloaded" and the current artifact count.
#[tokio::test]
async fn reload_returns_reloaded_status_with_artifact_count() {
    let router = helpers::build_app_router().await;
    let request = Request::builder()
        .method("POST")
        .uri("/reload")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json["status"], "reloaded",
        "reload response must have status = reloaded"
    );
    let artifacts = json["artifacts"]
        .as_u64()
        .expect("reload must return artifacts count");
    assert_eq!(
        artifacts, 3,
        "artifact count after reload must match fixture (3 nodes)"
    );
}
