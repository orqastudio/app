// Domain type submodules for the orqa-engine crate.
//
// Each module corresponds to a functional domain area. Types are extracted from the
// app's domain layer to make them available to all access layers without duplication.
// Business logic (parsing, validation, file I/O) remains in the app and will migrate
// to dedicated engine crates (graph, enforcement, workflow, etc.) in later phases.

pub mod artifact;
pub mod enforcement;
pub mod governance;
pub mod health;
pub mod knowledge;
pub mod lesson;
pub mod message;
pub mod project;
pub mod session;
pub mod settings;
pub mod streaming;
pub mod workflow;
