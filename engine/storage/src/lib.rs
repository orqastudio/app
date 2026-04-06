//! orqa-storage: unified SQLite storage for OrqaStudio.
//!
//! Owns the single `.state/orqa.db` database that consolidates all persistent
//! data previously spread across four separate SQLite files:
//!   - app/src-tauri: orqa.db (projects, sessions, messages, settings, themes,
//!     violations, health_snapshots)
//!   - daemon: daemon.db (duplicate of app tables, now removed)
//!   - daemon: events.db (log_events)
//!   - devtools: devtools-sessions.db (devtools_sessions, devtools_events)
//!
//! Thread-safety: `Storage` holds only a `PathBuf` and opens a fresh connection
//! on each `conn()` call, making it safe to share via `Arc` across async tasks
//! that use `spawn_blocking`.

#![warn(missing_docs)]

use std::path::{Path, PathBuf};
use std::sync::Arc;

use rusqlite::Connection;

pub use error::StorageError;
pub use frozen::Frozen;

/// Error types returned by all storage operations.
pub mod error;
/// Zero-cost immutability wrapper for data returned from storage.
pub mod frozen;
/// Migration runner: applies ordered schema versions to a database.
pub mod migrate;
/// Repository modules, one per domain table group.
pub mod repo;
/// SQL schema constants for the unified database.
pub mod schema;

use repo::devtools::DevtoolsRepo;
use repo::events::EventRepo;
use repo::health::HealthRepo;
use repo::issue_groups::IssueGroupRepo;
use repo::messages::MessageRepo;
use repo::projects::ProjectRepo;
use repo::sessions::SessionRepo;
use repo::settings::SettingsRepo;
use repo::themes::ThemeRepo;
use repo::violations::ViolationsRepo;

/// PRAGMAs applied to every new connection.
///
/// WAL mode provides concurrent readers with a serialised writer. The
/// busy_timeout prevents "database is locked" errors under contention.
const CONNECTION_PRAGMAS: &str = "
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;
PRAGMA busy_timeout = 5000;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = -8000;
PRAGMA temp_store = MEMORY;
";

/// The unified OrqaStudio storage layer.
///
/// Holds the path to the SQLite database file. All SQL connections are opened
/// on demand via `conn()` and must be used inside `spawn_blocking` closures
/// when called from async code. `Storage` is `Send + Sync` and can be safely
/// shared via `Arc`.
///
/// In-memory test instances use a temporary file in the system temp directory
/// via `open_in_memory()`. The file is deleted when the returned `Storage` is
/// dropped, ensuring test isolation without named shared-memory fragility.
pub struct Storage {
    /// Database path (file path for production and for in-memory test instances).
    db_path: PathBuf,
    /// Temp file handle kept alive so the file persists for the lifetime of this
    /// `Storage`. `None` for production instances opened via `Storage::open()`.
    _temp_file: Option<tempfile::NamedTempFile>,
}

impl Storage {
    /// Open (or create) the unified database at `{project_root}/.state/orqa.db`.
    ///
    /// Creates `.state/` if absent, applies PRAGMAs, runs pending migrations,
    /// and returns an `Arc<Storage>`. Callers should store the returned `Arc`
    /// and share it rather than calling `open` multiple times.
    pub fn open(project_root: &Path) -> Result<Arc<Self>, StorageError> {
        let state_dir = project_root.join(".state");
        std::fs::create_dir_all(&state_dir).map_err(|e| StorageError::Path(e.to_string()))?;

        let db_path = state_dir.join("orqa.db");
        let storage = Self {
            db_path,
            _temp_file: None,
        };

        // Run migrations synchronously on the calling thread before any
        // async callers start using the database.
        let conn = storage.conn()?;
        migrate::run_migrations(&conn)?;

        tracing::info!(
            path = %storage.db_path.display(),
            "[storage] unified database opened"
        );

        Ok(Arc::new(storage))
    }

    /// Open an isolated database backed by a temporary file for use in tests.
    ///
    /// Creates a `NamedTempFile` in the system temp directory and keeps it alive
    /// for the lifetime of the returned `Storage`. Each call gets a distinct file,
    /// preventing cross-test interference. The file is deleted when `Storage` drops.
    ///
    /// Not exposed as `Arc` because test code typically owns it directly.
    pub fn open_in_memory() -> Result<Self, StorageError> {
        // A NamedTempFile persists on disk until it is dropped, giving every
        // `conn()` call a stable path to open. This is more reliable than
        // SQLite named shared-memory URIs, which are destroyed when the last
        // connection is closed (causing "no such table" errors on subsequent opens).
        let temp = tempfile::NamedTempFile::new().map_err(|e| StorageError::Path(e.to_string()))?;
        let db_path = temp.path().to_path_buf();
        let storage = Self {
            db_path,
            _temp_file: Some(temp),
        };
        let conn = storage.conn()?;
        migrate::run_migrations(&conn)?;
        Ok(storage)
    }

    /// Open a fresh SQLite connection to the database.
    ///
    /// Applies all connection PRAGMAs on each open. Connections are not
    /// pooled — each call returns a new `rusqlite::Connection` that must
    /// be used and dropped within a single `spawn_blocking` closure.
    pub fn conn(&self) -> Result<Connection, StorageError> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute_batch(CONNECTION_PRAGMAS)?;
        Ok(conn)
    }

    // -------------------------------------------------------------------------
    // Repo accessors — zero-cost wrappers that borrow &self
    // -------------------------------------------------------------------------

    /// Access the projects repository.
    pub fn projects(&self) -> ProjectRepo<'_> {
        ProjectRepo { storage: self }
    }

    /// Access the sessions repository.
    pub fn sessions(&self) -> SessionRepo<'_> {
        SessionRepo { storage: self }
    }

    /// Access the messages repository.
    pub fn messages(&self) -> MessageRepo<'_> {
        MessageRepo { storage: self }
    }

    /// Access the settings repository.
    pub fn settings(&self) -> SettingsRepo<'_> {
        SettingsRepo { storage: self }
    }

    /// Access the themes repository.
    pub fn themes(&self) -> ThemeRepo<'_> {
        ThemeRepo { storage: self }
    }

    /// Access the violations repository.
    pub fn violations(&self) -> ViolationsRepo<'_> {
        ViolationsRepo { storage: self }
    }

    /// Access the health snapshots repository.
    pub fn health(&self) -> HealthRepo<'_> {
        HealthRepo { storage: self }
    }

    /// Access the log events repository.
    pub fn events(&self) -> EventRepo<'_> {
        EventRepo { storage: self }
    }

    /// Access the devtools sessions and events repository.
    pub fn devtools(&self) -> DevtoolsRepo<'_> {
        DevtoolsRepo { storage: self }
    }

    /// Access the issue groups repository.
    pub fn issue_groups(&self) -> IssueGroupRepo<'_> {
        IssueGroupRepo { storage: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_in_memory_succeeds() {
        let storage = Storage::open_in_memory().expect("in-memory db");
        // Verify connection is usable
        let conn = storage.conn().expect("conn");
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM _migrations", [], |r| r.get(0))
            .expect("query _migrations");
        assert!(n >= 3, "should have at least 3 migrations recorded");
    }

    #[test]
    fn open_creates_state_dir_and_db_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = Storage::open(dir.path()).expect("open");
        let db_path = dir.path().join(".state").join("orqa.db");
        assert!(db_path.exists(), "orqa.db should be created");
        // Verify it's usable
        let conn = storage.conn().expect("conn");
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM projects", [], |r| r.get(0))
            .expect("query projects");
        assert_eq!(n, 0);
    }

    #[test]
    fn foreign_keys_are_enabled() {
        let storage = Storage::open_in_memory().expect("in-memory db");
        let conn = storage.conn().expect("conn");
        let fk: i64 = conn
            .query_row("PRAGMA foreign_keys", [], |r| r.get(0))
            .expect("pragma");
        assert_eq!(fk, 1, "foreign keys should be enabled");
    }

    #[test]
    fn migrations_are_idempotent() {
        let storage = Storage::open_in_memory().expect("first open");
        let conn = storage.conn().expect("conn");
        // Running migrations a second time must not error.
        migrate::run_migrations(&conn).expect("second migration run");
    }
}
