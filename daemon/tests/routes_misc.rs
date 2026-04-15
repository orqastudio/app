//! Integration tests for miscellaneous daemon routes.
//
// Covers routes not exercised by smoke.rs:
//   GET  /plugins                      — list installed plugins (empty for fixture)
//   GET  /workflow/transitions          — evaluate status transitions
//   GET  /agents/behavioral-messages    — extract behavioral messages
//   GET  /search/status                 — search index status (no DB → error or empty)
//   GET  /hooks                         — list registered hooks
//   GET  /reload (POST)                 — rebuild graph state
//   POST /reload                        — confirm reload returns artifact + rule counts
//
// Routes backed by HealthState (sidecar, setup, startup, prompt) require the
// full build_router() path from lib.rs. Routes backed by GraphState can be
// tested with inline helpers.
//
// All tests use tower::ServiceExt::oneshot — no real port is bound.

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

// ---------------------------------------------------------------------------
// Plugin routes
// ---------------------------------------------------------------------------

/// GET /plugins returns 200 with a JSON array.
///
/// The fixture project has no installed plugins (project.json has "plugins": {})
/// so the list is expected to be empty. The key invariant is that the handler
/// does not error — an empty array is a valid response.
#[tokio::test]
async fn plugins_list_returns_200_and_json_array() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/plugins")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/plugins must return valid JSON");

    assert!(
        body.is_array(),
        "/plugins must return a JSON array, got: {body}"
    );
}

/// GET /plugins/:name returns 404 for an unknown plugin name.
#[tokio::test]
async fn plugins_get_unknown_returns_404() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/plugins/nonexistent-plugin-xyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// Workflow routes
// ---------------------------------------------------------------------------

/// GET /workflow/transitions returns 200 with a JSON array.
///
/// The fixture project has no custom status definitions, so the list may be
/// empty — but the response must be a JSON array with status 200.
#[tokio::test]
async fn workflow_transitions_returns_200_and_array() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/workflow/transitions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/workflow/transitions must return valid JSON");

    assert!(
        body.is_array(),
        "/workflow/transitions must return a JSON array, got: {body}"
    );
}

// ---------------------------------------------------------------------------
// Agent routes
// ---------------------------------------------------------------------------

/// GET /agents/behavioral-messages returns 200 with an object containing a messages array.
///
/// The fixture has RULE-test001 with a message, so `rule_count` and `behavioral_count`
/// are expected to be 1. The response shape is an object, not a bare array.
#[tokio::test]
async fn agents_behavioral_messages_returns_200_and_has_messages_field() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/agents/behavioral-messages")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/agents/behavioral-messages must return valid JSON");

    assert!(
        body.get("messages").is_some(),
        "/agents/behavioral-messages must return an object with 'messages' key, got: {body}"
    );
    assert!(
        body["messages"].is_array(),
        "'messages' must be an array, got: {body}"
    );
    // Fixture has RULE-test001 with a message — rule_count must be 1.
    assert_eq!(
        body["rule_count"].as_u64().unwrap_or(0),
        1,
        "fixture has exactly 1 rule with a behavioral message"
    );
}

/// GET /agents/:role returns 404 when no matching agent definition exists.
///
/// The fixture project has no agent files, so any role lookup must return 404.
#[tokio::test]
async fn agents_get_unknown_role_returns_404() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/agents/nonexistent-role")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ---------------------------------------------------------------------------
// Hook routes
// ---------------------------------------------------------------------------

/// GET /hooks returns 200 with a JSON object containing a "hooks" array.
///
/// The fixture project has no installed plugins, so the hooks array is empty.
/// The response must include the "hooks" key.
#[tokio::test]
async fn hooks_list_returns_200_with_hooks_key() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/hooks")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/hooks must return valid JSON");

    assert!(
        body.get("hooks").is_some(),
        "/hooks response must contain a 'hooks' key, got: {body}"
    );
    assert!(
        body["hooks"].is_array(),
        "'hooks' value must be an array, got: {body}"
    );
}

// ---------------------------------------------------------------------------
// Reload route
// ---------------------------------------------------------------------------

/// POST /reload returns 200 with status "reloaded" and numeric counts.
///
/// The fixture project has 3 known artifacts and 1 rule. After reload the
/// artifact count must be 3 and rule count must be 1.
#[tokio::test]
async fn reload_returns_reloaded_status_and_counts() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/reload")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/reload must return valid JSON");

    assert_eq!(
        body["status"],
        serde_json::json!("reloaded"),
        "reload response must have status: reloaded"
    );
    assert!(
        body["artifacts"].is_number(),
        "reload response must include numeric 'artifacts' count"
    );
    assert!(
        body["rules"].is_number(),
        "reload response must include numeric 'rules' count"
    );
    assert_eq!(
        body["artifacts"].as_u64().unwrap_or(0),
        3,
        "fixture project has 3 artifacts"
    );
    assert_eq!(
        body["rules"].as_u64().unwrap_or(0),
        1,
        "fixture project has 1 rule"
    );
}

// ---------------------------------------------------------------------------
// Graph routes (extended beyond smoke.rs)
// ---------------------------------------------------------------------------

/// GET /graph/health returns 200 with structural health metrics.
///
/// All metric fields must be present and numeric. The fixture has 3 nodes so
/// `total_nodes` must be 3.
#[tokio::test]
async fn graph_health_returns_200_with_metrics() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/graph/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/graph/health must return valid JSON");

    assert!(
        body.get("outlier_count").is_some(),
        "/graph/health must include 'outlier_count'"
    );
    assert!(
        body.get("delivery_connectivity").is_some(),
        "/graph/health must include 'delivery_connectivity'"
    );
    assert_eq!(
        body["total_nodes"].as_u64().unwrap_or(0),
        3,
        "fixture project has exactly 3 nodes"
    );
}

/// GET /graph/stats returns broken_refs field alongside the standard fields.
///
/// The fixture has no broken refs (all 3 artifacts are self-consistent).
#[tokio::test]
async fn graph_stats_broken_refs_is_zero_for_clean_fixture() {
    let (app, _db) = helpers::build_full_router().await;

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

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/graph/stats must return valid JSON");

    assert_eq!(
        body["broken_refs"].as_u64().unwrap_or(u64::MAX),
        0,
        "clean fixture project must have zero broken refs"
    );
}

// ---------------------------------------------------------------------------
// Artifact routes (extended beyond smoke.rs)
// ---------------------------------------------------------------------------

/// GET /artifacts/:id returns 200 with the correct artifact for a known ID.
#[tokio::test]
async fn artifacts_get_known_id_returns_correct_artifact() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/artifacts/EPIC-test001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/artifacts/:id must return valid JSON");

    assert_eq!(
        body["artifact_type"],
        serde_json::json!("epic"),
        "EPIC-test001 must have artifact_type 'epic'"
    );
    assert_eq!(
        body["id"],
        serde_json::json!("EPIC-test001"),
        "response id must match the requested id"
    );
}

/// GET /artifacts/:id returns 404 for an unknown artifact ID.
#[tokio::test]
async fn artifacts_get_unknown_id_returns_404() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/artifacts/EPIC-doesnotexist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

/// GET /artifacts?type=epic returns only epic artifacts.
///
/// The fixture has exactly one epic: EPIC-test001.
#[tokio::test]
async fn artifacts_filter_by_type_returns_matching_subset() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/artifacts?type=epic")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/artifacts?type=epic must return valid JSON");

    let list = body.as_array().expect("response must be an array");
    assert_eq!(list.len(), 1, "fixture has exactly 1 epic");
    assert_eq!(
        list[0]["artifact_type"],
        serde_json::json!("epic"),
        "filtered artifact must be type 'epic'"
    );
}

/// GET /artifacts?type=rule returns only rule artifacts.
///
/// The fixture has exactly one rule: RULE-test001.
#[tokio::test]
async fn artifacts_filter_by_type_rule_returns_one() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/artifacts?type=rule")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/artifacts?type=rule must return valid JSON");

    let list = body.as_array().expect("response must be an array");
    assert_eq!(list.len(), 1, "fixture has exactly 1 rule artifact");
}

/// GET /artifacts?type=nonexistent returns an empty array, not an error.
#[tokio::test]
async fn artifacts_filter_by_unknown_type_returns_empty_array() {
    let (app, _db) = helpers::build_full_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/artifacts?type=nonexistent-type-xyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("response must be valid JSON");

    let list = body.as_array().expect("response must be an array");
    assert!(
        list.is_empty(),
        "filtering by nonexistent type must return empty array"
    );
}

// ---------------------------------------------------------------------------
// Health route (extended beyond smoke.rs)
// ---------------------------------------------------------------------------

/// GET /health returns pid as a positive integer.
#[tokio::test]
async fn health_response_has_positive_pid() {
    let (app, _db) = helpers::build_full_router().await;

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

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("health response must be valid JSON");

    let pid = body["pid"]
        .as_u64()
        .expect("pid must be a non-negative integer in health response");

    assert!(pid > 0, "pid must be a positive integer, got {pid}");
}

/// GET /health returns uptime_seconds as a non-negative integer.
#[tokio::test]
async fn health_response_has_uptime_seconds() {
    let (app, _db) = helpers::build_full_router().await;

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

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    assert!(
        body.get("uptime_seconds").is_some(),
        "health response must include 'uptime_seconds'"
    );
    assert!(
        body["uptime_seconds"].is_number(),
        "'uptime_seconds' must be a number"
    );
}

/// GET /health returns artifact_count matching the fixture's known count.
#[tokio::test]
async fn health_response_artifact_count_matches_fixture() {
    let (app, _db) = helpers::build_full_router().await;

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

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(
        body["artifact_count"].as_u64().unwrap_or(0),
        3,
        "fixture project has exactly 3 artifacts"
    );
}
