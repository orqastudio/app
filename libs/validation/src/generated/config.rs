// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.
// Source: libs/types/src/platform/*.schema.json
// Regenerate: cargo build -p orqa-validation

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

/// A single artifact type with a filesystem path to scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactTypeConfig {
    /// Artifact type key matching platform artifact types.
    pub key: String,
    /// Display label for this type.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub label: Option<String>,
    /// Lucide icon name.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub icon: Option<String>,
    /// Relative directory path within .orqa/ to scan for this type.
    pub path: String,
}

/// An entry in the artifacts config — either a direct type or a group of types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactEntry {
    Variant0(serde_json::Value),
    Variant1(ArtifactTypeConfig),
}

/// The parent relationship config for a delivery type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryParentConfig {
    /// The parent artifact type key.
    pub r#type: String,
    /// Relationship type connecting child to parent.
    pub relationship: String,
}

/// A single delivery type defined in project.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTypeConfig {
    /// Unique delivery type key.
    pub key: String,
    /// Human-readable label.
    pub label: String,
    /// Relative directory path within .orqa/ for this delivery type.
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub parent: Option<DeliveryParentConfig>,
    /// Frontmatter field used as the gate condition for status promotion.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub gate_field: Option<String>,
}

/// The delivery configuration block from project.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryConfig {
    /// Ordered list of delivery type definitions.
    pub types: Vec<DeliveryTypeConfig>,
}

/// A project-level relationship type defined in project.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRelationshipConfig {
    /// Relationship type key (lowercase, hyphenated).
    pub key: String,
    /// Inverse relationship type key.
    pub inverse: String,
    /// Human-readable label for the forward direction.
    pub label: String,
    /// Human-readable label for the inverse direction.
    pub inverse_label: String,
}

/// A child project reference in an organisation-mode project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildProjectConfig {
    /// Logical name for this child project.
    pub name: String,
    /// Absolute or relative path to the child project root.
    pub path: String,
}

/// A status definition loaded from project.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDefinition {
    /// Unique status key (lowercase).
    pub key: String,
    /// Human-readable label.
    pub label: String,
    /// Lucide icon name.
    pub icon: String,
    /// Whether to animate the icon with a spin effect.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spin: Option<bool>,
    /// Valid next status keys from this status.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub transitions: Option<Vec<String>>,
}

/// Per-plugin configuration stored in project.json under 'plugins.<name>'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginProjectConfig {
    /// Whether the plugin has been installed.
    pub installed: bool,
    /// Whether the plugin is currently active.
    pub enabled: bool,
    /// Absolute or relative path to the plugin root.
    pub path: String,
    /// Map of relationship key → enabled boolean for plugin-provided relationships.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub relationships: Option<serde_json::Value>,
    /// Plugin-specific configuration values.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub config: Option<serde_json::Value>,
}

/// Minimal project settings loaded from {project}/.orqa/project.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Project display name.
    pub name: String,
    /// Whether this is an organisation-mode project with child projects.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organisation: Option<bool>,
    /// Child project references (only used when organisation is true).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub projects: Option<Vec<ChildProjectConfig>>,
    /// Artifact type navigation configuration.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub artifacts: Option<Vec<ArtifactEntry>>,
    /// Valid status definitions for this project.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub statuses: Option<Vec<StatusDefinition>>,
    /// Delivery hierarchy configuration.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delivery: Option<DeliveryConfig>,
    /// Project-level relationship types beyond the core platform relationships.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub relationships: Option<Vec<ProjectRelationshipConfig>>,
    /// Plugin configurations keyed by plugin name.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub plugins: Option<std::collections::HashMap<String, PluginProjectConfig>>,
}
