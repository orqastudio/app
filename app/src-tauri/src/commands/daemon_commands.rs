//! Daemon health check Tauri command.
//!
//! Routes the daemon health check through the Rust backend so the frontend
//! does not need to make a direct `fetch()` call, which is blocked by Tauri's
//! Content Security Policy inside the WebView.

use orqa_engine::ports::resolve_daemon_port;

use crate::error::OrqaError;

/// Response shape returned by the daemon's `/health` endpoint.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct DaemonHealthResponse {
    /// Health status string (e.g., "ok").
    pub status: String,
    /// Number of governance artifacts currently indexed.
    #[serde(default)]
    pub artifacts: u64,
    /// Number of enforcement rules currently loaded.
    #[serde(default)]
    pub rules: u64,
}

/// Query the daemon's health endpoint and return the result.
///
/// Makes an HTTP GET to the daemon's `/health` endpoint with a 3-second timeout.
/// The daemon port is resolved via `orqa_engine::ports::resolve_daemon_port()`
/// (ORQA_PORT_BASE, default 10100). Returns the daemon's JSON response on
/// success, or an error if the daemon is unreachable or returns a non-200 status.
#[tauri::command]
pub async fn daemon_health() -> Result<DaemonHealthResponse, OrqaError> {
    let daemon_port = resolve_daemon_port();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|e| OrqaError::Sidecar(format!("failed to build HTTP client: {e}")))?;

    let response = client
        .get(format!("http://127.0.0.1:{daemon_port}/health"))
        .send()
        .await
        .map_err(|e| OrqaError::Sidecar(format!("daemon unreachable: {e}")))?;

    if !response.status().is_success() {
        return Err(OrqaError::Sidecar(format!(
            "daemon returned HTTP {}",
            response.status()
        )));
    }

    let data: DaemonHealthResponse = response
        .json()
        .await
        .map_err(|e| OrqaError::Sidecar(format!("failed to parse daemon response: {e}")))?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daemon_health_response_deserialization() {
        let json = r#"{"status": "ok", "artifacts": 42, "rules": 7}"#;
        let resp: DaemonHealthResponse = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(resp.status, "ok");
        assert_eq!(resp.artifacts, 42);
        assert_eq!(resp.rules, 7);
    }

    #[test]
    fn daemon_health_response_defaults() {
        let json = r#"{"status": "ok"}"#;
        let resp: DaemonHealthResponse =
            serde_json::from_str(json).expect("should deserialize with defaults");
        assert_eq!(resp.status, "ok");
        assert_eq!(resp.artifacts, 0);
        assert_eq!(resp.rules, 0);
    }
}
