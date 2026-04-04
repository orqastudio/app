// App settings routes: read and write key/value settings in SQLite.
//
// Settings are stored in the unified SQLite storage (.state/orqa.db) under the
// `settings` table with a (key, scope) primary key. All handlers require
// HealthState so they can access the unified Storage.
//
// Endpoints:
//   GET /settings           — list all settings, optionally filtered by scope
//   PUT /settings/:key      — upsert a single setting value

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Query parameters for GET /settings.
#[derive(Debug, Deserialize)]
pub struct GetSettingsQuery {
    /// Optional scope filter (e.g. "app", "project", "user").
    pub scope: Option<String>,
}

/// Request body for PUT /settings/:key.
#[derive(Debug, Deserialize)]
pub struct SetSettingRequest {
    /// The new value for the setting.
    pub value: serde_json::Value,
    /// Scope for this setting (defaults to "app").
    pub scope: Option<String>,
}

/// Response helper when the storage is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "settings store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /settings — return all settings optionally filtered by scope.
///
/// Returns a flat key/value map where values are JSON. When the store is
/// absent, returns 503 rather than an empty map.
pub async fn get_settings(
    State(state): State<HealthState>,
    Query(query): Query<GetSettingsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let scope = query.scope.clone();

    tokio::task::spawn_blocking(move || {
        let result = if let Some(ref s) = scope {
            storage.settings().get_all(s)
        } else {
            storage.settings().get_all_any_scope()
        };

        result
            .map(|m| Json(serde_json::to_value(m).unwrap_or(serde_json::Value::Object(serde_json::Map::new()))))
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle PUT /settings/:key — upsert a setting value by key.
///
/// The scope defaults to "app" when not provided. Returns the stored
/// key and value on success.
pub async fn set_setting(
    State(state): State<HealthState>,
    Path(key): Path<String>,
    Json(req): Json<SetSettingRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;
    let scope = req.scope.unwrap_or_else(|| "app".to_owned());
    let value = req.value.clone();

    tokio::task::spawn_blocking(move || {
        storage.settings().set(&key, &value, &scope).map_err(|e| (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": e.to_string(), "code": "SET_FAILED" })),
        ))?;

        Ok(Json(serde_json::json!({ "key": key, "value": value, "scope": scope })))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

// ---------------------------------------------------------------------------
// Route tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use axum::routing::{get, put};
    use axum::Router;
    use http_body_util::BodyExt as _;
    use std::sync::Arc;
    use tower::ServiceExt as _;

    use crate::graph_state::GraphState;
    use orqa_storage::Storage;

    /// Build a fresh router with the settings routes wired to an in-memory store.
    fn settings_router() -> Router {
        let storage = Storage::open_in_memory()
            .map(Arc::new)
            .expect("in-memory storage");

        let state = HealthState::for_test(
            GraphState::build_empty(std::path::Path::new("/tmp/test")),
            Some(Arc::clone(&storage)),
        );

        let settings_sub = Router::new()
            .route("/", get(get_settings))
            .route("/{key}", put(set_setting))
            .with_state(state);

        Router::new().nest("/settings", settings_sub)
    }

    fn json_body(val: serde_json::Value) -> Body {
        Body::from(serde_json::to_vec(&val).unwrap())
    }

    async fn body_json(body: Body) -> serde_json::Value {
        let bytes = body.collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).expect("response must be valid JSON")
    }

    // ---- GET /settings --------------------------------------------------------

    #[tokio::test]
    async fn get_settings_returns_empty_object_initially() {
        // With no settings stored, GET /settings must return an empty JSON object.
        let app = settings_router();
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        let obj = body.as_object().expect("settings must be a JSON object");
        assert!(obj.is_empty(), "initially no settings must be present");
    }

    // ---- PUT /settings/:key ---------------------------------------------------

    #[tokio::test]
    async fn put_settings_sets_value() {
        // PUT /settings/theme_mode must persist the value.
        let app = settings_router();
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/settings/theme_mode")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "value": "dark" })))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        assert_eq!(body["key"], "theme_mode");
        assert_eq!(body["value"], "dark");
    }

    #[tokio::test]
    async fn get_settings_returns_previously_set_value() {
        // After PUT, GET /settings must include the key we set.
        let app = settings_router();

        app.clone()
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/settings/theme_mode")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "value": "dark" })))
                    .unwrap(),
            )
            .await
            .unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        assert_eq!(body["theme_mode"], "dark");
    }

    #[tokio::test]
    async fn settings_json_round_trip_complex_value() {
        // A complex JSON object must survive a PUT then GET cycle unchanged.
        let app = settings_router();
        let complex = serde_json::json!({
            "enabled": true,
            "count": 42,
            "tags": ["a", "b"],
            "nested": { "x": 1 }
        });

        app.clone()
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/settings/complex_cfg")
                    .header("content-type", "application/json")
                    .body(json_body(serde_json::json!({ "value": complex })))
                    .unwrap(),
            )
            .await
            .unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = body_json(resp.into_body()).await;
        assert_eq!(body["complex_cfg"], complex);
    }
}
