// Agent type definitions for the orqa-engine crate.
//
// Defines the base role taxonomy, per-agent specifications, and the assembled
// TaskAgent record that combines a spec with its generated system prompt.
// These types are the foundation for the prompt generation pipeline described
// in the architecture's `agent` crate domain (core.md §3.2).

use serde::{Deserialize, Serialize};

/// The canonical set of base roles in the hub-spoke orchestration model.
///
/// Each role maps to a distinct responsibility in the governance workflow.
/// The engine defines these roles; plugins provide domain-specific specialisations
/// on top of them. No governance pattern is baked in — only structural roles.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BaseRole {
    /// Coordinates ephemeral task-scoped workers. Reads structured summaries
    /// from findings files. Does not implement or self-assess quality.
    Orchestrator,
    /// Writes, edits, and tests code. Delegates review to a Reviewer.
    Implementer,
    /// Independently verifies acceptance criteria and produces PASS/FAIL verdicts.
    /// Does not implement; only verifies.
    Reviewer,
    /// Gathers information and writes research artifacts. Does not modify source code.
    Researcher,
    /// Creates and edits documentation. Does not modify source code.
    Writer,
    /// Designs approaches and maps dependencies. Does not implement.
    Planner,
    /// Designs UI/UX structure and component layouts.
    Designer,
    /// Maintains `.orqa/` governance artifacts. Does not modify source code.
    GovernanceSteward,
}

/// The specification that drives prompt generation for a single agent task.
///
/// Contains the role, the tools the agent is permitted to access, any
/// knowledge references the prompt pipeline should inject, and the
/// task description scoped to a single context window (P2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    /// The base role this agent will fulfil.
    pub role: BaseRole,
    /// Tool names the agent is permitted to use during its task.
    ///
    /// An empty list means no tools beyond text generation. The prompt
    /// pipeline uses this list to build the tool section of the system prompt.
    pub tool_access: Vec<String>,
    /// Knowledge artifact references to inject into the system prompt.
    ///
    /// Each string is a knowledge key (e.g. `"architecture/core"`). The prompt
    /// pipeline resolves these via the plugin registry and injects only the
    /// content relevant to the agent's task (P5: token efficiency).
    pub knowledge_refs: Vec<String>,
    /// The task description scoped to this agent's context window.
    ///
    /// This is the primary driver of the generated system prompt. It must
    /// describe exactly one task — no persistent agents, no accumulated context (P2).
    pub task_description: String,
}

/// A fully-assembled agent ready for execution.
///
/// Combines the `AgentSpec` with the generated system prompt produced by
/// the prompt pipeline. The generated prompt is deterministic for a given
/// spec and plugin registry state (P3: generated, not loaded).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAgent {
    /// The specification that was used to generate this agent's prompt.
    pub spec: AgentSpec,
    /// The generated system prompt ready to be sent to the LLM.
    ///
    /// Built by the prompt pipeline from the spec's role, tool_access,
    /// knowledge_refs, and task_description.
    pub generated_prompt: String,
}
