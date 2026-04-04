// Repository modules for orqa-storage.
//
// Each module wraps a subset of the unified database as a zero-cost repo struct
// that borrows a &Storage. All SQL is colocated with its repo rather than
// scattered across binary crates.

/// Devtools session and event persistence.
pub mod devtools;
/// Daemon log event persistence with batch insert support.
pub mod events;
/// Health snapshot persistence for graph integrity trend data.
pub mod health;
/// Message persistence for session turns and content blocks.
pub mod messages;
/// Project persistence — top-level container for sessions and artifacts.
pub mod projects;
/// Session persistence — interaction contexts between user and agent.
pub mod sessions;
/// Key-value settings persistence with scope support.
pub mod settings;
/// Design token theme and override persistence.
pub mod themes;
/// Enforcement violation record persistence.
pub mod violations;
