---
id: DOC-003
title: Error Taxonomy
description: The flat OrqaError enum — its variants, From conversions, serde serialization format, and propagation through domain → command → IPC → frontend.
created: "2026-03-02"
updated: "2026-03-10"
---


# Error Taxonomy

All Tauri commands return `Result<T, OrqaError>`. `OrqaError` is a single flat enum defined in `backend/src-tauri/src/error.rs`. There are no nested error enums.

**Architecture References:** [AD-003](AD-003) (error propagation via Result types), [AD-002](AD-002) (IPC boundary design)

---


## 1. Design Principles

1. **Every command returns `Result<T, OrqaError>`.** The application must never crash. Every error is caught and propagated up to the Tauri command boundary where it is serialized and returned to the frontend as a structured JSON error.

2. **Errors are stringly-typed within each variant.** Each variant carries a single `String` with the error detail. The variant tag conveys the error category; the string conveys the specific message. This differs from the typed-field design in the previous specification — the actual implementation uses strings.

3. **Errors cross the IPC boundary as tagged JSON.** `OrqaError` derives `serde::Serialize` with `#[serde(tag = "code", content = "message")]`. Tauri's blanket `impl<T: Serialize> From<T> for InvokeError` handles conversion to a Tauri invoke error automatically. No custom serialization logic exists.

4. **Error context is a string.** Lower-level errors (`std::io::Error`, `rusqlite::Error`, `serde_json::Error`) are converted to `OrqaError` via `From` impls that call `.to_string()` on the source error. The string message is the only context that crosses the boundary.

5. **No user-facing message separation.** There is no `user_message()` method. The frontend receives the single string from the variant and decides how to present it.

---


## 2. The OrqaError Enum

```rust
// backend/src-tauri/src/error.rs

use serde::Serialize;

/// Canonical error type for all OrqaStudio IPC commands.
///
/// Serialized as `{"code": "<variant>", "message": "<detail>"}` for the frontend.
/// The `Serialize` derive enables automatic conversion to `tauri::ipc::InvokeError`
/// via Tauri's blanket `impl<T: Serialize> From<T> for InvokeError`.
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

### Variant Reference

| Variant | Serde `code` | Used For |
|---------|-------------|----------|
| `NotFound(String)` | `not_found` | Missing resources (sessions, projects, artifacts) |
| `Database(String)` | `database` | SQLite errors from rusqlite, including lock failures |
| `FileSystem(String)` | `file_system` | `std::io::Error` — file reads, writes, directory operations |
| `Sidecar(String)` | `sidecar` | Sidecar process spawn, communication, and crash errors |
| `Validation(String)` | `validation` | Input validation failures (empty fields, invalid values) |
| `Scan(String)` | `scan` | Governance scanner and enforcement engine errors |
| `Serialization(String)` | `serialization` | `serde_json` errors and other serialization failures |
| `PermissionDenied(String)` | `permission_denied` | Path-scope violations and access control errors |
| `Search(String)` | `search` | Search engine errors |

---


## 3. From Conversions

Three standard library error types convert to `OrqaError` automatically via `From` impls. All conversions call `.to_string()` on the source error and wrap the message in the appropriate variant.

```rust
impl From<std::io::Error> for OrqaError {
    fn from(err: std::io::Error) -> Self {
        Self::FileSystem(err.to_string())
    }
}

impl From<serde_json::Error> for OrqaError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<rusqlite::Error> for OrqaError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database(err.to_string())
    }
}
```

These `From` impls enable `?` propagation throughout the codebase. A function that reads a file, parses JSON, and queries SQLite can use `?` on all three operations without any explicit error mapping.

---


## 4. Propagation Pattern

Errors flow in one direction through the layers:

```
domain function  →  command handler  →  IPC boundary  →  TypeScript invoke()
     ?                   ?               (serialize)         catch(err)
```

### Domain layer

Domain functions in `backend/src-tauri/src/domain/` return `Result<T, OrqaError>`. They construct variants directly or use `?` with the `From` impls:

```rust
// Direct construction — manual context
return Err(OrqaError::Validation("model cannot be empty".to_string()));

// From conversion via ?
let content = std::fs::read_to_string(&path)?;  // io::Error → OrqaError::FileSystem
```

### Command layer

Tauri command handlers in `backend/src-tauri/src/commands/` return `Result<T, OrqaError>` and delegate to domain functions using `?`:

```rust
#[tauri::command]
pub fn session_create(
    model: String,
    state: State<'_, AppState>,
) -> Result<Session, OrqaError> {
    if model.is_empty() {
        return Err(OrqaError::Validation("model cannot be empty".to_string()));
    }
    let db = state.db.lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;
    session_repo::create(&db, &model)  // returns Result<Session, OrqaError>
}
```

### IPC boundary

Tauri serializes `Err(OrqaError)` using the `Serialize` derive. The `#[serde(tag = "code", content = "message")]` attribute produces a two-field JSON object. No manual `Serialize` implementation exists.

---


## 5. IPC Serialization Format

Every `OrqaError` variant serializes to the same two-field JSON shape:

```json
{
  "code": "<serde_rename_value>",
  "message": "<the string carried by the variant>"
}
```

### Examples

`OrqaError::NotFound("project 42".to_string())` serializes to:

```json
{"code": "not_found", "message": "project 42"}
```

`OrqaError::Database("connection refused".to_string())` serializes to:

```json
{"code": "database", "message": "connection refused"}
```

`OrqaError::Validation("model cannot be empty".to_string())` serializes to:

```json
{"code": "validation", "message": "model cannot be empty"}
```

The `message` value is always the raw string passed to the variant constructor — it is the `Display` output of the original error (via `.to_string()`) for converted errors, or a hand-written message for directly constructed ones.

---


## 6. TypeScript Interface

The frontend receives errors thrown by `invoke()` with this shape:

```typescript
/** Error shape returned by all Tauri invoke() calls on failure. */
interface OrqaError {
  /** Error category code — one of the serde rename values */
  code:
    | "not_found"
    | "database"
    | "file_system"
    | "sidecar"
    | "validation"
    | "scan"
    | "serialization"
    | "permission_denied"
    | "search";
  /** Human-readable detail string */
  message: string;
}
```

TypeScript error handling in stores and components:

```typescript
try {
  const session = await invoke<Session>("session_create", { model });
} catch (err) {
  // err is OrqaError: { code: string, message: string }
  const orqaError = err as { code: string; message: string };
  if (orqaError.code === "validation") {
    // show validation message inline
  } else {
    // show generic error toast
  }
}
```

---


## 7. Testing

`backend/src-tauri/src/error.rs` contains unit tests covering:

- Each variant serializes with the correct `code` value
- Each variant serializes with a `string` in `message`
- `Display` output uses the `thiserror` format string (e.g., `"not found: session 99"`)
- `From<std::io::Error>` produces `OrqaError::FileSystem`
- `From<serde_json::Error>` produces `OrqaError::Serialization`
- All variants serialize correctly in a round-trip check (`all_variants_serialize_as_tagged_json`)

There is no test for `rusqlite::Error` conversion because constructing a `rusqlite::Error` in unit tests requires a database connection. The `From` impl is tested implicitly through integration tests in `backend/src-tauri/tests/`.

---


## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The flat `OrqaError` enum with typed `code` values makes all error categories explicit and visible — both in Rust source and in the JSON that crosses the IPC boundary. Developers and agents reading command signatures immediately know what can go wrong. |
| Learning Through Reflection | N/A — error taxonomy is infrastructure, not a learning or retrospection feature. |

---


## Related Documents

- [AD-003](AD-003) (Result types, no unwrap in production), [AD-002](AD-002) (IPC boundary)
- `backend/src-tauri/src/error.rs` — canonical source of truth for all variant definitions and From impls
