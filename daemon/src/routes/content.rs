// Content routes: knowledge artifact loading for the prompt pipeline.
//
// Knowledge artifacts are defined in .orqa/documentation/knowledge/ and
// serve as pre-loaded domain context injected into agent prompts.
//
// Endpoints:
//   GET /content/knowledge/:key — load a specific knowledge artifact by key

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

use orqa_validation::content::find_knowledge;

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Response shapes
// ---------------------------------------------------------------------------

/// Response body for GET /content/knowledge/:key.
#[derive(Debug, Serialize)]
pub struct KnowledgeResponse {
    /// The key used to locate this knowledge artifact.
    pub key: String,
    /// Full markdown content of the knowledge artifact.
    pub content: String,
    /// Relative path to the knowledge artifact file.
    pub file_path: String,
    /// Artifact ID.
    pub id: String,
    /// Display title of the knowledge artifact.
    pub title: String,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /content/knowledge/:key — load a knowledge artifact by directory key.
///
/// Searches all knowledge directories under the project root for a file matching
/// the given key. Tries both `<dir>/<key>/KNOW.md` and `<dir>/<key>.md` forms.
/// Returns 404 if no matching knowledge artifact is found.
pub async fn get_knowledge(
    State(state): State<GraphState>,
    Path(key): Path<String>,
) -> Result<Json<KnowledgeResponse>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let knowledge = find_knowledge(&project_root, &key)
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string(), "code": "KNOWLEDGE_SCAN_ERROR" })),
        ))?
        .ok_or_else(|| (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("knowledge key '{}' not found", key),
                "code": "NOT_FOUND"
            })),
        ))?;

    // Synthesise a relative file_path from the artifact id for the response.
    let file_path = format!(".orqa/documentation/knowledge/{}.md", knowledge.id);

    Ok(Json(KnowledgeResponse {
        key,
        content: knowledge.content,
        file_path,
        id: knowledge.id,
        title: knowledge.title,
    }))
}
