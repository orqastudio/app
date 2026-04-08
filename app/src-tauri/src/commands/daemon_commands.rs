// Daemon health check Tauri command.
//
// Routes the daemon health check through the Rust backend so the frontend
// does not need to make a direct `fetch()` call, which is blocked by Tauri's
// Content Security Policy inside the WebView.
//
// Uses `DaemonClient` from `crate::daemon_client` for the HTTP call.

use crate::daemon_client::DaemonHealthResponse;
use crate::error::OrqaError;
use crate::state::AppState;
use tauri::State;

/// Query the daemon's health endpoint and return the result.
///
/// Delegates to `DaemonClient::health()`. Returns the daemon's JSON response on
/// success, or an error if the daemon is unreachable or returns a non-200 status.
#[tauri::command]
pub async fn daemon_health(state: State<'_, AppState>) -> Result<DaemonHealthResponse, OrqaError> {
    match state.daemon.client.health().await {
        Ok(resp) => {
            tracing::debug!(
                status = %resp.status,
                artifacts = resp.artifact_count,
                rules = resp.rule_count,
                "daemon_health: ok"
            );
            Ok(resp)
        }
        Err(e) => {
            tracing::warn!(error = %e, "daemon_health: failed");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daemon_health_response_deserialization() {
        let json = r#"{"status": "ok", "artifact_count": 42, "rule_count": 7}"#;
        let resp: DaemonHealthResponse = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(resp.status, "ok");
        assert_eq!(resp.artifact_count, 42);
        assert_eq!(resp.rule_count, 7);
    }

    #[test]
    fn daemon_health_response_defaults() {
        let json = r#"{"status": "ok"}"#;
        let resp: DaemonHealthResponse =
            serde_json::from_str(json).expect("should deserialize with defaults");
        assert_eq!(resp.status, "ok");
        assert_eq!(resp.artifact_count, 0);
        assert_eq!(resp.rule_count, 0);
    }
}
