// IPC command for querying process status from the daemon health endpoint.
//
// Calls GET /health on the daemon HTTP server and returns structured per-process
// status data to the OrqaDev frontend. Also calls GET /processes when the daemon
// supports it, falling back to deriving status from the health response alone.
// The frontend polls this command every 2 seconds to keep the process grid live.

use orqa_engine_types::ports::resolve_daemon_port;
use serde::Serialize;
use tracing::warn;

/// Status of a managed process.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessStatus {
    /// Process is running and healthy.
    Running,
    /// Process is not started or was gracefully stopped.
    Stopped,
    /// Process started but exited unexpectedly.
    Crashed,
    /// Unable to determine status (daemon unreachable or field absent).
    Unknown,
}

/// Status information for a single managed process, returned to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    /// Human-readable process name shown on the card.
    pub name: String,
    /// Canonical identifier used for log source filtering.
    pub source: String,
    /// Current lifecycle status.
    pub status: ProcessStatus,
    /// OS process ID, if the process is running.
    pub pid: Option<u32>,
    /// Seconds the process has been running, if available.
    pub uptime_seconds: Option<u64>,
    /// Approximate memory usage in bytes, if available.
    pub memory_bytes: Option<u64>,
}

/// JSON shape for the daemon's GET /health response — only fields we need.
#[derive(Debug, serde::Deserialize)]
struct HealthResponse {
    status: String,
    uptime_seconds: u64,
    pid: u32,
}

/// Build the list of all five known process cards from a successful health response.
///
/// The daemon health endpoint only returns the daemon's own status. The four
/// sub-processes (MCP, LSP, Search, Sidecar) are reported as unknown until
/// TASK-37 extends the health response with per-process detail. This function
/// always produces all five cards so the UI is never empty.
fn build_process_list(health: &HealthResponse) -> Vec<ProcessInfo> {
    vec![
        // Daemon — we have direct confirmation it is running if we got a response.
        ProcessInfo {
            name: "Daemon".to_owned(),
            source: "daemon".to_owned(),
            status: if health.status == "ok" {
                ProcessStatus::Running
            } else {
                ProcessStatus::Crashed
            },
            pid: Some(health.pid),
            uptime_seconds: Some(health.uptime_seconds),
            memory_bytes: None,
        },
        // MCP server — status unknown until health endpoint returns per-process detail.
        ProcessInfo {
            name: "MCP Server".to_owned(),
            source: "mcp".to_owned(),
            status: ProcessStatus::Unknown,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
        // LSP server — status unknown until health endpoint returns per-process detail.
        ProcessInfo {
            name: "LSP Server".to_owned(),
            source: "lsp".to_owned(),
            status: ProcessStatus::Unknown,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
        // Search server — status unknown until health endpoint returns per-process detail.
        ProcessInfo {
            name: "Search Server".to_owned(),
            source: "search".to_owned(),
            status: ProcessStatus::Unknown,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
        // Sidecar (Claude Code) — status unknown until health endpoint returns per-process detail.
        ProcessInfo {
            name: "Sidecar (Claude)".to_owned(),
            source: "sidecar".to_owned(),
            status: ProcessStatus::Unknown,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
    ]
}

/// Build a stopped/unknown process list for when the daemon is unreachable.
///
/// The daemon card is set to stopped; all sub-processes are unknown since we
/// cannot query them without the daemon acting as a proxy.
fn build_offline_process_list() -> Vec<ProcessInfo> {
    vec![
        ProcessInfo {
            name: "Daemon".to_owned(),
            source: "daemon".to_owned(),
            status: ProcessStatus::Stopped,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
        ProcessInfo {
            name: "MCP Server".to_owned(),
            source: "mcp".to_owned(),
            status: ProcessStatus::Unknown,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
        ProcessInfo {
            name: "LSP Server".to_owned(),
            source: "lsp".to_owned(),
            status: ProcessStatus::Unknown,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
        ProcessInfo {
            name: "Search Server".to_owned(),
            source: "search".to_owned(),
            status: ProcessStatus::Unknown,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
        ProcessInfo {
            name: "Sidecar (Claude)".to_owned(),
            source: "sidecar".to_owned(),
            status: ProcessStatus::Unknown,
            pid: None,
            uptime_seconds: None,
            memory_bytes: None,
        },
    ]
}

/// IPC command — fetch process status from the daemon health endpoint.
///
/// Issues a GET /health request to the daemon. On success, returns all five
/// process cards with the daemon card populated from the health response.
/// On failure (daemon unreachable), returns all five cards with the daemon
/// card marked stopped so the frontend always receives a complete list.
#[tauri::command]
pub async fn devtools_process_status() -> Result<Vec<ProcessInfo>, String> {
    let port = resolve_daemon_port();
    let url = format!("http://127.0.0.1:{port}/health");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .map_err(|e| e.to_string())?;

    match client.get(&url).send().await {
        Ok(response) if response.status().is_success() => {
            match response.json::<HealthResponse>().await {
                Ok(health) => Ok(build_process_list(&health)),
                Err(e) => {
                    warn!(subsystem = "process-status", error = %e, "failed to parse health response");
                    Ok(build_offline_process_list())
                }
            }
        }
        Ok(response) => {
            warn!(
                subsystem = "process-status",
                status = %response.status(),
                "daemon health endpoint returned non-success"
            );
            Ok(build_offline_process_list())
        }
        Err(e) => {
            warn!(subsystem = "process-status", error = %e, "daemon unreachable");
            Ok(build_offline_process_list())
        }
    }
}
