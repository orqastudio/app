//! Project settings types — a standalone subset of the types from
//! `app/src-tauri/src/domain/project_settings.rs`.
//!
//! Only the types needed for graph building and integrity checks are included.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A single artifact type with a filesystem path to scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactTypeConfig {
    /// Machine-readable key identifying this artifact type (e.g. `"task"`, `"epic"`).
    pub key: String,
    /// Human-readable display label for this artifact type.
    #[serde(default)]
    pub label: Option<String>,
    /// Icon identifier for this artifact type in the UI.
    #[serde(default)]
    pub icon: Option<String>,
    /// Relative filesystem path where artifacts of this type are stored.
    pub path: String,
}

/// An entry in the artifacts config — either a direct type or a group of types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactEntry {
    /// A named group that contains child artifact type configs.
    Group {
        /// Machine-readable key for this group.
        key: String,
        /// Human-readable display label for this group.
        #[serde(default)]
        label: Option<String>,
        /// Icon identifier for this group in the UI.
        #[serde(default)]
        icon: Option<String>,
        /// Child artifact type configurations belonging to this group.
        children: Vec<ArtifactTypeConfig>,
    },
    /// A single artifact type (no grouping).
    Type(ArtifactTypeConfig),
}

/// The parent relationship config for a delivery type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryParentConfig {
    /// The artifact type key of the parent (e.g. `"epic"`).
    #[serde(rename = "type")]
    pub parent_type: String,
    /// The relationship type connecting the delivery artifact to its parent.
    pub relationship: String,
}

/// A single delivery type defined in `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTypeConfig {
    /// Machine-readable key for this delivery type.
    pub key: String,
    /// Human-readable display label for this delivery type.
    pub label: String,
    /// Relative filesystem path where delivery artifacts of this type are stored.
    pub path: String,
    /// Optional parent relationship config linking this delivery type to a parent type.
    #[serde(default)]
    pub parent: Option<DeliveryParentConfig>,
    /// Optional field name used as a gate condition for this delivery type.
    #[serde(default)]
    pub gate_field: Option<String>,
}

/// The delivery configuration block from `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryConfig {
    /// List of delivery type definitions for this project.
    #[serde(default)]
    pub types: Vec<DeliveryTypeConfig>,
}

/// A project-level relationship type defined in `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRelationshipConfig {
    /// Machine-readable key for this relationship type (e.g. `"enforced-by"`).
    pub key: String,
    /// Machine-readable key for the inverse relationship type.
    pub inverse: String,
    /// Human-readable label for the forward direction of this relationship.
    pub label: String,
    /// Human-readable label for the inverse direction of this relationship.
    pub inverse_label: String,
}

/// A child project reference in an organisation-mode project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildProjectConfig {
    /// Display name of the child project.
    pub name: String,
    /// Relative filesystem path to the child project root.
    pub path: String,
}

/// Per-plugin configuration stored in project.json under `plugins.<name>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginProjectConfig {
    /// Whether this plugin has been installed to the project.
    #[serde(default)]
    pub installed: bool,
    /// Whether this plugin is currently enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Filesystem path to the installed plugin directory.
    pub path: String,
    /// Optional map of relationship type keys to enabled/disabled flags.
    #[serde(default)]
    pub relationships: Option<HashMap<String, bool>>,
    /// Optional plugin-specific configuration values.
    #[serde(default)]
    pub config: Option<HashMap<String, serde_json::Value>>,
}

/// Minimal project settings loaded from `{project}/.orqa/project.json`.
///
/// Only the fields needed for graph building are included. Extra fields are
/// silently ignored via `#[serde(default)]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Human-readable name for this project.
    pub name: String,
    /// Whether this project is in organisation mode (scanning multiple sub-projects).
    #[serde(default)]
    pub organisation: bool,
    /// Child project references when in organisation mode.
    #[serde(default)]
    pub projects: Vec<ChildProjectConfig>,
    /// Artifact type definitions for this project.
    #[serde(default)]
    pub artifacts: Vec<ArtifactEntry>,
    /// Workflow status definitions for this project.
    #[serde(default)]
    pub statuses: Vec<StatusDefinition>,
    /// Delivery type configuration for this project.
    #[serde(default)]
    pub delivery: DeliveryConfig,
    /// Project-level relationship type definitions.
    #[serde(default)]
    pub relationships: Vec<ProjectRelationshipConfig>,
    /// Per-plugin configuration keyed by plugin name.
    #[serde(default)]
    pub plugins: HashMap<String, PluginProjectConfig>,
}

/// A status definition loaded from `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDefinition {
    /// Machine-readable key for this status (e.g. `"draft"`, `"in-progress"`).
    pub key: String,
    /// Human-readable display label for this status.
    pub label: String,
    /// Icon identifier for this status in the UI.
    pub icon: String,
    /// Whether the icon should spin (indicating an in-progress state).
    #[serde(default)]
    pub spin: bool,
    /// List of status keys this status may transition to.
    #[serde(default)]
    pub transitions: Vec<String>,
}
