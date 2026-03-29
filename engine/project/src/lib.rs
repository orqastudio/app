//! orqa-project: Project scanning, settings types, and file-backed settings store.
//!
//! Contains:
//!   - `scanner` — walks a project's filesystem to detect its technology stack and
//!     count governance artifacts
//!   - `settings` — Rust representation of `.orqa/project.json` (ProjectSettings,
//!     GovernanceCounts, PluginProjectConfig, ArtifactLinksConfig, etc.)
//!   - `store` — file-backed implementation of `ProjectSettingsStore`

/// Git state utilities: stash list and uncommitted file queries.
pub mod git;
/// Project filesystem scanner: stack detection and governance artifact counting.
pub mod scanner;
/// Project settings types: Rust representation of `.orqa/project.json`.
pub mod settings;
/// File-backed project settings store: reads and writes `project.json`.
pub mod store;

pub use settings::*;
