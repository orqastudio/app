// Migration system for orqa-storage.
//
// Tracks applied migrations in a `_migrations` table (version INTEGER, name TEXT).
// On each `Storage::open()` call, runs any migrations not yet applied in order.
// All migrations are idempotent — safe to run against an existing database.
//
// Migration list:
//   1  initial_core_schema      — projects, sessions, messages, settings, themes,
//                                 violations, health_snapshots, FTS5 triggers
//   2  log_events               — daemon log_events table
//   3  devtools_sessions        — devtools_sessions + devtools_events tables
//   4  issue_groups             — issue_groups table + fingerprint column on event tables

use rusqlite::Connection;

use crate::error::StorageError;
use crate::schema::{CORE_SCHEMA, DEVTOOLS_SCHEMA, ISSUE_GROUPS_SCHEMA, LOG_EVENTS_SCHEMA};

/// Ensure the `_migrations` tracking table exists.
fn ensure_migrations_table(conn: &Connection) -> Result<(), StorageError> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (
            version     INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            applied_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
        );",
    )?;
    Ok(())
}

/// Return true if `version` has already been applied.
fn is_applied(conn: &Connection, version: i64) -> Result<bool, StorageError> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM _migrations WHERE version = ?1",
        rusqlite::params![version],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Record that `version` was applied.
fn record_migration(conn: &Connection, version: i64, name: &str) -> Result<(), StorageError> {
    conn.execute(
        "INSERT INTO _migrations (version, name) VALUES (?1, ?2)",
        rusqlite::params![version, name],
    )?;
    Ok(())
}

/// Run all pending migrations against `conn` in version order.
///
/// Each migration is applied inside the same connection and recorded in
/// `_migrations` immediately after. Idempotent: already-applied versions
/// are skipped.
pub fn run_migrations(conn: &Connection) -> Result<(), StorageError> {
    ensure_migrations_table(conn)?;

    // Migration 1 — Core schema: projects, sessions, messages, settings, themes,
    //               enforcement_violations, health_snapshots, FTS5 triggers.
    if !is_applied(conn, 1)? {
        conn.execute_batch(CORE_SCHEMA)?;
        record_migration(conn, 1, "initial_core_schema")?;
    }

    // Migration 2 — Log events: daemon event bus persistence.
    if !is_applied(conn, 2)? {
        conn.execute_batch(LOG_EVENTS_SCHEMA)?;
        record_migration(conn, 2, "log_events")?;
    }

    // Migration 3 — Devtools session tables.
    if !is_applied(conn, 3)? {
        conn.execute_batch(DEVTOOLS_SCHEMA)?;
        record_migration(conn, 3, "devtools_sessions")?;
    }

    // Migration 4 — Issue groups: event deduplication and fingerprint tracking.
    if !is_applied(conn, 4)? {
        conn.execute_batch(ISSUE_GROUPS_SCHEMA)?;
        record_migration(conn, 4, "issue_groups")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn open_mem() -> Connection {
        Connection::open_in_memory().expect("in-memory db")
    }

    #[test]
    fn migrations_run_once() {
        let conn = open_mem();
        run_migrations(&conn).expect("first run");
        run_migrations(&conn).expect("second run is idempotent");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM _migrations", [], |r| r.get(0))
            .expect("count");
        // Should record exactly 4 migrations.
        assert_eq!(count, 4, "should record 4 migrations");
    }

    #[test]
    fn core_tables_exist_after_migration() {
        let conn = open_mem();
        run_migrations(&conn).expect("migrate");

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
            let exists: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    rusqlite::params![table],
                    |r| r.get(0),
                )
                .expect("query");
            assert_eq!(exists, 1, "table '{table}' should exist after migration");
        }
    }
}
