// Enforcement engine — re-exported from the orqa-engine crate.
//
// EnforcementEngine provides compiled-regex rule evaluation for file writes,
// bash commands, and project scans. The implementation lives in
// orqa_engine::enforcement::engine and is consumed here without duplication.

pub use orqa_engine::enforcement::engine::EnforcementEngine;
