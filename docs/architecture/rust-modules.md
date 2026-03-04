# Rust Module Architecture

**Date:** 2026-03-02 | **Updated:** 2026-03-04 | **Status:** Aligned with Phase 1 implementation | **References:** [Claude Integration](/research/claude-integration), [Tauri v2](/research/tauri-v2), [Persistence](/research/persistence)

Module tree, domain types, command handlers, and dependency graph for `src-tauri/src/`. Rust owns the domain model (AD-001). All functions return `Result<T, E>` (AD-003). No `unwrap()`, `expect()`, or `panic!()` in production code.

---

## 1. Module Tree

```
src-tauri/src/
├── main.rs                          # Tauri entry point (calls lib::run())
├── lib.rs                           # App builder, plugin registration, command registration, startup
├── state.rs                         # AppState struct (Tauri managed state)
├── error.rs                         # OrqaError enum (thiserror + serde), Result type alias
├── db.rs                            # Database initialization (rusqlite, PRAGMAs, migrations)
├── startup.rs                       # StartupTracker: async task status for frontend polling
│
├── domain/                          # Domain model types — no dependencies on Tauri or DB
│   ├── mod.rs                       # Re-exports all domain types
│   ├── project.rs                   # Project, ProjectSummary, DetectedStack, ScanResult
│   ├── session.rs                   # Session, SessionStatus
│   ├── message.rs                   # Message, ContentType, StreamStatus, Role
│   ├── artifact.rs                  # Artifact, ArtifactType, ComplianceStatus, DocNode
│   ├── settings.rs                  # Setting, SettingScope
│   ├── project_scanner.rs           # Filesystem walking for language/framework detection
│   ├── project_settings.rs          # File-based ProjectSettings (.orqa/project.json), GovernanceCounts
│   └── provider_event.rs            # ProviderEvent enum (streaming protocol)
│
├── repo/                            # Repository layer — database access (AD-014)
│   ├── mod.rs                       # Re-exports
│   ├── project_repo.rs              # ProjectRepo: CRUD for projects table
│   ├── session_repo.rs              # SessionRepo: CRUD for sessions table
│   ├── message_repo.rs              # MessageRepo: insert, update stream, FTS queries
│   ├── artifact_repo.rs             # ArtifactRepo: CRUD + FTS for artifacts table
│   ├── settings_repo.rs             # SettingsRepo: key-value with scope
│   └── theme_repo.rs                # ThemeRepo: project_themes + overrides
│
├── commands/                        # Tauri command handlers (#[tauri::command])
│   ├── mod.rs                       # Re-exports all command functions for registration
│   ├── project_commands.rs          # project_open, project_create, project_get, project_get_active, project_list
│   ├── session_commands.rs          # session_create, session_list, session_get, session_update_title, session_end, session_delete
│   ├── message_commands.rs          # message_list, message_search
│   ├── artifact_commands.rs         # artifact CRUD + doc_read, doc_tree_scan, governance_list, governance_read
│   ├── stream_commands.rs           # stream_send_message, stream_stop (Channel<T> streaming)
│   ├── sidecar_commands.rs          # sidecar_status, sidecar_restart
│   ├── settings_commands.rs         # settings_get, settings_set, settings_get_all
│   ├── project_settings_commands.rs # project_settings_read/write, project_scan, project_icon_upload/read
│   ├── search_commands.rs           # index_codebase, search_regex, search_semantic, get_index_status, init_embedder, get_startup_status
│   └── theme_commands.rs            # theme_get_project, theme_set_override, theme_clear_overrides
│
├── sidecar/                         # Sidecar process management (AD-007, AD-009)
│   ├── mod.rs                       # Re-exports
│   ├── manager.rs                   # SidecarManager: spawn via std::process::Command, health check
│   ├── protocol.rs                  # NDJSON serialization/deserialization, line framing
│   └── types.rs                     # SidecarRequest (6 variants), SidecarResponse (14 variants)
│
└── search/                          # DuckDB code indexer + ONNX semantic search (Phase 2-3)
    ├── mod.rs                       # SearchEngine: combined regex + semantic search interface
    ├── chunker.rs                   # Source code chunking for embedding
    ├── embedder.rs                  # ONNX Runtime embeddings (bge-small-en-v1.5, DirectML)
    ├── store.rs                     # DuckDB-backed vector store + inverted index
    └── types.rs                     # SearchResult, IndexStatus, ChunkInfo types
```

> **Modules removed from Phase 0e spec** (not yet implemented):
> - `tools/` — MCP tool implementations deferred to a later phase
> - `scanner/` — Codebase scanning logic moved into `domain/project_scanner.rs`
> - `watcher/` — Artifact file watcher deferred to a later phase
>
> **Modules added during Phase 1:**
> - `db.rs` — Direct `rusqlite` database initialization (replaces spec'd `tauri-plugin-sql`)
> - `startup.rs` — Async startup task tracker with status polling
> - `search/` — DuckDB-based code indexer with ONNX embeddings for semantic search
> - `commands/stream_commands.rs` — Streaming separated from message commands
> - `commands/project_settings_commands.rs` — File-based project settings
> - `commands/search_commands.rs` — Code search and indexing commands
> - `domain/project_scanner.rs` — Project filesystem scanning
> - `domain/project_settings.rs` — File-based settings model

---

## 2. Module Descriptions

### `main.rs` / `lib.rs`

Application entry point. `main.rs` calls `lib::run()`. `lib.rs` constructs the Tauri app builder inside a `.setup()` closure: initializes the database via `db::init_db()`, creates the `StartupTracker`, spawns the sidecar, pre-downloads the embedding model, registers all 6 plugins and 39 command handlers, and runs the app. Following Tauri v2 convention.

### `state.rs`

Defines `AppState`, the single struct passed as Tauri managed state. Holds: `db` (Mutex\<Connection\>), `sidecar` (SidecarManager — uses interior mutability, NOT Mutex-wrapped), `search` (Mutex\<Option\<SearchEngine\>\>), and `startup` (Arc\<StartupTracker\>). All command handlers receive `State<AppState>` as a parameter.

### `error.rs`

Defines `OrqaError` with 9 variants (via `thiserror` + `serde::Serialize`): NotFound, Database, FileSystem, Sidecar, Validation, Scan, Serialization, PermissionDenied, Search. Serialized as `{"code": "<variant>", "message": "<detail>"}` using `#[serde(tag = "code", content = "message")]`. Tauri auto-converts via blanket `impl<T: Serialize> From<T> for InvokeError`.

### `db.rs`

Database initialization using `rusqlite` directly (not `tauri-plugin-sql`). `init_db()` opens a connection, sets WAL mode and PRAGMAs, and runs the single migration file (`001_initial_schema.sql`) via `include_str!`. Returns a `rusqlite::Connection`.

### `startup.rs`

Generic startup task tracker. Tasks are registered with an ID and label, then updated with status (Pending, InProgress, Done, Error) and optional detail. The frontend polls via `get_startup_status` to show progress of long-running initialization (sidecar launch, embedding model download).

### `domain/`

Pure domain model types. No dependencies on Tauri, rusqlite, or serde_json beyond derive macros. This module is the source of truth for what Orqa Studio's data looks like (AD-001). Other modules depend on `domain/`; it depends on nothing else. Includes `project_scanner.rs` for filesystem walking and `project_settings.rs` for file-based project configuration.

### `repo/`

One repository per entity (AD-014). Each repo struct takes a database connection reference and provides typed methods for CRUD operations, queries, and FTS search. Repos return domain types, never raw SQL rows. All SQL is isolated to this layer. Currently 6 repos: project, session, message, artifact, settings, theme. (Spec'd repos for tasks, lessons, scanner_results, and metrics are deferred to later phases.)

### `commands/`

Thin command handlers. Each function is `#[tauri::command]`, receives `State<AppState>` and parameters, calls the appropriate repo or service, and returns `Result<T, OrqaError>`. No business logic lives here — commands are glue between the IPC boundary and the domain/repo layers. 11 command modules covering 39 total commands.

### `sidecar/`

Process lifecycle management for the Agent SDK sidecar binary. `SidecarManager` spawns the binary via `std::process::Command` (not `tauri-plugin-shell`), monitors its health, and handles restart. Uses interior mutability with per-field Mutex locks. The NDJSON protocol in `protocol.rs` handles stdin/stdout framing. `types.rs` defines `SidecarRequest` (6 variants) and `SidecarResponse` (14 variants) covering streaming events, health checks, summaries, and tool execution/approval.

### `search/`

DuckDB-based code indexer with ONNX embeddings for semantic search. `SearchEngine` provides combined regex + semantic search. `chunker.rs` splits source files into chunks. `embedder.rs` loads the bge-small-en-v1.5 ONNX model (auto-downloaded from Hugging Face on first use, using DirectML for GPU acceleration on Windows). `store.rs` manages the DuckDB vector store and inverted index.

---

## 3. Domain Types

All types derive `Debug`, `Clone`, `Serialize`, `Deserialize`. Integer IDs match SQLite `INTEGER PRIMARY KEY`.

### `domain/project.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub detected_stack: Option<DetectedStack>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedStack {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub package_manager: Option<String>,
    pub has_claude_config: bool,
    pub has_design_tokens: bool,
}
```

### `domain/session.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub project_id: i64,
    pub title: Option<String>,
    pub model: String,
    pub system_prompt: Option<String>,
    pub status: SessionStatus,
    pub summary: Option<String>,
    pub handoff_notes: Option<String>,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub total_cost_usd: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Completed,
    Abandoned,
    Error,
}
```

### `domain/message.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub session_id: i64,
    pub role: Role,
    pub content_type: ContentType,
    pub content: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_name: Option<String>,
    pub tool_input: Option<String>,      // JSON string
    pub tool_is_error: bool,
    pub turn_index: i32,
    pub block_index: i32,
    pub stream_status: StreamStatus,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    ToolUse,
    ToolResult,
    Thinking,
    Image,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StreamStatus {
    Pending,
    Complete,
    Error,
}
```

### `domain/artifact.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: i64,
    pub project_id: i64,
    pub artifact_type: ArtifactType,
    pub rel_path: String,
    pub name: String,
    pub description: Option<String>,
    pub content: String,                          // full file content, read from disk
    pub file_hash: Option<String>,
    pub file_size: Option<i64>,
    pub file_modified_at: Option<String>,
    pub compliance_status: ComplianceStatus,
    pub relationships: Option<Vec<ArtifactRelationship>>,  // deserialized, not raw JSON
    pub metadata: Option<serde_json::Value>,               // parsed YAML frontmatter
    pub created_at: String,
    pub updated_at: String,
}

/// A node in the documentation tree (used by doc_tree_scan).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocNode {
    pub label: String,
    pub path: Option<String>,
    pub children: Option<Vec<DocNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    Agent,
    Rule,
    Skill,
    Hook,
    Doc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Unknown,
    Error,
}
```

### `domain/settings.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,                   // JSON value
    pub scope: SettingScope,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SettingScope {
    App,
    Project(i64),                        // project_id
}
```

### `domain/project_settings.rs`

File-based project settings stored at `.orqa/project.json` within the project root. This is separate from the SQLite-backed `settings` table.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,            // filename in .orqa/ directory
    pub governance: GovernanceCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceCounts {
    pub agents: usize,
    pub rules: usize,
    pub skills: usize,
    pub hooks: usize,
    pub docs: usize,
}
```

> **Phase 0e domain types not yet implemented:** `task.rs`, `lesson.rs`, and `theme.rs` (as standalone modules) do not exist yet. Theme types are defined inline in `settings.rs`. Task and lesson domain types are deferred to later phases.

### `domain/provider_event.rs`

The provider-neutral streaming protocol (AD-017). Every sidecar implementation produces these events.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderEvent {
    /// Streaming text content
    TextDelta {
        text: String,
    },
    /// Extended thinking content
    ThinkingDelta {
        text: String,
    },
    /// AI is requesting a tool call
    ToolUseStart {
        tool_call_id: String,
        tool_name: String,
        input: serde_json::Value,
    },
    /// Result of a tool call being returned to the AI
    ToolResult {
        tool_call_id: String,
        content: String,
        is_error: bool,
    },
    /// A complete message turn has finished
    MessageComplete {
        input_tokens: i64,
        output_tokens: i64,
    },
    /// Session-level summary generated on end
    SessionSummary {
        summary: String,
        handoff_notes: String,
    },
    /// Sidecar or provider error
    Error {
        code: String,
        message: String,
        retryable: bool,
    },
    /// Sidecar is alive and ready
    Ready,
    /// Sidecar is shutting down
    Shutdown,
}
```

---

## 4. Command Handlers

Every command is registered in `lib.rs` via `tauri::Builder::invoke_handler(tauri::generate_handler![...])`. All return `Result<T, OrqaError>`. See [IPC Commands](/architecture/ipc-commands) for full parameter and return type documentation.

### `commands/project_commands.rs`

| Command | Description |
|---------|-------------|
| `project_open` | Open directory, run scan, upsert into DB |
| `project_create` | Create directory, scaffold `.claude/`, register |
| `project_get` | Fetch project by ID |
| `project_get_active` | Get currently active project |
| `project_list` | List all registered projects |

### `commands/session_commands.rs`

| Command | Description |
|---------|-------------|
| `session_create` | Create session, inject handoff notes from last session |
| `session_list` | List sessions ordered by updated_at DESC |
| `session_get` | Fetch session with metadata |
| `session_update_title` | Update session display title |
| `session_end` | Set status, trigger async summary generation |
| `session_delete` | Delete session and all messages |

### `commands/message_commands.rs`

| Command | Description |
|---------|-------------|
| `message_list` | Fetch all messages for a session, ordered |
| `message_search` | FTS5 cross-session search |

### `commands/stream_commands.rs`

| Command | Description |
|---------|-------------|
| `stream_send_message` | Write user message to DB, send to sidecar, stream response via Channel\<T\> |
| `stream_stop` | Cancel active stream |

### `commands/artifact_commands.rs`

| Command | Description |
|---------|-------------|
| `artifact_list` | List artifacts, optionally filtered by type |
| `artifact_get` | Fetch metadata + read file content from disk |
| `artifact_get_by_path` | Get artifact by relative path |
| `artifact_create` | Create template file on disk, index in DB |
| `artifact_update` | Write content to disk, update hash in DB |
| `artifact_delete` | Delete artifact file + DB record |
| `doc_read` | Read a documentation page by slug path |
| `doc_tree_scan` | Scan docs/ directory and return DocNode tree |
| `governance_list` | List all governance artifacts by category |
| `governance_read` | Read a governance artifact by relative path |

### `commands/sidecar_commands.rs`

| Command | Description |
|---------|-------------|
| `sidecar_status` | Return current sidecar state |
| `sidecar_restart` | Restart sidecar process |

### `commands/settings_commands.rs`

| Command | Description |
|---------|-------------|
| `settings_get` | Fetch a setting by key and scope |
| `settings_set` | Upsert a setting |
| `settings_get_all` | Fetch all settings for a scope |

### `commands/project_settings_commands.rs`

| Command | Description |
|---------|-------------|
| `project_settings_read` | Read file-based project settings (.orqa/project.json) |
| `project_settings_write` | Write file-based project settings |
| `project_scan` | Re-run project codebase scan |
| `project_icon_upload` | Upload/copy a project icon image |
| `project_icon_read` | Read project icon as base64 data URI |

### `commands/theme_commands.rs`

| Command | Description |
|---------|-------------|
| `theme_get_project` | Merge extracted tokens + overrides into final theme |
| `theme_set_override` | Create or update a manual theme override |
| `theme_clear_overrides` | Clear all manual theme overrides |

### `commands/search_commands.rs`

| Command | Description |
|---------|-------------|
| `index_codebase` | Index project files into DuckDB for code search |
| `search_regex` | Regex search across indexed codebase |
| `search_semantic` | Semantic similarity search using ONNX embeddings |
| `get_index_status` | Get codebase index statistics |
| `init_embedder` | Initialize the ONNX embedding model |
| `get_startup_status` | Get status of async startup tasks |

---

## 5. Repository Pattern (AD-014)

One repository struct per database entity. Each repo is stateless — it borrows a connection reference for each operation.

### Structure

```rust
// repo/project_repo.rs
pub struct ProjectRepo;

impl ProjectRepo {
    pub fn insert(conn: &Connection, project: &NewProject) -> Result<Project> { ... }
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Project>> { ... }
    pub fn get_by_path(conn: &Connection, path: &str) -> Result<Option<Project>> { ... }
    pub fn update(conn: &Connection, id: i64, update: &ProjectUpdate) -> Result<Project> { ... }
    pub fn delete(conn: &Connection, id: i64) -> Result<()> { ... }
}
```

### Connection Management

```rust
// repo/mod.rs
use rusqlite::Connection;
use std::sync::Mutex;

/// Type alias for the connection pool.
/// rusqlite::Connection is not Send, so it is wrapped in a Mutex
/// within Tauri managed state.
pub type DbPool = Mutex<Connection>;
```

The `AppState` holds a `DbPool`. Command handlers lock the mutex, obtain a `&Connection`, and pass it to repository methods. WAL mode ensures read operations do not block on write flushes during streaming.

### Repos

| Repo | Entity | Key Queries |
|------|--------|-------------|
| `ProjectRepo` | `projects` | by_id, by_path, upsert |
| `SessionRepo` | `sessions` | by_id, list_by_project, update_status, update_tokens |
| `MessageRepo` | `messages` | insert, update_stream_content, by_session_ordered, fts_search |
| `ArtifactRepo` | `artifacts` | by_id, by_project_and_type, upsert_by_path, fts_search |
| `SettingsRepo` | `settings` | get_by_key_scope, upsert, all_by_scope |
| `ThemeRepo` | `project_themes`, `project_theme_overrides` | active_theme, upsert_theme, upsert_override |

> **Phase 0e repos not yet implemented:** `TaskRepo`, `LessonRepo`, `ScannerRepo`, `MetricsRepo` are deferred until their corresponding domain types and tables are created.

### New / Update DTOs

Each repo uses dedicated structs for insert and update operations to avoid partial-object confusion:

```rust
// Used by ProjectRepo::insert — no id, no timestamps
pub struct NewProject {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub detected_stack: Option<DetectedStack>,
}

// Used by ProjectRepo::update — all fields optional
pub struct ProjectUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub detected_stack: Option<DetectedStack>,
}
```

---

## 6. Claude / Sidecar Integration (AD-007, AD-009)

### Sidecar Manager

`sidecar/manager.rs` owns the lifecycle of the sidecar binary. Uses `std::process::Command` (not `tauri-plugin-shell`) for process spawning. SidecarManager uses interior mutability with per-field Mutex locks, so it is NOT wrapped in a Mutex in AppState.

```
                                  ┌──────────────────────┐
                                  │   SidecarManager     │
                                  │                      │
   ensure_sidecar_running() ─────>│  spawn()             │
                                  │    │                  │
                                  │    ├─ std::process::  │
                                  │    │  Command::new()  │
                                  │    │                  │
                                  │    ├─ stdin handle ───┼──> write NDJSON requests
                                  │    │                  │
                                  │    └─ stdout handle ──┼──> StreamHandler
                                  │                      │
   sidecar_restart() ────────────>│  kill() + spawn()    │
                                  │                      │
   sidecar_status() ─────────────>│  status()            │
                                  │  (NotStarted |       │
                                  │   Starting |         │
                                  │   Connected |        │
                                  │   Error(String))     │
                                  └──────────────────────┘
```

### NDJSON Protocol

`sidecar/protocol.rs` handles framing. Each line on stdin/stdout is a self-contained JSON object terminated by `\n`.

```rust
// Writing a request to the sidecar
pub fn write_request(stdin: &mut ChildStdin, request: &SidecarRequest) -> Result<()> {
    let json = serde_json::to_string(request)?;
    stdin.write_all(json.as_bytes())?;
    stdin.write_all(b"\n")?;
    stdin.flush()?;
    Ok(())
}

// Reading events from the sidecar (blocking line reader in a spawned thread)
pub fn read_event(line: &str) -> Result<ProviderEvent> {
    let event: ProviderEvent = serde_json::from_str(line)?;
    Ok(event)
}
```

### Streaming Pipeline (AD-009)

```
Agent SDK (SSE from Claude API)
    │
    ▼
TypeScript sidecar (translate to ProviderEvent NDJSON)
    │
    ▼ stdout
Rust StreamHandler
    │  ├─ BufReader::read_line() in spawned thread
    │  ├─ serde_json::from_str::<ProviderEvent>()
    │  ├─ Write to DB (message_repo, buffered flush ~500ms)
    │  └─ channel.send(event)
    │
    ▼ Channel<ProviderEvent>
Tauri IPC (serialized JSON, ordered delivery)
    │
    ▼
Svelte onChannelMessage callback
    │  ├─ Accumulate text deltas into $state
    │  ├─ Render tool_use events as cards
    │  └─ Update token counts on MessageComplete
    │
    ▼
DOM (fine-grained reactive updates)
```

`sidecar/stream.rs` — the `StreamHandler`:

```rust
pub struct StreamHandler {
    channel: Channel<ProviderEvent>,
    message_repo: MessageRepo,
    buffer: String,                      // Accumulated text content
    last_flush: Instant,                 // Last DB write time
}

impl StreamHandler {
    /// Called for each NDJSON line from stdout.
    /// Parses the event, buffers text deltas, and forwards to the Channel.
    pub fn handle_line(&mut self, conn: &Connection, line: &str) -> Result<()> {
        let event = protocol::read_event(line)?;

        match &event {
            ProviderEvent::TextDelta { text } => {
                self.buffer.push_str(text);
                if self.last_flush.elapsed() > Duration::from_millis(500) {
                    self.flush_to_db(conn)?;
                }
            }
            ProviderEvent::MessageComplete { .. } => {
                self.flush_to_db(conn)?;
            }
            _ => {}
        }

        self.channel.send(event)?;
        Ok(())
    }
}
```

### Sidecar Types

```rust
// sidecar/types.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SidecarRequest {
    SendMessage { session_id: i64, content: String, model: Option<String>, system_prompt: Option<String> },
    CancelStream { session_id: i64 },
    GenerateSummary { session_id: i64, messages: Vec<MessageSummary> },
    HealthCheck,
    ToolResult { tool_call_id: String, output: String, is_error: bool },
    ToolApproval { tool_call_id: String, approved: bool, reason: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SidecarResponse {
    // Streaming events (forwarded to frontend via Channel<T>)
    StreamStart { message_id: i64, resolved_model: Option<String> },
    TextDelta { content: String },
    ThinkingDelta { content: String },
    ToolUseStart { tool_call_id: String, tool_name: String },
    ToolInputDelta { tool_call_id: String, content: String },
    ToolResult { tool_call_id: String, tool_name: String, result: String, is_error: bool },
    BlockComplete { block_index: i32, content_type: String },
    TurnComplete { input_tokens: i64, output_tokens: i64 },
    StreamError { code: String, message: String, recoverable: bool },
    StreamCancelled,
    // Non-streaming responses
    HealthOk { version: String },
    SummaryResult { session_id: i64, summary: String },
    // Tool execution (sidecar -> Orqa Studio)
    ToolExecute { tool_call_id: String, tool_name: String, input: String },
    ToolApprovalRequest { tool_call_id: String, tool_name: String, input: String },
}
```

> **Implementation Notes:** The Phase 0e spec had `SidecarResponse` as a simple `Event(ProviderEvent)` wrapper. The actual implementation flattens all events into `SidecarResponse` variants directly, making the protocol more explicit. The `SidecarRequest` now includes `CancelStream`, `HealthCheck`, `ToolResult`, and `ToolApproval` variants that evolved during Phase 1 implementation.

---

## 7. Error Types (AD-003)

### OrqaError

```rust
// error.rs
use serde::Serialize;

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

### Tauri Serialization

`OrqaError` derives `Serialize` with `#[serde(tag = "code", content = "message")]`, producing `{"code": "not_found", "message": "..."}`. Tauri auto-converts via its blanket `impl<T: Serialize> From<T> for InvokeError`. The Phase 0e spec used `From<OrqaError> for InvokeError` with `err.to_string()`, but the actual implementation uses serde for structured error JSON.

`From` impls exist for `std::io::Error` (-> FileSystem), `serde_json::Error` (-> Serialization), and `rusqlite::Error` (-> Database), all converting to string messages.

---

## 8. Dependency Graph

Arrows point from the dependent module to the module it depends on. The `domain` module is at the bottom -- it depends on nothing.

```
┌─────────────────────────────────────────────────────────┐
│                    main.rs / lib.rs                      │
│            (app builder, plugin registration,            │
│             startup tracker, command registration)       │
└────┬──────────┬──────────┬──────────┬───────────────────┘
     │          │          │          │
     ▼          ▼          ▼          ▼
┌──────────┐ ┌─────────┐ ┌────────┐ ┌───────────┐
│commands/ │ │sidecar/ │ │search/ │ │ startup.rs│
│          │ │         │ │        │ │           │
│ project  │ │ manager │ │ mod    │ └───────────┘
│ session  │ │protocol │ │chunker │
│ message  │ │ types   │ │embedder│
│ stream   │ │         │ │ store  │
│artifact  │ │         │ │ types  │
│ sidecar  │ │         │ │        │
│settings  │ │         │ └───┬────┘
│proj_sett │ │         │     │
│ search   │ │         │     │
│ theme    │ │         │     │
└────┬─────┘ └──┬──────┘     │
     │          │             │
     ▼          │             │
┌─────────┐    │  ┌──────────┐
│  repo/  │    │  │ state.rs │
│         │    │  │(AppState)│
│ project │    │  └────┬─────┘
│ session │    │       │
│ message │    │       │
│artifact │◄───┘       │
│settings │            │
│  theme  │            │
└────┬────┘            │
     │                 │
     ▼        ┌────────┘
┌─────────────┴───────────┐     ┌──────────┐
│       domain/           │     │ error.rs │
│                         │◄────│(OrqaErr)│
│ project  session        │     └──────────┘
│ message  artifact       │          ▲
│ settings                │          │
│ project_scanner         │     (all modules
│ project_settings        │      depend on
│ provider_event          │      error.rs)
└─────────────────────────┘
                          ┌──────────┐
                          │  db.rs   │
                          │(init_db) │
                          └──────────┘
```

### Dependency Rules

1. **`domain/`** depends on nothing (only std, serde, serde_json).
2. **`error.rs`** depends on thiserror, serde, rusqlite (for `From<rusqlite::Error>`), serde_json, and std::io.
3. **`db.rs`** depends on rusqlite and `error.rs`.
4. **`repo/`** depends on `domain/`, `error.rs`, and rusqlite.
5. **`commands/`** depends on `domain/`, `repo/`, `state.rs`, `error.rs`, `sidecar/`, `search/`, and `startup.rs`.
6. **`sidecar/`** depends on `domain/` (for Message), `error.rs`, and std::process.
7. **`search/`** depends on `domain/`, `error.rs`, duckdb, ort (ONNX Runtime), and tokenizers.
8. **`startup.rs`** depends on nothing (only std).
9. **`main.rs` / `lib.rs`** depends on everything (wires it all together).

### Circular Dependency Prevention

No module may depend on a module that depends on it. The layering is strict:

```
main/lib  →  commands  →  repo     →  domain
                       →  sidecar  →  domain
                       →  search   →  domain
```

---

## 9. State Management

### AppState

```rust
// state.rs
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::search::SearchEngine;
use crate::sidecar::manager::SidecarManager;
use crate::startup::StartupTracker;

pub struct AppState {
    /// SQLite connection (WAL mode, single writer, concurrent readers).
    /// Wrapped in Mutex because rusqlite::Connection is not Send.
    pub db: Mutex<Connection>,

    /// Sidecar process manager. Uses interior mutability via its own
    /// per-field Mutex locks — NOT wrapped in an outer Mutex.
    pub sidecar: SidecarManager,

    /// DuckDB-backed code search engine. Lazily initialized when a
    /// project is first indexed via index_codebase.
    pub search: Mutex<Option<SearchEngine>>,

    /// Tracks long-running startup tasks (sidecar launch, model download)
    /// for frontend polling via get_startup_status.
    pub startup: Arc<StartupTracker>,
}
```

### Registration

Database initialization and state construction happen inside `.setup()`. The PRAGMAs are set by `db::init_db()`, not inline.

```rust
// lib.rs (inside the run() function, .setup() closure)
let conn = db::init_db(db_path_str)?;

let tracker = startup::StartupTracker::new();
tracker.register("sidecar", "Sidecar");
tracker.register("embedding_model", "Embedding model");

let app_state = state::AppState {
    db: std::sync::Mutex::new(conn),
    sidecar: sidecar::manager::SidecarManager::new(),
    search: std::sync::Mutex::new(None),
    startup: Arc::clone(&tracker),
};
```

### Plugins (6)

```rust
.plugin(tauri_plugin_fs::init())
.plugin(tauri_plugin_shell::init())
.plugin(tauri_plugin_store::Builder::default().build())
.plugin(tauri_plugin_window_state::Builder::default().build())
.plugin(tauri_plugin_dialog::init())
.plugin(tauri_plugin_notification::init())
```

> **Note:** `tauri-plugin-sql` and `tauri-plugin-persisted-scope` from the Phase 0e spec are NOT used. Database access is via `rusqlite` directly through `db.rs`.

### Command Handler Pattern

Every command handler follows the same pattern:

```rust
#[tauri::command]
pub fn session_get(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<Session, OrqaError> {
    let conn = state.db.lock()
        .map_err(|e| OrqaError::Database(e.to_string()))?;
    SessionRepo::get_by_id(&conn, id)?
        .ok_or_else(|| OrqaError::NotFound(format!("session {id}")))
}
```

1. Extract `State<AppState>` from Tauri.
2. Lock the resource needed (db, sidecar, search, etc.).
3. Call the repository or service method.
4. Return `Result<T, OrqaError>`.

No business logic in the handler. No `unwrap()`. No `panic!()`.

---

## Related Documents

- [Architecture Decisions](/architecture/decisions) -- AD-001 through AD-018
- [IPC Commands](/architecture/ipc-commands) -- Full parameter and return type documentation for all 39 commands
- [SQLite Schema](/architecture/sqlite-schema) -- Full table definitions and migration strategy
- [MVP Feature Specification](/product/mvp-specification) -- Phase 1 scope and acceptance criteria
