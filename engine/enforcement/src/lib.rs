// orqa-enforcement: Enforcement pipeline for the OrqaStudio platform.
//
// Provides the full enforcement pipeline:
//   - `parser`  — parses YAML frontmatter from rule `.md` files into typed EnforcementRule values
//   - `engine`  — compiled-regex evaluation engine for file, bash, and scan events
//   - `store`   — loads all rule files from a directory (used by the engine tests and by callers)
//   - `scanner` — scans a project for governance files across the 6 canonical areas

pub mod engine;
pub mod parser;
pub mod scanner;
pub mod store;
