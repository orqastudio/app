// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.
// Source: libs/types/src/platform/*.schema.json
// Regenerate: cargo build -p orqa-validation

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

/// An artifact type schema contributed by a plugin. Plugin schemas must declare key, label, icon, idPrefix, and frontmatter. They may use $ref to reference core types in core.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginArtifactSchema {
    /// Unique artifact type key (lowercase, hyphenated). MUST NOT collide with core artifact type keys.
    pub key: String,
    /// Singular display label.
    pub label: String,
    /// Plural display label. Defaults to label + 's' if omitted.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub plural: Option<String>,
    /// Lucide icon name.
    pub icon: String,
    /// Default relative path within .orqa/ for storing artifacts of this type.
    #[serde(rename = "defaultPath")]
    pub default_path: String,
    /// ID prefix for auto-generated IDs (e.g. 'EPIC', 'TASK'). MUST NOT collide with core ID prefixes.
    #[serde(rename = "idPrefix")]
    pub id_prefix: String,
    /// Frontmatter field declarations for this artifact type.
    pub frontmatter: serde_json::Value,
    /// Status transition rules specific to this artifact type.
    #[serde(rename = "statusTransitions", skip_serializing_if = "Option::is_none", default)]
    pub status_transitions: Option<serde_json::Value>,
}

/// A relationship type contributed by a plugin. Relationships MUST include key, inverse, from, to, and semantic to integrate correctly with the validation engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRelationshipExtension {
    /// Relationship type key (lowercase, hyphenated). MUST NOT collide with core relationship keys.
    pub key: String,
    /// Inverse relationship key. May be the same as key for symmetric relationships.
    pub inverse: String,
    /// Human-readable label for the forward direction.
    pub label: String,
    /// Human-readable label for the inverse direction.
    #[serde(rename = "inverseLabel")]
    pub inverse_label: String,
    /// Artifact type keys that are valid sources for this relationship.
    pub from: Vec<String>,
    /// Artifact type keys that are valid targets for this relationship.
    pub to: Vec<String>,
    /// Human-readable explanation of what this relationship means.
    pub description: String,
    /// The semantic category this relationship belongs to. Determines how the validation engine treats it.
    pub semantic: String,
    /// Optional validation constraints for this relationship.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub constraints: Option<serde_json::Value>,
}

/// The 'provides' block of a plugin manifest. Declares all capabilities the plugin registers with the app at install time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifestProvides {
    /// Artifact type schemas this plugin introduces. Each schema extends the core type system.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub schemas: Option<Vec<PluginArtifactSchema>>,
    /// Relationship types this plugin introduces. Each relationship extends the core relationship vocabulary.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub relationships: Option<Vec<PluginRelationshipExtension>>,
    /// Knowledge artifact paths contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub knowledge: Option<Vec<String>>,
    /// Agent artifact paths contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub agents: Option<Vec<String>>,
    /// Custom view registrations contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub views: Option<Vec<serde_json::Value>>,
    /// Dashboard widget registrations contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub widgets: Option<Vec<serde_json::Value>>,
    /// Hook registrations contributed by this plugin.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hooks: Option<Vec<String>>,
}

/// The full orqa-plugin.json manifest for an OrqaStudio plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Package name. Scoped (@org/name) or unscoped (name).
    pub name: String,
    /// Semver version string.
    pub version: String,
    /// Human-readable name shown in the plugin manager UI.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none", default)]
    pub display_name: Option<String>,
    /// Short description of what the plugin does.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<serde_json::Value>,
    /// SPDX license identifier.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub license: Option<String>,
    /// Plugin dependencies — names of plugins that must be loaded first.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub requires: Option<Vec<String>>,
    /// Minimum versions required for this plugin to function.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub compatibility: Option<serde_json::Value>,
    pub provides: PluginManifestProvides,
}

/// Result of validating a plugin manifest at install time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInstallValidationResult {
    /// Whether the plugin manifest passed all validation checks.
    pub valid: bool,
    /// Blocking validation errors that must be resolved before installation.
    pub errors: Vec<String>,
    /// Non-blocking warnings about the plugin manifest.
    pub warnings: Vec<String>,
    /// Schema or relationship keys that conflict with existing definitions.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub key_collisions: Option<Vec<String>>,
}

