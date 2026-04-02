//! Artifact graph type re-exports for the Tauri app layer.
//!
//! All graph building, health computation, integrity checking, and artifact
//! field updates are handled by the daemon. The app layer accesses them via
//! `DaemonClient` HTTP calls. This module exists solely as a type re-export
//! shim so that app-layer code can refer to canonical graph types through a
//! stable import path without depending on the engine type crate directly.

pub use orqa_engine_types::{
    AppliedFix, ArtifactGraph, ArtifactNode, ArtifactRef, GraphHealth, GraphStats,
    IntegrityCategory, IntegrityCheck, IntegritySeverity,
};
