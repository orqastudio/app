//! Project settings types — a standalone subset of the types from
//! `app/backend/src-tauri/src/domain/project_settings.rs`.
//!
//! Only the types needed for graph building and integrity checks are included.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A single artifact type with a filesystem path to scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactTypeConfig {
    pub key: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    pub path: String,
}

/// An entry in the artifacts config — either a direct type or a group of types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactEntry {
    Group {
        key: String,
        #[serde(default)]
        label: Option<String>,
        #[serde(default)]
        icon: Option<String>,
        children: Vec<ArtifactTypeConfig>,
    },
    Type(ArtifactTypeConfig),
}

/// The parent relationship config for a delivery type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryParentConfig {
    #[serde(rename = "type")]
    pub parent_type: String,
    pub relationship: String,
}

/// A single delivery type defined in `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTypeConfig {
    pub key: String,
    pub label: String,
    pub path: String,
    #[serde(default)]
    pub parent: Option<DeliveryParentConfig>,
    #[serde(default)]
    pub gate_field: Option<String>,
}

/// The delivery configuration block from `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryConfig {
    #[serde(default)]
    pub types: Vec<DeliveryTypeConfig>,
}

/// A project-level relationship type defined in `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRelationshipConfig {
    pub key: String,
    pub inverse: String,
    pub label: String,
    pub inverse_label: String,
}

/// A child project reference in an organisation-mode project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildProjectConfig {
    pub name: String,
    pub path: String,
}

/// Per-plugin configuration stored in project.json under `plugins.<name>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginProjectConfig {
    #[serde(default)]
    pub installed: bool,
    #[serde(default)]
    pub enabled: bool,
    pub path: String,
    #[serde(default)]
    pub relationships: Option<HashMap<String, bool>>,
    #[serde(default)]
    pub config: Option<HashMap<String, serde_json::Value>>,
}

/// Minimal project settings loaded from `{project}/.orqa/project.json`.
///
/// Only the fields needed for graph building are included. Extra fields are
/// silently ignored via `#[serde(default)]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: String,
    #[serde(default)]
    pub organisation: bool,
    #[serde(default)]
    pub projects: Vec<ChildProjectConfig>,
    #[serde(default)]
    pub artifacts: Vec<ArtifactEntry>,
    #[serde(default)]
    pub statuses: Vec<StatusDefinition>,
    #[serde(default)]
    pub delivery: DeliveryConfig,
    #[serde(default)]
    pub relationships: Vec<ProjectRelationshipConfig>,
    #[serde(default)]
    pub plugins: HashMap<String, PluginProjectConfig>,
}

/// A status definition loaded from `project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDefinition {
    pub key: String,
    pub label: String,
    pub icon: String,
    #[serde(default)]
    pub spin: bool,
    #[serde(default)]
    pub transitions: Vec<String>,
}
