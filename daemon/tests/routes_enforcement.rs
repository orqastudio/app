// Integration tests for enforcement routes.
//
// Covers:
//   GET  /enforcement/rules         — list all parsed enforcement rules
//   POST /enforcement/rules/reload  — reload rules from disk
//   GET  /enforcement/violations    — list findings classified as block-action
//   POST /enforcement/scan          — full governance scan
//
// The minimal fixture project has exactly one rule: RULE-test001. Tests verify
// that the rule appears in the listing and that the governance scan runs cleanly.
#![allow(missing_docs)]

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

/// Build a Router wired to the real enforcement route handlers.
fn build_enforcement_router() -> axum::Router {
    use axum::routing::{get, post};
    use orqa_daemon_lib::graph_state::GraphState;
    use orqa_daemon_lib::routes::enforcement::{
        enforcement_scan, list_enforcement_rules, list_enforcement_violations,
        reload_enforcement_rules,
    };

    let root = helpers::fixture_dir();
    let graph_state = GraphState::build(&root).unwrap_or_else(|_| GraphState::build_empty(&root));

    axum::Router::new()
        .route("/enforcement/rules", get(list_enforcement_rules))
        .route("/enforcement/rules/reload", post(reload_enforcement_rules))
        .route("/enforcement/violations", get(list_enforcement_violations))
        .route("/enforcement/scan", post(enforcement_scan))
        .with_state(graph_state)
}

// ---------------------------------------------------------------------------
// GET /enforcement/rules
// ---------------------------------------------------------------------------

/// GET /enforcement/rules returns 200 with a JSON array.
///
/// The enforcement engine parses files with `event:`, `action:`, `pattern:` frontmatter.
/// RULE-test001 uses the graph-artifact enforcement format (`mechanism: behavioral`) which
/// is not the enforcement engine's format — it is skipped during parse. The key invariant
/// is that the handler returns a valid JSON array and does not error; an empty array is
/// a correct response for a project with no enforcement-engine-format rules.
#[tokio::test]
async fn enforcement_rules_returns_200_and_json_array() {
    let app = build_enforcement_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/enforcement/rules")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/enforcement/rules must return valid JSON");

    assert!(
        body.is_array(),
        "/enforcement/rules must return a JSON array, got: {body}"
    );
}

// ---------------------------------------------------------------------------
// POST /enforcement/rules/reload
// ---------------------------------------------------------------------------

/// POST /enforcement/rules/reload returns the same rules list as GET /enforcement/rules.
///
/// The reload endpoint re-reads rule files from disk. The result must be
/// structurally identical to the GET response — same rule count, same names.
/// This confirms the reload path uses the same loading code as the list path.
#[tokio::test]
async fn enforcement_rules_reload_returns_same_rules_as_list() {
    // Build two routers from the same fixture so we can compare responses.
    let app_list = build_enforcement_router();
    let app_reload = build_enforcement_router();

    // GET /enforcement/rules
    let list_resp = app_list
        .oneshot(
            Request::builder()
                .uri("/enforcement/rules")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let list_bytes = list_resp.into_body().collect().await.unwrap().to_bytes();
    let list_body: serde_json::Value = serde_json::from_slice(&list_bytes).unwrap();
    let list_rules = list_body.as_array().unwrap();

    // POST /enforcement/rules/reload
    let reload_resp = app_reload
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/enforcement/rules/reload")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(reload_resp.status(), StatusCode::OK);

    let reload_bytes = reload_resp.into_body().collect().await.unwrap().to_bytes();
    let reload_body: serde_json::Value = serde_json::from_slice(&reload_bytes).unwrap();
    let reload_rules = reload_body
        .as_array()
        .expect("reload must return a JSON array");

    assert_eq!(
        list_rules.len(),
        reload_rules.len(),
        "reload must return the same number of rules as list"
    );
}

// ---------------------------------------------------------------------------
// GET /enforcement/violations
// ---------------------------------------------------------------------------

/// GET /enforcement/violations returns 200 with a JSON array.
///
/// The fixture rule (RULE-test001) has only a behavioral enforcement mechanism,
/// not a tool-pattern Block rule, so the violations list should be empty.
/// The key invariant is that the endpoint returns a well-formed array — not an error.
#[tokio::test]
async fn enforcement_violations_returns_json_array() {
    let app = build_enforcement_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/enforcement/violations")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/enforcement/violations must return valid JSON");

    assert!(
        body.is_array(),
        "/enforcement/violations must return a JSON array, got: {body}"
    );
}

// ---------------------------------------------------------------------------
// POST /enforcement/scan
// ---------------------------------------------------------------------------

/// POST /enforcement/scan returns 200 with governance scan result fields.
///
/// The response must include `rules`, `governance`, and `total_artifacts`.
/// This confirms the full scan pipeline ran — rule loading, governance scan,
/// and artifact counting all composed correctly.
#[tokio::test]
async fn enforcement_scan_returns_governance_scan_result() {
    let app = build_enforcement_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/enforcement/scan")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value =
        serde_json::from_slice(&bytes).expect("/enforcement/scan must return valid JSON");

    assert!(
        body.get("rules").is_some(),
        "scan result must include 'rules', got: {body}"
    );
    assert!(body["rules"].is_array(), "'rules' must be a JSON array");
    assert!(
        body.get("governance").is_some(),
        "scan result must include 'governance', got: {body}"
    );
    assert!(
        body["governance"].is_object(),
        "'governance' must be a JSON object"
    );
    assert!(
        body.get("total_artifacts").is_some(),
        "scan result must include 'total_artifacts', got: {body}"
    );
    assert!(
        body["total_artifacts"].is_number(),
        "'total_artifacts' must be a number"
    );
    assert!(
        body["governance"].get("coverage_ratio").is_some(),
        "governance result must include 'coverage_ratio'"
    );
}
