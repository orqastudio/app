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
//! Thread-safety: `Storage` wraps `Arc<DatabaseConnection>` and can be freely
//! cloned and shared across async tasks. All async methods run directly on the
//! SeaORM connection pool — no `spawn_blocking` is required.

#![warn(missing_docs)]

use std::path::Path;
use std::sync::Arc;

pub use error::StorageError;
pub use frozen::Frozen;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};

/// SeaORM entity definitions for all 12 tables.
pub mod entity;
/// Error types returned by all storage operations.
pub mod error;
/// Zero-cost immutability wrapper for data returned from storage.
pub mod frozen;
/// Repository modules, one per domain table group.
pub mod repo;
/// Async repository trait contracts — pure domain types, no ORM leakage.
pub mod traits;

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

/// SQLite PRAGMAs applied once after the connection pool is established.
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
/// Wraps a SeaORM `DatabaseConnection` behind an `Arc` so it can be freely
/// cloned and shared across async tasks and Tauri state. The connection pool
/// is opened once on `Storage::open` and reused for all subsequent calls.
///
/// In-memory test instances use a temporary file in the system temp directory
/// via `open_in_memory()`. The file is deleted when the returned `Storage`
/// is dropped (via the `tempfile::TempDir` kept inside).
#[derive(Clone)]
pub struct Storage {
    /// Shared connection pool managed by SeaORM / sqlx.
    db: Arc<DatabaseConnection>,
    /// Temp directory kept alive so the database file persists for the
    /// lifetime of this `Storage`. Held for its `Drop` effect only.
    /// `None` for production instances.
    #[allow(dead_code)]
    temp_dir: Option<Arc<tempfile::TempDir>>,
}

impl Storage {
    /// Open (or create) the unified database at `{project_root}/.state/orqa.db`.
    ///
    /// Creates `.state/` if absent, applies PRAGMAs, runs pending migrations,
    /// and returns a `Storage`. Callers should wrap in `Arc` and share rather
    /// than calling `open` multiple times.
    pub async fn open(project_root: &Path) -> Result<Self, StorageError> {
        let state_dir = project_root.join(".state");
        std::fs::create_dir_all(&state_dir).map_err(|e| StorageError::Path(e.to_string()))?;

        let db_path = state_dir.join("orqa.db");
        let url = format!("sqlite://{}?mode=rwc", db_path.display());

        let storage = Self::connect(&url, None).await?;

        tracing::info!(
            path = %db_path.display(),
            "[storage] unified database opened"
        );

        Ok(storage)
    }

    /// Open an isolated database backed by a temporary directory for use in tests.
    ///
    /// Creates a `TempDir` and a SQLite file inside it. Each call gets a
    /// distinct file, preventing cross-test interference. The file is deleted
    /// when `Storage` drops.
    pub async fn open_in_memory() -> Result<Self, StorageError> {
        let temp_dir = tempfile::TempDir::new().map_err(|e| StorageError::Path(e.to_string()))?;
        let db_path = temp_dir.path().join("orqa_test.db");
        let url = format!("sqlite://{}?mode=rwc", db_path.display());

        let temp_arc = Arc::new(temp_dir);
        let mut storage = Self::connect(&url, None).await?;
        storage.temp_dir = Some(temp_arc);

        Ok(storage)
    }

    /// Build a `Storage` from a raw SQLite URL, for callers that manage their
    /// own database file path (e.g., integration test fixtures).
    async fn connect(url: &str, max_connections: Option<u32>) -> Result<Self, StorageError> {
        let mut opts = ConnectOptions::new(url);
        opts.max_connections(max_connections.unwrap_or(5))
            .sqlx_logging(false);

        let db = Database::connect(opts)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        // Apply PRAGMAs immediately after connecting.
        db.execute_unprepared(CONNECTION_PRAGMAS)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        // Run schema migrations before any caller can use the pool.
        // bridge_legacy_migrations handles existing DBs, Migrator::up handles fresh ones.
        orqa_storage_migration::run(db.get_database_backend(), &db)
            .await
            .map_err(|e| StorageError::Database(e.to_string()))?;

        Ok(Self {
            db: Arc::new(db),
            temp_dir: None,
        })
    }

    /// Return a reference to the underlying SeaORM connection.
    ///
    /// Repos call this to execute queries. The connection is shared across all
    /// callers — no new connection is opened per call.
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    // -------------------------------------------------------------------------
    // Repo accessors — zero-cost wrappers that clone the Arc<DatabaseConnection>
    // -------------------------------------------------------------------------

    /// Access the projects repository.
    pub fn projects(&self) -> ProjectRepo {
        ProjectRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the sessions repository.
    pub fn sessions(&self) -> SessionRepo {
        SessionRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the messages repository.
    pub fn messages(&self) -> MessageRepo {
        MessageRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the settings repository.
    pub fn settings(&self) -> SettingsRepo {
        SettingsRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the themes repository.
    pub fn themes(&self) -> ThemeRepo {
        ThemeRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the violations repository.
    pub fn violations(&self) -> ViolationsRepo {
        ViolationsRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the health snapshots repository.
    pub fn health(&self) -> HealthRepo {
        HealthRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the log events repository.
    pub fn events(&self) -> EventRepo {
        EventRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the devtools sessions and events repository.
    pub fn devtools(&self) -> DevtoolsRepo {
        DevtoolsRepo {
            db: Arc::clone(&self.db),
        }
    }

    /// Access the issue groups repository.
    pub fn issue_groups(&self) -> IssueGroupRepo {
        IssueGroupRepo {
            db: Arc::clone(&self.db),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ConnectionTrait, DbBackend, Statement};

    async fn open() -> Storage {
        Storage::open_in_memory().await.expect("in-memory db")
    }

    #[tokio::test]
    async fn open_in_memory_succeeds() {
        let storage = open().await;
        let result = storage
            .db()
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT COUNT(*) AS n FROM seaql_migrations".to_owned(),
            ))
            .await
            .expect("query seaql_migrations")
            .expect("row exists");
        let n: i64 = result.try_get("", "n").expect("get count");
        assert!(
            n >= 4,
            "should have at least 4 migrations recorded, got {n}"
        );
    }

    #[tokio::test]
    async fn open_creates_state_dir_and_db_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let _storage = Storage::open(dir.path()).await.expect("open");
        let db_path = dir.path().join(".state").join("orqa.db");
        assert!(db_path.exists(), "orqa.db should be created");
    }

    #[tokio::test]
    async fn foreign_keys_are_enabled() {
        let storage = open().await;
        let result = storage
            .db()
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "PRAGMA foreign_keys".to_owned(),
            ))
            .await
            .expect("query pragma")
            .expect("row exists");
        // SQLite PRAGMA foreign_keys returns the value in column "foreign_keys"
        let fk: i64 = result.try_get("", "foreign_keys").expect("get fk value");
        assert_eq!(fk, 1, "foreign keys should be enabled");
    }

    #[tokio::test]
    async fn migrations_are_idempotent() {
        let storage = open().await;
        // Running migrations a second time must not error.
        orqa_storage_migration::run(DbBackend::Sqlite, storage.db())
            .await
            .expect("second migration run");
    }

    #[tokio::test]
    async fn projects_table_is_empty_on_fresh_db() {
        let storage = open().await;
        let result = storage
            .db()
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT COUNT(*) AS cnt FROM projects".to_owned(),
            ))
            .await
            .expect("query projects")
            .expect("row exists");
        let n: i64 = result.try_get("", "cnt").expect("get count");
        assert_eq!(n, 0);
    }
}
