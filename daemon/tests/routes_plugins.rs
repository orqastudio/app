// Integration tests for plugin route handlers: POST /plugins/install/local.
//
// Tests dispatch requests via `tower::ServiceExt::oneshot` against a real
// axum Router. The install route runs engine validation (including manifest
// deserialisation) so invalid manifests are caught at the HTTP boundary.

#![allow(missing_docs)]

mod helpers;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

// ---------------------------------------------------------------------------
// POST /plugins/install/local — malformed target rejection
// ---------------------------------------------------------------------------

/// POST /plugins/install/local with a manifest containing an invalid `target`
/// value returns 422 UNPROCESSABLE_ENTITY with a JSON error body that names
/// the field and the valid values.
///
/// The fixture at `tests/fixtures/malformed-target-plugin/` declares
/// `"target": "wrong-enum"` on a content entry. The engine's manifest
/// deserialiser rejects this before any filesystem writes occur.
#[tokio::test]
async fn install_local_malformed_target_returns_422_with_error() {
    let (app, _db) = helpers::build_full_router().await;

    let fixture_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/malformed-target-plugin");

    let body = serde_json::json!({ "path": fixture_path.to_str().unwrap() });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/plugins/install/local")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "malformed target must return 422"
    );

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&bytes).expect("error body must be valid JSON");

    let error_msg = json
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    assert!(
        error_msg.contains("wrong-enum")
            || error_msg.contains("surrealdb")
            || error_msg.contains("runtime"),
        "error message must reference the invalid value or valid values, got: {error_msg}"
    );
}
