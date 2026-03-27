// orqa-engine: Core engine library for the OrqaStudio platform.
//
// This crate provides the following for all access layers (Tauri app, daemon, CLI, connectors):
//   - `types` — shared struct and enum definitions (no business logic, no I/O)
//   - `traits` — abstract storage interfaces that each access layer implements
//   - `error` — engine-level error type for I/O and serialization failures
//   - `config` — centralised project configuration loader
//   - `paths` — path constants and config-driven path resolution
//   - `graph` — artifact graph construction and query functions (re-exported from orqa_validation)
//   - `validation` — integrity check types and context-building functions (re-exported from orqa_validation)
//   - `metrics` — graph-theoretic metric types and computation functions (re-exported from orqa_validation)
//
// Consumers import from orqa_engine rather than directly from orqa_validation, keeping the
// dependency surface narrow and centralised.

pub mod config;
pub mod error;
pub mod graph;
pub mod lesson;
pub mod metrics;
pub mod paths;
pub mod platform;
pub mod project;
pub mod traits;
pub mod types;
pub mod utils;
pub mod validation;
