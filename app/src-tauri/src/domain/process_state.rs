// Process state domain types — re-exported from orqa-engine.
//
// All business logic has moved to `orqa_engine::workflow::state`.
// This file re-exports the public API for use throughout the Tauri app.

pub use orqa_engine::types::workflow::{ProcessViolation, SessionProcessState};
pub use orqa_engine::workflow::state::ProcessStateExt;
