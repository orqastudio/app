//! Project settings types for validation.
//!
//! Re-exports the canonical settings types from `orqa-engine-types`.
//! These types live in the foundation crate so all engine crates share
//! the same definitions without a dependency inversion.

pub use orqa_engine_types::config::{
    ArtifactEntry, ArtifactTypeConfig, ChildProjectConfig, DeliveryConfig, DeliveryParentConfig,
    DeliveryTypeConfig, PluginProjectConfig, ProjectRelationshipConfig, ProjectSettings,
    StatusDefinition,
};
