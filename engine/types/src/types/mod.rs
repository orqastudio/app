//! Domain type submodules for the orqa-engine-types crate.
//!
//! Each module corresponds to a functional domain area. Types are pure data shapes
//! with serde derives — no business logic, no I/O.

/// Artifact data types: parsed frontmatter, relationships, graph nodes.
pub mod artifact;
/// Structured event types for the daemon event bus.
pub mod event;
/// Enforcement types: rules, mechanisms, evaluation results.
pub mod enforcement;
/// Governance artifact types: epics, tasks, decisions.
pub mod governance;
/// Graph health metrics types.
pub mod health;
/// Knowledge artifact types.
pub mod knowledge;
/// Lesson artifact types.
pub mod lesson;
/// Message types for agent conversation history.
pub mod message;
/// Project-level types: artifact types, statuses, delivery config.
pub mod project;
/// Project settings types loaded from `project.json`.
pub mod project_settings;
/// Agent session types: session state, history, context.
pub mod session;
/// Settings types for agent and system configuration.
pub mod settings;
/// Streaming response types for token-by-token output.
pub mod streaming;
/// Workflow and state-machine types.
pub mod workflow;
