// orqa-project: Project scanning, settings types, and file-backed settings store.
//
// Contains:
//   - `scanner` — walks a project's filesystem to detect its technology stack and
//     count governance artifacts
//   - `settings` — Rust representation of `.orqa/project.json` (ProjectSettings,
//     GovernanceCounts, PluginProjectConfig, ArtifactLinksConfig, etc.)
//   - `store` — file-backed implementation of `ProjectSettingsStore`

pub mod scanner;
pub mod settings;
pub mod store;

pub use settings::*;
