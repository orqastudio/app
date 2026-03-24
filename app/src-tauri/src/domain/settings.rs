use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedTheme {
    pub project_id: i64,
    pub tokens: HashMap<String, ThemeToken>,
    pub source_files: Vec<String>,
    pub has_overrides: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeToken {
    pub name: String,
    pub value_light: String,
    pub value_dark: Option<String>,
    pub source: ThemeTokenSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ThemeTokenSource {
    Extracted,
    Override,
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarStatus {
    pub state: SidecarState,
    pub pid: Option<u32>,
    pub uptime_seconds: Option<u64>,
    pub cli_detected: bool,
    pub cli_version: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SidecarState {
    NotStarted,
    Starting,
    Connected,
    Error,
    Stopped,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_token_source_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(ThemeTokenSource::Extracted)
                .expect("serialization should succeed")
                .as_str(),
            Some("extracted")
        );
        assert_eq!(
            serde_json::to_value(ThemeTokenSource::Override)
                .expect("serialization should succeed")
                .as_str(),
            Some("override")
        );
        assert_eq!(
            serde_json::to_value(ThemeTokenSource::Default)
                .expect("serialization should succeed")
                .as_str(),
            Some("default")
        );
    }

    #[test]
    fn sidecar_state_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(SidecarState::NotStarted)
                .expect("serialization should succeed")
                .as_str(),
            Some("not_started")
        );
        assert_eq!(
            serde_json::to_value(SidecarState::Starting)
                .expect("serialization should succeed")
                .as_str(),
            Some("starting")
        );
        assert_eq!(
            serde_json::to_value(SidecarState::Connected)
                .expect("serialization should succeed")
                .as_str(),
            Some("connected")
        );
        assert_eq!(
            serde_json::to_value(SidecarState::Error)
                .expect("serialization should succeed")
                .as_str(),
            Some("error")
        );
        assert_eq!(
            serde_json::to_value(SidecarState::Stopped)
                .expect("serialization should succeed")
                .as_str(),
            Some("stopped")
        );
    }

    #[test]
    fn resolved_theme_roundtrip() {
        let mut tokens = HashMap::new();
        tokens.insert(
            "primary".to_string(),
            ThemeToken {
                name: "primary".to_string(),
                value_light: "oklch(0.7 0.15 250)".to_string(),
                value_dark: Some("oklch(0.8 0.15 250)".to_string()),
                source: ThemeTokenSource::Extracted,
            },
        );

        let theme = ResolvedTheme {
            project_id: 1,
            tokens,
            source_files: vec!["tailwind.config.ts".to_string()],
            has_overrides: false,
        };

        let json = serde_json::to_string(&theme).expect("serialization should succeed");
        let deserialized: ResolvedTheme =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert_eq!(deserialized.project_id, 1);
        assert!(!deserialized.has_overrides);
        assert!(deserialized.tokens.contains_key("primary"));
    }

    #[test]
    fn sidecar_status_serialization() {
        let status = SidecarStatus {
            state: SidecarState::Connected,
            pid: Some(12345),
            uptime_seconds: Some(3600),
            cli_detected: true,
            cli_version: Some("1.0.0".to_string()),
            error_message: None,
        };

        let json = serde_json::to_value(&status).expect("serialization should succeed");
        assert_eq!(json["state"], "connected");
        assert_eq!(json["pid"], 12345);
        assert!(json["error_message"].is_null());
    }

    #[test]
    fn sidecar_status_error_state() {
        let status = SidecarStatus {
            state: SidecarState::Error,
            pid: None,
            uptime_seconds: None,
            cli_detected: false,
            cli_version: None,
            error_message: Some("claude CLI not found in PATH".to_string()),
        };

        let json = serde_json::to_value(&status).expect("serialization should succeed");
        assert_eq!(json["state"], "error");
        assert!(!json["cli_detected"].as_bool().expect("should be a bool"));
        assert_eq!(json["error_message"], "claude CLI not found in PATH");
    }
}
