// Enforcement violation domain type — re-exported from the orqa-engine crate.
//
// EnforcementViolation represents a recorded rule violation from the
// enforcement_violations table. Populated when the enforcement engine blocks
// or warns on a tool call; surfaced in the governance UI.

pub use orqa_engine_types::types::enforcement::EnforcementViolation;
