// Application state managed by Tauri.
//
// Decomposed into logical sub-structs for clarity. All engine operations are
// delegated to the daemon via HTTP. The app is a pure UI layer with project-scoped
// SQLite for sessions, messages, settings, and health snapshots.

use std::sync::{Arc, Mutex};

use orqa_storage::Storage;

use crate::daemon_client::DaemonClient;
use crate::startup::StartupTracker;
use crate::watcher::SharedWatcher;

// ---------------------------------------------------------------------------
// Sub-structs — each groups a logically related slice of application state.
// ---------------------------------------------------------------------------

/// Project-scoped storage state.
///
/// Wrapped in `Mutex` so commands can swap the storage when a different project
/// is opened. The inner `Arc<Storage>` is cloned out under the lock so commands
/// can release the lock before executing SQL.
pub struct StorageState {
    /// Active project storage, or `None` before the first project is opened.
    pub storage: Mutex<Option<Arc<Storage>>>,
}

impl StorageState {
    /// Acquire the current storage, returning an error if no project is open.
    pub fn get(&self) -> Result<Arc<Storage>, crate::error::OrqaError> {
        self.storage
            .lock()
            .map_err(|e| {
                crate::error::OrqaError::Database(format!("storage lock poisoned: {e}"))
            })?
            .clone()
            .ok_or_else(|| {
                crate::error::OrqaError::NotFound("no project is open".to_owned())
            })
    }

    /// Replace the active storage with a new project's storage.
    pub fn set(&self, new_storage: Arc<Storage>) -> Result<(), crate::error::OrqaError> {
        *self.storage.lock().map_err(|e| {
            crate::error::OrqaError::Database(format!("storage lock poisoned: {e}"))
        })? = Some(new_storage);
        Ok(())
    }
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
/// is a pure UI layer with project-scoped SQLite for sessions, messages, and settings.
pub struct AppState {
    /// Project-scoped SQLite storage, swapped on project open.
    pub db: StorageState,
    /// HTTP client for daemon API calls.
    pub daemon: DaemonState,
    /// Startup task progress tracker.
    pub startup: StartupState,
    /// File watcher for artifact changes.
    pub artifacts: ArtifactState,
}
