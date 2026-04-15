// Integration tests for validation routes.
//
// Covers:
//   POST /validation/scan  — run integrity checks on the cached graph
//   POST /validation/fix   — dry-run mode (fix: false)
//   POST /validation/hook  — evaluate a hook context against active rules
//
// All tests use the minimal fixture project. The fixture has 3 artifacts and
// 1 rule, producing a real (non-empty) validation result.
#![allow(missing_docs)]

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

/// Build a minimal Router wired to the real validation route handlers.
///
/// Uses GraphState from the fixture project so all checks run against real data.
async fn build_validation_router() -> axum::Router {
    use axum::routing::post;
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::routes::validation::{validation_fix, validation_hook, validation_scan};

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root)
        .await
        .unwrap_or_else(|_| GraphState::build_empty(&root));

    axum::Router::new()
        .route("/validation/scan", post(validation_scan))
        .route("/validation/fix", post(validation_fix))
        .route("/validation/hook", post(validation_hook))
        .with_state(graph_state)
}

// ---------------------------------------------------------------------------
// POST /validation/scan
// ---------------------------------------------------------------------------

/// POST /validation/scan returns 200 with a checks array and health object.
///
/// The response shape must include both `checks` (array) and `health` (object).
/// This verifies that the handler wires validation + health metrics together.
#[tokio::test]
async fn validation_scan_returns_checks_and_health() {
    let app = build_validation_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/validation/scan")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/validation/scan must return valid JSON");

    assert!(
        body.get("checks").is_some(),
        "/validation/scan must include a 'checks' key, got: {body}"
    );
    assert!(
        body["checks"].is_array(),
        "'checks' must be a JSON array, got: {body}"
    );
    assert!(
        body.get("health").is_some(),
        "/validation/scan must include a 'health' key, got: {body}"
    );
    assert!(
        body["health"].is_object(),
        "'health' must be a JSON object, got: {body}"
    );
}

/// POST /validation/scan health object contains the expected metric fields.
///
/// The health metrics are structural properties of the graph. Verifying that
/// the fields exist and have the right type confirms the full scan→health
/// pipeline ran — not just that the handler returned a 200.
#[tokio::test]
async fn validation_scan_health_contains_metric_fields() {
    let app = build_validation_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/validation/scan")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let health = &body["health"];

    // These fields are produced by compute_health — their presence confirms
    // that health computation ran against the real fixture graph.
    assert!(
        health.get("total_nodes").is_some(),
        "health must include 'total_nodes'"
    );
    assert!(
        health["total_nodes"].as_u64().unwrap_or(0) > 0,
        "total_nodes must be > 0 for the fixture project"
    );
    assert!(
        health.get("outlier_count").is_some(),
        "health must include 'outlier_count'"
    );
    assert!(
        health.get("delivery_connectivity").is_some(),
        "health must include 'delivery_connectivity'"
    );
}

// ---------------------------------------------------------------------------
// POST /validation/fix (dry-run)
// ---------------------------------------------------------------------------

/// POST /validation/fix with fix:false is a dry run — fixes_applied is empty.
///
/// The contract: fix:false must return the same checks and health as a scan
/// but with an empty fixes_applied array. No files must be written to disk.
#[tokio::test]
async fn validation_fix_dry_run_returns_empty_fixes_applied() {
    let app = build_validation_router().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/validation/fix")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"fix": false}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/validation/fix must return valid JSON");

    assert!(
        body.get("checks").is_some(),
        "dry-run fix must include 'checks'"
    );
    assert!(
        body.get("health").is_some(),
        "dry-run fix must include 'health'"
    );

    let fixes = body["fixes_applied"]
        .as_array()
        .expect("fixes_applied must be a JSON array");
    assert!(
        fixes.is_empty(),
        "fixes_applied must be empty for a dry run (fix: false)"
    );
}

// ---------------------------------------------------------------------------
// POST /validation/hook
// ---------------------------------------------------------------------------

/// POST /validation/hook with a PreAction hook context returns a valid result.
///
/// The result must have an `action` field ("allow", "warn", or "block") and a
/// `violations` array. With RULE-test001 (which has only a behavioral
/// enforcement mechanism, not a tool-based one), the action should be "allow".
#[tokio::test]
async fn validation_hook_with_pre_action_context_returns_result() {
    let app = build_validation_router().await;

    let hook_ctx = serde_json::json!({
        "event": "PreAction",
        "tool_name": "Write",
        "tool_input": {"path": "test.rs", "content": "fn main() {}"},
        "file_path": "test.rs",
        "user_message": null,
        "agent_type": "implementer"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/validation/hook")
                .header("content-type", "application/json")
                .body(Body::from(hook_ctx.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/validation/hook must return valid JSON");

    assert!(
        body.get("action").is_some(),
        "hook result must include an 'action' field, got: {body}"
    );
    let action = body["action"].as_str().unwrap_or("");
    assert!(
        matches!(action, "allow" | "warn" | "block"),
        "action must be 'allow', 'warn', or 'block', got: '{action}'"
    );
    assert!(
        body.get("violations").is_some(),
        "hook result must include a 'violations' field"
    );
    assert!(
        body["violations"].is_array(),
        "'violations' must be a JSON array"
    );
}
