// Status transition evaluation — re-exported from orqa-engine.
//
// All business logic has moved to `orqa_engine::workflow::transitions`.
// This file re-exports the public API for use throughout the Tauri app.

pub use orqa_engine::types::workflow::ProposedTransition;
pub use orqa_engine::workflow::transitions::evaluate_transitions;
