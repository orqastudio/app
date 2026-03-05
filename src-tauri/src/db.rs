use rusqlite::Connection;

use crate::error::OrqaError;

/// Open (or create) the SQLite database at `path`, apply PRAGMAs, and run migrations.
pub fn init_db(path: &str) -> Result<Connection, OrqaError> {
    let conn = Connection::open(path)?;

    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;
        PRAGMA busy_timeout = 5000;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -8000;
        PRAGMA temp_store = MEMORY;
        ",
    )?;

    conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))?;
    conn.execute_batch(include_str!("../migrations/002_governance_bootstrap.sql"))?;
    conn.execute_batch(include_str!("../migrations/003_enforcement.sql"))?;
    run_migration_004(&conn)?;

    Ok(conn)
}

/// Create an in-memory SQLite database for testing.
pub fn init_memory_db() -> Result<Connection, OrqaError> {
    let conn = Connection::open_in_memory()?;

    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))?;
    conn.execute_batch(include_str!("../migrations/002_governance_bootstrap.sql"))?;
    conn.execute_batch(include_str!("../migrations/003_enforcement.sql"))?;
    run_migration_004(&conn)?;

    Ok(conn)
}

/// Migration 004: Add `sdk_session_id` column to sessions table.
///
/// Idempotent — checks `pragma_table_info` before altering.
fn run_migration_004(conn: &Connection) -> Result<(), OrqaError> {
    let has_col: bool = conn
        .prepare(
            "SELECT COUNT(*) FROM pragma_table_info('sessions') WHERE name = 'sdk_session_id'",
        )?
        .query_row([], |row| row.get::<_, i64>(0))
        .map(|count| count > 0)
        .unwrap_or(false);

    if !has_col {
        conn.execute_batch("ALTER TABLE sessions ADD COLUMN sdk_session_id TEXT")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_db_initializes_successfully() {
        let conn = init_memory_db().expect("in-memory DB should initialize");

        // Verify tables exist by querying sqlite_master
        let table_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
                [],
                |row| row.get(0),
            )
            .expect("should query sqlite_master");

        // We expect: projects, sessions, messages, artifacts, settings,
        // project_themes, project_theme_overrides, messages_fts (+ internal fts tables), artifacts_fts
        assert!(
            table_count >= 7,
            "expected at least 7 user tables, found {table_count}"
        );
    }

    #[test]
    fn foreign_keys_are_enabled() {
        let conn = init_memory_db().expect("in-memory DB should initialize");

        let fk_enabled: i64 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .expect("should query foreign_keys pragma");

        assert_eq!(fk_enabled, 1, "foreign keys should be enabled");
    }

    #[test]
    fn pending_messages_recovered_on_init() {
        let conn = init_memory_db().expect("in-memory DB should initialize");

        // Insert a project and session first (FK constraints)
        conn.execute(
            "INSERT INTO projects (name, path) VALUES ('test', '/tmp/test')",
            [],
        )
        .expect("should insert project");
        conn.execute(
            "INSERT INTO sessions (project_id, model) VALUES (1, 'auto')",
            [],
        )
        .expect("should insert session");

        // Insert a pending message
        conn.execute(
            "INSERT INTO messages (session_id, role, content, turn_index, block_index, stream_status) \
             VALUES (1, 'assistant', 'partial', 0, 0, 'pending')",
            [],
        )
        .expect("should insert message");

        // Re-run migration (simulates restart)
        conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))
            .expect("re-running migration should succeed");

        let status: String = conn
            .query_row(
                "SELECT stream_status FROM messages WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .expect("should query message");

        assert_eq!(
            status, "error",
            "pending messages should be recovered to error"
        );
    }

    #[test]
    fn file_db_initializes_and_cleans_up() {
        let dir = std::env::temp_dir().join("forge_test_db");
        std::fs::create_dir_all(&dir).expect("should create temp dir");
        let db_path = dir.join("test.db");
        let path_str = db_path.to_str().expect("path should be valid UTF-8");

        let conn = init_db(path_str).expect("file DB should initialize");
        drop(conn);

        assert!(db_path.exists(), "database file should exist");

        // Clean up
        std::fs::remove_dir_all(&dir).expect("should clean up temp dir");
    }
}
