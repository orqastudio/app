// Theme routes: read project themes and manage user overrides.
//
// Themes are design token maps extracted from source files (e.g., tailwind.config.ts).
// Overrides allow users to customise specific token values per project without
// modifying the source file. All theme data lives in the unified SQLite storage.
//
// Endpoints:
//   GET    /themes                         — list all active themes for the active project
//   GET    /themes/overrides               — list all token overrides for the active project
//   PUT    /themes/overrides/:token        — set (upsert) a single token override
//   DELETE /themes/overrides               — clear all overrides for the active project

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use orqa_storage::traits::ThemeRepository as _;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Query parameters for theme endpoints that require a project ID.
#[derive(Debug, Deserialize)]
pub struct ProjectIdQuery {
    pub project_id: i64,
}

/// Request body for PUT /themes/overrides/:token.
#[derive(Debug, Deserialize)]
pub struct SetOverrideRequest {
    pub project_id: i64,
    /// Override value for light mode.
    pub value_light: String,
    /// Override value for dark mode (optional — falls back to light value).
    pub value_dark: Option<String>,
}

/// Response helper when the storage layer is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "theme store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /themes — list all active themes for a project.
///
/// Returns the raw theme rows including token maps for light and dark mode.
pub async fn get_themes(
    State(state): State<HealthState>,
    Query(query): Query<ProjectIdQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .themes()
        .get_themes(query.project_id)
        .await
        .map(|rows| {
            let items: Vec<serde_json::Value> = rows
                .into_iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id,
                        "project_id": r.project_id,
                        "source_file": r.source_file,
                        "source_hash": r.source_hash,
                        "extracted_at": r.extracted_at,
                        "tokens_light": r.tokens_light,
                        "tokens_dark": r.tokens_dark,
                        "unmapped": r.unmapped,
                        "is_active": r.is_active,
                    })
                })
                .collect();
            Json(serde_json::json!({ "themes": items }))
        })
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
            )
        })
}

/// Handle GET /themes/overrides — list all token overrides for a project.
pub async fn get_overrides(
    State(state): State<HealthState>,
    Query(query): Query<ProjectIdQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .themes()
        .get_overrides(query.project_id)
        .await
        .map(|rows| {
            let items: Vec<serde_json::Value> = rows
                .into_iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id,
                        "project_id": r.project_id,
                        "token_name": r.token_name,
                        "value_light": r.value_light,
                        "value_dark": r.value_dark,
                    })
                })
                .collect();
            Json(serde_json::json!({ "overrides": items }))
        })
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "LIST_FAILED" })),
            )
        })
}

/// Handle PUT /themes/overrides/:token — upsert a token override for a project.
pub async fn set_override(
    State(state): State<HealthState>,
    Path(token_name): Path<String>,
    Json(req): Json<SetOverrideRequest>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .themes()
        .set_override(
            req.project_id,
            &token_name,
            &req.value_light,
            req.value_dark.as_deref(),
        )
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "SET_FAILED" })),
            )
        })
}

/// Handle DELETE /themes/overrides — clear all token overrides for a project.
pub async fn clear_overrides(
    State(state): State<HealthState>,
    Query(query): Query<ProjectIdQuery>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    storage
        .themes()
        .clear_overrides(query.project_id)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "CLEAR_FAILED" })),
            )
        })
}
