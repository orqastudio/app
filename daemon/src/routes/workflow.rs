// Workflow routes: status transition evaluation and application.
//
// Evaluation uses the cached artifact graph and status definitions loaded from
// project.json. Application writes the new status to disk and reloads the graph.
//
// Endpoints:
//   GET  /workflow/transitions        — evaluate all proposed transitions
//   POST /workflow/transitions/apply  — apply a single transition

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use orqa_engine_types::paths::SETTINGS_FILE;
use orqa_engine_types::types::project_settings::StatusDefinition;
use orqa_engine_types::types::workflow::ProposedTransition;
use orqa_validation::auto_fix::update_artifact_field;
use orqa_workflow::transitions::evaluate_transitions;

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Request / response shapes
// ---------------------------------------------------------------------------

/// Request body for POST /workflow/transitions/apply.
#[derive(Debug, Deserialize)]
pub struct ApplyTransitionRequest {
    /// ID of the artifact to transition.
    pub artifact_id: String,
    /// The status value to apply.
    pub proposed_status: String,
}

/// Response body for POST /workflow/transitions/apply.
#[derive(Debug, Serialize)]
pub struct ApplyTransitionResponse {
    /// The artifact that was transitioned.
    pub artifact_id: String,
    /// Status before the transition.
    pub old_status: String,
    /// Status after the transition.
    pub new_status: String,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Load status definitions from project.json, deserializing the statuses array
/// using the `StatusDefinition` type that includes `auto_rules`.
///
/// Returns an empty list if the settings file is absent or unparseable.
fn load_status_definitions(project_root: &std::path::Path) -> Vec<StatusDefinition> {
    #[derive(serde::Deserialize)]
    struct StatusesOnly {
        #[serde(default)]
        statuses: Vec<StatusDefinition>,
    }

    let settings_path = project_root.join(SETTINGS_FILE);
    let Ok(content) = std::fs::read_to_string(&settings_path) else {
        return Vec::new();
    };
    serde_json::from_str::<StatusesOnly>(&content)
        .map(|s| s.statuses)
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /workflow/transitions — evaluate all proposed status transitions.
///
/// Reads the cached graph and project status definitions, then returns all
/// transitions the workflow engine proposes. No mutations are performed.
pub async fn list_transitions(
    State(state): State<GraphState>,
) -> Json<Vec<ProposedTransition>> {
    let (graph, project_root) = {
        let Ok(guard) = state.0.read() else {
            return Json(Vec::new());
        };
        (guard.graph.clone(), guard.project_root.clone())
    };

    let statuses = load_status_definitions(&project_root);
    let transitions = evaluate_transitions(&graph, &statuses);
    Json(transitions)
}

/// Handle POST /workflow/transitions/apply — apply a single status transition.
///
/// Locates the artifact in the cached graph, writes the new status to disk via
/// `update_artifact_field`, and triggers a graph reload. Returns the old and
/// new status values.
pub async fn apply_transition(
    State(state): State<GraphState>,
    Json(req): Json<ApplyTransitionRequest>,
) -> Result<Json<ApplyTransitionResponse>, (StatusCode, Json<serde_json::Value>)> {
    let (file_path, old_status, project_root) = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        let node = guard.graph.nodes.get(&req.artifact_id)
            .or_else(|| guard.graph.nodes.values().find(|n| n.id == req.artifact_id))
            .ok_or_else(|| (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("artifact '{}' not found", req.artifact_id),
                    "code": "NOT_FOUND"
                })),
            ))?;
        let old_status = node.status.clone().unwrap_or_default();
        let file_path = guard.project_root.join(&node.path);
        (file_path, old_status, guard.project_root.clone())
    };

    update_artifact_field(&file_path, "status", &req.proposed_status).map_err(|e| (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(serde_json::json!({ "error": e.to_string(), "code": "UPDATE_FAILED" })),
    ))?;

    state.reload(&project_root);

    Ok(Json(ApplyTransitionResponse {
        artifact_id: req.artifact_id,
        old_status,
        new_status: req.proposed_status,
    }))
}
