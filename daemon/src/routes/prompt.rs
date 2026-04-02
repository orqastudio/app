// Prompt routes: system prompt generation, knowledge injection, context, compact context.
//
// These handlers are thin wrappers around the existing top-level module
// implementations, registered here so the router nests them under /prompt/*.
// All handlers use HealthState (registered on the main router).
//
// Endpoints:
//   POST /prompt/generate        — classify message and generate system prompt
//   POST /prompt/knowledge       — knowledge injection (declared + semantic)
//   POST /prompt/context         — active rules and workflows for CLAUDE.md generation
//   POST /prompt/compact-context — governance context for pre-compaction preservation

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::health::HealthState;

/// Handle POST /prompt/generate — classify and generate a system prompt.
///
/// Delegates to `crate::prompt::prompt_handler`.
pub async fn generate_prompt(
    req: Json<crate::prompt::PromptRequest>,
) -> Json<crate::prompt::PromptResponse> {
    crate::prompt::prompt_handler(req)
}

/// Handle POST /prompt/knowledge — inject knowledge from declared and semantic sources.
///
/// Delegates to `crate::knowledge::knowledge_handler` which requires HealthState
/// for the event bus and graph context.
pub async fn prompt_knowledge(
    state: State<HealthState>,
    req: Json<crate::knowledge::KnowledgeRequest>,
) -> Json<crate::knowledge::KnowledgeResponse> {
    crate::knowledge::knowledge_handler(state, req)
}

/// Handle POST /prompt/context — return active rules and workflows for CLAUDE.md.
///
/// Delegates to `crate::context::context_handler`.
pub async fn prompt_context(
    req: Json<crate::context::ContextRequest>,
) -> Json<crate::context::ContextResponse> {
    crate::context::context_handler(req)
}

/// Handle POST /prompt/compact-context — return governance context for pre-compaction.
///
/// Delegates to `crate::compact_context::compact_context_handler`.
pub async fn prompt_compact_context(
    req: Json<crate::compact_context::CompactContextRequest>,
) -> Result<Json<crate::compact_context::CompactContextResponse>, (StatusCode, String)> {
    crate::compact_context::compact_context_handler(req)
}
