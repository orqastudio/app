// Project submodule for the orqa-engine crate.
//
// Contains project settings types (`ProjectSettings`, `GovernanceCounts`, etc.) —
// the Rust representation of `.orqa/project.json` — and the project scanner that
// walks a project's filesystem to detect its technology stack and governance artifacts.
// The `store` submodule provides a file-backed implementation of `ProjectSettingsStore`.

pub mod scanner;
pub mod settings;
pub mod store;

pub use settings::*;
