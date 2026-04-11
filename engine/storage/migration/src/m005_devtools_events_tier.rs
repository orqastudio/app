// Migration 5: add the `tier` column to `devtools_events`.
//
// Migration 3 was updated after it had already been applied on some local
// databases to include a `tier TEXT NOT NULL DEFAULT 'Runtime'` column.
// `CREATE TABLE IF NOT EXISTS` is a no-op on existing tables, so those
// databases never received the new column and every insert now fails with
// "table devtools_events has no column named tier".
//
// This migration adds the column idempotently.  Fresh databases created from
// m003 already have the column — this ALTER is a no-op for them because we
// first check `PRAGMA table_info` for the column's presence.

use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

/// Migration 5 — add `tier` column to `devtools_events` if missing.
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &'static str {
        "m005_devtools_events_tier"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Add `tier` column to `devtools_events` when it is not already present.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Inspect current columns.  SQLite has no "ADD COLUMN IF NOT EXISTS",
        // so we query PRAGMA table_info and only issue the ALTER when needed.
        let rows = db
            .query_all_raw(Statement::from_string(
                DatabaseBackend::Sqlite,
                "PRAGMA table_info(devtools_events)".to_owned(),
            ))
            .await?;

        let has_tier = rows
            .iter()
            .any(|row| row.try_get::<String>("", "name").as_deref() == Ok("tier"));

        if !has_tier {
            db.execute_unprepared(
                "ALTER TABLE devtools_events \
                 ADD COLUMN tier TEXT NOT NULL DEFAULT 'Runtime'",
            )
            .await?;
        }

        Ok(())
    }

    /// Reverse migration 5 — SQLite cannot drop columns without recreating
    /// the table, so this is a no-op.  Dropping `tier` is not worth the
    /// complexity for pre-release data.
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
