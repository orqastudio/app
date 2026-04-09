//! SeaORM migration library for orqa-storage.
//!
//! Provides:
//!   - Migrator: the SeaORM migration list (m001–m004)
//!   - bridge_legacy_migrations: detects an existing _migrations table and
//!     inserts the corresponding rows into seaql_migrations so SeaORM knows
//!     which migrations are already applied on an existing database.
//!
//! Usage (fresh database):
//!   `Migrator::up(&db, None).await?;`
//!
//! Usage (existing database with _migrations table):
//!   `bridge_legacy_migrations(&db).await?;`
//!   `Migrator::up(&db, None).await?;`

#![warn(missing_docs)]

/// Migration 1: initial core schema (projects, sessions, messages, settings, themes, FTS5).
pub mod m001_initial_core_schema;
/// Migration 2: log_events table for daemon event bus persistence.
pub mod m002_log_events;
/// Migration 3: devtools_sessions and devtools_events tables.
pub mod m003_devtools_sessions;
/// Migration 4: issue_groups table and fingerprint columns on event tables.
pub mod m004_issue_groups;

use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::prelude::*;

/// The SeaORM migrator for orqa-storage.
///
/// Lists all 4 migrations in application order. Pass to `Migrator::up` or
/// `Migrator::fresh` to apply migrations.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m001_initial_core_schema::Migration),
            Box::new(m002_log_events::Migration),
            Box::new(m003_devtools_sessions::Migration),
            Box::new(m004_issue_groups::Migration),
        ]
    }
}

/// Run the full migration sequence on `db`.
///
/// This is the one-call entry point for `Storage::open`:
/// 1. Calls `bridge_legacy_migrations` to handle existing databases that used
///    the old `_migrations` tracking table.
/// 2. Calls `Migrator::up` to apply any pending migrations.
///
/// Safe to call multiple times — already-applied migrations are skipped.
///
/// # Errors
///
/// Returns `DbErr` if any migration or bridge step fails.
pub async fn run(_backend: DatabaseBackend, db: &DatabaseConnection) -> Result<(), DbErr> {
    bridge_legacy_migrations(db).await?;
    Migrator::up(db, None).await?;
    Ok(())
}

/// Bridge legacy `_migrations` table to SeaORM's `seaql_migrations` table.
///
/// Existing orqa.db databases track applied migrations in `_migrations`
/// (version INTEGER PK, name TEXT, applied_at TEXT). SeaORM tracks applied
/// migrations in `seaql_migrations` (version TEXT PK, applied_at INTEGER).
///
/// This function:
/// 1. Checks if `_migrations` exists. If not, returns immediately (fresh DB).
/// 2. Reads which version numbers have been applied from `_migrations`.
/// 3. Maps each applied version to the matching SeaORM migration name.
/// 4. Inserts the corresponding rows into `seaql_migrations` so SeaORM skips
///    those migrations on the next `Migrator::up` call.
///
/// Must be called BEFORE `Migrator::up` on any existing database.
///
/// # Errors
///
/// Returns `DbErr` if any database query fails.
#[allow(clippy::too_many_lines)]
pub async fn bridge_legacy_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Map legacy version numbers to SeaORM migration names.
    // These names must match MigrationName::name() in each migration struct.
    let version_to_name: &[(i64, &str)] = &[
        (1, "m001_initial_core_schema"),
        (2, "m002_log_events"),
        (3, "m003_devtools_sessions"),
        (4, "m004_issue_groups"),
    ];

    // Check if the legacy _migrations table exists.
    let legacy_exists: bool = db
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='_migrations' LIMIT 1"
                .to_owned(),
        ))
        .await?
        .is_some();

    if !legacy_exists {
        // Fresh database — SeaORM will create seaql_migrations on its own.
        return Ok(());
    }

    // Read all applied version numbers from the legacy table.
    let rows = db
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT version FROM _migrations ORDER BY version".to_owned(),
        ))
        .await?;

    let applied_versions: Vec<i64> = rows
        .iter()
        .filter_map(|row| row.try_get::<i64>("", "version").ok())
        .collect();

    if applied_versions.is_empty() {
        return Ok(());
    }

    // Ensure seaql_migrations exists. SeaORM creates it during the first
    // migration run, but we need it to exist before inserting bridge rows.
    db.execute_raw(Statement::from_string(
        DbBackend::Sqlite,
        "CREATE TABLE IF NOT EXISTS seaql_migrations (
            version     TEXT    NOT NULL PRIMARY KEY,
            applied_at  INTEGER NOT NULL
        )"
        .to_owned(),
    ))
    .await?;

    // Insert a seaql_migrations row for each already-applied legacy migration.
    // Use INSERT OR IGNORE so this is idempotent — safe to call multiple times.
    for version in &applied_versions {
        if let Some(&(_, name)) = version_to_name.iter().find(|(v, _)| v == version) {
            db.execute_raw(Statement::from_string(
                DbBackend::Sqlite,
                format!(
                    "INSERT OR IGNORE INTO seaql_migrations (version, applied_at) VALUES ('{name}', 0)"
                ),
            ))
            .await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Database;

    /// Open an in-memory SQLite database for testing.
    async fn open_mem() -> DatabaseConnection {
        Database::connect("sqlite::memory:")
            .await
            .expect("open mem db")
    }

    #[tokio::test]
    async fn fresh_database_migration() {
        // A fresh database with no prior data runs all 4 migrations cleanly.
        let db = open_mem().await;
        Migrator::up(&db, None).await.expect("fresh migration");

        // All 12 tables must exist.
        for table in &[
            "projects",
            "sessions",
            "messages",
            "settings",
            "project_themes",
            "project_theme_overrides",
            "enforcement_violations",
            "health_snapshots",
            "log_events",
            "devtools_sessions",
            "devtools_events",
            "issue_groups",
        ] {
            let row = db
                .query_one_raw(Statement::from_string(
                    DbBackend::Sqlite,
                    format!(
                        "SELECT 1 FROM sqlite_master WHERE type='table' AND name='{table}' LIMIT 1"
                    ),
                ))
                .await
                .expect("query");
            assert!(
                row.is_some(),
                "table '{table}' should exist after migration"
            );
        }

        // seaql_migrations must record all 4 migrations.
        let count_row = db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT COUNT(*) as cnt FROM seaql_migrations".to_owned(),
            ))
            .await
            .expect("count query")
            .expect("row");
        let count: i64 = count_row.try_get("", "cnt").expect("cnt");
        assert_eq!(count, 4, "should record 4 migrations in seaql_migrations");
    }

    #[tokio::test]
    async fn migration_is_idempotent() {
        // Running Migrator::up twice is safe (already-applied migrations skipped).
        let db = open_mem().await;
        Migrator::up(&db, None).await.expect("first run");
        Migrator::up(&db, None)
            .await
            .expect("second run is idempotent");

        let count_row = db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT COUNT(*) as cnt FROM seaql_migrations".to_owned(),
            ))
            .await
            .expect("count query")
            .expect("row");
        let count: i64 = count_row.try_get("", "cnt").expect("cnt");
        assert_eq!(count, 4, "should still record exactly 4 migrations");
    }

    #[tokio::test]
    async fn bridge_detects_no_legacy_table() {
        // On a fresh DB (no _migrations table), bridge should be a no-op.
        let db = open_mem().await;
        // Bridge before creating any tables — no _migrations table exists.
        bridge_legacy_migrations(&db).await.expect("bridge no-op");
        // Now run migrations normally — should work fine.
        Migrator::up(&db, None)
            .await
            .expect("migration after no-op bridge");
    }

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn bridge_marks_existing_migrations() {
        // Simulate an existing DB: create _migrations with versions 1 and 2,
        // apply the corresponding schema, then verify bridge + up only applies
        // migrations 3 and 4.
        let db = open_mem().await;

        // Create legacy tracking table.
        db.execute_raw(Statement::from_string(
            DbBackend::Sqlite,
            "CREATE TABLE _migrations (
                version     INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                applied_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
            )"
            .to_owned(),
        ))
        .await
        .expect("create _migrations");

        db.execute_raw(Statement::from_string(
            DbBackend::Sqlite,
            "INSERT INTO _migrations (version, name) VALUES (1, 'initial_core_schema'), (2, 'log_events')"
                .to_owned(),
        ))
        .await
        .expect("insert legacy rows");

        // Apply SQL for migrations 1 and 2 directly via execute_unprepared
        // since SchemaManager::new is not available outside the migration runner.
        db.execute_unprepared(
            "
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY, name TEXT NOT NULL, path TEXT NOT NULL UNIQUE,
    description TEXT, detected_stack TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id TEXT, title TEXT, model TEXT NOT NULL DEFAULT 'auto', system_prompt TEXT,
    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'completed', 'abandoned', 'error')),
    summary TEXT, handoff_notes TEXT,
    total_input_tokens INTEGER DEFAULT 0, total_output_tokens INTEGER DEFAULT 0,
    total_cost_usd REAL DEFAULT 0.0, provider_session_id TEXT,
    title_manually_set INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY,
    session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content_type TEXT NOT NULL DEFAULT 'text'
        CHECK (content_type IN ('text', 'tool_use', 'tool_result', 'thinking', 'image')),
    content TEXT, tool_call_id TEXT, tool_name TEXT, tool_input TEXT,
    tool_is_error INTEGER DEFAULT 0,
    turn_index INTEGER NOT NULL DEFAULT 0, block_index INTEGER NOT NULL DEFAULT 0,
    stream_status TEXT NOT NULL DEFAULT 'complete'
        CHECK (stream_status IN ('pending', 'complete', 'error')),
    input_tokens INTEGER, output_tokens INTEGER,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE TABLE IF NOT EXISTS settings (
    key TEXT NOT NULL, value TEXT NOT NULL, scope TEXT NOT NULL DEFAULT 'app',
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    PRIMARY KEY (key, scope)
);
CREATE TABLE IF NOT EXISTS project_themes (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_file TEXT NOT NULL, source_hash TEXT NOT NULL,
    extracted_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    tokens_light TEXT NOT NULL, tokens_dark TEXT, unmapped TEXT,
    is_active INTEGER NOT NULL DEFAULT 1
);
CREATE TABLE IF NOT EXISTS project_theme_overrides (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    token_name TEXT NOT NULL, value_light TEXT NOT NULL, value_dark TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE TABLE IF NOT EXISTS enforcement_violations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    rule_name TEXT NOT NULL, action TEXT NOT NULL, tool_name TEXT NOT NULL,
    detail TEXT, created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE TABLE IF NOT EXISTS health_snapshots (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    node_count INTEGER NOT NULL DEFAULT 0, edge_count INTEGER NOT NULL DEFAULT 0,
    broken_ref_count INTEGER NOT NULL DEFAULT 0, error_count INTEGER NOT NULL DEFAULT 0,
    warning_count INTEGER NOT NULL DEFAULT 0,
    largest_component_ratio REAL NOT NULL DEFAULT 0.0,
    avg_degree REAL NOT NULL DEFAULT 0.0, pillar_traceability REAL NOT NULL DEFAULT 100.0,
    outlier_count INTEGER NOT NULL DEFAULT 0, outlier_percentage REAL NOT NULL DEFAULT 0.0,
    delivery_connectivity REAL NOT NULL DEFAULT 0.0,
    learning_connectivity REAL NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
    content, tool_name, content='messages', content_rowid='id',
    tokenize='porter unicode61'
);
",
        )
        .await
        .expect("apply m001 schema");

        db.execute_unprepared(
            "
CREATE TABLE IF NOT EXISTS log_events (
    id INTEGER PRIMARY KEY, timestamp INTEGER NOT NULL,
    level TEXT NOT NULL, source TEXT NOT NULL, category TEXT NOT NULL,
    message TEXT NOT NULL, metadata TEXT NOT NULL, session_id TEXT
);
",
        )
        .await
        .expect("apply m002 schema");

        // Run the bridge — should mark m001 and m002 as applied.
        bridge_legacy_migrations(&db).await.expect("bridge");

        // Migrator::up should only apply m003 and m004.
        Migrator::up(&db, None).await.expect("up after bridge");

        // seaql_migrations must contain all 4 entries.
        let count_row = db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT COUNT(*) as cnt FROM seaql_migrations".to_owned(),
            ))
            .await
            .expect("count query")
            .expect("row");
        let count: i64 = count_row.try_get("", "cnt").expect("cnt");
        assert_eq!(count, 4, "seaql_migrations should have all 4 entries");

        // devtools_sessions and issue_groups must exist (applied by Migrator::up).
        for table in &["devtools_sessions", "issue_groups"] {
            let row = db
                .query_one_raw(Statement::from_string(
                    DbBackend::Sqlite,
                    format!(
                        "SELECT 1 FROM sqlite_master WHERE type='table' AND name='{table}' LIMIT 1"
                    ),
                ))
                .await
                .expect("query");
            assert!(row.is_some(), "table '{table}' should exist after up");
        }
    }

    #[tokio::test]
    async fn bridge_is_idempotent() {
        // Calling bridge multiple times is safe (INSERT OR IGNORE).
        let db = open_mem().await;

        db.execute_raw(Statement::from_string(
            DbBackend::Sqlite,
            "CREATE TABLE _migrations (
                version     INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                applied_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
            )"
            .to_owned(),
        ))
        .await
        .expect("create _migrations");

        db.execute_raw(Statement::from_string(
            DbBackend::Sqlite,
            "INSERT INTO _migrations (version, name) VALUES (1, 'initial_core_schema')".to_owned(),
        ))
        .await
        .expect("insert");

        bridge_legacy_migrations(&db).await.expect("first bridge");
        bridge_legacy_migrations(&db)
            .await
            .expect("second bridge is idempotent");

        let count_row = db
            .query_one_raw(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT COUNT(*) as cnt FROM seaql_migrations".to_owned(),
            ))
            .await
            .expect("count query")
            .expect("row");
        let count: i64 = count_row.try_get("", "cnt").expect("cnt");
        assert_eq!(
            count, 1,
            "should have exactly 1 entry after idempotent bridge"
        );
    }
}
