//! Domain logic for the OrqaStudio Tauri backend.
//!
//! Each submodule covers a distinct business-logic concern. All modules are
//! free of Tauri-specific types and can be tested in isolation.

/// Artifact ID, type, and metadata helpers — re-exported from orqa-engine-types.
pub mod artifact;
/// Artifact graph construction from `.orqa/` directory scans.
pub mod artifact_graph;
/// Project configuration loader: resolves `project.json` from a path.
pub mod config_loader;
/// Enforcement violation types and formatting.
pub mod enforcement_violation;
/// Governance artifact read operations: reads raw files from `.orqa/`.
pub mod governance;
/// Health snapshot persistence: stores graph health metrics over time.
pub mod health_snapshot;
/// Integrity engine: runs integrity checks against the artifact graph.
pub mod integrity_engine;
/// Conversation message types and persistence helpers.
pub mod message;
/// Path resolution: derives artifact and config paths from project root.
pub mod paths;
/// Platform configuration: reads plugin-contributed platform config.
pub mod platform_config;
/// Project metadata: reads and writes project-level settings.
pub mod project;
/// Provider events: Tauri event payloads emitted by streaming and tool execution.
pub mod provider_event;
/// Session CRUD: creates, lists, updates, and deletes conversation sessions.
pub mod session;
/// Application settings: persists and reads user preferences.
pub mod settings;
/// Setup wizard domain logic: checks Claude CLI auth and embedding model availability.
pub mod setup;
/// Status transition types: re-exported from orqa-engine-types.
pub mod status_transitions;
/// Time utilities: ISO-8601 timestamp formatting.
pub mod time_utils;
