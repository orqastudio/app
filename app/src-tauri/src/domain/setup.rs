use serde::{Deserialize, Serialize};

use crate::error::OrqaError;

/// Parsed credential details from the Claude credentials file.
pub struct CredentialDetails {
    pub authenticated: bool,
    pub subscription_type: Option<String>,
    pub rate_limit_tier: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<u64>,
}

/// Check whether the Claude CLI is installed and retrieve version info.
///
/// Runs `claude --version` to detect installation. Attempts to locate
/// the binary path via `where` (Windows) or `which` (Unix).
pub fn check_claude_cli() -> Result<ClaudeCliInfo, OrqaError> {
    let version_output = std::process::Command::new("claude")
        .args(["--version"])
        .output();

    match version_output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path = resolve_cli_path();
            Ok(ClaudeCliInfo {
                installed: true,
                version: if version.is_empty() {
                    None
                } else {
                    Some(version)
                },
                path,
                authenticated: false,
                subscription_type: None,
                rate_limit_tier: None,
                scopes: Vec::new(),
                expires_at: None,
            })
        }
        _ => Ok(ClaudeCliInfo {
            installed: false,
            version: None,
            path: None,
            authenticated: false,
            subscription_type: None,
            rate_limit_tier: None,
            scopes: Vec::new(),
            expires_at: None,
        }),
    }
}

/// Best-effort attempt to find the `claude` binary path.
pub fn resolve_cli_path() -> Option<String> {
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("cmd")
        .args(["/c", "where", "claude"])
        .output();

    #[cfg(not(target_os = "windows"))]
    let result = std::process::Command::new("which").arg("claude").output();

    match result {
        Ok(output) if output.status.success() => {
            let path = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if path.is_empty() {
                None
            } else {
                Some(path)
            }
        }
        _ => None,
    }
}

/// Check whether the Claude CLI is authenticated.
///
/// Runs `claude --version` to confirm the CLI is installed, then reads
/// the credentials file to detect authentication and subscription details.
/// Claude Code requires login during installation, so a working CLI
/// with a credentials file is a strong indicator of authentication.
pub fn check_claude_auth() -> Result<ClaudeCliInfo, OrqaError> {
    let version_output = std::process::Command::new("claude")
        .args(["--version"])
        .output();

    match version_output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();

            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .unwrap_or_default();
            let cred_path = std::path::Path::new(&home)
                .join(".claude")
                .join(".credentials.json");

            let cred_details = parse_credentials(&cred_path);
            let path = resolve_cli_path();

            Ok(ClaudeCliInfo {
                installed: true,
                version: if version.is_empty() {
                    None
                } else {
                    Some(version)
                },
                path,
                authenticated: cred_details.authenticated,
                subscription_type: cred_details.subscription_type,
                rate_limit_tier: cred_details.rate_limit_tier,
                scopes: cred_details.scopes,
                expires_at: cred_details.expires_at,
            })
        }
        _ => Ok(ClaudeCliInfo {
            installed: false,
            version: None,
            path: None,
            authenticated: false,
            subscription_type: None,
            rate_limit_tier: None,
            scopes: Vec::new(),
            expires_at: None,
        }),
    }
}

/// Extract `CredentialDetails` from a parsed OAuth JSON node.
pub fn extract_oauth_details(oauth: &serde_json::Value) -> CredentialDetails {
    let subscription_type = oauth
        .get("subscriptionType")
        .and_then(|v| v.as_str())
        .map(String::from);

    let rate_limit_tier = oauth
        .get("rateLimitTier")
        .and_then(|v| v.as_str())
        .map(String::from);

    let scopes = oauth
        .get("scopes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let expires_at = oauth.get("expiresAt").and_then(serde_json::Value::as_u64);

    CredentialDetails {
        authenticated: true,
        subscription_type,
        rate_limit_tier,
        scopes,
        expires_at,
    }
}

/// Read the credentials file and extract authentication and subscription info.
///
/// If the file cannot be read or parsed, falls back to checking file existence.
pub fn parse_credentials(path: &std::path::Path) -> CredentialDetails {
    let not_found = CredentialDetails {
        authenticated: path.exists(),
        subscription_type: None,
        rate_limit_tier: None,
        scopes: Vec::new(),
        expires_at: None,
    };

    let Ok(contents) = std::fs::read_to_string(path) else {
        return not_found;
    };

    let json: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            return CredentialDetails {
                authenticated: true,
                ..not_found
            }
        }
    };

    // The credentials file nests auth details under "claudeAiOauth"
    let oauth = json.get("claudeAiOauth").unwrap_or(&json);
    extract_oauth_details(oauth)
}

/// Trigger the Claude CLI login flow.
///
/// Runs `claude login` which opens the browser for OAuth authentication.
/// After completion, re-reads credentials and returns updated info.
pub fn reauthenticate_claude() -> Result<ClaudeCliInfo, OrqaError> {
    let login_result = std::process::Command::new("claude")
        .args(["login"])
        .output();

    match login_result {
        Ok(output) if output.status.success() => check_claude_auth(),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(OrqaError::Sidecar(format!(
                "claude login failed: {}",
                if stderr.is_empty() {
                    "unknown error".to_string()
                } else {
                    stderr
                }
            )))
        }
        Err(e) => Err(OrqaError::Sidecar(format!(
            "failed to run claude login: {e}"
        ))),
    }
}

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
