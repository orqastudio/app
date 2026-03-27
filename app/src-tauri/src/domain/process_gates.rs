// Process gate evaluation — delegates to the orqa-engine crate.
//
// Re-exports all process gate functions and the `GateResult` type from
// `orqa_engine::workflow::gates`. This keeps the app's callers (stream_loop)
// unchanged while the implementation lives in the engine crate.

pub use orqa_engine::types::workflow::GateResult;
pub use orqa_engine::workflow::gates::{
    evaluate_process_gates, evaluate_stop_verdicts, evaluate_write_verdicts, fired_gates,
};
