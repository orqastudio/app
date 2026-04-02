//! Integrity type re-exports for the Tauri app layer.
//!
//! All schema-driven integrity checks, context building, and traceability
//! computation are handled by the daemon. The app accesses them via
//! `DaemonClient` HTTP calls. This module re-exports the canonical types so
//! app-layer code has a stable import path without a direct engine-types dep.

pub use orqa_engine_types::{AncestryChain, AncestryNode, TraceabilityResult, TracedArtifact};
