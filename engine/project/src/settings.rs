//! Project settings types for the OrqaStudio engine.
//!
//! `ProjectSettings` is the Rust representation of `{project}/.orqa/project.json`.
//! It is the authoritative config for a project: artifact navigation, status transitions,
//! delivery hierarchy, plugin enablement, and display preferences.

use serde::{Deserialize, Serialize};

use orqa_engine_types::types::project::DetectedStack;

// Re-export config types from the canonical source in orqa-validation.
pub use orqa_validation::settings::{
    ArtifactEntry, ArtifactTypeConfig, DeliveryConfig, DeliveryParentConfig, DeliveryTypeConfig,
    ProjectRelationshipConfig,
};

// StatusAutoRule and StatusDefinition are defined in orqa-engine-types and re-exported here.
pub use orqa_engine_types::types::project_settings::{StatusAutoRule, StatusDefinition};

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
    /// Number of lesson files in `.orqa/learning/lessons/`.
    #[serde(default)]
    pub lessons: u32,
    /// Number of decision records in `.orqa/learning/decisions/`.
    #[serde(default)]
    pub decisions: u32,
    /// Number of enforcement rules in `.orqa/learning/rules/`.
    #[serde(default)]
    pub rules: u32,
    /// Number of documentation files in `.orqa/documentation/`.
    #[serde(default)]
    pub documentation: u32,
    /// Whether a `.claude/` configuration directory was detected.
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
    /// Installed version of the plugin (e.g. "0.1.4-dev").
    #[serde(default)]
    pub version: Option<String>,
    /// Per-relationship overrides (key -> enabled).
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
    /// Display name for this project.
    pub name: String,
    /// When `true`, this project is dogfooding — the app being built is the app being used.
    #[serde(default)]
    pub dogfood: bool,
    /// Optional short description displayed in project selection UI.
    #[serde(default)]
    pub description: Option<String>,
    /// Default AI model identifier (e.g. "auto", "claude-sonnet-4-6").
    #[serde(default = "default_model")]
    pub default_model: String,
    /// Directories excluded from artifact scanning (e.g. `node_modules`, `.git`).
    #[serde(default = "default_excluded_paths")]
    pub excluded_paths: Vec<String>,
    /// Last detected technology stack, cached in project.json by the scanner.
    #[serde(default)]
    pub stack: Option<DetectedStack>,
    /// Last detected governance artifact counts, cached in project.json by the scanner.
    #[serde(default)]
    pub governance: Option<GovernanceCounts>,
    /// Optional display icon for this project in the project switcher.
    #[serde(default)]
    pub icon: Option<String>,
    /// When `true`, the UI exposes the agent's thinking chain for debugging.
    #[serde(default)]
    pub show_thinking: bool,
    /// Optional custom system prompt prepended to every agent session.
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
    /// Delivery type hierarchy (milestone -> epic -> task) from `project.json`.
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
    "auto".to_owned()
}

/// Returns the default list of paths excluded from artifact scanning.
fn default_excluded_paths() -> Vec<String> {
    vec![
        "node_modules".to_owned(),
        ".git".to_owned(),
        "target".to_owned(),
        "dist".to_owned(),
        "build".to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_settings() -> ProjectSettings {
        ProjectSettings {
            name: "test-project".to_owned(),
            dogfood: false,
            description: Some("A test project".to_owned()),
            default_model: "auto".to_owned(),
            excluded_paths: default_excluded_paths(),
            stack: Some(DetectedStack {
                languages: vec!["rust".to_owned()],
                frameworks: vec!["tauri".to_owned()],
                package_manager: Some("cargo".to_owned()),
                has_claude_config: true,
                has_design_tokens: false,
            }),
            governance: Some(GovernanceCounts {
                lessons: 16,
                decisions: 44,
                rules: 45,
                documentation: 49,
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
        assert_eq!(gov.rules, 45);
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

    #[test]
    fn default_model_is_auto() {
        let settings: ProjectSettings = serde_json::from_str(r#"{"name":"test"}"#).expect("parse");
        assert_eq!(settings.default_model, "auto");
    }

    #[test]
    fn default_excluded_paths_has_five_entries() {
        let settings: ProjectSettings = serde_json::from_str(r#"{"name":"test"}"#).expect("parse");
        assert_eq!(settings.excluded_paths.len(), 5);
        assert!(settings.excluded_paths.contains(&"node_modules".to_owned()));
        assert!(settings.excluded_paths.contains(&"target".to_owned()));
        assert!(settings.excluded_paths.contains(&"dist".to_owned()));
    }

    #[test]
    fn artifact_link_display_mode_default_is_id() {
        let mode: ArtifactLinkDisplayMode = ArtifactLinkDisplayMode::default();
        assert_eq!(mode, ArtifactLinkDisplayMode::Id);
    }

    #[test]
    fn artifact_link_display_mode_serde_roundtrip() {
        let json = r#""title""#;
        let mode: ArtifactLinkDisplayMode = serde_json::from_str(json).expect("parse");
        assert_eq!(mode, ArtifactLinkDisplayMode::Title);
        let back = serde_json::to_string(&mode).expect("serialize");
        assert_eq!(back, r#""title""#);
    }

    #[test]
    fn plugin_config_with_version_deserializes() {
        // Verifies that plugin entries with a "version" field (written by `orqa install`)
        // are accepted by deny_unknown_fields deserialization.
        let json = r#"{
            "name": "test",
            "plugins": {
                "@orqastudio/plugin-cli": {
                    "path": "plugins/knowledge/cli",
                    "enabled": true,
                    "version": "0.1.4-dev",
                    "installed": true
                }
            }
        }"#;
        let settings: ProjectSettings =
            serde_json::from_str(json).expect("deserialization with version field should succeed");
        let plugin = settings
            .plugins
            .get("@orqastudio/plugin-cli")
            .expect("plugin should be present");
        assert_eq!(plugin.version.as_deref(), Some("0.1.4-dev"));
        assert!(plugin.installed);
        assert!(plugin.enabled);
    }
}
