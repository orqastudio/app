use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::daemon_client::DaemonClient;
use crate::startup::StartupTracker;
use crate::watcher::SharedWatcher;

// ---------------------------------------------------------------------------
// Sub-structs — each groups a logically related slice of application state.
// ---------------------------------------------------------------------------

/// Database connection state.
///
/// The `Mutex<Connection>` is safe for single-writer SQLite with WAL mode.
pub struct DbState {
    /// The SQLite connection, guarded by a mutex for single-writer access.
    pub conn: Mutex<Connection>,
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
/// All engine operations are delegated to the daemon via `DaemonState`. The app
/// is a pure UI layer with local SQLite for sessions, messages, and settings.
pub struct AppState {
    /// SQLite database connection.
    pub db: DbState,
    /// HTTP client for daemon API calls.
    pub daemon: DaemonState,
    /// Startup task progress tracker.
    pub startup: StartupState,
    /// File watcher for artifact changes.
    pub artifacts: ArtifactState,
}
