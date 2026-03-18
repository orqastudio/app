---
id: DOC-013
title: SQLite Schema
category: reference
description: "SQLite database schema covering sessions, messages, governance artifacts, and configuration tables."
created: 2026-03-02
updated: 2026-03-10
sort: 4
relationships:
  - target: AD-014
    type: documents
    rationale: Documentation page references AD-014
---


**Date:** 2026-03-02 | **Updated:** 2026-03-10 | **Status:** Current
**References:** [Persistence Research](RES-006) [AD-014](AD-014), [Design Tokens Research](RES-003)

Full table definitions, indexes, FTS5 configuration, and migration strategy for `orqa.db`.

---


## Database Configuration

PRAGMAs are set by `db::init_db()` in `backend/src-tauri/src/db.rs` using `rusqlite` (NOT `tauri-plugin-sql`). The database is opened directly via `rusqlite::Connection::open()` and wrapped in `Mutex<Connection>` inside `AppState`.

```rust
// backend/src-tauri/src/db.rs — init_db()
conn.execute_batch("
    PRAGMA journal_mode = WAL;
    PRAGMA foreign_keys = ON;
    PRAGMA busy_timeout = 5000;
    PRAGMA synchronous = NORMAL;
    PRAGMA cache_size = -8000;
    PRAGMA temp_store = MEMORY;
");
```

**WAL mode** is essential for streaming — it allows the UI to read session data while new tokens are being written.

---


## Core Tables (8 implemented + 3 planned)

### projects

```sql
CREATE TABLE IF NOT EXISTS projects (
    id              INTEGER PRIMARY KEY,
    name            TEXT NOT NULL,
    path            TEXT NOT NULL UNIQUE,
    description     TEXT,
    detected_stack  TEXT,               -- JSON: {"languages":["Rust","TypeScript"],"frameworks":["Tauri","Svelte"],...}
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_projects_path ON projects(path);
```

### sessions

```sql
CREATE TABLE IF NOT EXISTS sessions (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id         TEXT,                       -- nullable: future multi-user
    title           TEXT,
    model           TEXT NOT NULL DEFAULT 'auto',
    system_prompt   TEXT,
    status          TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active', 'completed', 'abandoned', 'error')),
    summary         TEXT,                       -- Claude-generated on session end
    handoff_notes   TEXT,                       -- session continuity context
    total_input_tokens  INTEGER DEFAULT 0,
    total_output_tokens INTEGER DEFAULT 0,
    total_cost_usd  REAL DEFAULT 0.0,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    -- Added by migration 006 (renamed from sdk_session_id added in migration 004)
    provider_session_id TEXT,                   -- Claude Agent SDK UUID for conversation resumption
    -- Added by migration 005
    title_manually_set  INTEGER DEFAULT 0       -- 1 if user explicitly renamed; prevents auto-overwrite
);

CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_id);
CREATE INDEX IF NOT EXISTS idx_sessions_created ON sessions(created_at);
CREATE INDEX IF NOT EXISTS idx_sessions_status ON sessions(status);
```

### messages

One row per content block (not per API message). A single assistant turn with text + tool_use produces multiple rows.

```sql
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
```

### scanner_results (NOT YET CREATED — post-MVP)

> This table is planned but does **not** exist in `001_initial_schema.sql`. It will be added in a future migration when scanner features are implemented.

```sql
-- PLANNED — not in current schema
CREATE TABLE scanner_results (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    scanner_name    TEXT NOT NULL,
    passed          INTEGER NOT NULL,       -- 0 or 1
    score           REAL,                   -- 0.0 - 1.0
    details         TEXT,                   -- JSON: scanner-specific findings
    failed_items    TEXT,                   -- JSON: array of artifact IDs/paths
    duration_ms     INTEGER,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
```

### metrics (NOT YET CREATED — post-MVP)

> This table is planned but does **not** exist in `001_initial_schema.sql`. It will be added in a future migration when metrics features are implemented.

```sql
-- PLANNED — not in current schema
CREATE TABLE metrics (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    metric_name     TEXT NOT NULL,
    value           REAL NOT NULL,
    unit            TEXT,                   -- "percent", "count", "seconds"
    dimensions      TEXT,                   -- JSON: {"agent":"backend-engineer"}
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
```

### tasks (NOT YET CREATED — post-MVP)

> This table is planned but does **not** exist in `001_initial_schema.sql`. It will be added in a future migration when task tracking features are implemented.

```sql
-- PLANNED — not in current schema
CREATE TABLE tasks (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    session_id      INTEGER REFERENCES sessions(id) ON DELETE SET NULL,
    title           TEXT NOT NULL,
    description     TEXT,
    status          TEXT NOT NULL DEFAULT 'pending'
                    CHECK (status IN ('pending', 'in_progress', 'blocked', 'completed', 'cancelled')),
    priority        INTEGER DEFAULT 0,
    assigned_agent  TEXT,
    metadata        TEXT,                   -- JSON
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    completed_at    TEXT
);
```

### lessons (NOT YET CREATED — post-MVP)

> This table is planned but does **not** exist in `001_initial_schema.sql`. It will be added in a future migration when self-learning loop features are implemented.

```sql
-- PLANNED — not in current schema
CREATE TABLE lessons (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    session_id      INTEGER REFERENCES sessions(id) ON DELETE SET NULL,
    title           TEXT NOT NULL,
    pattern         TEXT NOT NULL,           -- problem pattern
    fix             TEXT NOT NULL,           -- correct approach
    occurrence_count INTEGER NOT NULL DEFAULT 1,
    last_occurred_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    severity        TEXT DEFAULT 'medium'
                    CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    metadata        TEXT,                   -- JSON
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
```

### settings

```sql
CREATE TABLE IF NOT EXISTS settings (
    key             TEXT NOT NULL,
    value           TEXT NOT NULL,           -- JSON value
    scope           TEXT NOT NULL DEFAULT 'app',  -- 'app' or project_id
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    PRIMARY KEY (key, scope)
);
```

### project_themes

Per-project design token storage. See [Design Tokens Research](RES-003) Q4.

```sql
CREATE TABLE IF NOT EXISTS project_themes (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_file     TEXT NOT NULL,
    source_hash     TEXT NOT NULL,
    extracted_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    tokens_light    TEXT NOT NULL,           -- JSON: {"primary":"oklch(...)","background":"oklch(...)"}
    tokens_dark     TEXT,                    -- JSON: dark mode overrides (nullable)
    unmapped        TEXT,                    -- JSON: raw values that couldn't be mapped
    is_active       INTEGER NOT NULL DEFAULT 1
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_themes_project_source ON project_themes(project_id, source_file);
CREATE INDEX IF NOT EXISTS idx_themes_active ON project_themes(project_id, is_active);
```

### project_theme_overrides

User manual overrides for auto-extracted tokens.

```sql
CREATE TABLE IF NOT EXISTS project_theme_overrides (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    token_name      TEXT NOT NULL,
    value_light     TEXT NOT NULL,           -- OKLCH value
    value_dark      TEXT,                    -- OKLCH value (nullable)
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_overrides_project_token ON project_theme_overrides(project_id, token_name);
```

### governance_analyses

Stores the output of a governance analysis run — the scan data, summary, strengths, and gaps. Added in migration 002.

```sql
CREATE TABLE IF NOT EXISTS governance_analyses (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    scan_data       TEXT NOT NULL,
    summary         TEXT NOT NULL,
    strengths       TEXT NOT NULL,
    gaps            TEXT NOT NULL,
    session_id      INTEGER REFERENCES sessions(id),
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_governance_analyses_project
    ON governance_analyses(project_id, created_at);
```

### governance_recommendations

Stores Claude-generated recommendations from a governance analysis. Each recommendation has a workflow status (`pending` → `approved`/`rejected` → `applied`). Added in migration 002.

```sql
CREATE TABLE IF NOT EXISTS governance_recommendations (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    analysis_id     INTEGER NOT NULL REFERENCES governance_analyses(id) ON DELETE CASCADE,
    category        TEXT NOT NULL,
    priority        TEXT NOT NULL
                    CHECK (priority IN ('critical', 'recommended', 'optional')),
    title           TEXT NOT NULL,
    description     TEXT NOT NULL,
    artifact_type   TEXT NOT NULL,
    target_path     TEXT NOT NULL,
    content         TEXT NOT NULL,
    rationale       TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'pending'
                    CHECK (status IN ('pending', 'approved', 'rejected', 'applied')),
    applied_at      TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_governance_recommendations_project
    ON governance_recommendations(project_id, status);
CREATE INDEX IF NOT EXISTS idx_governance_recommendations_analysis
    ON governance_recommendations(analysis_id);
```

### enforcement_violations

Tracks every rule enforcement event that occurs during tool execution — whether a rule blocked a tool call or issued a warning. Used for auditing which rules triggered and for the self-learning loop. Added in migration 003.

```sql
CREATE TABLE IF NOT EXISTS enforcement_violations (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    rule_name  TEXT    NOT NULL,
    action     TEXT    NOT NULL,   -- 'block' or 'warn'
    tool_name  TEXT    NOT NULL,   -- e.g. 'write_file', 'bash'
    detail     TEXT,               -- file path, command snippet, etc.
    created_at TEXT    NOT NULL DEFAULT (datetime('now'))
);
```

---


## FTS5 Virtual Tables (1)

### messages_fts

External content FTS5 — references `messages` table, no content duplication.

```sql
CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
    content,
    tool_name,
    content='messages',
    content_rowid='id',
    tokenize='porter unicode61'
);

-- Sync triggers
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
```

---


## Migration Strategy

### Approach

Numbered SQL migration files in `backend/src-tauri/migrations/`, executed by `db::init_db()` using `rusqlite` directly (NOT `tauri-plugin-sql`). Migrations 001–003 are loaded via `include_str!()` and are fully idempotent. Migrations 004–006 are implemented as Rust functions in `backend/src-tauri/src/db.rs` using `pragma_table_info` guards because SQLite does not support `ALTER TABLE ADD/RENAME COLUMN IF NOT EXISTS`.

```
backend/src-tauri/migrations/
  001_initial_schema.sql         # Core tables, indexes, FTS5 virtual tables, stream recovery
  002_governance_bootstrap.sql   # governance_analyses, governance_recommendations
  003_enforcement.sql            # enforcement_violations
  004_sdk_session_id.sql         # (stub — actual logic in run_migration_004 Rust fn)
  005_title_manually_set.sql     # (stub — actual logic in run_migration_005 Rust fn)
```

Migration 006 (rename `sdk_session_id` → `provider_session_id`) has no corresponding `.sql` file; it is implemented entirely as `run_migration_006()` in `db.rs`.

### Migration Execution

`db::init_db()` opens the database with `rusqlite::Connection::open()`, applies PRAGMAs, then runs all migrations in order:

```rust
// backend/src-tauri/src/db.rs
pub fn init_db(path: &str) -> Result<Connection, OrqaError> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA journal_mode = WAL; ...");
    conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))?;
    conn.execute_batch(include_str!("../migrations/002_governance_bootstrap.sql"))?;
    conn.execute_batch(include_str!("../migrations/003_enforcement.sql"))?;
    run_migration_004(&conn)?;
    run_migration_005(&conn)?;
    run_migration_006(&conn)?;
    conn.execute_batch(include_str!("../migrations/007_drop_governance_tables.sql"))?;
    conn.execute_batch(include_str!("../migrations/008_health_snapshots.sql"))?;
    conn.execute_batch(include_str!("../migrations/009_drop_artifacts_table.sql"))?;
    Ok(conn)
}
```

The database path is `orqa.db` in the Tauri app data directory, resolved during `.setup()` in `lib.rs`.

For tests, `db::init_memory_db()` creates an in-memory SQLite database with the same schema, running all 9 migrations identically.

### Migration 001: Initial Schema

Contains the 7 core tables (`projects`, `sessions`, `messages`, `artifacts`, `settings`, `project_themes`, `project_theme_overrides`), 2 FTS5 virtual tables (`messages_fts`, `artifacts_fts`), all indexes, sync triggers, and the stream recovery statement.

> **Note:** `artifacts` and `artifacts_fts` created here are dropped by migration 009. They exist in this file for schema history integrity — deployed databases that ran 001 need 009 to clean them up.

Ends with a stream recovery statement:

```sql
UPDATE messages SET stream_status = 'error'
WHERE stream_status = 'pending';
```

### Migration 002: Governance Bootstrap

Adds `governance_analyses` and `governance_recommendations` tables. These are dropped by migration 007.

### Migration 003: Enforcement Violations

Adds the `enforcement_violations` table which tracks every rule enforcement event (block or warn) during tool execution. Used for auditing which rules triggered and for the self-learning loop.

### Migration 004: `sdk_session_id` Column

Adds `sdk_session_id TEXT` to `sessions` if neither `sdk_session_id` nor `provider_session_id` already exists. Implemented as `run_migration_004()` in `db.rs` due to SQLite's lack of `ADD COLUMN IF NOT EXISTS`.

### Migration 005: `title_manually_set` Column

Adds `title_manually_set INTEGER DEFAULT 0` to `sessions`. Tracks whether the user explicitly renamed the session to prevent auto-naming from overwriting user titles. Implemented as `run_migration_005()` in `db.rs`.

### Migration 006: Rename Column

Renames `sdk_session_id` → `provider_session_id` on the `sessions` table. Implemented as `run_migration_006()` in `db.rs`. Requires SQLite 3.25.0+ for `ALTER TABLE RENAME COLUMN` support.

### Migration 007: Drop Governance Tables

Drops `governance_recommendations` and `governance_analyses`. These were added in migration 002 and removed per [AD-032](AD-032) (SQLite is for conversation persistence only).

### Migration 008: Health Snapshots

Adds `health_snapshots` table for storing graph integrity metric trends over time.

### Migration 009: Drop Artifacts Table

Drops `artifacts_fts` and `artifacts`. Artifacts are file-based (`.orqa/`) — SQLite persistence is not needed per [AD-032](AD-032).

### Rules

- Migrations are append-only. Never modify a deployed migration.
- Migrations 001–003 and 007–009 use `IF NOT EXISTS` / `IF EXISTS` throughout and are safely re-runnable.
- Migrations 004–006 check `pragma_table_info` before altering to achieve idempotency.
- Destructive changes (column removal, type changes) require a new migration that copies data.
- Test migrations against an empty database and against the previous version.

---


## Streaming Write Pattern

During active streaming, tokens are buffered in Rust and flushed to SQLite periodically:

```
Token arrival → In-memory buffer → Flush every ~500ms → UPDATE messages SET content = ?
                                                      → SET stream_status = 'complete' on finish
```

On app startup, recover interrupted streams:

```sql
UPDATE messages SET stream_status = 'error'
WHERE stream_status = 'pending';
```

---


## Query Patterns

### Session List (Chat Panel dropdown)

```sql
SELECT id, title, status, created_at,
       (SELECT COUNT(*) FROM messages WHERE session_id = s.id) as message_count,
       (SELECT content FROM messages WHERE session_id = s.id AND role = 'user'
        ORDER BY turn_index LIMIT 1) as preview
FROM sessions s
WHERE project_id = ?
ORDER BY updated_at DESC;
```

### Cross-Session Search (Ctrl+K)

```sql
SELECT m.id, m.content, m.session_id, s.title,
       highlight(messages_fts, 0, '<mark>', '</mark>') as highlighted,
       bm25(messages_fts) as rank
FROM messages_fts
JOIN messages m ON m.id = messages_fts.rowid
JOIN sessions s ON s.id = m.session_id
WHERE messages_fts MATCH ?
  AND s.project_id = ?
ORDER BY rank
LIMIT 50;
```

### Tool Call History

```sql
SELECT tool_name, COUNT(*) as count,
       SUM(CASE WHEN tool_is_error = 1 THEN 1 ELSE 0 END) as errors
FROM messages
WHERE session_id = ? AND content_type IN ('tool_use', 'tool_result')
GROUP BY tool_name;
```

### Theme Resolution

```sql
SELECT pt.tokens_light, pt.tokens_dark, pt.unmapped,
       pto.token_name, pto.value_light, pto.value_dark
FROM project_themes pt
LEFT JOIN project_theme_overrides pto ON pto.project_id = pt.project_id
WHERE pt.project_id = ? AND pt.is_active = 1;
```

---


## Global Store (post-MVP)

Cross-project learning requires app-level storage outside any per-project `orqa.db` database. A global SQLite database (e.g., `~/.orqa/global.db`) would store:

- **Global lessons** — Lessons promoted from project scope, with a reference to their source project
- **Global rules** — Rules promoted from project-local rules for cross-project enforcement
- **Cross-project metrics and patterns** — Aggregated data across projects
- **Tag-based categorization** — Language, framework, and domain tags for relevance matching when onboarding new projects

This database is separate from per-project `orqa.db` files. Global lessons reference their origin project but are not owned by it — deleting a project does not remove its contributed global lessons.

Schema design for `global.db` will be specified when cross-project learning features are implemented. The key constraint is that per-project databases remain self-contained; the global store is additive.

---


## Table Summary

| Table | Rows (est. 1yr heavy use) | Migration | Status |
|-------|---------------------------|-----------|--------|
| projects | 10-50 | 001 | Implemented |
| sessions | 1,000-5,000 | 001 | Implemented |
| messages | 100,000-500,000 | 001 | Implemented |
| settings | 20-50 | 001 | Implemented |
| project_themes | 1-5 per project | 001 | Implemented |
| project_theme_overrides | 0-30 per project | 001 | Implemented |
| messages_fts | (mirrors messages) | 001 | Implemented |
| enforcement_violations | 1,000-50,000 | 003 | Implemented |
| health_snapshots | 100-10,000 | 008 | Implemented |
| artifacts | — | 001→009 | Dropped (file-based) |
| artifacts_fts | — | 001→009 | Dropped (file-based) |
| governance_analyses | — | 002→007 | Dropped (file-based) |
| governance_recommendations | — | 002→007 | Dropped (file-based) |
| scanner_results | 1,000-10,000 | PLANNED | Not yet created |
| tasks | 100-1,000 per project | PLANNED | Not yet created |
| metrics | 10,000-100,000 | PLANNED | Not yet created |
| lessons | 50-500 per project | PLANNED | Not yet created |
