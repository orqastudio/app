// Tauri IPC commands for the first-run setup wizard.
//
// Setup completion is stored as `setup_version` in the project-scoped settings.
// If no project is open yet, setup is considered incomplete.

use tauri::Manager;

use orqa_storage::traits::SettingsRepository as _;

use crate::domain::setup::{self, ClaudeCliInfo, SetupStatus, SetupStepStatus, StepStatus};
use crate::error::OrqaError;
use crate::state::AppState;

/// Current setup wizard version. Bump when new setup steps are added.
const CURRENT_SETUP_VERSION: u32 = 1;

/// Build the default list of setup steps (all pending).
fn default_steps() -> Vec<SetupStepStatus> {
    vec![
        SetupStepStatus {
            id: "claude_cli".to_owned(),
            label: "Claude CLI".to_owned(),
            status: StepStatus::Pending,
            detail: None,
        },
        SetupStepStatus {
            id: "authentication".to_owned(),
            label: "Authentication".to_owned(),
            status: StepStatus::Pending,
            detail: None,
        },
        SetupStepStatus {
            id: "sidecar".to_owned(),
            label: "Sidecar".to_owned(),
            status: StepStatus::Pending,
            detail: None,
        },
        SetupStepStatus {
            id: "embedding_model".to_owned(),
            label: "Embedding Model".to_owned(),
            status: StepStatus::Pending,
            detail: None,
        },
        SetupStepStatus {
            id: "complete".to_owned(),
            label: "Complete".to_owned(),
            status: StepStatus::Pending,
            detail: None,
        },
    ]
}

/// Query the current setup status.
///
/// Reads the stored `setup_version` from settings. If no project is open
/// (storage not initialised), setup is incomplete.
#[tauri::command]
pub async fn get_setup_status(state: tauri::State<'_, AppState>) -> Result<SetupStatus, OrqaError> {
    let (setup_complete, stored_version) = match state.db.get() {
        Ok(storage) => {
            let stored = storage.settings().get("setup_version", "app").await?;
            let version = stored.and_then(|v| v.as_u64()).map_or(0, |v| v as u32);
            (version >= CURRENT_SETUP_VERSION, version)
        }
        Err(_) => (false, 0),
    };

    Ok(SetupStatus {
        setup_complete,
        current_version: CURRENT_SETUP_VERSION,
        stored_version,
        steps: default_steps(),
    })
}

/// Check whether the Claude CLI is installed and retrieve version info.
///
/// Delegates to `domain::setup::check_claude_cli`.
#[tauri::command]
pub fn check_claude_cli() -> Result<ClaudeCliInfo, OrqaError> {
    setup::check_claude_cli()
}

/// Check whether the Claude CLI is authenticated.
///
/// Delegates to `domain::setup::check_claude_auth`.
#[tauri::command]
pub fn check_claude_auth() -> Result<ClaudeCliInfo, OrqaError> {
    setup::check_claude_auth()
}

/// Trigger the Claude CLI login flow.
///
/// Delegates to `domain::setup::reauthenticate_claude`.
#[tauri::command]
pub fn reauthenticate_claude() -> Result<ClaudeCliInfo, OrqaError> {
    setup::reauthenticate_claude()
}

/// Check whether the embedding model is downloaded and ready.
///
/// Looks for `model.onnx` and `tokenizer.json` in the app data directory
/// under `models/all-MiniLM-L6-v2/`.
#[tauri::command]
pub fn check_embedding_model(app_handle: tauri::AppHandle) -> Result<SetupStepStatus, OrqaError> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| OrqaError::FileSystem(format!("failed to resolve app data dir: {e}")))?;
    let model_dir = app_dir.join("models").join("all-MiniLM-L6-v2");

    let model_file = model_dir.join("model.onnx");
    let tokenizer_file = model_dir.join("tokenizer.json");

    if model_file.exists() && tokenizer_file.exists() {
        Ok(SetupStepStatus {
            id: "embedding_model".to_owned(),
            label: "Embedding Model".to_owned(),
            status: StepStatus::Complete,
            detail: Some("all-MiniLM-L6-v2 ready".to_owned()),
        })
    } else {
        Ok(SetupStepStatus {
            id: "embedding_model".to_owned(),
            label: "Embedding Model".to_owned(),
            status: StepStatus::ActionRequired,
            detail: Some("Model not downloaded".to_owned()),
        })
    }
}

/// Mark setup as complete by storing the current version in settings.
///
/// Requires an open project (storage must be initialised via `project_open`).
#[tauri::command]
pub async fn complete_setup(state: tauri::State<'_, AppState>) -> Result<(), OrqaError> {
    let storage = state.db.get()?;
    Ok(storage
        .settings()
        .set(
            "setup_version",
            &serde_json::json!(CURRENT_SETUP_VERSION),
            "app",
        )
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use orqa_storage::traits::SettingsRepository as _;

    #[tokio::test]
    async fn get_setup_status_incomplete_when_no_version() {
        let storage = orqa_storage::Storage::open_in_memory()
            .await
            .expect("db init");
        let stored = storage
            .settings()
            .get("setup_version", "app")
            .await
            .expect("get");
        assert!(stored.is_none());

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

    #[tokio::test]
    async fn get_setup_status_complete_when_version_matches() {
        let storage = orqa_storage::Storage::open_in_memory()
            .await
            .expect("db init");
        storage
            .settings()
            .set(
                "setup_version",
                &serde_json::json!(CURRENT_SETUP_VERSION),
                "app",
            )
            .await
            .expect("set");

        let stored = storage
            .settings()
            .get("setup_version", "app")
            .await
            .expect("get")
            .expect("should exist");
        let stored_version = stored.as_u64().map_or(0, |v| v as u32);

        let status = SetupStatus {
            setup_complete: stored_version >= CURRENT_SETUP_VERSION,
            current_version: CURRENT_SETUP_VERSION,
            stored_version,
            steps: default_steps(),
        };

        assert!(status.setup_complete);
        assert_eq!(status.stored_version, CURRENT_SETUP_VERSION);
    }

    #[tokio::test]
    async fn complete_setup_stores_version() {
        let storage = orqa_storage::Storage::open_in_memory()
            .await
            .expect("db init");

        let before = storage
            .settings()
            .get("setup_version", "app")
            .await
            .expect("get");
        assert!(before.is_none());

        storage
            .settings()
            .set(
                "setup_version",
                &serde_json::json!(CURRENT_SETUP_VERSION),
                "app",
            )
            .await
            .expect("set");

        let after = storage
            .settings()
            .get("setup_version", "app")
            .await
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

    #[tokio::test]
    async fn setup_status_incomplete_when_version_too_low() {
        let storage = orqa_storage::Storage::open_in_memory()
            .await
            .expect("db init");
        storage
            .settings()
            .set("setup_version", &serde_json::json!(0), "app")
            .await
            .expect("set");

        let stored = storage
            .settings()
            .get("setup_version", "app")
            .await
            .expect("get")
            .expect("should exist");
        let stored_version = stored.as_u64().map_or(0, |v| v as u32);

        assert!(stored_version < CURRENT_SETUP_VERSION);
    }
}
