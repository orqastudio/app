//! Plugin manifest reader — Rust representation of the PluginManifest type.
//!
//! Reads and validates `orqa-plugin.json` from a plugin directory.
//! The manifest is the authoritative descriptor of what a plugin provides.

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::EngineError;

/// Minimal Rust representation of a plugin manifest.
///
/// Only the fields the engine needs are parsed. The full manifest is handled
/// by the TypeScript SDK on the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub provides: PluginProvides,
    /// Recorded decisions from previous installations when relationship or
    /// artifact type keys collided with core or other plugins.
    /// Key: the relationship/schema key. Value: the decision made.
    #[serde(default, rename = "mergeDecisions")]
    pub merge_decisions: Vec<MergeDecision>,
}

/// A recorded decision about a key collision during plugin installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeDecision {
    /// The relationship or schema key that collided.
    pub key: String,
    /// What was decided: "merged" (union from/to) or "renamed" (key namespaced).
    pub decision: String,
    /// The original key name if renamed (e.g. "merged-into" before becoming "sw-merged-into").
    #[serde(rename = "originalKey", skip_serializing_if = "Option::is_none")]
    pub original_key: Option<String>,
    /// The source that owns the existing definition (e.g. "core" or a plugin name).
    #[serde(rename = "existingSource")]
    pub existing_source: String,
}

/// What a plugin declares it provides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginProvides {
    #[serde(default)]
    pub schemas: Vec<serde_json::Value>,
    #[serde(default)]
    pub views: Vec<serde_json::Value>,
    #[serde(default)]
    pub widgets: Vec<serde_json::Value>,
    #[serde(default)]
    pub relationships: Vec<serde_json::Value>,
    pub sidecar: Option<serde_json::Value>,
    #[serde(default, rename = "cliTools")]
    pub cli_tools: Vec<serde_json::Value>,
    #[serde(default)]
    pub hooks: Vec<serde_json::Value>,
}

const MANIFEST_FILENAME: &str = "orqa-plugin.json";

/// Read a plugin manifest from a directory.
///
/// Returns `EngineError::Plugin` if the manifest file is absent, and
/// `EngineError::Serialization` if the JSON cannot be parsed.
pub fn read_manifest(plugin_dir: &Path) -> Result<PluginManifest, EngineError> {
    let manifest_path = plugin_dir.join(MANIFEST_FILENAME);

    if !manifest_path.exists() {
        return Err(EngineError::Plugin(format!(
            "manifest not found: {}",
            manifest_path.display()
        )));
    }

    let contents = std::fs::read_to_string(&manifest_path)?;
    let manifest: PluginManifest = serde_json::from_str(&contents)?;

    Ok(manifest)
}

/// Validate a plugin manifest, returning a list of error messages.
///
/// An empty return value means the manifest is valid.
pub fn validate_manifest(manifest: &PluginManifest) -> Vec<String> {
    let mut errors = Vec::new();

    if manifest.name.is_empty() {
        errors.push("missing required field: name".to_string());
    }

    if manifest.version.is_empty() {
        errors.push("missing required field: version".to_string());
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_minimal_manifest() {
        let json = r#"{
            "name": "@orqastudio/test-plugin",
            "version": "0.1.0",
            "provides": {
                "schemas": [],
                "views": [],
                "widgets": [],
                "relationships": []
            }
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.name, "@orqastudio/test-plugin");
        assert_eq!(manifest.version, "0.1.0");
        assert!(manifest.provides.cli_tools.is_empty());
    }

    #[test]
    fn validate_rejects_empty_name() {
        let manifest = PluginManifest {
            name: String::new(),
            version: "0.1.0".to_string(),
            display_name: None,
            description: None,
            provides: PluginProvides {
                schemas: vec![],
                views: vec![],
                widgets: vec![],
                relationships: vec![],
                sidecar: None,
                cli_tools: vec![],
                hooks: vec![],
            },
            merge_decisions: vec![],
        };

        let errors = validate_manifest(&manifest);
        assert!(errors.iter().any(|e| e.contains("name")));
    }

    #[test]
    fn validate_rejects_empty_version() {
        let manifest = PluginManifest {
            name: "@orqastudio/test".to_string(),
            version: String::new(),
            display_name: None,
            description: None,
            provides: PluginProvides {
                schemas: vec![],
                views: vec![],
                widgets: vec![],
                relationships: vec![],
                sidecar: None,
                cli_tools: vec![],
                hooks: vec![],
            },
            merge_decisions: vec![],
        };

        let errors = validate_manifest(&manifest);
        assert!(errors.iter().any(|e| e.contains("version")));
    }
}
