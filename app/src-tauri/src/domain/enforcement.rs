// Enforcement domain types — re-exported from the orqa-engine crate.
//
// EventType, RuleAction, Condition, EnforcementEntry, ScanFinding, EnforcementRule,
// Verdict, and EnforcementViolation define the enforcement rule system: event types,
// rule actions, conditions, entries, scan findings, parsed rules, verdicts, and
// violation records. These types flow between the enforcement engine, MCP hooks,
// and the governance UI.

pub use orqa_engine::types::enforcement::*;
