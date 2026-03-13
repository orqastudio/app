---
id: "SKILL-014"
title: "Orqa Repository Pattern"
description: "OrqaStudio's persistence layer using the repository pattern with rusqlite.

  Covers repository anatomy, connection management, query patterns, migrations,

  error handling, and testing.

  Use when: Adding a new database entity, writing a new repository module,

  modifying persistence logic, writing migration SQL, or testing database code.\n"
status: "active"
created: "2026-03-01"
updated: "2026-03-10"
layer: "project"
scope:
  - "AGENT-002"
  - "AGENT-006"
category: "domain"
file-patterns:
  - "backend/src-tauri/src/repo/**"
version: "1.0.0"
user-invocable: true
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "Repository pattern creates structured data access"
---


OrqaStudio persists structured data in SQLite via the `rusqlite` crate. Every database entity follows the same repository pattern: a module of free functions in `backend/src-tauri/src/repo/` that accepts a `&Connection` and returns `Result<T, OrqaError>`.

## Repository Anatomy

Repositories are modules of free functions — not structs, not traits. Each module lives in `backend/src-tauri/src/repo/` and is re-exported from `backend/src-tauri/src/repo/mod.rs`.

Every repository provides a consistent interface:

| Function | Signature Pattern | Purpose |
|----------|-------------------|---------|
| `create` | `(conn, ...fields) -> Result<T, OrqaError>` | INSERT then return the full record |
| `get` | `(conn, id) -> Result<T, OrqaError>` | SELECT by primary key |
| `list` | `(conn, parent_id, ...filters) -> Result<Vec<T>, OrqaError>` | SELECT with optional filters |
| `update_*` | `(conn, id, ...fields) -> Result<(), OrqaError>` | UPDATE specific fields |
| `delete` | `(conn, id) -> Result<(), OrqaError>` | DELETE by primary key |

### Real Example: project_repo

```rust
// backend/src-tauri/src/repo/project_repo.rs
pub fn create(conn: &Connection, name: &str, path: &str, description: Option<&str>) -> Result<Project, OrqaError> {
    conn.execute(
        "INSERT INTO projects (name, path, description) VALUES (?1, ?2, ?3)",
        params![name, path, description],
    )?;
    let id = conn.last_insert_rowid();
    get(conn, id)
}

pub fn get(conn: &Connection, id: i64) -> Result<Project, OrqaError> {
    conn.query_row(
        "SELECT id, name, path, description, detected_stack, created_at, updated_at FROM projects WHERE id = ?1",
        params![id],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                description: row.get(3)?,
                detected_stack: row.get::<_, Option<String>>(4)?
                    .and_then(|s| serde_json::from_str::<DetectedStack>(&s).ok()),
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => OrqaError::NotFound(format!("project {id}")),
        other => OrqaError::Database(other.to_string()),
    })
}
```

**Key patterns:**
1. `create` inserts then calls `get` to return the full record with server-generated fields
2. `get` maps `QueryReturnedNoRows` to `OrqaError::NotFound`
3. JSON-serialized columns stored as `TEXT`, deserialized with `serde_json` in the row mapper
4. All functions take `&Connection` as first parameter — they never own or manage the connection

## Connection Management

The database connection is in `AppState` behind a `Mutex`:

```rust
// backend/src-tauri/src/state.rs
pub struct AppState {
    pub db: Mutex<Connection>,
}
```

Commands acquire the lock with this exact pattern:

```rust
let conn = state.db.lock()
    .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
session_repo::create(&conn, project_id, model, None)
```

The lock is released when the `MutexGuard` drops. Keep the critical section short.

## Query Patterns

### Parameterized queries only

```rust
// CORRECT
conn.execute("INSERT INTO settings (key, value, scope) VALUES (?1, ?2, ?3)", params![key, value_str, scope])?;

// FORBIDDEN — SQL injection risk
conn.execute(&format!("INSERT INTO settings (key, value) VALUES ('{key}', '{value}')"), [])?;
```

### Multi-row queries

```rust
let mut stmt = conn.prepare("SELECT ... WHERE project_id = ?1 ORDER BY ...")?;
let rows = stmt.query_map(params![project_id], map_summary)?;
let mut results = Vec::new();
for row in rows { results.push(row?); }
Ok(results)
```

### Optional results

```rust
use rusqlite::OptionalExtension;
let value: Option<String> = conn
    .query_row("SELECT value FROM settings WHERE key = ?1", params![key], |row| row.get(0))
    .optional()?;
```

### Update with row-count check

```rust
pub fn update_title(conn: &Connection, id: i64, title: &str) -> Result<(), OrqaError> {
    let rows = conn.execute(
        "UPDATE sessions SET title = ?1, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ?2",
        params![title, id],
    )?;
    if rows == 0 { return Err(OrqaError::NotFound(format!("session {id}"))); }
    Ok(())
}
```

### Upsert with ON CONFLICT

```rust
conn.execute(
    "INSERT INTO settings (key, value, scope, updated_at) VALUES (?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
     ON CONFLICT(key, scope) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
    params![key, value_str, scope],
)?;
```

### Timestamps

All timestamps are generated by SQLite using `strftime('%Y-%m-%dT%H:%M:%fZ', 'now')` — either as column defaults or inline in UPDATE statements. Repos never generate timestamps in Rust.

## Migration Strategy

Migrations live in `backend/src-tauri/migrations/` as numbered SQL files. Schema creation uses `CREATE TABLE IF NOT EXISTS` for idempotency.

Column additions use Rust helper functions that check `pragma_table_info` first (since `ALTER TABLE ADD COLUMN` is not idempotent):

```rust
fn run_migration_005(conn: &Connection) -> Result<(), OrqaError> {
    let has_col: bool = conn
        .prepare("SELECT COUNT(*) FROM pragma_table_info('sessions') WHERE name = 'title_manually_set'")?
        .query_row([], |row| row.get::<_, i64>(0))
        .map(|count| count > 0)
        .unwrap_or(false);
    if !has_col {
        conn.execute_batch("ALTER TABLE sessions ADD COLUMN title_manually_set INTEGER DEFAULT 0")?;
    }
    Ok(())
}
```

Always add migrations to both `init_db` (production) AND `init_memory_db` (tests).

## Adding a New Entity

1. **Migration** — `backend/src-tauri/migrations/NNN_description.sql` with `CREATE TABLE IF NOT EXISTS`
2. **Domain type** — `backend/src-tauri/src/domain/<entity>.rs` with `Serialize, Deserialize, Debug, Clone`
3. **Repository** — `backend/src-tauri/src/repo/<entity>_repo.rs` with standard functions
4. **Command** — `backend/src-tauri/src/commands/<entity>_commands.rs` with thin wrappers
5. **TypeScript type** — `ui/src/lib/types/` matching the Rust struct
6. **Store** — `ui/src/lib/stores/<entity>.svelte.ts` calling `invoke()`

## Testing Repositories

Every repo test uses `init_memory_db()` — a fresh in-memory database with all migrations:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_memory_db;

    #[test]
    fn create_and_get() {
        let conn = init_memory_db().expect("db init");
        let item = create(&conn, /* fields */).expect("create");
        let fetched = get(&conn, item.id).expect("get");
        assert_eq!(fetched.name, item.name);
    }
}
```

For entities with foreign keys, use a `setup()` helper that inserts parent records:

```rust
fn setup() -> Connection {
    let conn = init_memory_db().expect("db init");
    project_repo::create(&conn, "test", "/test", None).expect("create project");
    session_repo::create(&conn, 1, "auto", None).expect("create session");
    conn
}
```

### What to test

- Create and retrieve round-trip
- Not-found cases (`OrqaError::NotFound`)
- List with filters and pagination
- Update persistence and zero-row updates
- Delete and cascade deletes
- Constraint enforcement (unique, FK violations)

## Non-SQLite Repositories

The `lesson_repo` reads/writes markdown files from `.orqa/process/lessons/`. Same interface pattern (list, get, create) but operates on `Path` instead of `Connection`:

```rust
pub fn list(project_path: &Path) -> Result<Vec<Lesson>, OrqaError> { ... }
pub fn get(project_path: &Path, id: &str) -> Result<Lesson, OrqaError> { ... }
pub fn create(project_path: &Path, new: &NewLesson) -> Result<Lesson, OrqaError> { ... }
```

## Current Repositories

| Module | Storage | Entities |
|--------|---------|----------|
| `project_repo` | SQLite | `Project`, `ProjectSummary` |
| `session_repo` | SQLite | `Session`, `SessionSummary` |
| `message_repo` | SQLite | `Message`, `SearchResult` |
| `artifact_repo` | SQLite | `Artifact`, `ArtifactSummary` |
| `settings_repo` | SQLite | Key-value settings (scoped) |
| `governance_repo` | SQLite | `GovernanceAnalysis`, `Recommendation` |
| `theme_repo` | SQLite | `ThemeRow`, `ThemeOverrideRow` |
| `lesson_repo` | Filesystem | `Lesson` (markdown + YAML frontmatter) |
| `enforcement_rules_repo` | SQLite | Enforcement rules and scan findings |
| `project_settings_repo` | File (`.orqa/project.json`) | `ProjectSettings` (name, model, stack, governance counts) |

## Related Skills

- **composability** — the overarching philosophy
- **orqa-domain-services** — services that consume repositories
- **orqa-error-composition** — how DB errors flow to frontend
- **orqa-ipc-patterns** — the full chain from store to repo
