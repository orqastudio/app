use serde::{Deserialize, Serialize};

use crate::domain::project::DetectedStack;

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

/// A single artifact type with a filesystem path to scan.
///
/// `label` and `icon` are optional — the scanner reads them from the directory's
/// README.md frontmatter when absent, falling back to a humanized key name.
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
///
/// Serde uses `untagged` matching: `Group` must come before `Type` in the enum
/// because it has a required `children` field that distinguishes it from a bare type.
///
/// `label` and `icon` on both variants are optional — presentation metadata comes
/// primarily from README.md frontmatter in each directory, not from this config.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactEntry {
    /// A named group containing multiple artifact types.
    Group {
        key: String,
        #[serde(default)]
        label: Option<String>,
        #[serde(default)]
        icon: Option<String>,
        children: Vec<ArtifactTypeConfig>,
    },
    /// A direct artifact type with its own path.
    Type(ArtifactTypeConfig),
}

/// The parent relationship config for a delivery type.
///
/// `parent_type` is the key of the parent delivery type.
/// `field` is the frontmatter field on this type that references the parent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryParentConfig {
    #[serde(rename = "type")]
    pub parent_type: String,
    pub field: String,
}

/// A single delivery type defined in `project.json`.
///
/// Delivery types form a hierarchy (e.g. milestone → epic → task).
/// `parent` is `None` for the root type.
/// `gate_field` names the frontmatter field that acts as the completion gate question.
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
    pub skills: u32,
    #[serde(default)]
    pub has_claude_config: bool,
}

/// File-based project settings stored in `{project}/.orqa/project.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: String,
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
}

fn default_model() -> String {
    "auto".to_string()
}

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
                skills: 49,
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
