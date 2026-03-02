# SQLite Schema

**Date:** 2026-03-02 | **Status:** Phase 0e specification
**References:** [Persistence Research](/research/persistence) (AD-014), [Design Tokens Research](/research/design-tokens)

Full table definitions, indexes, FTS5 configuration, and migration strategy for `forge.db`.

---

## Database Configuration

```sql
-- Connection initialization (run on every connection open)
PRAGMA journal_mode = WAL;           -- Concurrent reads during streaming writes
PRAGMA foreign_keys = ON;            -- Enforce referential integrity
PRAGMA busy_timeout = 5000;          -- 5s retry on lock contention
PRAGMA synchronous = NORMAL;         -- Safe with WAL, better write performance
PRAGMA cache_size = -8000;           -- 8MB page cache
PRAGMA temp_store = MEMORY;          -- Temp tables in memory
```

**WAL mode** is essential for streaming — it allows the UI to read session data while new tokens are being written.

---

## Core Tables (11)

### projects

```sql
CREATE TABLE projects (
    id              INTEGER PRIMARY KEY,
    name            TEXT NOT NULL,
    path            TEXT NOT NULL UNIQUE,
    description     TEXT,
    -- Tier 1 scan results (JSON)
    detected_stack  TEXT,               -- {"languages":["Rust","TypeScript"],"frameworks":["Tauri","Svelte"],...}
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX idx_projects_path ON projects(path);
```

### sessions

```sql
CREATE TABLE sessions (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id         TEXT,                       -- nullable: future multi-user
    title           TEXT,
    model           TEXT NOT NULL DEFAULT 'claude-sonnet-4-20250514',
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

CREATE INDEX idx_sessions_project ON sessions(project_id);
CREATE INDEX idx_sessions_created ON sessions(created_at);
CREATE INDEX idx_sessions_status ON sessions(status);
```

### messages

One row per content block (not per API message). A single assistant turn with text + tool_use produces multiple rows.

```sql
CREATE TABLE messages (
    id              INTEGER PRIMARY KEY,
    session_id      INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    role            TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content_type    TEXT NOT NULL DEFAULT 'text'
                    CHECK (content_type IN ('text', 'tool_use', 'tool_result', 'thinking', 'image')),
    content         TEXT,                   -- text/tool_result/thinking content
    -- Tool-specific (null for non-tool messages)
    tool_call_id    TEXT,                   -- Claude's tool_use id
    tool_name       TEXT,                   -- "Read", "Edit", "Bash", etc.
    tool_input      TEXT,                   -- JSON input object
    tool_is_error   INTEGER DEFAULT 0,      -- 1 if tool_result is an error
    -- Ordering
    turn_index      INTEGER NOT NULL DEFAULT 0,
    block_index     INTEGER NOT NULL DEFAULT 0,
    -- Streaming
    stream_status   TEXT NOT NULL DEFAULT 'complete'
                    CHECK (stream_status IN ('pending', 'complete', 'error')),
    -- Token usage (assistant messages only)
    input_tokens    INTEGER,
    output_tokens   INTEGER,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX idx_messages_session ON messages(session_id, turn_index, block_index);
CREATE INDEX idx_messages_tool ON messages(tool_name) WHERE tool_name IS NOT NULL;
CREATE INDEX idx_messages_stream ON messages(stream_status) WHERE stream_status = 'pending';
```

### artifacts

Governance artifact metadata. Content lives on disk as markdown.

```sql
CREATE TABLE artifacts (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    artifact_type   TEXT NOT NULL CHECK (artifact_type IN ('agent', 'rule', 'skill', 'hook', 'doc')),
    rel_path        TEXT NOT NULL,          -- e.g. ".claude/agents/backend-engineer.md"
    name            TEXT NOT NULL,
    description     TEXT,
    file_hash       TEXT,                   -- SHA-256 for change detection
    file_size       INTEGER,
    file_modified_at TEXT,
    last_scanned_at TEXT,
    compliance_status TEXT DEFAULT 'unknown'
                    CHECK (compliance_status IN ('compliant', 'non_compliant', 'unknown', 'error')),
    relationships   TEXT,                   -- JSON: [{"type":"references","target":"path"}]
    metadata        TEXT,                   -- JSON: extracted frontmatter
    last_edited_by  TEXT,                   -- future multi-user
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX idx_artifacts_path ON artifacts(project_id, rel_path);
CREATE INDEX idx_artifacts_type ON artifacts(project_id, artifact_type);
```

### scanner_results

```sql
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

CREATE INDEX idx_scanner_project ON scanner_results(project_id, scanner_name);
CREATE INDEX idx_scanner_created ON scanner_results(created_at);
```

### metrics

```sql
CREATE TABLE metrics (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    metric_name     TEXT NOT NULL,
    value           REAL NOT NULL,
    unit            TEXT,                   -- "percent", "count", "seconds"
    dimensions      TEXT,                   -- JSON: {"agent":"backend-engineer"}
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX idx_metrics_project ON metrics(project_id, metric_name);
CREATE INDEX idx_metrics_created ON metrics(created_at);
```

### tasks

```sql
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

CREATE INDEX idx_tasks_project ON tasks(project_id, status);
CREATE INDEX idx_tasks_session ON tasks(session_id) WHERE session_id IS NOT NULL;
```

### lessons

```sql
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

CREATE INDEX idx_lessons_project ON lessons(project_id);
CREATE INDEX idx_lessons_recurrence ON lessons(occurrence_count DESC);
```

### settings

```sql
CREATE TABLE settings (
    key             TEXT NOT NULL,
    value           TEXT NOT NULL,           -- JSON value
    scope           TEXT NOT NULL DEFAULT 'app',  -- 'app' or project_id
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    PRIMARY KEY (key, scope)
);
```

### project_themes

Per-project design token storage. See [Design Tokens Research](/research/design-tokens) Q4.

```sql
CREATE TABLE project_themes (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_file     TEXT NOT NULL,           -- e.g. "tailwind.config.ts", "src/app.css"
    source_hash     TEXT NOT NULL,           -- SHA-256 for cache invalidation
    extracted_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    tokens_light    TEXT NOT NULL,           -- JSON: {"primary":"oklch(...)","background":"oklch(...)"}
    tokens_dark     TEXT,                    -- JSON: dark mode overrides (nullable)
    unmapped        TEXT,                    -- JSON: raw values that couldn't be mapped
    is_active       INTEGER NOT NULL DEFAULT 1
);

CREATE UNIQUE INDEX idx_themes_project_source ON project_themes(project_id, source_file);
CREATE INDEX idx_themes_active ON project_themes(project_id, is_active);
```

### project_theme_overrides

User manual overrides for auto-extracted tokens.

```sql
CREATE TABLE project_theme_overrides (
    id              INTEGER PRIMARY KEY,
    project_id      INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    token_name      TEXT NOT NULL,           -- e.g. "primary", "background"
    value_light     TEXT NOT NULL,           -- OKLCH value
    value_dark      TEXT,                    -- OKLCH value (nullable)
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX idx_overrides_project_token ON project_theme_overrides(project_id, token_name);
```

---

## FTS5 Virtual Tables (2)

### messages_fts

External content FTS5 — references `messages` table, no content duplication.

```sql
CREATE VIRTUAL TABLE messages_fts USING fts5(
    content,
    tool_name,
    content='messages',
    content_rowid='id',
    tokenize='porter unicode61'
);

-- Sync triggers
CREATE TRIGGER messages_ai AFTER INSERT ON messages BEGIN
    INSERT INTO messages_fts(rowid, content, tool_name)
    VALUES (new.id, new.content, new.tool_name);
END;

CREATE TRIGGER messages_ad AFTER DELETE ON messages BEGIN
    INSERT INTO messages_fts(messages_fts, rowid, content, tool_name)
    VALUES ('delete', old.id, old.content, old.tool_name);
END;

CREATE TRIGGER messages_au AFTER UPDATE OF content ON messages BEGIN
    INSERT INTO messages_fts(messages_fts, rowid, content, tool_name)
    VALUES ('delete', old.id, old.content, old.tool_name);
    INSERT INTO messages_fts(rowid, content, tool_name)
    VALUES (new.id, new.content, new.tool_name);
END;
```

### artifacts_fts

Contentless FTS5 — index only, no content stored. Content read from disk on demand.

```sql
CREATE VIRTUAL TABLE artifacts_fts USING fts5(
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

Numbered SQL migration files executed sequentially by `tauri-plugin-sql`. Each migration is a `.sql` file in `src-tauri/migrations/`.

```
src-tauri/migrations/
  001_initial_schema.sql
  002_add_themes.sql
  ...
```

### Migration Execution

`tauri-plugin-sql` handles migrations automatically on database open:

```rust
// src-tauri/src/lib.rs
use tauri_plugin_sql::{Migration, MigrationKind};

let migrations = vec![
    Migration {
        version: 1,
        description: "initial schema",
        sql: include_str!("../migrations/001_initial_schema.sql"),
        kind: MigrationKind::Up,
    },
    Migration {
        version: 2,
        description: "add project themes",
        sql: include_str!("../migrations/002_add_themes.sql"),
        kind: MigrationKind::Up,
    },
];

tauri::Builder::default()
    .plugin(tauri_plugin_sql::Builder::new()
        .add_migrations("sqlite:forge.db", migrations)
        .build())
```

### Migration 001: Initial Schema

Contains all 11 core tables, 2 FTS5 tables, all indexes, and all triggers defined above. This is the full Phase 1 schema.

### Migration 002: Add Themes

Contains `project_themes` and `project_theme_overrides` tables. Separated from 001 because theme tables were decided later in the research process and may not be needed for the earliest prototype builds.

### Rules

- Migrations are append-only. Never modify a deployed migration.
- Each migration is idempotent where possible (`CREATE TABLE IF NOT EXISTS`).
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

### Session List (sidebar)

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

Cross-project learning requires app-level storage outside any per-project `forge.db` database. A global SQLite database (e.g., `~/.forge/global.db`) would store:

- **Global lessons** — Lessons promoted from project scope, with a reference to their source project
- **Global rules** — Rules promoted from project-local rules for cross-project enforcement
- **Cross-project metrics and patterns** — Aggregated data across projects
- **Tag-based categorization** — Language, framework, and domain tags for relevance matching when onboarding new projects

This database is separate from per-project `forge.db` files. Global lessons reference their origin project but are not owned by it — deleting a project does not remove its contributed global lessons.

Schema design for `global.db` will be specified when Phase 5 implementation begins. The key constraint is that per-project databases remain self-contained; the global store is additive.

---

## Table Summary

| Table | Rows (est. 1yr heavy use) | Phase |
|-------|---------------------------|-------|
| projects | 10-50 | 1 |
| sessions | 1,000-5,000 | 1 |
| messages | 100,000-500,000 | 1 |
| artifacts | 50-500 per project | 1 |
| scanner_results | 1,000-10,000 | 3 |
| metrics | 10,000-100,000 | 5 |
| tasks | 100-1,000 per project | 1 |
| lessons | 50-500 per project | 5 |
| settings | 20-50 | 1 |
| project_themes | 1-5 per project | 1 |
| project_theme_overrides | 0-30 per project | 1 |
| messages_fts | (mirrors messages) | 1 |
| artifacts_fts | (mirrors artifacts) | 1 |
