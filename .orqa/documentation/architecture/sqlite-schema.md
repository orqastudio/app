---
id: DOC-013
title: SQLite Schema
description: SQLite database schema covering sessions, messages, governance artifacts, and configuration tables.
created: "2026-03-02"
updated: "2026-03-04"
---

**Date:** 2026-03-02 | **Updated:** 2026-03-04 | **Status:** Aligned with Phase 1 implementation
**References:** [Persistence Research](RES-006) ([AD-014](AD-014)), [Design Tokens Research](RES-003)

Full table definitions, indexes, FTS5 configuration, and migration strategy for `orqa.db`.

---

## Database Configuration

PRAGMAs are set by `db::init_db()` in `src-tauri/src/db.rs` using `rusqlite` (NOT `tauri-plugin-sql`). The database is opened directly via `rusqlite::Connection::open()` and wrapped in `Mutex<Connection>` inside `AppState`.

```rust
// src-tauri/src/db.rs — init_db()
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

## Core Tables (7 implemented + 4 planned)

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
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
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

### artifacts

Governance artifact metadata. Content lives on disk as markdown.

Hookify rules (`.orqa/governance/hookify.*.local.md`) share `artifact_type = 'hook'` with lifecycle hooks (`.orqa/governance/hooks/`). The `hook_kind` column distinguishes them. This mirrors the UI where both subtypes appear under the single "Hooks" Activity Bar icon. In CLI-only environments, these may reside under `.claude/` as a compatibility layer.

```sql
CREATE TABLE IF NOT EXISTS artifacts (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    artifact_type   TEXT NOT NULL CHECK (artifact_type IN ('agent', 'rule', 'skill', 'hook', 'doc')),
    rel_path        TEXT NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT,
    hook_kind       TEXT CHECK (hook_kind IN ('lifecycle', 'hookify')),
    file_hash       TEXT,
    file_size       INTEGER,
    file_modified_at TEXT,
    last_scanned_at TEXT,
    compliance_status TEXT DEFAULT 'unknown'
                    CHECK (compliance_status IN ('compliant', 'non_compliant', 'unknown', 'error')),
    relationships   TEXT,                   -- JSON: [{"type":"references","target":"path"}]
    metadata        TEXT,                   -- JSON: extracted frontmatter
    last_edited_by  TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_artifacts_path ON artifacts(project_id, rel_path);
CREATE INDEX IF NOT EXISTS idx_artifacts_type ON artifacts(project_id, artifact_type);
CREATE INDEX IF NOT EXISTS idx_artifacts_hook_kind ON artifacts(project_id, hook_kind)
    WHERE hook_kind IS NOT NULL;
```

### scanner_results (NOT YET CREATED — Phase 3+)

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

### metrics (NOT YET CREATED — Phase 5)

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

### tasks (NOT YET CREATED — Phase 1+)

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

### lessons (NOT YET CREATED — Phase 5)

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

---

## FTS5 Virtual Tables (2)

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

### artifacts_fts

Contentless FTS5 — index only, no content stored. Content read from disk on demand.

```sql
CREATE VIRTUAL TABLE IF NOT EXISTS artifacts_fts USING fts5(
    name,
    content,
    content='',
    contentless_delete=1,
    tokenize='porter unicode61'
);
```

---

## Migration Strategy

### Approach

Numbered SQL migration files in `src-tauri/migrations/`, executed by `db::init_db()` using `rusqlite` directly (NOT `tauri-plugin-sql`). Each migration uses `IF NOT EXISTS` to be idempotent and re-runnable.

```
src-tauri/migrations/
  001_initial_schema.sql       # All Phase 1 tables, indexes, triggers, FTS5, stream recovery
```

> **Note:** There is no `002_add_themes.sql`. The `project_themes` and `project_theme_overrides` tables are included in `001_initial_schema.sql`.

### Migration Execution

`db::init_db()` opens the database with `rusqlite::Connection::open()`, applies PRAGMAs, then executes the migration SQL via `include_str!()`:

```rust
// src-tauri/src/db.rs
pub fn init_db(path: &str) -> Result<Connection, OrqaError> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA journal_mode = WAL; ...");
    conn.execute_batch(include_str!("../migrations/001_initial_schema.sql"))?;
    Ok(conn)
}
```

The database path is `orqa.db` in the Tauri app data directory, resolved during `.setup()` in `lib.rs`.

For tests, `db::init_memory_db()` creates an in-memory SQLite database with the same schema.

### Migration 001: Initial Schema

Contains all 7 implemented tables (`projects`, `sessions`, `messages`, `artifacts`, `settings`, `project_themes`, `project_theme_overrides`), 2 FTS5 virtual tables (`messages_fts`, `artifacts_fts`), all indexes, sync triggers, and the stream recovery statement. All statements use `IF NOT EXISTS` to be safely re-runnable on app restart.

Ends with a stream recovery statement:

```sql
UPDATE messages SET stream_status = 'error'
WHERE stream_status = 'pending';
```

### Rules

- Migrations are append-only. Never modify a deployed migration.
- Each migration is idempotent (`CREATE TABLE IF NOT EXISTS`, `CREATE INDEX IF NOT EXISTS`, `CREATE TRIGGER IF NOT EXISTS`).
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

## Global Store (Phase 5)

Cross-project learning requires app-level storage outside any per-project `orqa.db` database. A global SQLite database (e.g., `~/.orqa/global.db`) would store:

- **Global lessons** — Lessons promoted from project scope, with a reference to their source project
- **Global rules** — Rules promoted from project-local rules for cross-project enforcement
- **Cross-project metrics and patterns** — Aggregated data across projects
- **Tag-based categorization** — Language, framework, and domain tags for relevance matching when onboarding new projects

This database is separate from per-project `orqa.db` files. Global lessons reference their origin project but are not owned by it — deleting a project does not remove its contributed global lessons.

Schema design for `global.db` will be specified when Phase 5 implementation begins. The key constraint is that per-project databases remain self-contained; the global store is additive.

---

## Table Summary

| Table | Rows (est. 1yr heavy use) | Phase | Status |
|-------|---------------------------|-------|--------|
| projects | 10-50 | 1 | Implemented |
| sessions | 1,000-5,000 | 1 | Implemented |
| messages | 100,000-500,000 | 1 | Implemented |
| artifacts | 50-500 per project | 1 | Implemented |
| settings | 20-50 | 1 | Implemented |
| project_themes | 1-5 per project | 1 | Implemented |
| project_theme_overrides | 0-30 per project | 1 | Implemented |
| messages_fts | (mirrors messages) | 1 | Implemented |
| artifacts_fts | (mirrors artifacts) | 1 | Implemented |
| scanner_results | 1,000-10,000 | 3 | Not yet created |
| tasks | 100-1,000 per project | 1+ | Not yet created |
| metrics | 10,000-100,000 | 5 | Not yet created |
| lessons | 50-500 per project | 5 | Not yet created |
