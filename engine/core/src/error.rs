// Error module for the orqa-engine crate.
//
// Re-exports the engine error type from orqa-engine-types so that all engine
// modules and consumers can use crate::error::EngineError unchanged.

pub use orqa_engine_types::error::*;
