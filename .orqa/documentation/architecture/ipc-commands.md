---
id: DOC-005
title: IPC Command Catalog
description: Catalog of all Tauri IPC commands defining the frontend-backend communication contract.
created: "2026-03-02"
updated: "2026-03-09"
---

**Date:** 2026-03-02 | **Status:** Phase 0e specification | **References:** [Tauri v2 Research](RES-007) ([AD-002](AD-002), [AD-012](AD-012)), [MVP Specification](DOC-042)

Complete catalog of `#[tauri::command]` functions for Phase 1 (MVP). Every frontend-to-backend call crosses the IPC boundary through one of these commands. Streaming data uses `Channel<T>` ([AD-009](AD-009)) rather than `invoke()`.

All commands return `Result<T, OrqaError>` where `OrqaError` is a `thiserror`-derived enum serialized as a JSON object with `code` and `message` fields ([AD-003](AD-003)).

---

## Error Envelope

Every command returns the same error shape on failure:

```rust
#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum OrqaError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("database error: {0}")]
    Database(String),

    #[error("file system error: {0}")]
    FileSystem(String),

    #[error("sidecar error: {0}")]
    Sidecar(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("scan error: {0}")]
    Scan(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("search error: {0}")]
    Search(String),
}
```

TypeScript receives this as:

```typescript
interface OrqaError {
  code: "not_found" | "database" | "file_system" | "sidecar" | "validation" | "scan" | "serialization" | "permission_denied" | "search";
  message: string;
}
```

---

## Project Commands

### `project_open`

Open an existing directory as a OrqaStudio™ project. Registers it in SQLite, runs Tier 1 + Tier 2 codebase scan, indexes `.orqa/` governance artifacts, and extracts design tokens if found.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | `String` | Yes | Absolute path to the project directory |

**Returns:** `Result<Project, OrqaError>`

**Error cases:**
- `NotFound` — path does not exist or is not a directory
- `PermissionDenied` — path is outside allowed scope
- `Database` — failed to insert/update project record
- `Scan` — Tier 1 or Tier 2 scan failure (non-fatal; project still opens with partial data)

**TS mirror:** `Promise<Project>`

---

### `project_create`

Create a new project directory with scaffolded `.claude/` governance skeleton. Registers in SQLite, optionally runs `git init`.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | `String` | Yes | Project name |
| `parent_path` | `String` | Yes | Parent directory where project folder is created |
| `init_git` | `bool` | No (default `true`) | Whether to run `git init` |

**Returns:** `Result<Project, OrqaError>`

**Error cases:**
- `FileSystem` — cannot create directory or write scaffold files
- `PermissionDenied` — parent path outside allowed scope
- `Validation` — name is empty or contains invalid characters
- `Database` — failed to insert project record

**TS mirror:** `Promise<Project>`

---

### `project_list`

List all registered projects, ordered by most recently opened.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| *(none)* | | | |

**Returns:** `Result<Vec<ProjectSummary>, OrqaError>`

**Error cases:**
- `Database` — query failure

**TS mirror:** `Promise<ProjectSummary[]>`

---

### `project_get`

Get full project details including detected stack, artifact counts, and theme status.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |

**Returns:** `Result<Project, OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Database` — query failure

**TS mirror:** `Promise<Project>`

---

### `project_get_active`

Get the currently active project (last-opened). Returns `None` if no project has been opened.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| *(none)* | | | |

**Returns:** `Result<Option<Project>, OrqaError>`

**Error cases:**
- `Database` — query failure

**TS mirror:** `Promise<Project | null>`

---

### `project_scan`

Scan a project directory to detect languages, frameworks, package manager, and governance artifact counts. Pure filesystem operation — does not require a database record or project ID.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | `String` | Yes | Absolute path to the project directory |
| `excluded_paths` | `Option<Vec<String>>` | No | Directory names to skip (defaults to `["node_modules", ".git", "target", "dist", "build"]`) |

**Returns:** `Result<ProjectScanResult, OrqaError>`

```rust
pub struct ProjectScanResult {
    pub stack: DetectedStack,
    pub governance: GovernanceCounts,
    pub scan_duration_ms: u64,
}
```

**Error cases:**
- `FileSystem` — path does not exist or cannot be read
- `Scan` — scan failure during directory walk

**TS mirror:** `Promise<ProjectScanResult>`

---

### `project_settings_read`

Read project settings from `.orqa/project.json` at the given project path. Returns `None` if the file does not exist (not an error — triggers setup wizard in the UI).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | `String` | Yes | Absolute path to the project directory |

**Returns:** `Result<Option<ProjectSettings>, OrqaError>`

```rust
pub struct ProjectSettings {
    pub name: String,
    pub description: Option<String>,
    pub default_model: String,
    pub excluded_paths: Vec<String>,
    pub stack: Option<DetectedStack>,
    pub governance: Option<GovernanceCounts>,
}

pub struct GovernanceCounts {
    pub docs: u32,
    pub agents: u32,
    pub rules: u32,
    pub skills: u32,
    pub hooks: u32,
    pub has_claude_config: bool,
}
```

**Error cases:**
- `Serialization` — file exists but contains malformed JSON
- `FileSystem` — permission denied or I/O error reading file

**TS mirror:** `Promise<ProjectSettings | null>`

---

### `project_settings_write`

Write project settings to `.orqa/project.json`. Creates the `.orqa/` directory if it does not exist. Returns the written settings as confirmation.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | `String` | Yes | Absolute path to the project directory |
| `settings` | `ProjectSettings` | Yes | The settings object to write |

**Returns:** `Result<ProjectSettings, OrqaError>`

**Error cases:**
- `FileSystem` — cannot create `.orqa/` directory or write file
- `Serialization` — settings cannot be serialized (should not happen with valid types)

**TS mirror:** `Promise<ProjectSettings>`

---

## Session Commands

### `session_create`

Create a new conversation session for the active project. If there is a previous session with handoff notes, they are attached to this session's system prompt context.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |
| `model` | `Option<String>` | No | Model selection. `None` or `"auto"` both mean auto-select (provider chooses the best available model based on rate limits and availability). An explicit model string (e.g., `"claude-opus-4-6"`) pins the session to that model. Default from settings applies when omitted. |
| `system_prompt` | `Option<String>` | No | Custom system prompt override |

**Returns:** `Result<Session, OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Database` — failed to insert session
- `Sidecar` — sidecar not running (session created but not usable for conversation)

**TS mirror:** `Promise<Session>`

---

### `session_list`

List sessions for a project, ordered by most recent. Includes preview snippet and message count.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |
| `status` | `Option<String>` | No | Filter by status: `"active"`, `"completed"`, `"abandoned"`, `"error"` |
| `limit` | `Option<i64>` | No | Max results (default 50) |
| `offset` | `Option<i64>` | No | Pagination offset (default 0) |

**Returns:** `Result<Vec<SessionSummary>, OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Validation` — invalid status value
- `Database` — query failure

**TS mirror:** `Promise<SessionSummary[]>`

---

### `session_get`

Get full session details including metadata and token usage.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `session_id` | `i64` | Yes | Session ID |

**Returns:** `Result<Session, OrqaError>`

**Error cases:**
- `NotFound` — no session with this ID
- `Database` — query failure

**TS mirror:** `Promise<Session>`

---

### `session_update_title`

Update a session's display title.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `session_id` | `i64` | Yes | Session ID |
| `title` | `String` | Yes | New title |

**Returns:** `Result<(), OrqaError>`

**Error cases:**
- `NotFound` — no session with this ID
- `Validation` — title is empty
- `Database` — update failure

**TS mirror:** `Promise<void>`

---

### `session_end`

End a session. Triggers handoff summary generation (async, does not block). Sets status to `completed`.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `session_id` | `i64` | Yes | Session ID |

**Returns:** `Result<(), OrqaError>`

**Error cases:**
- `NotFound` — no session with this ID
- `Database` — update failure
- `Sidecar` — summary generation failed (session still ends, uses rule-based fallback summary)

**TS mirror:** `Promise<void>`

---

### `session_delete`

Delete a session and all its messages. Irreversible.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `session_id` | `i64` | Yes | Session ID |

**Returns:** `Result<(), OrqaError>`

**Error cases:**
- `NotFound` — no session with this ID
- `Database` — delete failure (foreign key cascades handle messages)

**TS mirror:** `Promise<void>`

---

## Message Commands

### `message_list`

List all messages (content blocks) for a session, ordered by `turn_index` then `block_index`.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `session_id` | `i64` | Yes | Session ID |
| `limit` | `Option<i64>` | No | Max results (default: all) |
| `offset` | `Option<i64>` | No | Pagination offset (default 0) |

**Returns:** `Result<Vec<Message>, OrqaError>`

**Error cases:**
- `NotFound` — no session with this ID
- `Database` — query failure

**TS mirror:** `Promise<Message[]>`

---

### `message_search`

Full-text search across messages using FTS5. Searches within a project scope. Returns highlighted snippets with session context.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID (scope) |
| `query` | `String` | Yes | FTS5 search query |
| `limit` | `Option<i64>` | No | Max results (default 50) |

**Returns:** `Result<Vec<SearchResult>, OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Validation` — empty or malformed FTS5 query
- `Database` — query failure

**TS mirror:** `Promise<SearchResult[]>`

---

## Streaming Commands

### `stream_send_message`

Send a user message to the active session and begin streaming the AI response. The response streams back via `Channel<StreamEvent>` (not via the return value). The command returns immediately after the message is accepted by the sidecar.

This is the primary conversation command. It:
1. Persists the user message in SQLite
2. Sends the message to the sidecar via stdin NDJSON
3. Streams response events back via `Channel<T>`
4. Persists assistant response blocks as they complete

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `session_id` | `i64` | Yes | Session ID |
| `content` | `String` | Yes | User message text |
| `on_event` | `Channel<StreamEvent>` | Yes | Tauri channel for streaming events |

**Returns:** `Result<MessageId, OrqaError>`

The `MessageId` is the ID of the persisted user message. Assistant messages stream via the channel.

**Error cases:**
- `NotFound` — no session with this ID
- `Validation` — content is empty
- `Sidecar` — sidecar not running or not connected
- `Database` — failed to persist user message

**TS mirror:** `Promise<number>` (with events arriving via the channel callback)

---

### `stream_stop`

Abort the current streaming response. Sends a cancellation signal to the sidecar. The current partial response is preserved with `stream_status = 'error'`.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `session_id` | `i64` | Yes | Session ID |

**Returns:** `Result<(), OrqaError>`

**Error cases:**
- `NotFound` — no session with this ID, or no active stream
- `Sidecar` — failed to send cancellation signal

**TS mirror:** `Promise<void>`

---

## Artifact Commands

### `artifact_list`

List governance artifacts for a project, optionally filtered by type.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |
| `artifact_type` | `Option<String>` | No | Filter: `"agent"`, `"rule"`, `"skill"`, `"hook"`, `"doc"` |

**Returns:** `Result<Vec<ArtifactSummary>, OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Validation` — invalid artifact type
- `Database` — query failure

**TS mirror:** `Promise<ArtifactSummary[]>`

---

### `artifact_get`

Get full artifact details including rendered content read from disk.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `artifact_id` | `i64` | Yes | Artifact ID |

**Returns:** `Result<Artifact, OrqaError>`

**Error cases:**
- `NotFound` — no artifact with this ID, or file missing from disk
- `FileSystem` — file read failure
- `Database` — query failure

**TS mirror:** `Promise<Artifact>`

---

### `artifact_get_by_path`

Get an artifact by its relative path within the project. Useful when navigating from file references.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |
| `rel_path` | `String` | Yes | Relative path, e.g. `".claude/agents/backend-engineer.md"` |

**Returns:** `Result<Artifact, OrqaError>`

**Error cases:**
- `NotFound` — no artifact at this path
- `FileSystem` — file read failure
- `Database` — query failure

**TS mirror:** `Promise<Artifact>`

---

### `artifact_create`

Create a new governance artifact. Writes the file to disk and indexes it in SQLite.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |
| `artifact_type` | `String` | Yes | One of: `"agent"`, `"rule"`, `"skill"`, `"hook"`, `"doc"` |
| `name` | `String` | Yes | Artifact name (used for filename) |
| `content` | `String` | Yes | Full file content (markdown with optional YAML frontmatter) |

**Returns:** `Result<Artifact, OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Validation` — invalid type, empty name, or name produces invalid filename
- `FileSystem` — cannot write file (permissions, disk full)
- `Database` — failed to insert record

**TS mirror:** `Promise<Artifact>`

---

### `artifact_update`

Update an artifact's content on disk. Re-indexes metadata in SQLite.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `artifact_id` | `i64` | Yes | Artifact ID |
| `content` | `String` | Yes | New full file content |

**Returns:** `Result<Artifact, OrqaError>`

**Error cases:**
- `NotFound` — no artifact with this ID
- `FileSystem` — cannot write file
- `Database` — failed to update record

**TS mirror:** `Promise<Artifact>`

---

### `artifact_delete`

Delete an artifact. Removes the file from disk and the record from SQLite.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `artifact_id` | `i64` | Yes | Artifact ID |

**Returns:** `Result<(), OrqaError>`

**Error cases:**
- `NotFound` — no artifact with this ID
- `FileSystem` — cannot delete file
- `Database` — failed to delete record

**TS mirror:** `Promise<void>`

---

## Theme Commands

### `theme_get_project`

Get the resolved theme for a project. Merges auto-extracted tokens with any user overrides. Returns OrqaStudio defaults for any unmapped tokens.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |

**Returns:** `Result<ResolvedTheme, OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Database` — query failure

**TS mirror:** `Promise<ResolvedTheme>`

---

### `theme_set_override`

Set a manual override for a specific design token. Overrides persist until cleared.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |
| `token_name` | `String` | Yes | Token name, e.g. `"primary"`, `"background"` |
| `value_light` | `String` | Yes | OKLCH color value for light mode |
| `value_dark` | `Option<String>` | No | OKLCH color value for dark mode |

**Returns:** `Result<(), OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Validation` — invalid token name or color value
- `Database` — upsert failure

**TS mirror:** `Promise<void>`

---

### `theme_clear_overrides`

Remove all manual theme overrides for a project. Reverts to auto-extracted tokens (or OrqaStudio defaults if none).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | `i64` | Yes | Project ID |

**Returns:** `Result<(), OrqaError>`

**Error cases:**
- `NotFound` — no project with this ID
- `Database` — delete failure

**TS mirror:** `Promise<void>`

---

## Settings Commands

### `settings_get`

Get a single setting value by key. Settings are scoped to `"app"` or a specific project ID.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `key` | `String` | Yes | Setting key |
| `scope` | `Option<String>` | No | `"app"` (default) or a project ID as string |

**Returns:** `Result<Option<serde_json::Value>, OrqaError>`

**Error cases:**
- `Database` — query failure

**TS mirror:** `Promise<unknown | null>`

---

### `settings_set`

Set a single setting value.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `key` | `String` | Yes | Setting key |
| `value` | `serde_json::Value` | Yes | JSON value |
| `scope` | `Option<String>` | No | `"app"` (default) or a project ID as string |

**Returns:** `Result<(), OrqaError>`

**Error cases:**
- `Validation` — empty key
- `Serialization` — value cannot be serialized
- `Database` — upsert failure

**TS mirror:** `Promise<void>`

---

### `settings_get_all`

Get all settings for a given scope, returned as a key-value map.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `scope` | `Option<String>` | No | `"app"` (default) or a project ID as string |

**Returns:** `Result<HashMap<String, serde_json::Value>, OrqaError>`

**Error cases:**
- `Database` — query failure

**TS mirror:** `Promise<Record<string, unknown>>`

---

## Sidecar Commands

### `sidecar_status`

Get the current sidecar process status.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| *(none)* | | | |

**Returns:** `Result<SidecarStatus, OrqaError>`

**Error cases:**
- *(always succeeds — returns status even if sidecar is not running)*

**TS mirror:** `Promise<SidecarStatus>`

---

### `sidecar_restart`

Kill the current sidecar process (if any) and spawn a new one.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| *(none)* | | | |

**Returns:** `Result<SidecarStatus, OrqaError>`

**Error cases:**
- `Sidecar` — failed to spawn new sidecar process
- `FileSystem` — sidecar binary not found

**TS mirror:** `Promise<SidecarStatus>`

---

## Documentation Commands

### `doc_read`

Read a documentation file from the project's `docs/` directory. Returns an `Artifact` struct with the file content loaded from disk. Path traversal (`..`) is rejected.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `rel_path` | `String` | Yes | Relative path within `docs/` (e.g. `"product/vision.md"`) |

**Returns:** `Result<Artifact, OrqaError>`

**Error cases:**
- `Validation` — path contains `..` (traversal attempt)
- `NotFound` — no active project or file does not exist
- `FileSystem` — file read failure

**TS mirror:** `Promise<Artifact>`

---

### `doc_tree_scan`

Scan the active project's `docs/` directory and return a tree structure of documentation files and directories.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| *(none)* | | | Uses the active project |

**Returns:** `Result<Vec<DocNode>, OrqaError>`

**Error cases:**
- `NotFound` — no active project
- `FileSystem` — directory scan failure

**TS mirror:** `Promise<DocNode[]>`

---

### `governance_list`

List governance artifacts (agents, rules, skills, hooks) by scanning the `.claude/` directory on disk. Does not use the database.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `artifact_type` | `String` | Yes | One of: `"agent"`, `"rule"`, `"skill"`, `"hook"`. Not `"doc"` (use `doc_tree_scan`). |

**Returns:** `Result<Vec<ArtifactSummary>, OrqaError>`

**Error cases:**
- `Validation` — invalid type or `"doc"` passed
- `NotFound` — no active project

**TS mirror:** `Promise<ArtifactSummary[]>`

---

### `governance_read`

Read a governance artifact file from the active project's `.claude/` directory. Returns an `Artifact` struct with content loaded from disk. Path traversal (`..`) is rejected.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `rel_path` | `String` | Yes | Relative path within project (e.g. `".claude/agents/backend-engineer.md"`) |

**Returns:** `Result<Artifact, OrqaError>`

**Error cases:**
- `Validation` — path contains `..`
- `NotFound` — no active project or file does not exist
- `FileSystem` — file read failure

**TS mirror:** `Promise<Artifact>`

---

## Project Settings Commands (File-Based)

### `project_settings_read`

Read project settings from the `.orqa/project.json` file in the project directory. Returns `None` if the settings file does not exist yet.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | `String` | Yes | Absolute path to the project directory |

**Returns:** `Result<Option<ProjectSettings>, OrqaError>`

**Error cases:**
- `Serialization` — malformed JSON in settings file

**TS mirror:** `Promise<ProjectSettings | null>`

---

### `project_settings_write`

Write project settings to the `.orqa/project.json` file. Creates the `.orqa/` directory if it does not exist. Returns the written settings for confirmation.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | `String` | Yes | Absolute path to the project directory |
| `settings` | `ProjectSettings` | Yes | Settings to write |

**Returns:** `Result<ProjectSettings, OrqaError>`

**Error cases:**
- `FileSystem` — cannot create directory or write file
- `Serialization` — settings cannot be serialized

**TS mirror:** `Promise<ProjectSettings>`

---

### `project_scan`

Scan a project directory for language, framework, and governance info. Uses file extension heuristics and root-level config file detection.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `path` | `String` | Yes | Absolute path to the project directory |
| `excluded_paths` | `Option<Vec<String>>` | No | Directory names to skip (defaults to node_modules, .git, target, dist, build) |

**Returns:** `Result<ProjectScanResult, OrqaError>`

**Error cases:**
- `Validation` — path does not exist or is not a directory

**TS mirror:** `Promise<ProjectScanResult>`

---

### `project_icon_upload`

Upload a project icon by copying an image file to `.orqa/icon.{ext}`. Validates file extension and removes any existing icon files before copying.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_path` | `String` | Yes | Absolute path to the project directory |
| `source_path` | `String` | Yes | Absolute path to the source image file |

**Returns:** `Result<String, OrqaError>` — icon filename (e.g. `"icon.png"`)

**Error cases:**
- `NotFound` — source file does not exist
- `Validation` — unsupported file extension (allowed: png, jpg, jpeg, svg, ico)
- `FileSystem` — cannot copy file

**TS mirror:** `Promise<string>`

---

### `project_icon_read`

Read a project icon and return it as a base64-encoded data URI (`data:{mime};base64,...`).

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_path` | `String` | Yes | Absolute path to the project directory |
| `icon_filename` | `String` | Yes | Icon filename returned by `project_icon_upload` |

**Returns:** `Result<String, OrqaError>` — data URI string

**Error cases:**
- `NotFound` — icon file does not exist
- `FileSystem` — file read failure

**TS mirror:** `Promise<string>`

---

## Search Commands

### `index_codebase`

Index a project's codebase for search. Creates or replaces a DuckDB index at `<project_path>/.orqa/search.duckdb`. Chunks source files and stores them for regex and semantic search.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_path` | `String` | Yes | Absolute path to the project directory |
| `excluded_paths` | `Vec<String>` | Yes | Directory names to skip during indexing |

**Returns:** `Result<IndexStatus, String>`

**Error cases:**
- Returns `Err(String)` on file system or indexing failures

**TS mirror:** `Promise<IndexStatus>`

---

### `search_regex`

Regex search across indexed code chunks. The codebase must be indexed first via `index_codebase`.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `pattern` | `String` | Yes | Regex pattern to search for |
| `path` | `Option<String>` | No | Filter results to a specific file path |
| `max_results` | `Option<u32>` | No | Maximum results to return (default 20) |

**Returns:** `Result<Vec<SearchResult>, String>`

**Error cases:**
- Returns `Err(String)` if search index not initialized

**TS mirror:** `Promise<SearchResult[]>`

---

### `search_semantic`

Semantic similarity search across indexed code. Embeds the query and finds the most similar code chunks. Requires codebase to be indexed and ONNX embedding model to be initialized.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | `String` | Yes | Natural language query |
| `max_results` | `Option<u32>` | No | Maximum results to return (default 10) |

**Returns:** `Result<Vec<SearchResult>, String>`

**Error cases:**
- Returns `Err(String)` if search index not initialized

**TS mirror:** `Promise<SearchResult[]>`

---

### `get_index_status`

Get the current status of the code search index for a project. If no engine is loaded but a database file exists on disk, it will be loaded automatically.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_path` | `String` | Yes | Absolute path to the project directory |

**Returns:** `Result<IndexStatus, String>`

**Error cases:**
- Returns `Err(String)` on lock failures

**TS mirror:** `Promise<IndexStatus>`

---

### `init_embedder`

Initialize the ONNX embedding model, downloading from Hugging Face if needed. Must be called before `search_semantic` can be used.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `model_dir` | `String` | Yes | Path to store/load the embedding model |

**Returns:** `Result<(), String>`

**Error cases:**
- Returns `Err(String)` on download or model loading failure

**TS mirror:** `Promise<void>`

---

## Startup Commands

### `get_startup_status`

Get the current status of all startup initialization tasks (sidecar launch, embedding model download). Returns a snapshot with each task's status and optional detail string.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| *(none)* | | | |

**Returns:** `Result<StartupSnapshot, String>`

**Error cases:**
- Returns `Err(String)` on lock failures

**TS mirror:** `Promise<StartupSnapshot>`

---

## Channel Event Types (Streaming)

These are **not** commands. They are the event payloads sent over `Channel<StreamEvent>` during active streaming ([AD-009](AD-009)). The frontend registers a callback when calling `stream_send_message` and receives these events in order.

### `StreamEvent` Enum

```rust
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", content = "data")]
pub enum StreamEvent {
    /// Streaming has started. Sent once at the beginning.
    /// When model is "auto", resolved_model contains the actual model chosen by the provider.
    StreamStart {
        message_id: i64,
        resolved_model: Option<String>,
    },

    /// A chunk of assistant text content.
    TextDelta {
        content: String,
    },

    /// A chunk of assistant thinking/reasoning content.
    ThinkingDelta {
        content: String,
    },

    /// The assistant is starting a tool call.
    ToolUseStart {
        tool_call_id: String,
        tool_name: String,
    },

    /// A chunk of tool call input JSON (streamed incrementally).
    ToolInputDelta {
        tool_call_id: String,
        content: String,
    },

    /// A tool call has completed execution. Contains the result.
    ToolResult {
        tool_call_id: String,
        tool_name: String,
        result: String,
        is_error: bool,
    },

    /// A content block (text, thinking, or tool_use) has finished.
    BlockComplete {
        block_index: i32,
        content_type: String,
    },

    /// The full assistant turn is complete. Token usage is final.
    TurnComplete {
        input_tokens: i64,
        output_tokens: i64,
    },

    /// An error occurred during streaming.
    StreamError {
        code: String,
        message: String,
        recoverable: bool,
    },

    /// Streaming was cancelled by the user (via stream_stop).
    StreamCancelled,
}
```

### Event Sequence (Normal Flow)

```
StreamStart { message_id, resolved_model }  # resolved_model is non-null when session model is "auto"
  TextDelta { content }        # repeated, 0+ times
  ThinkingDelta { content }    # repeated, 0+ times (if extended thinking is on)
  BlockComplete { ... }        # after each content block finishes
  ToolUseStart { ... }         # 0+ tool calls
    ToolInputDelta { ... }     # repeated per tool call
  BlockComplete { ... }
  ToolResult { ... }           # after tool execution
  TextDelta { content }        # assistant may continue after tool results
  BlockComplete { ... }
TurnComplete { input_tokens, output_tokens }
```

### Event Sequence (Error Flow)

```
StreamStart { message_id }
  TextDelta { content }        # partial content may have arrived
  StreamError { code, message, recoverable }
```

### Event Sequence (Cancellation Flow)

```
StreamStart { message_id }
  TextDelta { content }        # partial content
  StreamCancelled
```

---

## TypeScript Mirror Types

All types used by the frontend to communicate with the Rust backend. These are generated or hand-maintained to stay in sync with the Rust `serde::Serialize` types.

```typescript
// =============================================================================
// ui/lib/types/ipc.ts
// Generated from Rust types — keep in sync with src-tauri/src/domain/types.rs
// =============================================================================

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

export interface OrqaError {
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
  message: string;
}

// ---------------------------------------------------------------------------
// Project
// ---------------------------------------------------------------------------

export interface Project {
  id: number;
  name: string;
  path: string;
  description: string | null;
  detected_stack: DetectedStack | null;
  created_at: string; // ISO 8601
  updated_at: string;
}

export interface ProjectSummary {
  id: number;
  name: string;
  path: string;
  detected_stack: DetectedStack | null;
  session_count: number;
  artifact_count: number;
  updated_at: string;
}

export interface DetectedStack {
  languages: string[];
  frameworks: string[];
  package_manager: string | null;
  has_claude_config: boolean;
  has_design_tokens: boolean;
}

export interface ScanResult {
  project_id: number;
  detected_stack: DetectedStack;
  artifact_counts: Record<ArtifactType, number>;
  design_tokens_found: boolean;
  scan_duration_ms: number;
}

// ---------------------------------------------------------------------------
// Session
// ---------------------------------------------------------------------------

export interface Session {
  id: number;
  project_id: number;
  title: string | null;
  model: string;
  system_prompt: string | null;
  status: SessionStatus;
  summary: string | null;
  handoff_notes: string | null;
  total_input_tokens: number;
  total_output_tokens: number;
  total_cost_usd: number;
  created_at: string;
  updated_at: string;
}

export interface SessionSummary {
  id: number;
  title: string | null;
  status: SessionStatus;
  message_count: number;
  preview: string | null; // first user message snippet
  created_at: string;
  updated_at: string;
}

export type SessionStatus = "active" | "completed" | "abandoned" | "error";

// ---------------------------------------------------------------------------
// Message
// ---------------------------------------------------------------------------

export interface Message {
  id: number;
  session_id: number;
  role: MessageRole;
  content_type: ContentType;
  content: string | null;
  tool_call_id: string | null;
  tool_name: string | null;
  tool_input: string | null; // JSON string
  tool_is_error: boolean;
  turn_index: number;
  block_index: number;
  stream_status: StreamStatus;
  input_tokens: number | null;
  output_tokens: number | null;
  created_at: string;
}

export type MessageRole = "user" | "assistant" | "system";
export type ContentType = "text" | "tool_use" | "tool_result" | "thinking" | "image";
export type StreamStatus = "pending" | "complete" | "error";

export type MessageId = number;

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

export interface SearchResult {
  message_id: number;
  session_id: number;
  session_title: string | null;
  content: string;
  highlighted: string; // content with <mark> tags
  rank: number;
}

// ---------------------------------------------------------------------------
// Artifact
// ---------------------------------------------------------------------------

export interface Artifact {
  id: number;
  project_id: number;
  artifact_type: ArtifactType;
  rel_path: string;
  name: string;
  description: string | null;
  content: string; // full file content, read from disk
  file_hash: string | null;
  file_size: number | null;
  file_modified_at: string | null;
  compliance_status: ComplianceStatus;
  relationships: ArtifactRelationship[] | null;
  metadata: Record<string, unknown> | null; // parsed YAML frontmatter
  created_at: string;
  updated_at: string;
}

export interface ArtifactSummary {
  id: number;
  artifact_type: ArtifactType;
  rel_path: string;
  name: string;
  description: string | null;
  compliance_status: ComplianceStatus;
  file_modified_at: string | null;
}

export type ArtifactType = "agent" | "rule" | "skill" | "hook" | "doc";

export type ComplianceStatus = "compliant" | "non_compliant" | "unknown" | "error";

export interface ArtifactRelationship {
  type: "references" | "extends" | "depends_on";
  target: string; // relative path
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

export interface ResolvedTheme {
  project_id: number;
  tokens: Record<string, ThemeToken>;
  source_files: string[];
  has_overrides: boolean;
}

export interface ThemeToken {
  name: string;
  value_light: string; // OKLCH
  value_dark: string | null; // OKLCH
  source: "extracted" | "override" | "default";
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

// Settings are accessed as key-value pairs.
// No dedicated type — values are `unknown` (JSON).
// Common setting keys for Phase 1:
//   "theme_mode"        -> "light" | "dark" | "system"
//   "font_size"         -> number
//   "default_model"     -> "auto" | "claude-opus-4-6" | "claude-sonnet-4-6" | "claude-haiku-4-5"
//   "last_project_id"   -> number
//   "last_session_id"   -> number
//   "nav_panel_collapsed" -> boolean

// ---------------------------------------------------------------------------
// Sidecar
// ---------------------------------------------------------------------------

export interface SidecarStatus {
  state: SidecarState;
  pid: number | null;
  uptime_seconds: number | null;
  cli_detected: boolean;
  cli_version: string | null;
  error_message: string | null;
}

export type SidecarState = "not_started" | "starting" | "connected" | "error" | "stopped";

// ---------------------------------------------------------------------------
// Streaming Events (received via Channel<T>, not invoke)
// ---------------------------------------------------------------------------

export type StreamEvent =
  | { type: "stream_start"; data: { message_id: number; resolved_model: string | null } }
  | { type: "text_delta"; data: { content: string } }
  | { type: "thinking_delta"; data: { content: string } }
  | { type: "tool_use_start"; data: { tool_call_id: string; tool_name: string } }
  | { type: "tool_input_delta"; data: { tool_call_id: string; content: string } }
  | { type: "tool_result"; data: { tool_call_id: string; tool_name: string; result: string; is_error: boolean } }
  | { type: "block_complete"; data: { block_index: number; content_type: string } }
  | { type: "turn_complete"; data: { input_tokens: number; output_tokens: number } }
  | { type: "stream_error"; data: { code: string; message: string; recoverable: boolean } }
  | { type: "stream_cancelled"; data: null };
```

---

## Command Registration

All commands are registered in the Tauri builder. This is the canonical list:

```rust
// src-tauri/src/lib.rs
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        // Sidecar
        sidecar_status,
        sidecar_restart,
        // Streaming
        stream_send_message,
        stream_stop,
        // Project
        project_open,
        project_create,
        project_get,
        project_get_active,
        project_list,
        // Session
        session_create,
        session_list,
        session_get,
        session_update_title,
        session_end,
        session_delete,
        // Message
        message_list,
        message_search,
        // Artifact
        artifact_list,
        artifact_get,
        artifact_get_by_path,
        artifact_create,
        artifact_update,
        artifact_delete,
        // Documentation (artifact_commands.rs)
        doc_read,
        doc_tree_scan,
        governance_list,
        governance_read,
        // Project settings (file-based)
        project_settings_read,
        project_settings_write,
        project_scan,
        project_icon_upload,
        project_icon_read,
        // Settings (SQLite key-value)
        settings_get,
        settings_set,
        settings_get_all,
        // Theme
        theme_get_project,
        theme_set_override,
        theme_clear_overrides,
        // Search
        index_codebase,
        search_regex,
        search_semantic,
        get_index_status,
        init_embedder,
        // Startup
        get_startup_status,
    ])
```

**Total: 39 commands + 10 streaming event types.**

---

## Command Summary

| Domain | Command | Method | Description |
|--------|---------|--------|-------------|
| Sidecar | `sidecar_status` | invoke | Get sidecar process status |
| Sidecar | `sidecar_restart` | invoke | Restart sidecar process |
| Streaming | `stream_send_message` | invoke + Channel | Send message, stream AI response |
| Streaming | `stream_stop` | invoke | Cancel active stream |
| Project | `project_open` | invoke | Open directory as project, scan codebase |
| Project | `project_create` | invoke | Create new project with scaffold |
| Project | `project_get` | invoke | Get full project details |
| Project | `project_get_active` | invoke | Get currently active project |
| Project | `project_list` | invoke | List all registered projects |
| Session | `session_create` | invoke | Create new conversation session |
| Session | `session_list` | invoke | List sessions for a project |
| Session | `session_get` | invoke | Get full session details |
| Session | `session_update_title` | invoke | Update session display title |
| Session | `session_end` | invoke | End session, trigger handoff summary |
| Session | `session_delete` | invoke | Delete session and all messages |
| Message | `message_list` | invoke | List messages for a session |
| Message | `message_search` | invoke | FTS5 search across project messages |
| Artifact | `artifact_list` | invoke | List artifacts by type |
| Artifact | `artifact_get` | invoke | Get artifact with disk content |
| Artifact | `artifact_get_by_path` | invoke | Get artifact by relative path |
| Artifact | `artifact_create` | invoke | Create artifact file + index |
| Artifact | `artifact_update` | invoke | Update artifact file + re-index |
| Artifact | `artifact_delete` | invoke | Delete artifact file + record |
| Documentation | `doc_read` | invoke | Read a documentation page by slug path |
| Documentation | `doc_tree_scan` | invoke | Scan docs/ directory and return tree structure |
| Documentation | `governance_list` | invoke | List all governance artifacts by category |
| Documentation | `governance_read` | invoke | Read a governance artifact by relative path |
| Project Settings | `project_settings_read` | invoke | Read file-based project settings (.orqa/project.json) |
| Project Settings | `project_settings_write` | invoke | Write file-based project settings |
| Project Settings | `project_scan` | invoke | Re-run project codebase scan |
| Project Settings | `project_icon_upload` | invoke | Upload/copy a project icon image |
| Project Settings | `project_icon_read` | invoke | Read project icon as base64 data |
| Settings | `settings_get` | invoke | Get single setting value |
| Settings | `settings_set` | invoke | Set single setting value |
| Settings | `settings_get_all` | invoke | Get all settings for scope |
| Theme | `theme_get_project` | invoke | Get resolved theme tokens |
| Theme | `theme_set_override` | invoke | Set manual token override |
| Theme | `theme_clear_overrides` | invoke | Clear all manual overrides |
| Search | `index_codebase` | invoke | Index project files into DuckDB for code search |
| Search | `search_regex` | invoke | Regex search across indexed codebase |
| Search | `search_semantic` | invoke | Semantic similarity search using ONNX embeddings |
| Search | `get_index_status` | invoke | Get codebase index statistics |
| Search | `init_embedder` | invoke | Initialize the ONNX embedding model |
| Startup | `get_startup_status` | invoke | Get status of async startup tasks |

---

## Frontend Usage Patterns

### Invoke Wrapper

All `invoke()` calls go through a typed wrapper that handles error unwrapping:

```typescript
// ui/lib/ipc.ts
import { invoke } from "@tauri-apps/api/core";
import type { OrqaError } from "$lib/types/ipc";

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    // Tauri serializes OrqaError as a JSON string
    throw typeof error === "string" ? JSON.parse(error) as OrqaError : error;
  }
}
```

### Streaming Setup

```typescript
// ui/lib/ipc.ts
import { Channel } from "@tauri-apps/api/core";
import type { StreamEvent } from "$lib/types/ipc";

export function createStreamChannel(
  onEvent: (event: StreamEvent) => void
): Channel<StreamEvent> {
  const channel = new Channel<StreamEvent>();
  channel.onmessage = onEvent;
  return channel;
}

// Usage in a page/container:
const channel = createStreamChannel((event) => {
  switch (event.type) {
    case "text_delta":
      appendToCurrentMessage(event.data.content);
      break;
    case "tool_use_start":
      addToolCallCard(event.data);
      break;
    case "turn_complete":
      finalizeMessage(event.data);
      break;
    case "stream_error":
      handleStreamError(event.data);
      break;
    // ...
  }
});

await invoke("stream_send_message", {
  session_id: activeSessionId,
  content: userInput,
  on_event: channel,
});
```

---

## Related Documents

- [SQLite Schema](DOC-013) — Table definitions these commands operate on
- [Architecture Decisions](DOC-001) — [AD-002](AD-002) (IPC boundary), [AD-003](AD-003) (errors), [AD-009](AD-009) (streaming)
- [MVP Specification](DOC-042) — Features F-001 through F-013 that these commands support
- [Information Architecture](DOC-040) — UI views that call these commands
