// Migration 4: issue_groups table and fingerprint columns.
//
// Creates the issue_groups table for devtools event deduplication and adds a
// nullable fingerprint column to both log_events and devtools_events. Rows
// predating this migration have NULL fingerprint (backward compatible).
//
// The issue_groups table stores one row per unique fingerprint (derived from
// source + level + message_template + stack_top). The sparkline_buckets and
// recent_event_ids columns store JSON arrays as TEXT.

use sea_orm_migration::prelude::*;

/// Migration 4 — issue_groups table and fingerprint columns.
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m004_issue_groups"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Create issue_groups and add fingerprint to event tables.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
-- Issue groups: deduplicated event fingerprints with occurrence metadata.
CREATE TABLE IF NOT EXISTS issue_groups (
    fingerprint     TEXT PRIMARY KEY,
    title           TEXT NOT NULL,
    component       TEXT NOT NULL,
    level           TEXT NOT NULL,
    first_seen      INTEGER NOT NULL,
    last_seen       INTEGER NOT NULL,
    count           INTEGER NOT NULL DEFAULT 1,
    sparkline_buckets TEXT NOT NULL DEFAULT '[]',
    recent_event_ids TEXT NOT NULL DEFAULT '[]'
);

CREATE INDEX IF NOT EXISTS idx_issue_groups_last_seen ON issue_groups(last_seen DESC);
CREATE INDEX IF NOT EXISTS idx_issue_groups_count ON issue_groups(count DESC);
CREATE INDEX IF NOT EXISTS idx_issue_groups_component ON issue_groups(component);

-- Add fingerprint column to existing event tables (nullable for backward compat).
ALTER TABLE log_events ADD COLUMN fingerprint TEXT;
ALTER TABLE devtools_events ADD COLUMN fingerprint TEXT;
",
        )
        .await?;

        Ok(())
    }

    /// Reverse migration 4 — drop issue_groups.
    ///
    /// Fingerprint columns added to log_events and devtools_events cannot be
    /// removed in SQLite without recreating the tables. Down migration only drops
    /// the issue_groups table; the fingerprint columns are left in place.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP TABLE IF EXISTS issue_groups;")
            .await?;

        Ok(())
    }
}
