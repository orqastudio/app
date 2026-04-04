// Setup wizard routes: check environment prerequisites and mark setup complete.
//
// These routes let the frontend wizard guide users through the initial
// setup steps: Claude CLI installation, authentication, and embedding
// model availability. Setup completion state is stored in the unified SQLite
// settings table under the key "setup_version" in scope "app".
//
// Endpoints:
//   GET  /setup/status           — get overall setup completion status
//   GET  /setup/claude-cli       — check Claude CLI installation
//   GET  /setup/claude-auth      — check Claude authentication state
//   POST /setup/claude-reauth    — trigger Claude CLI re-authentication
//   GET  /setup/embedding-model  — check embedding model availability
//   POST /setup/complete         — mark setup as complete

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Setup versioning
// ---------------------------------------------------------------------------

/// Current setup wizard version. Bump when new required setup steps are added.
const CURRENT_SETUP_VERSION: u64 = 1;

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

/// Status of a single setup step.
#[derive(Debug, Serialize)]
pub struct SetupStepStatus {
    pub id: &'static str,
    pub label: &'static str,
    pub status: &'static str,
    pub detail: Option<String>,
}

/// Overall setup status returned by GET /setup/status.
#[derive(Debug, Serialize)]
pub struct SetupStatus {
    pub setup_complete: bool,
    pub current_version: u64,
    pub stored_version: u64,
    pub steps: Vec<SetupStepStatus>,
}

/// Information about the Claude CLI installation and authentication state.
#[derive(Debug, Serialize)]
pub struct ClaudeCliInfo {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub authenticated: bool,
}

/// Response helper when the storage is unavailable.
fn storage_unavailable() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": "setup store unavailable",
            "code": "STORE_UNAVAILABLE"
        })),
    )
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Attempt to locate the `claude` binary via `where` (Windows) or `which` (Unix).
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
                .to_owned();
            if path.is_empty() { None } else { Some(path) }
        }
        _ => None,
    }
}

/// Run `claude --version` and return version string on success.
fn probe_claude_version() -> Option<String> {
    std::process::Command::new("claude")
        .args(["--version"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .filter(|v| !v.is_empty())
}

/// Check whether the Claude credentials file exists with a non-empty API key or token.
///
/// A credentials file present in the home directory is treated as an indicator
/// of prior authentication. This is heuristic-only and does not perform a live
/// API check.
fn is_claude_authenticated() -> bool {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default();
    if home.is_empty() {
        return false;
    }

    // Claude Code stores credentials at ~/.claude/.credentials.json
    let creds = std::path::Path::new(&home)
        .join(".claude")
        .join(".credentials.json");

    creds.exists()
        && std::fs::metadata(&creds).map(|m| m.len() > 2).unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Handle GET /setup/status — return overall setup completion state.
///
/// Reads the stored setup_version from the settings table. Returns all
/// default steps as "pending" (detailed step checks run as separate endpoints).
pub async fn get_setup_status(
    State(state): State<HealthState>,
) -> Result<Json<SetupStatus>, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    tokio::task::spawn_blocking(move || {
        let stored_version = storage.settings()
            .get("setup_version", "app")
            .ok()
            .flatten()
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let steps = vec![
            SetupStepStatus { id: "claude_cli", label: "Claude CLI", status: "pending", detail: None },
            SetupStepStatus { id: "authentication", label: "Authentication", status: "pending", detail: None },
            SetupStepStatus { id: "embedding_model", label: "Embedding Model", status: "pending", detail: None },
        ];

        Ok(Json(SetupStatus {
            setup_complete: stored_version >= CURRENT_SETUP_VERSION,
            current_version: CURRENT_SETUP_VERSION,
            stored_version,
            steps,
        }))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}

/// Handle GET /setup/claude-cli — check Claude CLI installation.
///
/// Runs `claude --version` to detect installation and resolves the binary path.
pub async fn check_claude_cli() -> Json<ClaudeCliInfo> {
    let version = probe_claude_version();
    let path = resolve_cli_path();
    Json(ClaudeCliInfo {
        installed: version.is_some(),
        version,
        path,
        authenticated: false,
    })
}

/// Handle GET /setup/claude-auth — check Claude CLI authentication.
///
/// Checks for the credentials file in the user's home directory as a proxy
/// for authentication state.
pub async fn check_claude_auth() -> Json<ClaudeCliInfo> {
    let version = probe_claude_version();
    let path = resolve_cli_path();
    let authenticated = is_claude_authenticated();
    Json(ClaudeCliInfo {
        installed: version.is_some(),
        version,
        path,
        authenticated,
    })
}

/// Handle POST /setup/claude-reauth — trigger Claude CLI re-authentication.
///
/// Spawns `claude login` in a detached process. Returns 202 Accepted; the
/// login flow runs in the background and the user completes it in a terminal.
pub async fn reauthenticate_claude() -> (StatusCode, Json<serde_json::Value>) {
    let spawned = std::process::Command::new("claude")
        .args(["login"])
        .spawn()
        .is_ok();

    if spawned {
        (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({ "status": "login_started" })),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "failed to spawn claude login",
                "code": "SPAWN_FAILED"
            })),
        )
    }
}

/// Handle GET /setup/embedding-model — check embedding model availability.
///
/// Looks for the ONNX model files in the standard data directory under
/// `~/.local/share/orqa/models/` (Linux/Mac) or `%APPDATA%\orqa\models\`
/// (Windows). Returns whether the model is ready for use by the search engine.
pub async fn check_embedding_model() -> Json<serde_json::Value> {
    let model_ready = tokio::task::spawn_blocking(|| {
        // Resolve the base models directory from HOME / APPDATA.
        let base = std::env::var("APPDATA")
            .map(|d| std::path::PathBuf::from(d).join("orqa").join("models"))
            .or_else(|_| {
                std::env::var("HOME").map(|h| {
                    std::path::PathBuf::from(h)
                        .join(".local/share")
                        .join("orqa")
                        .join("models")
                })
            });

        if let Ok(model_dir) = base {
            let model_file = model_dir.join("model.onnx");
            let tokenizer_file = model_dir.join("tokenizer.json");
            return model_file.exists() && tokenizer_file.exists();
        }
        false
    })
    .await
    .unwrap_or(false);

    Json(serde_json::json!({
        "id": "embedding_model",
        "available": model_ready,
        "detail": if model_ready { "model files present" } else { "model files not found" }
    }))
}

/// Handle POST /setup/complete — mark the setup wizard as complete.
///
/// Stores the current setup version number in the settings table. Returns 204.
pub async fn complete_setup(
    State(state): State<HealthState>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let storage = state.storage.clone().ok_or_else(storage_unavailable)?;

    tokio::task::spawn_blocking(move || {
        storage.settings()
            .set("setup_version", &serde_json::json!(CURRENT_SETUP_VERSION), "app")
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(|e| (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string(), "code": "SETTINGS_SAVE_FAILED" })),
            ))
    })
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": e.to_string(), "code": "TASK_PANIC" })),
    ))?
}
