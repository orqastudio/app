// Project settings types for the OrqaStudio engine.
//
// `ProjectSettings` is the Rust representation of `{project}/.orqa/project.json`.
// It is the authoritative config for a project: artifact navigation, status transitions,
// delivery hierarchy, plugin enablement, and display preferences.

use serde::{Deserialize, Serialize};

use crate::types::project::DetectedStack;

// Re-export config types from the canonical source in orqa-validation.
pub use orqa_validation::settings::{
    ArtifactEntry, ArtifactTypeConfig, DeliveryConfig, DeliveryParentConfig, DeliveryTypeConfig,
    ProjectRelationshipConfig,
};

/// An automatic transition rule on a status definition.
///
/// `condition` is a named condition evaluated by the transition engine.
/// `target` is the status key to transition to when the condition is met.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusAutoRule {
    pub condition: String,
    pub target: String,
}

/// A status definition loaded from `project.json`.
///
/// Extends the validation lib's `StatusDefinition` with `auto_rules` for the
/// transition engine (app-specific feature).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDefinition {
    pub key: String,
    pub label: String,
    pub icon: String,
    #[serde(default)]
    pub spin: bool,
    /// Ordered list of status keys that can be manually transitioned to from this status.
    #[serde(default)]
    pub transitions: Vec<String>,
    /// Automatic transition rules evaluated by the transition engine.
    #[serde(default)]
    pub auto_rules: Vec<StatusAutoRule>,
}

/// Display mode for artifact link chips.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactLinkDisplayMode {
    /// Show the artifact's ID (e.g. "EPIC-001").
    #[default]
    Id,
    /// Show the artifact's title (e.g. "My Epic Title").
    Title,
}

/// Per-type colour and display settings for artifact link chips.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtifactLinksConfig {
    /// Per-type prefix display mode override (e.g. `{ "EPIC": "title", "TASK": "id" }`).
    /// Absent prefixes fall back to `"id"`.
    #[serde(rename = "displayModes", default)]
    pub display_modes: std::collections::HashMap<String, ArtifactLinkDisplayMode>,
    /// Optional per-type prefix hex colour override (e.g. `{ "EPIC": "#3b82f6" }`).
    #[serde(default)]
    pub colors: std::collections::HashMap<String, String>,
}

/// Governance artifact counts for a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceCounts {
    #[serde(default)]
    pub lessons: u32,
    #[serde(default)]
    pub decisions: u32,
    #[serde(default)]
    pub agents: u32,
    #[serde(default)]
    pub rules: u32,
    #[serde(default)]
    pub knowledge: u32,
    #[serde(default)]
    pub has_claude_config: bool,
}

/// Per-plugin configuration stored in project.json under `plugins.<name>`.
///
/// Tracks installation state and whether the plugin is active.
/// The artifact scanner and graph builder only load plugins where
/// both `installed` AND `enabled` are `true`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginProjectConfig {
    /// Whether this plugin has been installed into the project.
    #[serde(default)]
    pub installed: bool,
    /// Whether this plugin is active (its schemas, relationships, and views are loaded).
    #[serde(default)]
    pub enabled: bool,
    /// Relative path to the plugin directory (from project root).
    pub path: String,
    /// Per-relationship overrides (key → enabled).
    #[serde(default)]
    pub relationships: Option<std::collections::HashMap<String, bool>>,
    /// Plugin-specific settings.
    #[serde(default)]
    pub config: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// File-based project settings stored in `{project}/.orqa/project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSettings {
    pub name: String,
    /// When `true`, this project is dogfooding — the app being built is the app being used.
    #[serde(default)]
    pub dogfood: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_model")]
    pub default_model: String,
    #[serde(default = "default_excluded_paths")]
    pub excluded_paths: Vec<String>,
    #[serde(default)]
    pub stack: Option<DetectedStack>,
    #[serde(default)]
    pub governance: Option<GovernanceCounts>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub show_thinking: bool,
    #[serde(default)]
    pub custom_system_prompt: Option<String>,
    /// Config-driven artifact navigation tree.
    ///
    /// Each entry is either a direct artifact type or a group of types.
    /// When absent, the scanner returns an empty navigation tree.
    #[serde(default)]
    pub artifacts: Vec<ArtifactEntry>,
    /// Artifact link chip display settings (display mode and per-type colours).
    #[serde(rename = "artifactLinks", default)]
    pub artifact_links: ArtifactLinksConfig,
    /// Status definitions loaded from `project.json`.
    ///
    /// When absent, the app falls back to built-in defaults.
    #[serde(default)]
    pub statuses: Vec<StatusDefinition>,
    /// Delivery type hierarchy (milestone → epic → task) from `project.json`.
    ///
    /// When absent, defaults to an empty hierarchy.
    #[serde(default)]
    pub delivery: DeliveryConfig,
    /// Project-level relationship types that extend the canonical vocabulary.
    ///
    /// When absent, no project relationships are defined.
    #[serde(default)]
    pub relationships: Vec<ProjectRelationshipConfig>,
    /// Per-plugin configuration keyed by plugin name.
    ///
    /// Only plugins with `installed: true` AND `enabled: true` are loaded
    /// by the artifact scanner and graph builder.
    #[serde(default)]
    pub plugins: std::collections::HashMap<String, PluginProjectConfig>,
}

/// Returns the default model identifier used when none is specified in project.json.
fn default_model() -> String {
    "auto".to_string()
}

/// Returns the default list of paths excluded from artifact scanning.
fn default_excluded_paths() -> Vec<String> {
    vec![
        "node_modules".to_string(),
        ".git".to_string(),
        "target".to_string(),
        "dist".to_string(),
        "build".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test-project".to_string(),
            dogfood: false,
            description: Some("A test project".to_string()),
            default_model: "auto".to_string(),
            excluded_paths: default_excluded_paths(),
            stack: Some(DetectedStack {
                languages: vec!["rust".to_string()],
                frameworks: vec!["tauri".to_string()],
                package_manager: Some("cargo".to_string()),
                has_claude_config: true,
                has_design_tokens: false,
            }),
            governance: Some(GovernanceCounts {
                lessons: 16,
                decisions: 44,
                agents: 7,
                rules: 45,
                knowledge: 49,
                has_claude_config: true,
            }),
            icon: None,
            show_thinking: false,
            custom_system_prompt: None,
            artifacts: vec![],
            artifact_links: ArtifactLinksConfig {
                display_modes: std::collections::HashMap::new(),
                colors: std::collections::HashMap::new(),
            },
            statuses: vec![],
            delivery: DeliveryConfig::default(),
            relationships: vec![],
            plugins: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn roundtrip_serialization() {
        let settings = sample_settings();
        let json = serde_json::to_string_pretty(&settings).expect("serialization should succeed");
        let deserialized: ProjectSettings =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.name, settings.name);
        assert_eq!(deserialized.description, settings.description);
        assert_eq!(deserialized.default_model, settings.default_model);
        assert_eq!(deserialized.excluded_paths, settings.excluded_paths);
        assert!(deserialized.stack.is_some());
        assert!(deserialized.governance.is_some());

        let gov = deserialized.governance.as_ref().expect("governance");
        assert_eq!(gov.lessons, 16);
        assert_eq!(gov.agents, 7);
        assert!(gov.has_claude_config);
    }

    #[test]
    fn serde_defaults_applied_for_missing_fields() {
        let json = r#"{"name": "minimal"}"#;
        let settings: ProjectSettings =
            serde_json::from_str(json).expect("deserialization should succeed");

        assert_eq!(settings.name, "minimal");
        assert!(settings.description.is_none());
        assert_eq!(settings.default_model, "auto");
        assert_eq!(settings.excluded_paths.len(), 5);
        assert!(settings.stack.is_none());
        assert!(settings.governance.is_none());
        assert!(!settings.show_thinking);
        assert!(settings.custom_system_prompt.is_none());
    }
}
