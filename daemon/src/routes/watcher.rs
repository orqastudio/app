// Watcher pause/resume control routes.
//
// Provides two endpoints for external callers to pause and resume the file
// watcher during operations that must not trigger incremental SurrealDB syncs
// (e.g. bulk migration ingestion). The pause/resume state is held in a shared
// `WatcherControl` and consulted by the debouncer callback before dispatching
// any sync work.
//
// Endpoints:
//   POST /watcher/pause   — pause event emission; idempotent (already-paused is 200)
//   POST /watcher/resume  — resume event emission; idempotent (already-running is 200)
//
// Daemon restart always puts the watcher in `Running` — `WatcherControl::new()`
// initialises to `Running` so no persistent state survives a restart.

use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// Watcher state machine
// ---------------------------------------------------------------------------

/// Whether the watcher is currently emitting events.
///
/// `Paused` suppresses all sync work triggered by file-change events. The
/// watcher OS machinery stays alive so no events are lost at the OS level;
/// they are simply not acted on while paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherRunState {
    Running,
    Paused,
}

/// Shared watcher control handle.
///
/// A clone of this value is held by the debouncer callback and by the route
/// handlers. The inner `Mutex<WatcherRunState>` is always lock-held only for
/// the duration of a single field read or write — never across await points.
#[derive(Clone)]
pub struct WatcherControl(pub Arc<Mutex<WatcherRunState>>);

impl Default for WatcherControl {
    fn default() -> Self {
        Self::new()
    }
}

impl WatcherControl {
    /// Create a new `WatcherControl` initialised to `Running`.
    ///
    /// Daemon restart always boots the watcher in `Running` — calling code
    /// must never persist the paused state across process boundaries.
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(WatcherRunState::Running)))
    }

    /// Return `true` when the watcher is in `Running` state.
    ///
    /// Called by the debouncer callback to gate sync work. Returns `true` (allow)
    /// when the lock is poisoned so a lock failure does not silently break syncing.
    pub fn is_running(&self) -> bool {
        self.0
            .lock()
            .map(|g| *g == WatcherRunState::Running)
            .unwrap_or(true)
    }

    /// Transition to `Paused`. Idempotent — already-paused is a no-op.
    pub fn pause(&self) {
        if let Ok(mut g) = self.0.lock() {
            *g = WatcherRunState::Paused;
        }
    }

    /// Transition to `Running`. Idempotent — already-running is a no-op.
    pub fn resume(&self) {
        if let Ok(mut g) = self.0.lock() {
            *g = WatcherRunState::Running;
        }
    }
}

// ---------------------------------------------------------------------------
// Route handlers
// ---------------------------------------------------------------------------

/// JSON response body for pause/resume endpoints.
#[derive(Serialize)]
pub struct WatcherStateResponse {
    /// Current state after the operation: "running" or "paused".
    pub state: &'static str,
}

/// Handle POST /watcher/pause — pause file-watcher event emission.
///
/// Idempotent: calling pause when already paused returns 200 with `"paused"`.
/// The operation is instantaneous; in-flight sync work that started before the
/// pause call completes normally.
pub async fn pause_watcher(
    axum::extract::State(control): axum::extract::State<WatcherControl>,
) -> (StatusCode, Json<WatcherStateResponse>) {
    control.pause();
    (
        StatusCode::OK,
        Json(WatcherStateResponse { state: "paused" }),
    )
}

/// Handle POST /watcher/resume — resume file-watcher event emission.
///
/// Idempotent: calling resume when already running returns 200 with `"running"`.
/// File changes that occurred while paused are not replayed — the ingest caller
/// is responsible for ensuring consistency via its own mechanisms.
pub async fn resume_watcher(
    axum::extract::State(control): axum::extract::State<WatcherControl>,
) -> (StatusCode, Json<WatcherStateResponse>) {
    control.resume();
    (
        StatusCode::OK,
        Json(WatcherStateResponse { state: "running" }),
    )
}

/// Handle GET /watcher/status — return current watcher state.
///
/// Used by integration tests to verify the watcher is in the expected state
/// without triggering a state change.
pub async fn watcher_status(
    axum::extract::State(control): axum::extract::State<WatcherControl>,
) -> Json<WatcherStateResponse> {
    let state = if control.is_running() {
        "running"
    } else {
        "paused"
    };
    Json(WatcherStateResponse { state })
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_control_is_running() {
        let ctrl = WatcherControl::new();
        assert!(ctrl.is_running(), "new control must start in Running state");
    }

    #[test]
    fn pause_sets_paused() {
        let ctrl = WatcherControl::new();
        ctrl.pause();
        assert!(!ctrl.is_running(), "after pause, is_running must be false");
    }

    #[test]
    fn resume_restores_running() {
        let ctrl = WatcherControl::new();
        ctrl.pause();
        ctrl.resume();
        assert!(ctrl.is_running(), "after resume, is_running must be true");
    }

    #[test]
    fn pause_is_idempotent() {
        let ctrl = WatcherControl::new();
        ctrl.pause();
        ctrl.pause(); // second pause is a no-op
        assert!(!ctrl.is_running(), "double-pause must still be paused");
    }

    #[test]
    fn resume_is_idempotent() {
        let ctrl = WatcherControl::new();
        ctrl.resume(); // already running
        assert!(ctrl.is_running(), "resume-when-running must stay running");
    }
}
