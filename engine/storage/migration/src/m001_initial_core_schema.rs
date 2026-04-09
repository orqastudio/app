// Migration 1: initial core schema.
//
// Creates the foundational tables for OrqaStudio's unified SQLite database:
// projects, sessions, messages, settings, project_themes, project_theme_overrides,
// enforcement_violations, health_snapshots, plus the FTS5 virtual table and its
// three triggers for full-text search over message content.
//
// Also runs the crash-recovery UPDATE to mark any messages stuck in 'pending'
// stream_status (from a prior crash) as 'error'.

use sea_orm_migration::prelude::*;

/// Migration 1 — initial core schema.
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &'static str {
        "m001_initial_core_schema"
    }
}

#[async_trait::async_trait]
#[allow(clippy::too_many_lines)]
impl MigrationTrait for Migration {
    /// Apply the initial core schema.
    ///
    /// Uses execute_unprepared for the entire migration because it contains
    /// FTS5 virtual tables and triggers that cannot be expressed via SeaORM's
    /// schema builder API.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
-- Projects: top-level container for sessions and governance artifacts.
CREATE TABLE IF NOT EXISTS projects (
    id              INTEGER PRIMARY KEY,
    name            TEXT NOT NULL,
    path            TEXT NOT NULL UNIQUE,
    description     TEXT,
    detected_stack  TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_projects_path ON projects(path);

-- Sessions: a single interaction context between a user and an LLM agent.
CREATE TABLE IF NOT EXISTS sessions (
    id                  INTEGER PRIMARY KEY,
    project_id          INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id             TEXT,
    title               TEXT,
    model               TEXT NOT NULL DEFAULT 'auto',
    system_prompt       TEXT,
    status              TEXT NOT NULL DEFAULT 'active'
                        CHECK (status IN ('active', 'completed', 'abandoned', 'error')),
    summary             TEXT,
    handoff_notes       TEXT,
    total_input_tokens  INTEGER DEFAULT 0,
    total_output_tokens INTEGER DEFAULT 0,
    total_cost_usd      REAL DEFAULT 0.0,
    provider_session_id TEXT,
    title_manually_set  INTEGER DEFAULT 0,
    created_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_id);
CREATE INDEX IF NOT EXISTS idx_sessions_created ON sessions(created_at);
CREATE INDEX IF NOT EXISTS idx_sessions_status ON sessions(status);

-- Messages: content blocks within a session turn.
CREATE TABLE IF NOT EXISTS messages (
    id              INTEGER PRIMARY KEY,
    session_id      INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    role            TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content_type    TEXT NOT NULL DEFAULT 'text'
                    CHECK (content_type IN ('text', 'tool_use', 'tool_result', 'thinking', 'image')),
    content         TEXT,
    tool_call_id    TEXT,
    tool_name       TEXT,
    tool_input      TEXT,
    tool_is_error   INTEGER DEFAULT 0,
    turn_index      INTEGER NOT NULL DEFAULT 0,
    block_index     INTEGER NOT NULL DEFAULT 0,
    stream_status   TEXT NOT NULL DEFAULT 'complete'
                    CHECK (stream_status IN ('pending', 'complete', 'error')),
    input_tokens    INTEGER,
    output_tokens   INTEGER,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id, turn_index, block_index);
CREATE INDEX IF NOT EXISTS idx_messages_tool ON messages(tool_name) WHERE tool_name IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_messages_stream ON messages(stream_status) WHERE stream_status = 'pending';

-- Settings: key-value pairs with scope (e.g., 'app', 'project:123').
CREATE TABLE IF NOT EXISTS settings (
    key             TEXT NOT NULL,
    value           TEXT NOT NULL,
    scope           TEXT NOT NULL DEFAULT 'app',
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    PRIMARY KEY (key, scope)
);

-- Project themes: design token maps extracted from source files.
CREATE TABLE IF NOT EXISTS project_themes (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_file     TEXT NOT NULL,
    source_hash     TEXT NOT NULL,
    extracted_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    tokens_light    TEXT NOT NULL,
    tokens_dark     TEXT,
    unmapped        TEXT,
    is_active       INTEGER NOT NULL DEFAULT 1
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_themes_project_source ON project_themes(project_id, source_file);
CREATE INDEX IF NOT EXISTS idx_themes_active ON project_themes(project_id, is_active);

-- Project theme overrides: per-project token value overrides.
CREATE TABLE IF NOT EXISTS project_theme_overrides (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    token_name      TEXT NOT NULL,
    value_light     TEXT NOT NULL,
    value_dark      TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_overrides_project_token ON project_theme_overrides(project_id, token_name);

-- Enforcement violations: recorded rule matches (block or warn).
-- No ON DELETE CASCADE — violations are audit records and must be retained.
CREATE TABLE IF NOT EXISTS enforcement_violations (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id  INTEGER NOT NULL REFERENCES projects(id),
    rule_name   TEXT NOT NULL,
    action      TEXT NOT NULL,
    tool_name   TEXT NOT NULL,
    detail      TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Health snapshots: point-in-time graph health metrics.
CREATE TABLE IF NOT EXISTS health_snapshots (
    id                      INTEGER PRIMARY KEY,
    project_id              INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    node_count              INTEGER NOT NULL DEFAULT 0,
    edge_count              INTEGER NOT NULL DEFAULT 0,
    broken_ref_count        INTEGER NOT NULL DEFAULT 0,
    error_count             INTEGER NOT NULL DEFAULT 0,
    warning_count           INTEGER NOT NULL DEFAULT 0,
    largest_component_ratio REAL NOT NULL DEFAULT 0.0,
    avg_degree              REAL NOT NULL DEFAULT 0.0,
    pillar_traceability     REAL NOT NULL DEFAULT 100.0,
    outlier_count           INTEGER NOT NULL DEFAULT 0,
    outlier_percentage      REAL NOT NULL DEFAULT 0.0,
    delivery_connectivity   REAL NOT NULL DEFAULT 0.0,
    learning_connectivity   REAL NOT NULL DEFAULT 0.0,
    created_at              TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_health_snapshots_project
    ON health_snapshots(project_id, id DESC);

-- FTS5 virtual table for full-text search over message content.
-- SQLite-only: uses fts5 extension with porter stemmer.
CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
    content,
    tool_name,
    content='messages',
    content_rowid='id',
    tokenize='porter unicode61'
);

-- Trigger: keep FTS5 index in sync after INSERT on messages.
CREATE TRIGGER IF NOT EXISTS messages_ai AFTER INSERT ON messages BEGIN
    INSERT INTO messages_fts(rowid, content, tool_name)
    VALUES (new.id, new.content, new.tool_name);
END;

-- Trigger: keep FTS5 index in sync after DELETE on messages.
CREATE TRIGGER IF NOT EXISTS messages_ad AFTER DELETE ON messages BEGIN
    INSERT INTO messages_fts(messages_fts, rowid, content, tool_name)
    VALUES ('delete', old.id, old.content, old.tool_name);
END;

-- Trigger: keep FTS5 index in sync after UPDATE of message content.
CREATE TRIGGER IF NOT EXISTS messages_au AFTER UPDATE OF content ON messages BEGIN
    INSERT INTO messages_fts(messages_fts, rowid, content, tool_name)
    VALUES ('delete', old.id, old.content, old.tool_name);
    INSERT INTO messages_fts(rowid, content, tool_name)
    VALUES (new.id, new.content, new.tool_name);
END;

-- Crash recovery: mark any messages stuck in 'pending' as 'error'.
-- This handles daemon restarts that interrupted an in-progress stream.
UPDATE messages SET stream_status = 'error' WHERE stream_status = 'pending';
",
        )
        .await?;

        Ok(())
    }

    /// Reverse migration 1 — drop all core tables.
    ///
    /// Drops in dependency order (children before parents) to satisfy foreign keys.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
DROP TRIGGER IF EXISTS messages_au;
DROP TRIGGER IF EXISTS messages_ad;
DROP TRIGGER IF EXISTS messages_ai;
DROP TABLE IF EXISTS messages_fts;
DROP TABLE IF EXISTS health_snapshots;
DROP TABLE IF EXISTS enforcement_violations;
DROP TABLE IF EXISTS project_theme_overrides;
DROP TABLE IF EXISTS project_themes;
DROP TABLE IF EXISTS settings;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS projects;
",
        )
        .await?;

        Ok(())
    }
}
