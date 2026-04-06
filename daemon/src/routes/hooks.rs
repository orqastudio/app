// Hook management routes: list registered hooks and generate dispatcher files.
//
// Hooks are registered by plugins in their manifests. The hook registry
// is built from the project's installed plugins. Hook dispatchers are
// generated into the project's .claude/hooks/ directory.
//
// Endpoints:
//   GET  /hooks          — list all registered hooks from installed plugins
//   POST /hooks/generate — generate hook dispatcher files into .claude/hooks/

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use orqa_plugin::hooks::{generate_dispatchers, read_hook_registry};

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /hooks — list all hooks registered by installed plugins.
///
/// Reads the hook registry from plugin manifests for the current project.
/// Returns an empty list if no plugins provide hooks.
pub async fn list_hooks(State(state): State<GraphState>) -> Json<serde_json::Value> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Json(serde_json::json!({ "hooks": [] }));
        };
        guard.project_root.clone()
    };

    let result = tokio::task::spawn_blocking(move || read_hook_registry(&project_root))
        .await
        .unwrap_or_default();

    Json(serde_json::json!({ "hooks": result }))
}

/// Handle POST /hooks/generate — generate hook dispatcher shell scripts.
///
/// Writes dispatcher scripts to `.claude/hooks/` in the project root based on
/// all hooks registered by installed plugins. Returns the list of generated
/// files and any errors encountered.
pub async fn generate_hook_dispatchers(
    State(state): State<GraphState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    tokio::task::spawn_blocking(move || {
        generate_dispatchers(&project_root)
            .map(|r| Json(serde_json::to_value(&r).unwrap_or(serde_json::Value::Null)))
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": e, "code": "GENERATE_FAILED" })),
                )
            })
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
        )
    })?
}
