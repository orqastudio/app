use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupStatus {
    pub setup_complete: bool,
    pub current_version: u32,
    pub stored_version: u32,
    pub steps: Vec<SetupStepStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupStepStatus {
    pub id: String,
    pub label: String,
    pub status: StepStatus,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    Pending,
    Checking,
    Complete,
    Error,
    ActionRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCliInfo {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub authenticated: bool,
    pub subscription_type: Option<String>,
    pub rate_limit_tier: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_status_serializes_snake_case() {
        assert_eq!(
            serde_json::to_value(StepStatus::Pending)
                .expect("serialization should succeed")
                .as_str(),
            Some("pending")
        );
        assert_eq!(
            serde_json::to_value(StepStatus::Checking)
                .expect("serialization should succeed")
                .as_str(),
            Some("checking")
        );
        assert_eq!(
            serde_json::to_value(StepStatus::Complete)
                .expect("serialization should succeed")
                .as_str(),
            Some("complete")
        );
        assert_eq!(
            serde_json::to_value(StepStatus::Error)
                .expect("serialization should succeed")
                .as_str(),
            Some("error")
        );
        assert_eq!(
            serde_json::to_value(StepStatus::ActionRequired)
                .expect("serialization should succeed")
                .as_str(),
            Some("action_required")
        );
    }

    #[test]
    fn step_status_deserializes_snake_case() {
        let parsed: StepStatus =
            serde_json::from_str("\"action_required\"").expect("deserialization should succeed");
        assert_eq!(parsed, StepStatus::ActionRequired);

        let parsed: StepStatus =
            serde_json::from_str("\"pending\"").expect("deserialization should succeed");
        assert_eq!(parsed, StepStatus::Pending);
    }

    #[test]
    fn setup_status_roundtrip() {
        let status = SetupStatus {
            setup_complete: false,
            current_version: 1,
            stored_version: 0,
            steps: vec![
                SetupStepStatus {
                    id: "claude_cli".to_string(),
                    label: "Claude CLI".to_string(),
                    status: StepStatus::Complete,
                    detail: Some("v1.2.3".to_string()),
                },
                SetupStepStatus {
                    id: "authentication".to_string(),
                    label: "Authentication".to_string(),
                    status: StepStatus::Pending,
                    detail: None,
                },
            ],
        };

        let json = serde_json::to_string(&status).expect("serialization should succeed");
        let deserialized: SetupStatus =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert!(!deserialized.setup_complete);
        assert_eq!(deserialized.current_version, 1);
        assert_eq!(deserialized.stored_version, 0);
        assert_eq!(deserialized.steps.len(), 2);
        assert_eq!(deserialized.steps[0].id, "claude_cli");
        assert_eq!(deserialized.steps[0].status, StepStatus::Complete);
        assert_eq!(deserialized.steps[1].detail, None);
    }

    #[test]
    fn setup_status_serialization_structure() {
        let status = SetupStatus {
            setup_complete: true,
            current_version: 1,
            stored_version: 1,
            steps: vec![SetupStepStatus {
                id: "embedding_model".to_string(),
                label: "Embedding Model".to_string(),
                status: StepStatus::ActionRequired,
                detail: Some("Model not downloaded".to_string()),
            }],
        };

        let json = serde_json::to_value(&status).expect("serialization should succeed");
        assert_eq!(json["setup_complete"], true);
        assert_eq!(json["current_version"], 1);
        assert_eq!(json["steps"][0]["status"], "action_required");
        assert_eq!(json["steps"][0]["detail"], "Model not downloaded");
    }

    #[test]
    fn claude_cli_info_roundtrip() {
        let info = ClaudeCliInfo {
            installed: true,
            version: Some("1.0.42".to_string()),
            path: Some("/usr/local/bin/claude".to_string()),
            authenticated: true,
            subscription_type: Some("pro".to_string()),
            rate_limit_tier: Some("default_claude_pro".to_string()),
            scopes: vec!["user:inference".to_string(), "user:profile".to_string()],
            expires_at: Some(1772671490973),
        };

        let json = serde_json::to_string(&info).expect("serialization should succeed");
        let deserialized: ClaudeCliInfo =
            serde_json::from_str(&json).expect("deserialization should succeed");

        assert!(deserialized.installed);
        assert_eq!(deserialized.version.as_deref(), Some("1.0.42"));
        assert_eq!(deserialized.path.as_deref(), Some("/usr/local/bin/claude"));
        assert!(deserialized.authenticated);
        assert_eq!(deserialized.subscription_type.as_deref(), Some("pro"));
        assert_eq!(
            deserialized.rate_limit_tier.as_deref(),
            Some("default_claude_pro")
        );
        assert_eq!(deserialized.scopes.len(), 2);
        assert_eq!(deserialized.expires_at, Some(1772671490973));
    }

    #[test]
    fn claude_cli_info_not_installed() {
        let info = ClaudeCliInfo {
            installed: false,
            version: None,
            path: None,
            authenticated: false,
            subscription_type: None,
            rate_limit_tier: None,
            scopes: Vec::new(),
            expires_at: None,
        };

        let json = serde_json::to_value(&info).expect("serialization should succeed");
        assert!(!json["installed"].as_bool().expect("should be bool"));
        assert!(json["version"].is_null());
        assert!(json["path"].is_null());
        assert!(!json["authenticated"].as_bool().expect("should be bool"));
        assert!(json["subscription_type"].is_null());
        assert!(json["rate_limit_tier"].is_null());
        assert!(json["scopes"]
            .as_array()
            .expect("should be array")
            .is_empty());
        assert!(json["expires_at"].is_null());
    }
}
