// Project scanner — re-exported from the orqa-engine crate.
//
// All scanning logic (language, framework, package manager detection, governance
// artifact counting) lives in the engine so it is available to all access layers.

pub use orqa_engine::project::scanner::*;
