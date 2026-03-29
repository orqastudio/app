//! Enforcement pipeline for the OrqaStudio platform.
//!
//! Provides the full enforcement pipeline:
//!   - `parser`  — parses YAML frontmatter from rule `.md` files into typed EnforcementRule values
//!   - `engine`  — compiled-regex evaluation engine for file, bash, and scan events
//!   - `store`   — loads all rule files from a directory (used by the engine tests and by callers)
//!   - `scanner` — scans a project for governance files across config-defined artifact areas

/// YAML rule parser: reads `.md` rule files and produces typed `EnforcementRule` values.
pub mod engine;
/// Regex evaluation engine for file, bash, and scan events.
pub mod parser;
/// Governance file scanner: discovers rules, hooks, agents across canonical areas.
pub mod scanner;
/// Rule store: loads all rule files from a directory.
pub mod store;
