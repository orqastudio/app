// Domain type submodules for the orqa-engine-types crate.
//
// Each module corresponds to a functional domain area. Types are pure data shapes
// with serde derives — no business logic, no I/O.

pub mod artifact;
pub mod enforcement;
pub mod governance;
pub mod health;
pub mod knowledge;
pub mod lesson;
pub mod message;
pub mod project;
pub mod project_settings;
pub mod session;
pub mod settings;
pub mod streaming;
pub mod workflow;
