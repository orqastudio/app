// Startup status route: report the overall daemon initialization state.
//
// The daemon tracks startup readiness through the graph and subprocess
// registry. This endpoint lets the frontend query whether the daemon is
// fully initialized and all managed processes are running.
//
// Endpoints:
//   GET /startup/status — get status of all startup tasks

use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::health::HealthState;

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

/// A single startup task and its current state.
#[derive(Debug, Serialize)]
pub struct StartupTask {
    /// Unique task identifier.
    pub id: String,
    /// Human-readable description.
    pub label: String,
    /// One of: "pending", "running", "complete", "failed".
    pub status: &'static str,
    /// Optional detail message (e.g. error description).
    pub detail: Option<String>,
}

/// Snapshot of all daemon startup tasks.
#[derive(Debug, Serialize)]
pub struct StartupSnapshot {
    /// All registered startup tasks with their current status.
    pub tasks: Vec<StartupTask>,
    /// True when all tasks have reached "complete" status.
    pub all_done: bool,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Handle GET /startup/status — return a snapshot of all daemon startup tasks.
///
/// Reports the state of the artifact graph (loaded/empty) and all managed
/// subprocesses. The frontend uses this to show a loading screen until the
/// daemon is fully ready.
pub async fn get_startup_status(
    State(state): State<HealthState>,
) -> Json<StartupSnapshot> {
    let mut tasks = Vec::new();

    // Graph loading task: report ready when artifact count > 0 or the graph
    // state is accessible (daemon has started, graph is at least empty).
    let graph_ok = state.graph_state.0.read().is_ok();
    let artifact_count = state.graph_state.artifact_count();
    tasks.push(StartupTask {
        id: "graph".to_owned(),
        label: "Artifact Graph".to_owned(),
        status: if graph_ok { "complete" } else { "failed" },
        detail: if graph_ok {
            Some(format!("{artifact_count} artifacts loaded"))
        } else {
            Some("graph state unavailable".to_owned())
        },
    });

    // Subprocess tasks: one per managed process snapshot.
    let snapshots = state
        .process_snapshots
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default();

    for snap in &snapshots {
        let (status, detail) = match snap.status.as_str() {
            "running" => ("complete", None),
            "stopped" => ("pending", None),
            "crashed" => ("failed", Some(format!("process {} crashed", snap.name))),
            "binary_not_found" => ("failed", Some(format!("{} binary not found", snap.name))),
            other => ("pending", Some(other.to_owned())),
        };
        tasks.push(StartupTask {
            id: snap.name.clone(),
            label: snap.name.clone(),
            status,
            detail,
        });
    }

    let all_done = tasks.iter().all(|t| t.status == "complete");

    Json(StartupSnapshot { tasks, all_done })
}
