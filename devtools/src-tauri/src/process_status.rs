// IPC command for querying process status from the daemon health endpoint.
//
// Calls GET /health on the daemon HTTP server and reads the `processes` array
// from the response. The daemon event loop populates that array with a
// ProcessSnapshot for each managed subprocess on every 250 ms polling cycle.
//
// OrqaDev auto-discovers processes from the health response — there is no
// hardcoded process list here. New processes the daemon manages (e.g., a future
// ONNX server) appear in OrqaDev automatically without any frontend changes.
//
// The frontend polls this command every 2 seconds to keep the process grid live.

use orqa_engine_types::ports::resolve_daemon_port;
use serde::{Deserialize, Serialize};
use tracing::warn;

/// Status of a managed process, returned to the frontend.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessStatus {
    /// Process is running and healthy.
    Running,
    /// Process is not started or was gracefully stopped.
    Stopped,
    /// Process started but exited unexpectedly.
    Crashed,
    /// Binary not found — process was never started.
    #[serde(rename = "not_found")]
    NotFound,
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
    /// Absolute path to the binary that was spawned. Shown on card hover/expand.
    pub binary_path: Option<String>,
}

/// Per-process detail as returned in the daemon health response `processes` array.
///
/// The daemon serialises `ProcessSnapshot` from `subprocess.rs` into this shape.
/// Fields match exactly so deserialisation is direct.
#[derive(Debug, Deserialize)]
struct HealthProcessEntry {
    name: String,
    source: String,
    status: String,
    pid: Option<u32>,
    uptime_seconds: Option<u64>,
    binary_path: Option<String>,
}

/// JSON shape for the daemon's GET /health response — only fields we need.
#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
    uptime_seconds: u64,
    pid: u32,
    /// Per-process detail populated by the daemon event loop.
    /// Empty when the daemon has not yet completed its first polling cycle.
    #[serde(default)]
    processes: Vec<HealthProcessEntry>,
}

/// Convert a status string from the daemon into a `ProcessStatus` enum.
///
/// The daemon uses the strings "running", "stopped", "crashed", and "not_found".
/// Anything else is treated as `Unknown` so future daemon additions don't break
/// older OrqaDev builds.
fn parse_status(s: &str) -> ProcessStatus {
    match s {
        "running" => ProcessStatus::Running,
        "stopped" => ProcessStatus::Stopped,
        "crashed" => ProcessStatus::Crashed,
        "not_found" => ProcessStatus::NotFound,
        _ => ProcessStatus::Unknown,
    }
}

/// Build the daemon's own `ProcessInfo` card from the health response fields.
///
/// The daemon process is not a managed subprocess — it reports its own status
/// directly in the health response root fields. We synthesise a card for it
/// so the daemon always appears in the process grid.
fn daemon_card(health: &HealthResponse) -> ProcessInfo {
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
        binary_path: None,
    }
}

/// Build the daemon's own card when the daemon is unreachable.
fn daemon_offline_card() -> ProcessInfo {
    ProcessInfo {
        name: "Daemon".to_owned(),
        source: "daemon".to_owned(),
        status: ProcessStatus::Stopped,
        pid: None,
        uptime_seconds: None,
        memory_bytes: None,
        binary_path: None,
    }
}

/// Convert a `HealthProcessEntry` from the health response into a `ProcessInfo`.
///
/// `memory_bytes` is not yet tracked by the daemon — set to `None`.
fn entry_to_info(entry: HealthProcessEntry) -> ProcessInfo {
    ProcessInfo {
        name: entry.name,
        source: entry.source,
        status: parse_status(&entry.status),
        pid: entry.pid,
        uptime_seconds: entry.uptime_seconds,
        memory_bytes: None,
        binary_path: entry.binary_path,
    }
}

/// IPC command — fetch process status from the daemon health endpoint.
///
/// Issues a GET /health request to the daemon. On success, builds the process
/// list by prepending the daemon card (synthesised from health root fields)
/// followed by each entry in the `processes` array. This means new subprocesses
/// registered in the daemon appear in OrqaDev automatically — no hardcoded list.
///
/// On failure (daemon unreachable), returns only the daemon card in stopped
/// state so the UI always shows at least one card.
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
                Ok(health) => {
                    // Daemon card always comes first, then auto-discovered subprocesses.
                    let mut list = vec![daemon_card(&health)];
                    for entry in health.processes {
                        list.push(entry_to_info(entry));
                    }
                    Ok(list)
                }
                Err(e) => {
                    warn!(subsystem = "process-status", error = %e, "failed to parse health response");
                    Ok(vec![daemon_offline_card()])
                }
            }
        }
        Ok(response) => {
            warn!(
                subsystem = "process-status",
                status = %response.status(),
                "daemon health endpoint returned non-success"
            );
            Ok(vec![daemon_offline_card()])
        }
        Err(e) => {
            warn!(subsystem = "process-status", error = %e, "daemon unreachable");
            Ok(vec![daemon_offline_card()])
        }
    }
}
