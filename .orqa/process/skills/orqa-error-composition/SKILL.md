---
id: "SKILL-010"
title: "Orqa Error Composition"
description: "OrqaStudio error composition and flow: OrqaError anatomy, From implementations,

  error propagation through domain/repo/command layers, IPC serialization, and

  frontend error handling patterns.

  Use when: Adding new error variants, implementing error handling in commands or

  domain services, wiring error states in Svelte stores, or debugging error flow

  across the Tauri boundary.\n"
status: "active"
created: "2026-03-01"
updated: "2026-03-10"
layer: "project"
scope:
  - "AGENT-002"
  - "AGENT-006"
category: "domain"
file-patterns:
  - "backend/src-tauri/src/domain/**"
  - "backend/src-tauri/src/commands/**"
version: "1.0.0"
user-invocable: true
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "Error composition creates structured error propagation"
---


OrqaStudio uses a single canonical error type (`OrqaError`) that flows from deep in the Rust backend, through the Tauri IPC boundary, and into Svelte stores where it is surfaced to the user. Every Rust function returns `Result<T, OrqaError>`. No `unwrap()`, `expect()`, or `panic!()` in production code.

## OrqaError Anatomy

```rust
// backend/src-tauri/src/error.rs
#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum OrqaError {
    #[error("not found: {0}")]
    #[serde(rename = "not_found")]
    NotFound(String),

    #[error("database error: {0}")]
    #[serde(rename = "database")]
    Database(String),

    #[error("file system error: {0}")]
    #[serde(rename = "file_system")]
    FileSystem(String),

    #[error("sidecar error: {0}")]
    #[serde(rename = "sidecar")]
    Sidecar(String),

    #[error("validation error: {0}")]
    #[serde(rename = "validation")]
    Validation(String),

    #[error("scan error: {0}")]
    #[serde(rename = "scan")]
    Scan(String),

    #[error("serialization error: {0}")]
    #[serde(rename = "serialization")]
    Serialization(String),

    #[error("permission denied: {0}")]
    #[serde(rename = "permission_denied")]
    PermissionDenied(String),

    #[error("search error: {0}")]
    #[serde(rename = "search")]
    Search(String),
}
```

Serializes as `{"code": "not_found", "message": "session 42"}` — the frontend matches on `code`.

### Variant Selection Guide

| Variant | When to use |
|---------|-------------|
| `NotFound` | Entity lookup returned no rows |
| `Database` | SQLite errors, lock poisoning |
| `FileSystem` | File I/O failures, missing directories |
| `Sidecar` | Agent SDK sidecar communication failures |
| `Validation` | Invalid user input, bad enum values |
| `Scan` | Governance scanner failures, invalid globs |
| `Serialization` | JSON/YAML parse failures |
| `PermissionDenied` | Path traversal, scope violations |
| `Search` | ChunkHound / code search failures |

## From Implementations

```rust
impl From<std::io::Error> for OrqaError {
    fn from(err: std::io::Error) -> Self { Self::FileSystem(err.to_string()) }
}
impl From<rusqlite::Error> for OrqaError {
    fn from(err: rusqlite::Error) -> Self { Self::Database(err.to_string()) }
}
impl From<serde_json::Error> for OrqaError {
    fn from(err: serde_json::Error) -> Self { Self::Serialization(err.to_string()) }
}
```

These enable `?` propagation:

```rust
let content = std::fs::read_to_string(file_path)?;     // io::Error -> FileSystem
let parsed: Config = serde_json::from_str(&raw)?;       // serde -> Serialization
conn.execute("INSERT ...", params![...])?;               // rusqlite -> Database
```

## Error Flow: Repo -> Domain -> Command -> IPC -> Frontend

### Layer 1: Repository

```rust
// backend/src-tauri/src/repo/session_repo.rs
pub fn get(conn: &Connection, id: i64) -> Result<Session, OrqaError> {
    conn.query_row("SELECT ... WHERE id = ?1", params![id], |row| { Ok(Session { /* ... */ }) })
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => OrqaError::NotFound(format!("session {id}")),
        other => OrqaError::Database(other.to_string()),
    })
}
```

### Layer 2: Domain / Helper

```rust
fn get_project_path(project_id: i64, state: &State<'_, AppState>) -> Result<String, OrqaError> {
    let conn = acquire_db(state)?;                            // Database error
    let project = project_repo::get(&conn, project_id)?;     // NotFound error
    Ok(project.path)
}
```

### Layer 3: Tauri Command

```rust
#[tauri::command]
pub fn governance_scan(project_id: i64, state: State<'_, AppState>) -> Result<GovernanceScanResult, OrqaError> {
    let project_path = get_project_path(project_id, &state)?;
    scan_governance(Path::new(&project_path))
}
```

### Layer 4: IPC Boundary

`Err(OrqaError::NotFound("session 42"))` serializes to `{"code": "not_found", "message": "session 42"}` automatically via Tauri.

### Layer 5: Frontend Store

```typescript
// ui/src/lib/stores/session.svelte.ts
async selectSession(sessionId: number): Promise<void> {
    this.isLoading = true;
    this.error = null;
    try {
        this.activeSession = await invoke<Session>("session_get", { sessionId });
    } catch (err) {
        this.error = err instanceof Error ? err.message : String(err);
    } finally {
        this.isLoading = false;
    }
}
```

### Layer 6: Component

```svelte
<!-- ui/src/lib/components/shared/ErrorDisplay.svelte -->
<div class="flex flex-col items-center justify-center py-8 text-center">
    <CircleAlertIcon class="mb-3 h-10 w-10 text-destructive" />
    <p class="text-sm text-destructive">{message}</p>
    {#if onRetry}<button onclick={onRetry}>Retry</button>{/if}
</div>
```

## The ? Operator Chain

```text
Command function
    ├── acquire_db(state)?        → OrqaError::Database
    ├── repo::get(&conn, id)?     → OrqaError::NotFound or ::Database
    ├── domain::process(data)?    → OrqaError::Validation, ::Scan, etc.
    ├── std::fs::write(path, d)?  → OrqaError::FileSystem (via From)
    └── serde_json::from_str(s)?  → OrqaError::Serialization (via From)
```

When no `From` impl exists, use `map_err`:

```rust
state.db.lock()
    .map_err(|e| OrqaError::Database(format!("failed to acquire db lock: {e}")))?;

glob::glob(pattern)
    .map_err(|e| OrqaError::Scan(format!("invalid glob pattern '{pattern}': {e}")))?;
```

## Contextual Error Construction

```rust
// Good: context tells you WHAT failed
OrqaError::FileSystem(format!("cannot read file '{file_path}': {e}"))
OrqaError::Sidecar(format!("failed to send tool approval: {e}"))

// Bad: no context
OrqaError::FileSystem(e.to_string())  // What file? What operation?
```

## Adding New Error Variants

1. Add the variant to `OrqaError` in `backend/src-tauri/src/error.rs`
2. Add `From` implementation if there is a common external error type
3. Add a serialization test
4. Update this skill's variant selection guide
5. Update TypeScript error handling if frontend needs to distinguish the code

## Frontend Error Handling Patterns

### Standard Store Pattern

```typescript
class SomeStore {
    isLoading = $state(false);
    error = $state<string | null>(null);

    async doSomething(): Promise<void> {
        this.isLoading = true;
        this.error = null;
        try { /* await invoke(...) */ }
        catch (err) { this.error = err instanceof Error ? err.message : String(err); }
        finally { this.isLoading = false; }
    }
}
```

### Streaming Error Pattern

```typescript
case "stream_error":
    this.error = event.data.message;
    this.isStreaming = false;
    break;
```

### Non-Critical Error Swallowing (Explicit Only)

```typescript
try {
    await invoke("settings_set", { key: "last_session_id", value: sessionId, scope: "app" });
} catch {
    // Non-critical — best-effort persistence
}
```

### Recovery Patterns

| Pattern | When | Example |
|---------|------|---------|
| Retry | Transient failures | `ErrorDisplay` with `onRetry` callback |
| Fallback | Non-critical data missing | `restoreSession` returns `false`, app continues |
| User notification | Actionable errors | Store sets `this.error`, component shows `ErrorDisplay` |
| Stream termination | Fatal stream errors | `stream_error` sets `isStreaming = false` |

## Anti-Patterns

### Silent Error Swallowing

```typescript
// FORBIDDEN without a comment explaining why
try { await invoke("important"); } catch { }
```

### Using unwrap() / expect() in Production

```rust
// FORBIDDEN
let conn = state.db.lock().unwrap();
```

### Generic String Errors

```rust
// WRONG
Err("something went wrong".to_string())
// RIGHT
Err(OrqaError::Validation("session name cannot be empty".to_string()))
```

### Wrong Variant Mapping

```rust
// WRONG: io::Error is not a database error
std::fs::read_to_string(path).map_err(|e| OrqaError::Database(e.to_string()))?;
// RIGHT: rely on From<io::Error>
std::fs::read_to_string(path)?;
```

## Key Files

| File | Purpose |
|------|---------|
| `backend/src-tauri/src/error.rs` | `OrqaError` enum, `From` implementations |
| `backend/src-tauri/src/repo/*.rs` | Repository layer error conversion |
| `backend/src-tauri/src/domain/*.rs` | Domain services error propagation |
| `backend/src-tauri/src/commands/*.rs` | Command handlers returning `Result<T, OrqaError>` |
| `ui/src/lib/stores/*.svelte.ts` | Frontend error state |
| `ui/src/lib/components/shared/ErrorDisplay.svelte` | Shared error display |

## Related Skills

- **composability** — error composition is a core composability principle
- **orqa-ipc-patterns** — the IPC boundary errors cross
- **orqa-domain-services** — how domain services propagate errors
- **orqa-repository-pattern** — how DB errors are converted
