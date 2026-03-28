//! Project settings types for validation.
//!
//! Self-contained copy of the types needed for graph building and integrity
//! checks, mirroring `libs/mcp-server/src/settings.rs`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A single artifact type with a filesystem path to scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactTypeConfig {
    /// Unique key identifying this artifact type (e.g. `"task"`, `"epic"`).
    pub key: String,
    /// Human-readable label shown in the UI.
    #[serde(default)]
    pub label: Option<String>,
    /// Icon identifier for the UI.
    #[serde(default)]
    pub icon: Option<String>,
    /// Filesystem path where artifacts of this type are stored.
    pub path: String,
}

/// An entry in the artifacts config — either a direct type or a group of types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactEntry {
    /// A group of related artifact types sharing a common key prefix.
    Group {
        /// Group key (used as path prefix).
        key: String,
        /// Human-readable group label.
        #[serde(default)]
        label: Option<String>,
        /// Icon identifier for the group.
        #[serde(default)]
        icon: Option<String>,
        /// Artifact types within this group.
        children: Vec<ArtifactTypeConfig>,
    },
    /// A single artifact type entry.
    Type(ArtifactTypeConfig),
}

/// The parent relationship config for a delivery type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryParentConfig {
    /// The artifact type of the parent (e.g. `"epic"`).
    #[serde(rename = "type")]
    pub parent_type: String,
    /// The relationship type linking this delivery artifact to its parent.
    pub relationship: String,
}

/// A single delivery type defined in `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTypeConfig {
    /// Unique key for this delivery type.
    pub key: String,
    /// Human-readable label.
    pub label: String,
    /// Filesystem path where artifacts of this type are stored.
    pub path: String,
    /// Optional parent relationship config.
    #[serde(default)]
    pub parent: Option<DeliveryParentConfig>,
    /// Optional field name used as a gate condition.
    #[serde(default)]
    pub gate_field: Option<String>,
}

/// The delivery configuration block from `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryConfig {
    /// Delivery type definitions.
    #[serde(default)]
    pub types: Vec<DeliveryTypeConfig>,
}

/// A project-level relationship type defined in `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRelationshipConfig {
    /// Unique key for this relationship type.
    pub key: String,
    /// Inverse relationship key.
    pub inverse: String,
    /// Human-readable label for the forward direction.
    pub label: String,
    /// Human-readable label for the inverse direction.
    pub inverse_label: String,
}

/// A child project reference in an organisation-mode project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildProjectConfig {
    /// Display name of the child project.
    pub name: String,
    /// Relative path to the child project root.
    pub path: String,
}

/// Per-plugin configuration stored in project.json under `plugins.<name>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginProjectConfig {
    /// Whether the plugin has been installed into this project.
    #[serde(default)]
    pub installed: bool,
    /// Whether the plugin is currently enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Filesystem path to the plugin root.
    pub path: String,
    /// Per-relationship-type enable/disable overrides.
    #[serde(default)]
    pub relationships: Option<HashMap<String, bool>>,
    /// Arbitrary plugin-specific configuration values.
    #[serde(default)]
    pub config: Option<HashMap<String, serde_json::Value>>,
}

/// A status definition loaded from `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDefinition {
    /// Unique status key (e.g. `"active"`, `"closed"`).
    pub key: String,
    /// Human-readable label.
    pub label: String,
    /// Icon identifier.
    pub icon: String,
    /// Whether the status icon should spin (indicates in-progress state).
    #[serde(default)]
    pub spin: bool,
    /// Status keys this status can transition to.
    #[serde(default)]
    pub transitions: Vec<String>,
}

/// Minimal project settings loaded from `{project}/.orqa/project.json`.
///
/// Only the fields needed for graph building and validation are included.
/// Extra fields are silently ignored via `#[serde(default)]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Project display name.
    pub name: String,
    /// Whether this is an organisation-mode project with child projects.
    #[serde(default)]
    pub organisation: bool,
    /// Child project references (organisation mode only).
    #[serde(default)]
    pub projects: Vec<ChildProjectConfig>,
    /// Artifact type definitions for this project.
    #[serde(default)]
    pub artifacts: Vec<ArtifactEntry>,
    /// Valid status definitions for this project.
    #[serde(default)]
    pub statuses: Vec<StatusDefinition>,
    /// Delivery type configuration.
    #[serde(default)]
    pub delivery: DeliveryConfig,
    /// Project-level relationship type definitions.
    #[serde(default)]
    pub relationships: Vec<ProjectRelationshipConfig>,
    /// Installed plugin configuration.
    #[serde(default)]
    pub plugins: HashMap<String, PluginProjectConfig>,
}
