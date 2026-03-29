// Centralised project configuration types and loader for the engine-types crate.
//
// This module defines the foundational project configuration types that all engine
// crates depend on. These types represent `.orqa/project.json` at the minimal level
// needed for path resolution and plugin discovery. Access layers that need app-specific
// fields (e.g. GovernanceCounts, DetectedStack) define their own extended settings types.
//
// All code that needs to read `.orqa/project.json` MUST go through `load_project_settings`.
// This ensures a single code path for file I/O and deserialisation.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::EngineError;
use crate::paths::SETTINGS_FILE;

// ---------------------------------------------------------------------------
// Project configuration types
// ---------------------------------------------------------------------------

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
/// Only the fields needed for path resolution, graph building, and plugin discovery
/// are included here. Extra fields are silently ignored via `#[serde(default)]`.
/// Access layers needing app-specific fields define their own extended settings types.
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

// ---------------------------------------------------------------------------
// Settings loader
// ---------------------------------------------------------------------------

/// Load and parse project settings from `{project_root}/.orqa/project.json`.
///
/// Returns `Ok(None)` if the settings file does not exist.
/// Returns `Err` if the file exists but cannot be read or parsed.
pub fn load_project_settings(project_root: &Path) -> Result<Option<ProjectSettings>, EngineError> {
    let settings_file = project_root.join(SETTINGS_FILE);

    if !settings_file.exists() {
        return Ok(None);
    }

    let contents = std::fs::read_to_string(&settings_file)?;
    let settings: ProjectSettings = serde_json::from_str(&contents)?;
    Ok(Some(settings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::ORQA_DIR;

    fn minimal_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test".to_string(),
            organisation: false,
            projects: vec![],
            artifacts: vec![],
            statuses: vec![],
            delivery: Default::default(),
            relationships: vec![],
            plugins: HashMap::new(),
        }
    }

    #[test]
    fn returns_none_when_no_settings_file() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let result = load_project_settings(tmp.path());
        assert!(result.is_ok());
        assert!(result.expect("should be Ok").is_none());
    }

    #[test]
    fn loads_valid_settings() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let orqa_dir = tmp.path().join(ORQA_DIR);
        std::fs::create_dir_all(&orqa_dir).expect("create .orqa");

        let settings = minimal_settings();
        let json = serde_json::to_string_pretty(&settings).expect("serialize");
        std::fs::write(orqa_dir.join("project.json"), json).expect("write");

        let loaded = load_project_settings(tmp.path())
            .expect("load should succeed")
            .expect("settings should exist");
        assert_eq!(loaded.name, "test");
    }

    #[test]
    fn returns_error_on_malformed_json() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let orqa_dir = tmp.path().join(ORQA_DIR);
        std::fs::create_dir_all(&orqa_dir).expect("create .orqa");

        std::fs::write(orqa_dir.join("project.json"), "{ invalid json }").expect("write");

        let result = load_project_settings(tmp.path());
        assert!(result.is_err());
        let err = result.expect_err("should be error");
        assert!(matches!(err, EngineError::Serialization(_)));
    }
}
