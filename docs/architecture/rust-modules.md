# Rust Module Architecture

**Date:** 2026-03-02 | **Status:** Phase 0e specification | **References:** [Claude Integration](/research/claude-integration), [Tauri v2](/research/tauri-v2), [Persistence](/research/persistence)

Module tree, domain types, command handlers, and dependency graph for `src-tauri/src/`. Rust owns the domain model (AD-001). All functions return `Result<T, E>` (AD-003). No `unwrap()`, `expect()`, or `panic!()` in production code.

---

## 1. Module Tree

```
src-tauri/src/
├── main.rs                          # Tauri entry point, app builder, plugin registration
├── lib.rs                           # Re-exports, app builder function, migration setup
├── state.rs                         # AppState struct (Tauri managed state)
├── error.rs                         # ForgeError enum (thiserror), Result type alias
│
├── domain/                          # Domain model types — no dependencies on Tauri or DB
│   ├── mod.rs                       # Re-exports all domain types
│   ├── project.rs                   # Project, DetectedStack
│   ├── session.rs                   # Session, SessionStatus
│   ├── message.rs                   # Message, ContentType, StreamStatus, Role
│   ├── artifact.rs                  # Artifact, ArtifactType, ComplianceStatus
│   ├── task.rs                      # Task, TaskStatus
│   ├── lesson.rs                    # Lesson, Severity
│   ├── settings.rs                  # Setting, SettingScope
│   ├── theme.rs                     # ProjectTheme, ThemeOverride, DesignTokens
│   └── provider_event.rs           # ProviderEvent enum (streaming protocol)
│
├── repo/                            # Repository layer — database access (AD-014)
│   ├── mod.rs                       # Re-exports, DbPool type alias
│   ├── project_repo.rs              # ProjectRepo: CRUD for projects table
│   ├── session_repo.rs              # SessionRepo: CRUD for sessions table
│   ├── message_repo.rs              # MessageRepo: insert, update stream, FTS queries
│   ├── artifact_repo.rs             # ArtifactRepo: CRUD + FTS for artifacts table
│   ├── task_repo.rs                 # TaskRepo: CRUD for tasks table
│   ├── lesson_repo.rs               # LessonRepo: CRUD + occurrence tracking
│   ├── settings_repo.rs             # SettingsRepo: key-value with scope
│   ├── scanner_repo.rs              # ScannerRepo: insert + query scanner_results
│   ├── metrics_repo.rs              # MetricsRepo: insert + time-series queries
│   └── theme_repo.rs                # ThemeRepo: project_themes + overrides
│
├── commands/                        # Tauri command handlers (#[tauri::command])
│   ├── mod.rs                       # Re-exports all command functions for registration
│   ├── project_commands.rs          # project_open, project_create, project_get, project_scan
│   ├── session_commands.rs          # session_create, session_list, session_get, session_end
│   ├── message_commands.rs          # stream_send_message, message_list, message_search
│   ├── artifact_commands.rs         # artifact_list, artifact_get, artifact_update, artifact_create
│   ├── sidecar_commands.rs          # sidecar_restart, sidecar_status
│   ├── settings_commands.rs         # settings_get, settings_set, settings_get_all
│   └── theme_commands.rs            # theme_get_project, theme_set_override
│
├── sidecar/                         # Sidecar process management (AD-007, AD-009)
│   ├── mod.rs                       # Re-exports
│   ├── manager.rs                   # SidecarManager: spawn, kill, restart, health check
│   ├── protocol.rs                  # NDJSON serialization/deserialization, line framing
│   ├── stream.rs                    # StreamHandler: stdout reader → ProviderEvent parser → Channel<T>
│   └── types.rs                     # SidecarStatus, SidecarRequest, SidecarResponse
│
├── tools/                           # MCP tool implementations (AD-010)
│   ├── mod.rs                       # ToolRegistry, tool dispatch
│   ├── mcp_server.rs                # MCP server protocol handler (JSON-RPC over stdio)
│   ├── read.rs                      # Read tool: file content with line ranges
│   ├── write.rs                     # Write tool: create/overwrite files
│   ├── edit.rs                      # Edit tool: exact string replacement
│   ├── bash.rs                      # Bash tool: scoped shell command execution
│   ├── glob.rs                      # Glob tool: file pattern matching
│   ├── grep.rs                      # Grep tool: regex content search
│   └── security.rs                  # Path validation, scope enforcement, deny lists
│
├── scanner/                         # Codebase scanning (AD-016)
│   ├── mod.rs                       # ScanOrchestrator, ScanTier enum
│   ├── tier1.rs                     # Manifest heuristics: package.json, Cargo.toml, etc.
│   ├── tier2.rs                     # hyperpolyglot language detection
│   └── theme_extractor.rs           # Extract design tokens from tailwind.config, CSS vars
│
├── search/                          # Built-in code search (see search-engine.md)
│   ├── mod.rs                       # SearchEngine struct, public API
│   ├── store.rs                     # DuckDB connection, schema, queries
│   ├── chunker.rs                   # .gitignore-aware file walking, code splitting
│   ├── embedder.rs                  # ONNX Runtime + DirectML, bge-small-en-v1.5
│   └── types.rs                     # SearchResult, ChunkInfo, IndexStatus
│
└── watcher/                         # File system watcher for .claude/ artifacts
    ├── mod.rs                       # Re-exports
    └── artifact_watcher.rs          # notify-debouncer-full, 500ms debounce, SHA-256 diffing
```

---

## 2. Module Descriptions

### `main.rs` / `lib.rs`

Application entry point. Constructs the Tauri app builder, registers all plugins (AD-012), adds managed state, registers command handlers, and runs the app. `lib.rs` exposes a `run()` function that `main.rs` calls, following Tauri v2 convention. Migration setup for `tauri-plugin-sql` lives here.

### `state.rs`

Defines `AppState`, the single struct passed as Tauri managed state. Holds the database connection pool, sidecar manager handle, file watcher handle, and any shared configuration. All command handlers receive `State<AppState>` as a parameter.

### `error.rs`

Defines `ForgeError` (via `thiserror`) and a `Result<T>` type alias. Every function in the codebase returns this Result. The error type implements `Into<InvokeError>` so Tauri can serialize errors to the frontend.

### `domain/`

Pure domain model types. No dependencies on Tauri, rusqlite, or serde_json beyond derive macros. This module is the source of truth for what Forge's data looks like (AD-001). Other modules depend on `domain/`; it depends on nothing else.

### `repo/`

One repository per entity (AD-014). Each repo struct takes a database connection reference and provides typed methods for CRUD operations, queries, and FTS search. Repos return domain types, never raw SQL rows. All SQL is isolated to this layer.

### `commands/`

Thin command handlers. Each function is `#[tauri::command]`, receives `State<AppState>` and parameters, calls the appropriate repo or service, and returns `Result<T, ForgeError>`. No business logic lives here — commands are glue between the IPC boundary and the domain/repo layers.

### `sidecar/`

Process lifecycle management for the Agent SDK sidecar binary. `SidecarManager` spawns the Bun-compiled TypeScript binary via `tauri-plugin-shell`, monitors its health, and handles restart on crash. `StreamHandler` reads NDJSON lines from stdout, deserializes them into `ProviderEvent` variants, and forwards them through a Tauri `Channel<T>`.

### `tools/`

Native Rust implementations of the six core tools (Read, Write, Edit, Bash, Glob, Grep). Each tool is a struct implementing a common `Tool` trait. `mcp_server.rs` wraps these tools as an MCP server that the sidecar connects to. `security.rs` enforces path scope restrictions and deny lists (`.ssh`, `.gnupg`).

### `scanner/`

Codebase analysis at multiple tiers. Tier 1 reads manifest files (package.json, Cargo.toml, pyproject.toml) for fast heuristic detection. Tier 2 uses `hyperpolyglot` for language detection across all files. `theme_extractor.rs` parses Tailwind config and CSS custom properties to extract design tokens for per-project theming.

### `watcher/`

Watches the `.claude/` directory using `notify` crate with `notify-debouncer-full` (500ms debounce). When a governance artifact changes on disk, it recomputes the SHA-256 hash and updates the SQLite cache via `ArtifactRepo`. Emits Tauri events so the frontend can refresh.

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
    pub build_tool: Option<String>,
}
```

### `domain/session.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub project_id: i64,
    pub user_id: Option<String>,
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
    pub file_hash: Option<String>,
    pub file_size: Option<i64>,
    pub file_modified_at: Option<String>,
    pub last_scanned_at: Option<String>,
    pub compliance_status: ComplianceStatus,
    pub relationships: Option<String>,   // JSON
    pub metadata: Option<String>,        // JSON (extracted frontmatter)
    pub last_edited_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
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

### `domain/task.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub session_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: i32,
    pub assigned_agent: Option<String>,
    pub metadata: Option<String>,        // JSON
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}
```

### `domain/lesson.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: i64,
    pub project_id: i64,
    pub session_id: Option<i64>,
    pub title: String,
    pub pattern: String,
    pub fix: String,
    pub occurrence_count: i32,
    pub last_occurred_at: String,
    pub severity: Severity,
    pub metadata: Option<String>,        // JSON
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
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

### `domain/theme.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTheme {
    pub id: i64,
    pub project_id: i64,
    pub source_file: String,
    pub source_hash: String,
    pub extracted_at: String,
    pub tokens_light: DesignTokens,
    pub tokens_dark: Option<DesignTokens>,
    pub unmapped: Option<String>,        // JSON
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeOverride {
    pub id: i64,
    pub project_id: i64,
    pub token_name: String,
    pub value_light: String,             // OKLCH value
    pub value_dark: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignTokens {
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub accent: Option<String>,
    pub muted: Option<String>,
    pub destructive: Option<String>,
    // Extensible — additional tokens stored as key-value pairs
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, String>,
}
```

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

Every command is registered in `main.rs` / `lib.rs` via `tauri::Builder::invoke_handler(tauri::generate_handler![...])`. All return `Result<T, ForgeError>`.

### `commands/project_commands.rs`

| Command | Signature | Description |
|---------|-----------|-------------|
| `project_open` | `(path: String) -> Result<Project>` | Open directory, run Tier 1 scan, upsert into DB |
| `project_create` | `(name: String, parent: String) -> Result<Project>` | Create directory, scaffold `.claude/`, register |
| `project_get` | `(id: i64) -> Result<Project>` | Fetch project by ID |
| `project_scan` | `(id: i64, tier: u8) -> Result<DetectedStack>` | Run Tier 1 or Tier 2 scan, update project |

### `commands/session_commands.rs`

| Command | Signature | Description |
|---------|-----------|-------------|
| `session_create` | `(project_id: i64, model: Option<String>) -> Result<Session>` | Create session, inject handoff notes from last session |
| `session_list` | `(project_id: i64) -> Result<Vec<Session>>` | List sessions ordered by updated_at DESC |
| `session_get` | `(id: i64) -> Result<Session>` | Fetch session with metadata |
| `session_end` | `(id: i64) -> Result<()>` | Set status, trigger async summary generation |

### `commands/message_commands.rs`

| Command | Signature | Description |
|---------|-----------|-------------|
| `stream_send_message` | `(session_id: i64, content: String, channel: Channel<ProviderEvent>) -> Result<()>` | Write user message to DB, send to sidecar, stream response via Channel |
| `message_list` | `(session_id: i64) -> Result<Vec<Message>>` | Fetch all messages for a session, ordered |
| `message_search` | `(project_id: i64, query: String) -> Result<Vec<SearchResult>>` | FTS5 cross-session search |

### `commands/artifact_commands.rs`

| Command | Signature | Description |
|---------|-----------|-------------|
| `artifact_list` | `(project_id: i64, artifact_type: Option<ArtifactType>) -> Result<Vec<Artifact>>` | List artifacts, optionally filtered by type |
| `artifact_get` | `(id: i64) -> Result<ArtifactDetail>` | Fetch metadata + read file content from disk |
| `artifact_update` | `(id: i64, content: String) -> Result<Artifact>` | Write content to disk, update hash in DB |
| `artifact_create` | `(project_id: i64, artifact_type: ArtifactType, name: String) -> Result<Artifact>` | Create template file on disk, index in DB |

### `commands/sidecar_commands.rs`

| Command | Signature | Description |
|---------|-----------|-------------|
| `sidecar_restart` | `() -> Result<SidecarStatus>` | Kill current sidecar (if any) and spawn a new one |
| `sidecar_status` | `() -> Result<SidecarStatus>` | Return current sidecar state |

### `commands/settings_commands.rs`

| Command | Signature | Description |
|---------|-----------|-------------|
| `settings_get` | `(key: String, scope: SettingScope) -> Result<Option<Setting>>` | Fetch a setting by key and scope |
| `settings_set` | `(key: String, value: String, scope: SettingScope) -> Result<()>` | Upsert a setting |
| `settings_get_all` | `(scope: SettingScope) -> Result<Vec<Setting>>` | Fetch all settings for a scope |

### `commands/theme_commands.rs`

| Command | Signature | Description |
|---------|-----------|-------------|
| `theme_get_project` | `(project_id: i64) -> Result<ResolvedTheme>` | Merge extracted tokens + overrides into final theme |
| `theme_set_override` | `(project_id: i64, token: String, light: String, dark: Option<String>) -> Result<()>` | Create or update a manual theme override |

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
| `TaskRepo` | `tasks` | by_id, by_project_and_status, update_status |
| `LessonRepo` | `lessons` | by_id, by_project, increment_occurrence |
| `SettingsRepo` | `settings` | get_by_key_scope, upsert, all_by_scope |
| `ScannerRepo` | `scanner_results` | insert, latest_by_scanner, by_project |
| `MetricsRepo` | `metrics` | insert, time_series_query, by_project_and_name |
| `ThemeRepo` | `project_themes`, `project_theme_overrides` | active_theme, upsert_theme, upsert_override |

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

`sidecar/manager.rs` owns the lifecycle of the Bun-compiled TypeScript sidecar binary.

```
                                  ┌──────────────────────┐
                                  │   SidecarManager     │
                                  │                      │
   sidecar_restart() ────────────>│  spawn()             │
                                  │    │                  │
                                  │    ├─ tauri-plugin-   │
                                  │    │  shell::spawn()  │
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SidecarStatus {
    NotStarted,
    Starting,
    Connected,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SidecarRequest {
    /// Send a user message for a conversation turn
    SendMessage {
        session_id: i64,
        content: String,
        system_prompt: Option<String>,
        history: Vec<Message>,
    },
    /// Request a session handoff summary
    GenerateSummary {
        session_id: i64,
        messages: Vec<Message>,
    },
    /// Graceful shutdown
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SidecarResponse {
    /// Wraps a ProviderEvent from the Agent SDK
    Event(ProviderEvent),
    /// MCP tool call routed from the Agent SDK to Forge
    ToolCall {
        tool_call_id: String,
        tool_name: String,
        input: serde_json::Value,
    },
}
```

---

## 7. Tool Implementations (AD-010)

### Tool Trait

```rust
// tools/mod.rs
use async_trait::async_trait;

#[async_trait]
pub trait Tool: Send + Sync {
    /// Tool name as registered in MCP (e.g., "Read", "Write")
    fn name(&self) -> &'static str;

    /// JSON Schema describing the tool's input parameters
    fn input_schema(&self) -> serde_json::Value;

    /// Execute the tool and return the result as a string
    async fn execute(&self, input: serde_json::Value) -> Result<ToolOutput>;
}

pub struct ToolOutput {
    pub content: String,
    pub is_error: bool,
}

/// Registry that maps tool names to implementations
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new(project_root: PathBuf) -> Self { ... }
    pub fn get(&self, name: &str) -> Option<&dyn Tool> { ... }
    pub fn list_definitions(&self) -> Vec<ToolDefinition> { ... }
}
```

### Tool Implementations

| Tool | Module | Input | Description |
|------|--------|-------|-------------|
| Read | `tools/read.rs` | `{ file_path, offset?, limit? }` | Read file contents with optional line range. Returns content with line numbers. |
| Write | `tools/write.rs` | `{ file_path, content }` | Create or overwrite a file. Creates parent directories if needed. |
| Edit | `tools/edit.rs` | `{ file_path, old_string, new_string, replace_all? }` | Exact string replacement. Fails if `old_string` is not found or not unique. |
| Bash | `tools/bash.rs` | `{ command, timeout? }` | Execute a shell command. Timeout defaults to 120s. Returns stdout + stderr. |
| Glob | `tools/glob.rs` | `{ pattern, path? }` | Find files matching a glob pattern. Returns sorted file paths. |
| Grep | `tools/grep.rs` | `{ pattern, path?, glob?, type?, output_mode? }` | Regex search across files. Supports context lines, file type filtering. |

### MCP Server

`tools/mcp_server.rs` implements the MCP protocol (JSON-RPC 2.0 over stdio) so the sidecar can call Forge's tools:

```
Sidecar (Agent SDK)                         Forge (Rust)
    │                                           │
    │  ── MCP initialize ────────────────────>  │
    │  <── capabilities + tool list ──────────  │
    │                                           │
    │  ── tools/call { name, input } ────────>  │
    │       │                                   │
    │       │              ToolRegistry.get(name)│
    │       │              tool.execute(input)   │
    │       │              security.validate()   │
    │       │                                   │
    │  <── result { content, is_error } ──────  │
```

The sidecar registers Forge's MCP server as `mcpServers: { "forge": { command: "..." } }` and disables built-in tools with `tools: []`.

### Security (`tools/security.rs`)

```rust
pub struct SecurityScope {
    allowed_roots: Vec<PathBuf>,         // Project root + $HOME
    denied_paths: Vec<PathBuf>,          // .ssh, .gnupg, etc.
}

impl SecurityScope {
    /// Returns Ok(canonical_path) if the path is within scope, Err otherwise.
    pub fn validate_path(&self, path: &Path) -> Result<PathBuf> { ... }

    /// Validate a shell command against the allowlist.
    pub fn validate_command(&self, command: &str) -> Result<()> { ... }
}
```

---

## 8. Error Types (AD-003)

### ForgeError

```rust
// error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForgeError {
    // --- Database ---
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Migration error: {0}")]
    Migration(String),

    // --- Sidecar ---
    #[error("Sidecar not running")]
    SidecarNotRunning,

    #[error("Sidecar spawn failed: {0}")]
    SidecarSpawnFailed(String),

    #[error("Sidecar communication error: {0}")]
    SidecarProtocol(String),

    #[error("Claude Code CLI not found — install with: npm install -g @anthropic-ai/claude-code")]
    ClaudeCliNotFound,

    // --- Tools ---
    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Tool execution failed: {tool_name}: {message}")]
    ToolExecution {
        tool_name: String,
        message: String,
    },

    // --- File System ---
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Path outside security scope: {0}")]
    PathOutOfScope(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // --- Serialization ---
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    // --- Domain ---
    #[error("Project not found: {0}")]
    ProjectNotFound(i64),

    #[error("Session not found: {0}")]
    SessionNotFound(i64),

    #[error("Artifact not found: {0}")]
    ArtifactNotFound(i64),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    // --- Provider ---
    #[error("Provider error: {code}: {message}")]
    Provider {
        code: String,
        message: String,
    },

    #[error("Rate limited — retry after {retry_after_secs}s")]
    RateLimited {
        retry_after_secs: u64,
    },
}

/// Convenience type alias used throughout the codebase.
pub type Result<T> = std::result::Result<T, ForgeError>;
```

### Tauri Serialization

Tauri command handlers require errors to be serializable. `ForgeError` implements `Into<tauri::ipc::InvokeError>` so that error details reach the frontend as structured JSON:

```rust
impl From<ForgeError> for tauri::ipc::InvokeError {
    fn from(err: ForgeError) -> Self {
        tauri::ipc::InvokeError::from(err.to_string())
    }
}
```

---

## 9. Dependency Graph

Arrows point from the dependent module to the module it depends on. The `domain` module is at the bottom — it depends on nothing.

```
┌─────────────────────────────────────────────────────────────────────┐
│                          main.rs / lib.rs                          │
│                   (app builder, plugin registration)               │
└────┬──────────┬──────────┬──────────┬───────────┬──────────────────┘
     │          │          │          │           │
     ▼          ▼          ▼          ▼           ▼
┌─────────┐ ┌─────────┐ ┌────────┐ ┌─────────┐ ┌─────────┐
│commands/│ │sidecar/ │ │tools/  │ │scanner/ │ │watcher/ │
│         │ │         │ │        │ │         │ │         │
│ project │ │ manager │ │ read   │ │ tier1   │ │artifact │
│ session │ │protocol │ │ write  │ │ tier2   │ │_watcher │
│ message │ │ stream  │ │ edit   │ │ theme   │ │         │
│artifact │ │ types   │ │ bash   │ │_extract │ │         │
│ sidecar │ │         │ │ glob   │ │         │ │         │
│settings │ │         │ │ grep   │ │         │ │         │
│ theme   │ │         │ │mcp_srv │ │         │ │         │
│         │ │         │ │security│ │         │ │         │
└────┬────┘ └──┬──┬───┘ └───┬────┘ └────┬────┘ └────┬────┘
     │         │  │         │           │           │
     │         │  │         │           │           │
     ▼         │  ▼         │           │           │
┌─────────┐   │ ┌─────────┐│           │           │
│  repo/  │   │ │ state.rs││           │           │
│         │   │ │(AppState)│◄──────────┼───────────┘
│ project │   │ └────┬─────┘           │
│ session │   │      │                 │
│ message │   │      │                 │
│artifact │◄──┼──────┼─────────────────┘
│  task   │   │      │
│ lesson  │   │      │
│settings │   │      │
│ scanner │   │      │
│ metrics │   │      │
│  theme  │   │      │
└────┬────┘   │      │
     │        │      │
     ▼        ▼      ▼
┌─────────────────────────┐     ┌──────────┐
│       domain/           │     │ error.rs │
│                         │◄────│(ForgeErr)│
│ project  session        │     └──────────┘
│ message  artifact       │          ▲
│ task     lesson         │          │
│ settings theme          │     (all modules
│ provider_event          │      depend on
│                         │      error.rs)
└─────────────────────────┘
```

### Dependency Rules

1. **`domain/`** depends on nothing (only std, serde, serde_json).
2. **`error.rs`** depends on thiserror, rusqlite (for `From<rusqlite::Error>`), and std::io.
3. **`repo/`** depends on `domain/`, `error.rs`, and rusqlite.
4. **`commands/`** depends on `domain/`, `repo/`, `state.rs`, `error.rs`, `sidecar/`, and `scanner/`.
5. **`sidecar/`** depends on `domain/` (for ProviderEvent, Message), `error.rs`, and tauri-plugin-shell.
6. **`tools/`** depends on `domain/`, `error.rs`, and std::fs/std::process. No database dependency.
7. **`scanner/`** depends on `domain/`, `repo/`, and `error.rs`.
8. **`watcher/`** depends on `repo/`, `domain/`, `error.rs`, and the `notify` crate.
9. **`main.rs` / `lib.rs`** depends on everything (wires it all together).

### Circular Dependency Prevention

No module may depend on a module that depends on it. The layering is strict:

```
main/lib  →  commands  →  repo     →  domain
                       →  sidecar  →  domain
                       →  tools    →  domain
                       →  scanner  →  domain
                       →  watcher  →  domain
```

---

## 10. State Management

### AppState

```rust
// state.rs
use std::sync::Mutex;
use rusqlite::Connection;
use crate::sidecar::manager::SidecarManager;
use crate::tools::ToolRegistry;
use crate::watcher::artifact_watcher::ArtifactWatcher;

pub struct AppState {
    /// SQLite connection (WAL mode, single writer, concurrent readers).
    /// Wrapped in Mutex because rusqlite::Connection is not Send.
    pub db: Mutex<Connection>,

    /// Sidecar process manager. Wrapped in Mutex for interior mutability
    /// (spawn/kill modify internal state).
    pub sidecar: Mutex<SidecarManager>,

    /// Tool registry for MCP tool dispatch.
    /// Initialized on project open with the project root path.
    pub tools: Mutex<Option<ToolRegistry>>,

    /// File watcher for .claude/ artifacts.
    /// Initialized on project open, stopped on project close.
    pub watcher: Mutex<Option<ArtifactWatcher>>,

    /// Currently active project ID. None if no project is open.
    pub active_project_id: Mutex<Option<i64>>,
}
```

### Registration

```rust
// lib.rs (inside the run() function)

pub fn run() {
    let db = Connection::open(db_path).expect("failed to open database");
    // PRAGMA initialization
    db.execute_batch("
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;
        PRAGMA busy_timeout = 5000;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -8000;
        PRAGMA temp_store = MEMORY;
    ").expect("failed to set pragmas");

    let app_state = AppState {
        db: Mutex::new(db),
        sidecar: Mutex::new(SidecarManager::new()),
        tools: Mutex::new(None),
        watcher: Mutex::new(None),
        active_project_id: Mutex::new(None),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_sql::Builder::new()
            .add_migrations("sqlite:forge.db", migrations())
            .build())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Project
            commands::project_open,
            commands::project_create,
            commands::project_get,
            commands::project_scan,
            // Session
            commands::session_create,
            commands::session_list,
            commands::session_get,
            commands::session_end,
            // Message
            commands::stream_send_message,
            commands::message_list,
            commands::message_search,
            // Artifact
            commands::artifact_list,
            commands::artifact_get,
            commands::artifact_update,
            commands::artifact_create,
            // Sidecar
            commands::sidecar_status,
            commands::sidecar_restart,
            // Settings
            commands::settings_get,
            commands::settings_set,
            commands::settings_get_all,
            // Theme
            commands::theme_get_project,
            commands::theme_set_override,
        ])
        .run(tauri::generate_context!())
        .expect("error running Forge");
}
```

### Command Handler Pattern

Every command handler follows the same pattern:

```rust
#[tauri::command]
pub fn session_get(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<Session, ForgeError> {
    let conn = state.db.lock().map_err(|e| ForgeError::Database(
        rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(1), Some(e.to_string())
        )
    ))?;
    SessionRepo::get_by_id(&conn, id)?
        .ok_or(ForgeError::SessionNotFound(id))
}
```

1. Extract `State<AppState>` from Tauri.
2. Lock the resource needed (db, sidecar, etc.).
3. Call the repository or service method.
4. Return `Result<T, ForgeError>`.

No business logic in the handler. No `unwrap()`. No `panic!()`.

---

## Related Documents

- [Architecture Decisions](/architecture/decisions) -- AD-001 through AD-017
- [SQLite Schema](/architecture/sqlite-schema) -- Full table definitions and migration strategy
- [MVP Feature Specification](/product/mvp-specification) -- Phase 1 scope and acceptance criteria
