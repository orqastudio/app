//! Foundation crate for the OrqaStudio engine.
//!
//! Contains shared type definitions, error types, abstract traits, and utility
//! functions used by all engine domain crates (agent, enforcement, workflow,
//! prompt, plugin) and re-exported through the engine facade.
//!
//! This crate has no business logic — only data shapes, contracts, and pure helpers.
//! The config, paths, and platform modules are included here (not in the engine facade)
//! because they form the foundational layer that all domain crates depend on.

/// Project configuration types (settings, plugins, delivery).
pub mod config;
/// Unified error types for engine operations.
pub mod error;
/// Fingerprinting utilities for log event deduplication.
pub mod fingerprint;
/// Filesystem path resolution for project artifacts and state.
pub mod paths;
/// Platform-level type definitions shared across crates.
pub mod platform;
/// Port constants and resolution helpers for all OrqaStudio services.
pub mod ports;
/// Abstract traits defining engine contracts (capabilities, interfaces).
pub mod traits;
/// Shared data structures for all engine domain types.
pub mod types;

/// Re-exports of event types for direct access from this crate's root.
pub use types::event::{EventLevel, EventSource, LogEvent};
/// Re-exports of graph types for direct access from this crate's root.
pub use types::graph::{
    AncestryChain, AncestryNode, AppliedFix, ArtifactGraph, ArtifactNode, ArtifactRef, GraphHealth,
    GraphStats, IntegrityCategory, IntegrityCheck, IntegritySeverity, OutlierAgeDistribution,
    TraceabilityResult, TracedArtifact,
};
/// Utility helpers (time formatting, etc.).
pub mod utils;
