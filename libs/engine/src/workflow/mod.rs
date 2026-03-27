// Workflow module for the orqa-engine crate.
//
// Provides the workflow evaluation engine: status transition logic, session process
// state tracking, and session activity tracking. These modules implement the state
// machine that evaluates artifact status transitions and enforces process gates
// during agent sessions.
//
// Submodules:
//   - `transitions` — `evaluate_transitions()` with named condition evaluators
//   - `state`       — `ProcessStateExt` trait for session process state tracking
//   - `tracker`     — `WorkflowTracker` for session-level event accumulation

pub mod state;
pub mod tracker;
pub mod transitions;
