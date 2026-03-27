// Workflow domain types for the OrqaStudio engine.
//
// Defines structs for status transitions, process compliance violations, session
// process state, and gate evaluation results. These types support the workflow
// engine: the state machine that evaluates artifact status transitions and enforces
// documentation-first process gates during agent sessions.

use serde::{Deserialize, Serialize};

/// A status transition proposed by the evaluation engine.
///
/// Transitions are never applied directly — they are returned to the caller
/// so that the frontend can present them to the user before any mutation
/// occurs. `auto_apply` signals that the transition is unambiguous and may be
/// applied programmatically without human confirmation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedTransition {
    /// Artifact identifier, e.g. `"EPIC-048"`.
    pub artifact_id: String,
    /// Relative path from the project root, e.g. `".orqa/delivery/epics/EPIC-048.md"`.
    pub artifact_path: String,
    /// Current `status` frontmatter value.
    pub current_status: String,
    /// Status value to transition to.
    pub proposed_status: String,
    /// Human-readable explanation of why this transition is proposed.
    pub reason: String,
    /// When `true` the transition is unambiguous and can be applied without
    /// explicit human approval (e.g. a task becoming blocked because a
    /// dependency is not yet complete).
    pub auto_apply: bool,
}

/// A process compliance violation detected during a session.
///
/// Violations are emitted as `StreamEvent::ProcessViolation` after each turn completes,
/// so the frontend can surface them to the user without blocking execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessViolation {
    /// Machine-readable check identifier (e.g. `"docs_before_code"`).
    pub check: String,
    /// Human-readable description of the violation.
    pub message: String,
    /// Severity level: `"warn"` or `"block"`.
    pub severity: String,
}

/// Tracks process compliance state across a single session.
///
/// Resets when a new session begins. Currently enforces documentation-first
/// checks: docs must be read and knowledge must be loaded before code is written.
#[derive(Debug, Default)]
pub struct SessionProcessState {
    /// The session this state belongs to. `None` before any message is sent.
    pub session_id: Option<i64>,
    /// Set when any `read_file` call targets a path inside `docs/` or `.orqa/rules/`.
    pub docs_read: bool,
    /// Set when any `load_knowledge` tool call is made.
    pub knowledge_loaded: bool,
    /// Set when any `write_file` or `edit_file` call targets a `.rs`, `.ts`, or `.svelte` file.
    pub code_written: bool,
}

/// The result of a single process gate evaluation.
///
/// When `fired` is `true`, `message` contains a thinking prompt to inject into
/// the agent's context to guide it back toward correct process.
#[derive(Debug, Clone)]
pub struct GateResult {
    /// Machine-readable gate identifier (e.g. `"understand-first"`).
    pub gate_name: String,
    /// Thinking prompt to inject into the agent context when the gate fires.
    pub message: String,
    /// Whether this gate fired (condition was met and action should be taken).
    pub fired: bool,
}
