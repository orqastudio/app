// App settings routes: read and write key/value settings in SQLite.
//
// Settings are stored in the daemon SQLite database (.state/daemon.db)
// under the `settings` table with a (key, scope) primary key. All
// handlers require HealthState so they can access the DaemonStore.
//
// Endpoints:
//   GET /settings           — list all settings, optionally filtered by scope
//   PUT /settings/:key      — upsert a single setting value

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::health::HealthState;
use crate::store::{settings_get_all, settings_set};

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

/// Response helper when the daemon store is unavailable.
fn store_unavailable() -> (StatusCode, Json<serde_json::Value>) {
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
/// absent, returns an empty map rather than an error.
pub async fn get_settings(
    State(state): State<HealthState>,
    Query(query): Query<GetSettingsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;
    let scope = query.scope.clone();

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;

        settings_get_all(&conn, scope.as_deref())
            .map(|m| Json(serde_json::to_value(m).unwrap_or(serde_json::Value::Object(serde_json::Map::new()))))
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e, "code": "LIST_FAILED" })),
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
    let store = state.daemon_store.clone().ok_or_else(store_unavailable)?;
    let scope = req.scope.unwrap_or_else(|| "app".to_owned());
    let value = req.value.clone();

    tokio::task::spawn_blocking(move || {
        let conn = store.connect().map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "DB_ERROR" })),
        ))?;

        settings_set(&conn, &key, &value, &scope).map_err(|e| (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": e, "code": "SET_FAILED" })),
        ))?;

        Ok(Json(serde_json::json!({ "key": key, "value": value, "scope": scope })))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}
