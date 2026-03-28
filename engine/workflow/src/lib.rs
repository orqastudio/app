//! orqa-workflow: Workflow evaluation engine for the OrqaStudio platform.
//!
//! Provides the workflow evaluation engine: status transition logic, session process
//! state tracking, session activity tracking, and process gate evaluation. These
//! modules implement the state machine that evaluates artifact status transitions
//! and enforces process gates during agent sessions.
//!
//! Submodules:
//!   - `transitions` — `evaluate_transitions()` with named condition evaluators
//!   - `state`       — `ProcessStateExt` trait for session process state tracking
//!   - `tracker`     — `WorkflowTracker` for session-level event accumulation
//!   - `gates`       — process gate evaluation: 5 gates that inject thinking prompts

/// Process gate evaluation: fires thinking prompts for process compliance.
pub mod gates;
/// Process state extension trait for session-level tracking.
pub mod state;
/// Workflow tracker: accumulates session-level events for gate evaluation.
pub mod tracker;
/// Status transition logic: evaluates artifact state machine transitions.
pub mod transitions;
