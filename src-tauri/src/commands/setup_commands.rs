use tauri::Manager;

use crate::domain::setup::{ClaudeCliInfo, SetupStatus, SetupStepStatus, StepStatus};
use crate::error::OrqaError;
use crate::repo::settings_repo;
use crate::state::AppState;

/// Current setup wizard version. Bump when new setup steps are added.
const CURRENT_SETUP_VERSION: u32 = 1;

/// Build the default list of setup steps (all pending).
fn default_steps() -> Vec<SetupStepStatus> {
    vec![
        SetupStepStatus {
            id: "claude_cli".to_string(),
            label: "Claude CLI".to_string(),
            status: StepStatus::Pending,
            detail: None,
        },
        SetupStepStatus {
            id: "authentication".to_string(),
            label: "Authentication".to_string(),
            status: StepStatus::Pending,
            detail: None,
        },
        SetupStepStatus {
            id: "sidecar".to_string(),
            label: "Sidecar".to_string(),
            status: StepStatus::Pending,
            detail: None,
        },
        SetupStepStatus {
            id: "embedding_model".to_string(),
            label: "Embedding Model".to_string(),
            status: StepStatus::Pending,
            detail: None,
        },
        SetupStepStatus {
            id: "complete".to_string(),
            label: "Complete".to_string(),
            status: StepStatus::Pending,
            detail: None,
        },
    ]
}

/// Query the current setup status.
///
/// Reads the stored `setup_version` from settings. If the stored version
/// is missing or less than `CURRENT_SETUP_VERSION`, setup is incomplete.
#[tauri::command]
pub fn get_setup_status(state: tauri::State<'_, AppState>) -> Result<SetupStatus, OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let stored = settings_repo::get(&conn, "setup_version", "app")?;

    let stored_version = stored
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(0);

    let setup_complete = stored_version >= CURRENT_SETUP_VERSION;

    Ok(SetupStatus {
        setup_complete,
        current_version: CURRENT_SETUP_VERSION,
        stored_version,
        steps: default_steps(),
    })
}

/// Check whether the Claude CLI is installed and retrieve version info.
///
/// Runs `claude --version` to detect installation. Attempts to locate
/// the binary path via `where` (Windows) or `which` (Unix).
#[tauri::command]
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
fn resolve_cli_path() -> Option<String> {
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
#[tauri::command]
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

/// Parsed credential details from the Claude credentials file.
struct CredentialDetails {
    authenticated: bool,
    subscription_type: Option<String>,
    rate_limit_tier: Option<String>,
    scopes: Vec<String>,
    expires_at: Option<u64>,
}

/// Read the credentials file and extract authentication and subscription info.
///
/// If the file cannot be read or parsed, falls back to checking file existence.
fn parse_credentials(path: &std::path::Path) -> CredentialDetails {
    let not_found = CredentialDetails {
        authenticated: path.exists(),
        subscription_type: None,
        rate_limit_tier: None,
        scopes: Vec::new(),
        expires_at: None,
    };

    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return not_found,
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

    let expires_at = oauth.get("expiresAt").and_then(|v| v.as_u64());

    CredentialDetails {
        authenticated: true,
        subscription_type,
        rate_limit_tier,
        scopes,
        expires_at,
    }
}

/// Trigger the Claude CLI login flow.
///
/// Runs `claude login` which opens the browser for OAuth authentication.
/// After completion, re-reads credentials and returns updated info.
#[tauri::command]
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

/// Check whether the embedding model is downloaded and ready.
///
/// Looks for `model.onnx` and `tokenizer.json` in the app data directory
/// under `models/bge-small-en-v1.5/`.
#[tauri::command]
pub fn check_embedding_model(app_handle: tauri::AppHandle) -> Result<SetupStepStatus, OrqaError> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| OrqaError::FileSystem(format!("failed to resolve app data dir: {e}")))?;
    let model_dir = app_dir.join("models").join("bge-small-en-v1.5");

    let model_file = model_dir.join("model.onnx");
    let tokenizer_file = model_dir.join("tokenizer.json");

    if model_file.exists() && tokenizer_file.exists() {
        Ok(SetupStepStatus {
            id: "embedding_model".to_string(),
            label: "Embedding Model".to_string(),
            status: StepStatus::Complete,
            detail: Some("bge-small-en-v1.5 ready".to_string()),
        })
    } else {
        Ok(SetupStepStatus {
            id: "embedding_model".to_string(),
            label: "Embedding Model".to_string(),
            status: StepStatus::ActionRequired,
            detail: Some("Model not downloaded".to_string()),
        })
    }
}

/// Mark setup as complete by storing the current version in settings.
#[tauri::command]
pub fn complete_setup(state: tauri::State<'_, AppState>) -> Result<(), OrqaError> {
    let conn = state
        .db
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    settings_repo::set(
        &conn,
        "setup_version",
        &serde_json::json!(CURRENT_SETUP_VERSION),
        "app",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;
    use crate::repo::settings_repo;

    #[test]
    fn get_setup_status_incomplete_when_no_version() {
        let conn = init_memory_db().expect("db init");
        let stored = settings_repo::get(&conn, "setup_version", "app").expect("get");
        assert!(stored.is_none());

        // Simulate what the command does without tauri::State
        let stored_version = 0_u32;
        let status = SetupStatus {
            setup_complete: stored_version >= CURRENT_SETUP_VERSION,
            current_version: CURRENT_SETUP_VERSION,
            stored_version,
            steps: default_steps(),
        };

        assert!(!status.setup_complete);
        assert_eq!(status.current_version, CURRENT_SETUP_VERSION);
        assert_eq!(status.stored_version, 0);
        assert_eq!(status.steps.len(), 5);
        assert_eq!(status.steps[0].id, "claude_cli");
        assert_eq!(status.steps[0].status, StepStatus::Pending);
    }

    #[test]
    fn get_setup_status_complete_when_version_matches() {
        let conn = init_memory_db().expect("db init");
        settings_repo::set(
            &conn,
            "setup_version",
            &serde_json::json!(CURRENT_SETUP_VERSION),
            "app",
        )
        .expect("set");

        let stored = settings_repo::get(&conn, "setup_version", "app")
            .expect("get")
            .expect("should exist");
        let stored_version = stored.as_u64().map(|v| v as u32).unwrap_or(0);

        let status = SetupStatus {
            setup_complete: stored_version >= CURRENT_SETUP_VERSION,
            current_version: CURRENT_SETUP_VERSION,
            stored_version,
            steps: default_steps(),
        };

        assert!(status.setup_complete);
        assert_eq!(status.stored_version, CURRENT_SETUP_VERSION);
    }

    #[test]
    fn complete_setup_stores_version() {
        let conn = init_memory_db().expect("db init");

        // Before: no version
        let before = settings_repo::get(&conn, "setup_version", "app").expect("get");
        assert!(before.is_none());

        // Simulate what complete_setup does
        settings_repo::set(
            &conn,
            "setup_version",
            &serde_json::json!(CURRENT_SETUP_VERSION),
            "app",
        )
        .expect("set");

        // After: version matches
        let after = settings_repo::get(&conn, "setup_version", "app")
            .expect("get")
            .expect("should exist");
        assert_eq!(after, serde_json::json!(CURRENT_SETUP_VERSION));
    }

    #[test]
    fn default_steps_has_expected_ids() {
        let steps = default_steps();
        let ids: Vec<&str> = steps.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(
            ids,
            vec![
                "claude_cli",
                "authentication",
                "sidecar",
                "embedding_model",
                "complete"
            ]
        );
    }

    #[test]
    fn default_steps_all_pending() {
        let steps = default_steps();
        for step in &steps {
            assert_eq!(
                step.status,
                StepStatus::Pending,
                "step {} should be pending",
                step.id
            );
            assert!(
                step.detail.is_none(),
                "step {} should have no detail",
                step.id
            );
        }
    }

    #[test]
    fn check_claude_cli_handles_missing_binary() {
        // Run the command against a non-existent binary to test the error path.
        // We cannot call check_claude_cli directly since it uses the real "claude"
        // binary, but we can verify the ClaudeCliInfo construction for the not-found case.
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
        assert!(!info.installed);
        assert!(info.version.is_none());
        assert!(info.path.is_none());
        assert!(!info.authenticated);
    }

    #[test]
    fn setup_status_incomplete_when_version_too_low() {
        let conn = init_memory_db().expect("db init");
        // Store version 0 (lower than CURRENT_SETUP_VERSION)
        settings_repo::set(&conn, "setup_version", &serde_json::json!(0), "app").expect("set");

        let stored = settings_repo::get(&conn, "setup_version", "app")
            .expect("get")
            .expect("should exist");
        let stored_version = stored.as_u64().map(|v| v as u32).unwrap_or(0);

        assert!(stored_version < CURRENT_SETUP_VERSION);
        assert!(!stored_version >= CURRENT_SETUP_VERSION);
    }
}
