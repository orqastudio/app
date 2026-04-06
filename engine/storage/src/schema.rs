// Unified schema for the orqa-storage database.
//
// All CREATE TABLE and CREATE INDEX statements for the single orqa.db database.
// This consolidates tables from all four previously separate databases:
//   - app/src-tauri: projects, sessions, messages, settings, themes, violations,
//     health_snapshots
//   - daemon/events.db: log_events
//   - devtools/devtools-sessions.db: devtools_sessions, devtools_events
//
// Tables that were purely duplicated in daemon.db (projects, sessions, messages,
// settings) are dropped in favor of this single authoritative copy.

/// Core application tables: projects, sessions, messages, settings, themes,
/// violations, health snapshots. Ported from app/src-tauri/migrations/001–011
/// as a single canonical schema.
pub const CORE_SCHEMA: &str = "
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

-- FTS5 virtual table for full-text search over message content.
CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
    content,
    tool_name,
    content='messages',
    content_rowid='id',
    tokenize='porter unicode61'
);

CREATE TRIGGER IF NOT EXISTS messages_ai AFTER INSERT ON messages BEGIN
    INSERT INTO messages_fts(rowid, content, tool_name)
    VALUES (new.id, new.content, new.tool_name);
END;

CREATE TRIGGER IF NOT EXISTS messages_ad AFTER DELETE ON messages BEGIN
    INSERT INTO messages_fts(messages_fts, rowid, content, tool_name)
    VALUES ('delete', old.id, old.content, old.tool_name);
END;

CREATE TRIGGER IF NOT EXISTS messages_au AFTER UPDATE OF content ON messages BEGIN
    INSERT INTO messages_fts(messages_fts, rowid, content, tool_name)
    VALUES ('delete', old.id, old.content, old.tool_name);
    INSERT INTO messages_fts(rowid, content, tool_name)
    VALUES (new.id, new.content, new.tool_name);
END;

-- Enforcement violations: recorded rule matches (block or warn).
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
-- Tracks outlier-based pipeline health model (outlier count, connectivity).
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

-- Recover any interrupted message streams from a previous crash.
UPDATE messages SET stream_status = 'error' WHERE stream_status = 'pending';
";

/// Log events table: daemon event bus persistence (previously events.db).
/// Stores structured log events from all subsystems for replay and audit.
pub const LOG_EVENTS_SCHEMA: &str = "
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
";

/// Migration 4 — Issue groups table for devtools event deduplication,
/// plus fingerprint column on existing event tables.
///
/// The issue_groups table stores one row per unique fingerprint (derived from
/// source + level + message_template + stack_top). Occurrence metadata (count,
/// first/last seen, sparkline) is maintained by the daemon's event bus as events
/// arrive. The fingerprint columns on log_events and devtools_events are nullable
/// for backward compatibility with rows that predate this migration.
pub const ISSUE_GROUPS_SCHEMA: &str = "
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
";

/// Devtools session tables (previously devtools-sessions.db).
/// Each open/close cycle of the devtools window is one devtools_session row.
pub const DEVTOOLS_SCHEMA: &str = "
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
";
