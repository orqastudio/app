// Agent routes: load agent preambles and extract behavioral messages.
//
// Agents are defined in the .orqa/process/agents directory. These endpoints
// let the prompt pipeline load agent definitions without reading files directly.
//
// Endpoints:
//   GET /agents/:role                — load agent preamble/definition by role
//   GET /agents/behavioral-messages  — extract behavioral messages from all agents

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

use orqa_validation::content::{extract_behavioral_messages, find_agent, AgentContent};

use crate::graph_state::GraphState;

// ---------------------------------------------------------------------------
// Response shapes
// ---------------------------------------------------------------------------

/// Response body for GET /agents/:role.
#[derive(Debug, Serialize)]
pub struct AgentResponse {
    /// The queried role identifier.
    pub role: String,
    /// The agent's preamble text extracted from the agent definition file.
    pub preamble: String,
    /// Relative path to the agent definition file from the project root.
    pub file_path: String,
    /// Full agent content for prompt construction.
    pub content: AgentContent,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /agents/:role — load the preamble for a named agent role.
///
/// Scans the .orqa/process/agents directory for a file matching the given role
/// (case-insensitive title match) and returns its content. Returns 404 if no
/// matching agent definition is found.
pub async fn get_agent(
    State(state): State<GraphState>,
    Path(role): Path<String>,
) -> Result<Json<AgentResponse>, (StatusCode, Json<serde_json::Value>)> {
    let project_root = {
        let Ok(guard) = state.0.read() else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "state lock poisoned", "code": "LOCK_ERROR" })),
            ));
        };
        guard.project_root.clone()
    };

    let agent = find_agent(&project_root, &role)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "AGENT_SCAN_ERROR" })),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("agent role '{}' not found", role),
                    "code": "NOT_FOUND"
                })),
            )
        })?;

    // The AgentContent itself contains the preamble and file path via frontmatter.
    // We synthesise a file_path from the agent's id for backwards compatibility.
    let preamble = agent.preamble.clone();
    let file_path = format!(".orqa/process/agents/{}.md", agent.id);

    Ok(Json(AgentResponse {
        role,
        preamble,
        file_path,
        content: agent,
    }))
}

/// Handle GET /agents/behavioral-messages — extract behavioral messages from all agents.
///
/// Reads the cached artifact graph and extracts behavioral enforcement messages
/// from all active rules. Returns an empty list if no rules are configured.
pub async fn get_behavioral_messages(
    State(state): State<GraphState>,
) -> Json<orqa_validation::content::BehavioralMessages> {
    let (graph, project_root, rule_type_key) = {
        let Ok(guard) = state.0.read() else {
            return Json(orqa_validation::content::BehavioralMessages {
                messages: Vec::new(),
                rules: Vec::new(),
                rule_count: 0,
                behavioral_count: 0,
            });
        };
        // Find the rule type key from the plugin registry, falling back to "rule".
        // The engine does not hardcode "rule" — callers supply the type key from
        // their plugin registry so any plugin can define the rule type.
        let rule_key = guard
            .artifact_types
            .iter()
            .find(|t| t.pipeline_category.as_deref() == Some("rule"))
            .map_or_else(|| "rule".to_owned(), |t| t.key.clone());
        (guard.graph.clone(), guard.project_root.clone(), rule_key)
    };

    match extract_behavioral_messages(&graph, &project_root, &rule_type_key) {
        Ok(result) => Json(result),
        Err(_) => Json(orqa_validation::content::BehavioralMessages {
            messages: Vec::new(),
            rules: Vec::new(),
            rule_count: 0,
            behavioral_count: 0,
        }),
    }
}
