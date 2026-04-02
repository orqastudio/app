// Status transition types — re-exported from orqa-engine-types.
//
// ProposedTransition is defined in orqa_engine_types::types::workflow.
// The evaluation logic runs in the daemon via GET /workflow/transitions.

pub use orqa_engine_types::types::workflow::ProposedTransition;
