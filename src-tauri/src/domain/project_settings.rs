use serde::{Deserialize, Serialize};

use crate::domain::project::DetectedStack;

/// A single artifact type with a filesystem path to scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactTypeConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub icon: Option<String>,
    pub path: String,
}

/// An entry in the artifacts config — either a direct type or a group of types.
///
/// Serde uses `untagged` matching: `Group` must come before `Type` in the enum
/// because it has a required `children` field that distinguishes it from a bare type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactEntry {
    /// A named group containing multiple artifact types.
    Group {
        key: String,
        label: String,
        #[serde(default)]
        icon: Option<String>,
        children: Vec<ArtifactTypeConfig>,
    },
    /// A direct artifact type with its own path.
    Type(ArtifactTypeConfig),
}

/// Governance artifact counts for a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceCounts {
    pub docs: u32,
    pub agents: u32,
    pub rules: u32,
    pub skills: u32,
    pub hooks: u32,
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
                docs: 10,
                agents: 3,
                rules: 5,
                skills: 2,
                hooks: 1,
                has_claude_config: true,
            }),
            icon: None,
            show_thinking: false,
            custom_system_prompt: None,
            artifacts: vec![],
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
        assert_eq!(gov.docs, 10);
        assert_eq!(gov.agents, 3);
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
