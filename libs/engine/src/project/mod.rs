// Project configuration submodule for the orqa-engine crate.
//
// Contains the `ProjectSettings` type — the Rust representation of `.orqa/project.json` —
// along with supporting types for artifact navigation, status transitions,
// delivery hierarchy, and plugin configuration.

pub mod settings;

pub use settings::*;
