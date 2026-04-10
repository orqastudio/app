// Application state managed by Tauri.
//
// The app is a pure HTTP client — all storage operations go through the daemon
// via libs/db. There is no direct SQLite access in the app process.

use std::sync::Arc;

use orqa_db::DbClient;

use crate::daemon_client::DaemonClient;
use crate::startup::StartupTracker;
use crate::watcher::SharedWatcher;

// ---------------------------------------------------------------------------
// Sub-structs — each groups a logically related slice of application state.
// ---------------------------------------------------------------------------

/// Typed HTTP client for all daemon storage API calls.
///
/// Wraps `orqa_db::DbClient` so commands can call `state.db.projects().list()`
/// etc. without opening SQLite directly. The client is cheap to use — the
/// underlying `reqwest::Client` is `Arc`-backed and connection-pooled.
pub struct DbClientState {
    /// Pre-configured client pointing at the daemon's storage API.
    pub client: DbClient,
}

/// Long-running initialization task tracking.
///
/// The `StartupTracker` tracks long-running initialization tasks for the frontend.
pub struct StartupState {
    /// Shared reference to the startup tracker, shared with background init tasks.
    pub tracker: Arc<StartupTracker>,
}

/// Daemon HTTP client state.
///
/// Holds a `DaemonClient` that all graph, validation, artifact, and stream
/// commands use to delegate requests to the daemon. The client is cheap to
/// clone (it wraps `reqwest::Client` which is `Arc`-backed internally).
pub struct DaemonState {
    /// HTTP client for all daemon API calls.
    pub client: DaemonClient,
}

/// Artifact filesystem watcher state.
///
/// The artifact graph is owned by the daemon — the app holds only the file
/// watcher so the frontend receives change notifications when `.orqa/` changes.
pub struct ArtifactState {
    /// Active `.orqa/` file-system watcher.
    ///
    /// Replaced via `artifact_watch_start` whenever a different project is opened.
    /// Dropping the inner value stops the underlying watcher.
    pub watcher: SharedWatcher,
}

// ---------------------------------------------------------------------------
// Top-level application state
// ---------------------------------------------------------------------------

/// Application state managed by Tauri.
///
/// Decomposed into logical sub-structs for clarity and reduced lock contention.
/// All storage operations go through `db` (HTTP to daemon). All graph/validation
/// operations go through `daemon` (HTTP to daemon). The app process holds no
/// SQLite connection.
pub struct AppState {
    /// HTTP client for daemon storage API calls (sessions, messages, projects, etc.).
    pub db: DbClientState,
    /// HTTP client for daemon API calls (graph, validation, artifacts, etc.).
    pub daemon: DaemonState,
    /// Startup task progress tracker.
    pub startup: StartupState,
    /// File watcher for artifact changes.
    pub artifacts: ArtifactState,
}
