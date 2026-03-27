// Navigation tree scanner — re-exported from orqa-engine.
//
// All logic (artifact_scan_tree and supporting helpers) lives in the engine crate at
// `orqa_engine::artifact::reader`. This module re-exports so the rest of the app can
// import from `crate::domain::artifact_reader`.

pub use orqa_engine::artifact::reader::*;
