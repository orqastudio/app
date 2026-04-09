// Migration 2: log_events table.
//
// Creates the log_events table used by the daemon event bus for persistent
// storage of structured log events from all subsystems. Previously stored in
// a separate events.db; now unified in orqa.db.

use sea_orm_migration::prelude::*;

/// Migration 2 — log_events table.
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &'static str {
        "m002_log_events"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create the log_events table and its indexes.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
CREATE TABLE IF NOT EXISTS log_events (
    id          INTEGER PRIMARY KEY,
    timestamp   INTEGER NOT NULL,
    level       TEXT    NOT NULL,
    source      TEXT    NOT NULL,
    category    TEXT    NOT NULL,
    message     TEXT    NOT NULL,
    metadata    TEXT    NOT NULL,
    session_id  TEXT
);
CREATE INDEX IF NOT EXISTS idx_log_events_timestamp ON log_events(timestamp);
CREATE INDEX IF NOT EXISTS idx_log_events_source    ON log_events(source);
CREATE INDEX IF NOT EXISTS idx_log_events_level     ON log_events(level);
",
        )
        .await?;

        Ok(())
    }

    /// Reverse migration 2 — drop log_events table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP TABLE IF EXISTS log_events;")
            .await?;

        Ok(())
    }
}
