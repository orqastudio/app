// Migration 3: devtools_sessions and devtools_events tables.
//
// Creates the devtools tables for capturing UI inspector events. Previously
// stored in a separate devtools-sessions.db; now unified in orqa.db. Each
// open/close cycle of the devtools window produces one devtools_sessions row.

use sea_orm_migration::prelude::*;

/// Migration 3 — devtools_sessions and devtools_events tables.
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &'static str {
        "m003_devtools_sessions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create devtools_sessions and devtools_events tables with their indexes.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
CREATE TABLE IF NOT EXISTS devtools_sessions (
    id          TEXT    PRIMARY KEY,
    started_at  INTEGER NOT NULL,
    ended_at    INTEGER,
    label       TEXT,
    event_count INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_devtools_sessions_started
    ON devtools_sessions(started_at DESC);

CREATE TABLE IF NOT EXISTS devtools_events (
    rowid           INTEGER PRIMARY KEY AUTOINCREMENT,
    original_id     INTEGER NOT NULL,
    session_id      TEXT    NOT NULL
                        REFERENCES devtools_sessions(id) ON DELETE CASCADE,
    timestamp       INTEGER NOT NULL,
    level           TEXT    NOT NULL,
    source          TEXT    NOT NULL,
    tier            TEXT    NOT NULL DEFAULT 'Runtime',
    category        TEXT    NOT NULL,
    message         TEXT    NOT NULL,
    metadata        TEXT    NOT NULL DEFAULT '{}',
    daemon_event_id INTEGER
);
CREATE INDEX IF NOT EXISTS idx_devtools_events_session
    ON devtools_events(session_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_devtools_events_timestamp
    ON devtools_events(timestamp);
CREATE INDEX IF NOT EXISTS idx_devtools_events_level
    ON devtools_events(level);
CREATE INDEX IF NOT EXISTS idx_devtools_events_source
    ON devtools_events(source);
",
        )
        .await?;

        Ok(())
    }

    /// Reverse migration 3 — drop devtools tables.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
DROP TABLE IF EXISTS devtools_events;
DROP TABLE IF EXISTS devtools_sessions;
",
        )
        .await?;

        Ok(())
    }
}
