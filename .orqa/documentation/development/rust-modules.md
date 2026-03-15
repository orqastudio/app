---
id: DOC-010
title: Rust Module Architecture
description: Module layout and dependency structure of the Rust backend crate.
created: 2026-03-02
updated: 2026-03-10
sort: 7
relationships:
  - target: RES-002
    type: documents
    rationale: Documentation page references RES-002
  - target: RES-007
    type: documents
    rationale: Documentation page references RES-007
  - target: RES-006
    type: documents
    rationale: Documentation page references RES-006
  - target: AD-001
    type: documents
    rationale: Documentation page references AD-001
  - target: AD-003
    type: documents
    rationale: Documentation page references AD-003
  - target: AD-014
    type: documents
    rationale: Documentation page references AD-014
  - type: informed-by
    target: PILLAR-001
    rationale: This document describes architecture or practices that serve the Clarity Through Structure pillar
  - type: informed-by
    target: PILLAR-002
    rationale: This document describes features or patterns that serve the Learning Through Reflection pillar
---

**References:** [Claude Integration](RES-002), [Tauri v2](RES-007), [Persistence](RES-006)

Module tree, domain types, command handlers, and dependency graph for `backend/src-tauri/src/`. Rust owns the domain model [AD-001](AD-001). All functions return `Result<T, E>` [AD-003](AD-003). No `unwrap()`, `expect()`, or `panic!()` in production code.

---

## 1. Module Tree

```
backend/src-tauri/src/
├── main.rs                          # Tauri entry point (calls lib::run())
├── lib.rs                           # App builder, plugin registration, command registration, startup
├── state.rs                         # AppState struct (Tauri managed state)
├── error.rs                         # OrqaError enum (thiserror + serde), Result type alias
├── logging.rs                       # Tracing subscriber setup (stdout + file, env-filter)
├── db.rs                            # Database initialization (rusqlite, PRAGMAs, migrations)
├── startup.rs                       # StartupTracker: async task status for frontend polling
├── watcher.rs                       # File-system watcher for .orqa/ changes (notify crate)
│
├── domain/                          # Domain model types and business logic
│   ├── mod.rs                       # Re-exports all domain types
│   ├── artifact.rs                  # Artifact, ArtifactType, ArtifactSummary, NavTree, DocNode
│   ├── artifact_fs.rs               # File-system helpers for reading/writing artifact files
│   ├── artifact_graph.rs            # ArtifactGraph, ArtifactNode, ArtifactRef, GraphStats
│   ├── artifact_reader.rs           # Config-driven artifact tree scanner (reads project.json)
│   ├── enforcement.rs               # EnforcementRule, ScanFinding, RuleAction types
│   ├── enforcement_engine.rs        # EnforcementEngine: compiles regex entries, runs scan
│   ├── enforcement_parser.rs        # YAML frontmatter parser for enforcement rule files
│   ├── governance.rs                # GovernanceAnalysis, GovernanceScanResult, Recommendation types
│   ├── governance_analysis.rs       # Claude prompt building, response parsing, persistence helpers
│   ├── governance_scanner.rs        # Filesystem walker for governance files
│   ├── lessons.rs                   # Lesson, NewLesson types
│   ├── message.rs                   # Message, MessageRole, ContentType, StreamStatus
│   ├── paths.rs                     # Path constants (ORQA_DIR, SEARCH_DB, etc.)
│   ├── process_gates.rs             # Process compliance gates: understand-first, docs-before-code, evidence-before-done
│   ├── process_state.rs             # SessionProcessState: tracks docs-read/skills-loaded per session
│   ├── project.rs                   # Project, ProjectSummary, DetectedStack
│   ├── project_scanner.rs           # ProjectScanResult: language/framework detection
│   ├── project_settings.rs          # ProjectSettings (.orqa/project.json), ArtifactConfig
│   ├── provider_event.rs            # StreamEvent enum (streaming protocol from sidecar)
│   ├── session.rs                   # Session, SessionStatus, SessionSummary
│   ├── session_title.rs             # Heuristics for deriving session titles from first message
│   ├── settings.rs                  # Setting, SidecarStatus, SidecarState, ResolvedTheme, ThemeToken
│   ├── setup.rs                     # SetupStatus, SetupStepStatus, StepStatus, ClaudeCliInfo
│   ├── skill_injector.rs            # Path-to-skill injection engine (RULE-042): maps file patterns to skills
│   ├── stream_loop.rs               # Stream loop orchestration: sidecar -> DB -> channel
│   ├── system_prompt.rs             # System prompt assembly from governance context
│   ├── time_utils.rs                # Timestamp formatting helpers
│   ├── tool_executor.rs             # Tool execution dispatch (read_file, glob, grep, write_file, etc.)
│   └── workflow_tracker.rs          # Session event tracking for process gates (reads, writes, skills loaded)
│
├── repo/                            # Repository layer — database access
│   ├── mod.rs                       # Re-exports
│   ├── artifact_repo.rs             # ArtifactRepo: CRUD + FTS for artifacts table
│   ├── enforcement_rules_repo.rs    # EnforcementRulesRepo: load rules from .orqa/rules/
│   ├── governance_repo.rs           # GovernanceRepo: analyses + recommendations tables
│   ├── lesson_repo.rs               # LessonRepo: file-based IMPL-NNN.md in .orqa/process/lessons/
│   ├── message_repo.rs              # MessageRepo: insert, update stream, FTS queries
│   ├── project_repo.rs              # ProjectRepo: CRUD for projects table
│   ├── project_settings_repo.rs     # ProjectSettingsRepo: read/write .orqa/project.json
│   ├── session_repo.rs              # SessionRepo: CRUD for sessions table
│   ├── settings_repo.rs             # SettingsRepo: key-value with scope
│   └── theme_repo.rs                # ThemeRepo: project_themes + project_theme_overrides
│
├── commands/                        # Tauri command handlers (#[tauri::command])
│   ├── mod.rs                       # Re-exports all command functions for registration
│   ├── artifact_commands.rs         # 9 commands: artifact CRUD, read_artifact, artifact_scan_tree, artifact_watch_start
│   ├── enforcement_commands.rs      # 3 commands: rules_list, rules_reload, scan_governance
│   ├── governance_commands.rs       # 7 commands: scan, analyze, analysis_get, recommendations CRUD
│   ├── graph_commands.rs            # 8 commands: resolve, references, get_by_type, stats, refresh
│   ├── lesson_commands.rs           # 5 commands: list, get, create, increment_recurrence, scan_promotions
│   ├── message_commands.rs          # 2 commands: message_list, message_search
│   ├── project_commands.rs          # 5 commands: open, create, get, get_active, list
│   ├── project_settings_commands.rs # 5 commands: settings read/write, icon upload/read, project_scan
│   ├── search_commands.rs           # 6 commands: index, search_regex, search_semantic, get_index_status, init_embedder, get_startup_status
│   ├── session_commands.rs          # 6 commands: create, list, get, update_title, end, delete
│   ├── settings_commands.rs         # 3 commands: get, set, get_all
│   ├── setup_commands.rs            # 6 commands: get_status, check_cli, check_auth, reauthenticate, check_model, complete
│   ├── sidecar_commands.rs          # 2 commands: sidecar_status, sidecar_restart
│   └── stream_commands.rs           # 4 commands: stream_send_message, stream_stop, stream_tool_approval_respond, system_prompt_preview
│
├── sidecar/                         # Sidecar process management
│   ├── mod.rs                       # Re-exports
│   ├── manager.rs                   # SidecarManager: spawn via std::process::Command, health check
│   ├── protocol.rs                  # NDJSON serialization/deserialization, line framing
│   └── types.rs                     # SidecarRequest (6 variants), SidecarResponse (15 variants)
│
└── search/                          # DuckDB code indexer + ONNX semantic search
    ├── mod.rs                       # SearchEngine: combined regex + semantic search interface
    ├── chunker.rs                   # Source code chunking for embedding
    ├── embedder.rs                  # ONNX Runtime embeddings (bge-small-en-v1.5, DirectML)
    ├── store.rs                     # DuckDB-backed vector store + inverted index
    └── types.rs                     # SearchResult, IndexStatus, ChunkInfo types
```

---

## 2. Module Descriptions

### `main.rs` / `lib.rs`

Application entry point. `main.rs` calls `lib::run()`. `lib.rs` constructs the Tauri app builder inside a `.setup()` closure: initializes the database via `db::init_db()`, creates the `StartupTracker`, spawns the sidecar, pre-downloads the embedding model, registers all 6 Tauri plugins and all command handlers, and runs the app.

### `state.rs` — AppState (9 fields)

Defines `AppState`, the single struct passed as Tauri managed state. All command handlers receive `State<AppState>` as a parameter.

```rust
pub struct AppState {
    /// SQLite connection (WAL mode). Not Send — wrapped in Mutex.
    pub db: Mutex<Connection>,
    /// Sidecar process manager. Uses interior mutability (its own Mutex fields).
    pub sidecar: SidecarManager,
    /// DuckDB-backed code search engine. Lazily initialized on first index.
    pub search: Mutex<Option<SearchEngine>>,
    /// Tracks long-running startup tasks for frontend polling.
    pub startup: Arc<StartupTracker>,
    /// Pending tool approval channels: tool_call_id -> sender for approval decision.
    pub pending_approvals: Mutex<HashMap<String, SyncSender<bool>>>,
    /// Rule enforcement engine. None until a project is opened.
    pub enforcement: Mutex<Option<EnforcementEngine>>,
    /// Session-level process compliance state. Tracks docs-read/skills-loaded.
    pub process_state: Mutex<SessionProcessState>,
    /// Active .orqa/ file-system watcher. Replaced via artifact_watch_start.
    pub artifact_watcher: SharedWatcher,
    /// Cached bidirectional artifact graph. Invalidated by the artifact watcher.
    pub artifact_graph: Mutex<Option<ArtifactGraph>>,
}
```

### `error.rs`

Defines `OrqaError` with 9 variants (via `thiserror` + `serde::Serialize`): `NotFound`, `Database`, `FileSystem`, `Sidecar`, `Validation`, `Scan`, `Serialization`, `PermissionDenied`, `Search`. Serialized as `{"code": "<variant>", "message": "<detail>"}` using `#[serde(tag = "code", content = "message")]`. `From` impls exist for `std::io::Error`, `serde_json::Error`, and `rusqlite::Error`.

### `db.rs`

Database initialization using `rusqlite` directly. `init_db()` opens a connection, sets WAL mode and PRAGMAs, and runs migration files via `include_str!`. Returns a `rusqlite::Connection`. `init_memory_db()` is available for tests.

### `startup.rs`

Generic startup task tracker. Tasks register with an ID and label, then update with status (Pending, InProgress, Done, Error). The frontend polls via `get_startup_status` to show progress of long-running initialization tasks (sidecar launch, embedding model download).

### `watcher.rs`

File-system watcher using the `notify` crate. Watches `.orqa/` recursively with a 500ms debounce. When any file changes, emits a single `artifact-changed` Tauri event to all windows so the frontend can invalidate its nav-tree cache. Also invalidates the cached `ArtifactGraph` in `AppState`.

### `domain/`

30 modules covering the full domain model and business logic. Modules with significant complexity have dedicated sub-modules:

- **Artifact subsystem**: `artifact.rs` (types), `artifact_fs.rs` (file I/O), `artifact_reader.rs` (config-driven tree scanning), `artifact_graph.rs` (bidirectional reference graph)
- **Enforcement subsystem**: `enforcement.rs` (types), `enforcement_engine.rs` (regex compilation and scanning), `enforcement_parser.rs` (YAML frontmatter parsing)
- **Governance subsystem**: `governance.rs` (types), `governance_analysis.rs` (Claude integration helpers), `governance_scanner.rs` (filesystem walker)
- **Stream subsystem**: `stream_loop.rs` (orchestration), `provider_event.rs` (protocol types), `tool_executor.rs` (tool dispatch)
- **Process subsystem**: `process_gates.rs` (compliance gates), `process_state.rs` (session state), `workflow_tracker.rs` (event tracking), `skill_injector.rs` (path-to-skill mapping)
- **Project subsystem**: `project.rs` (types), `project_scanner.rs` (language detection), `project_settings.rs` (file-based config)

Note on dependencies: `tool_executor.rs` and `stream_loop.rs` import `AppState` and call repos directly, meaning `domain/` is not a pure leaf in the dependency graph. These modules are boundary orchestrators that live in `domain/` for cohesion but accept runtime dependencies as parameters.

### `logging.rs`

Tracing subscriber setup. Configures `tracing-subscriber` with stdout and optional file output, filtered by `RUST_LOG` / `ORQA_LOG` env vars.

### `repo/`

10 repositories, one per concern. Each repo is stateless — it borrows a connection reference for each operation and returns domain types, never raw SQL rows. Two repos are file-based rather than SQLite-backed: `lesson_repo` reads/writes `IMPL-NNN.md` files in `.orqa/process/lessons/`, and `project_settings_repo` reads/writes `.orqa/project.json`.

### `commands/`

14 thin command modules, approximately 79 total commands. Each function is `#[tauri::command]`, receives `State<AppState>` and parameters, and calls the appropriate repo or domain service. No business logic in the handlers. See IPC Commands for the full command catalog with signatures.

### `sidecar/`

Process lifecycle management for the Agent SDK sidecar. Uses `std::process::Command` (not `tauri-plugin-shell`) for process spawning. `SidecarManager` uses interior mutability with per-field Mutex locks. The NDJSON protocol in `protocol.rs` handles stdin/stdout framing. `types.rs` defines `SidecarRequest` (6 variants) and `SidecarResponse` (15 variants).

### `search/`

DuckDB-based code indexer with ONNX embeddings for semantic search. `SearchEngine` provides combined regex + semantic search. `chunker.rs` splits source files into chunks. `embedder.rs` loads the bge-small-en-v1.5 ONNX model (auto-downloaded from Hugging Face on first use, using DirectML for GPU acceleration on Windows). `store.rs` manages the DuckDB vector store and inverted index.

---

## 3. Repository Layer

10 repositories across SQLite and file-based storage.

| Repo | Storage | Concern |
|------|---------|---------|
| `ProjectRepo` | SQLite `projects` | Project CRUD, by_path, upsert, get_active |
| `SessionRepo` | SQLite `sessions` | Session CRUD, list with filter/pagination, end, update_title |
| `MessageRepo` | SQLite `messages` | Insert, update stream content, list by session, FTS5 search |
| `ArtifactRepo` | SQLite `artifacts` | CRUD + FTS, by_path, index_artifact |
| `SettingsRepo` | SQLite `settings` | Key-value with scope, get/set/get_all |
| `ThemeRepo` | SQLite `project_themes`, `project_theme_overrides` | Active theme tokens, set/clear overrides |
| `GovernanceRepo` | SQLite `governance_analyses`, `recommendations` | Save analysis, list/update/apply recommendations |
| `EnforcementRulesRepo` | Files (`.orqa/process/rules/*.md`) | Load and parse YAML-fronmatted rule files |
| `LessonRepo` | Files (`.orqa/process/lessons/IMPL-NNN.md`) | List, get, create, increment recurrence |
| `ProjectSettingsRepo` | File (`.orqa/project.json`) | Read/write project settings and artifacts config |

---

## 4. AppState and Initialization

`AppState` is constructed in `lib.rs` inside the `.setup()` closure and registered via `.manage()`. The 9 fields are:

| Field | Type | Initialized |
|-------|------|-------------|
| `db` | `Mutex<Connection>` | `db::init_db()` in setup |
| `sidecar` | `SidecarManager` | `SidecarManager::new()` in setup |
| `search` | `Mutex<Option<SearchEngine>>` | `None` — lazy on first `index_codebase` |
| `startup` | `Arc<StartupTracker>` | `StartupTracker::new()` in setup |
| `pending_approvals` | `Mutex<HashMap<String, SyncSender<bool>>>` | Empty map in setup |
| `enforcement` | `Mutex<Option<EnforcementEngine>>` | `None` — loaded on `project_open` |
| `process_state` | `Mutex<SessionProcessState>` | Default in setup |
| `artifact_watcher` | `SharedWatcher` | `SharedWatcher::default()` in setup |
| `artifact_graph` | `Mutex<Option<ArtifactGraph>>` | `None` — lazy on first graph query |

---

## 5. Streaming Pipeline

```
AI Provider (SSE, via Agent SDK)
    │
    ▼
TypeScript sidecar (translate to StreamEvent NDJSON)
    │
    ▼ stdout
Rust stream_loop (BufReader::lines() in spawned thread)
    │  ├─ serde_json::from_str::<StreamEvent>()
    │  ├─ Write to DB (message_repo, buffered ~500ms)
    │  ├─ Tool approval gating (pending_approvals map)
    │  └─ channel.send(event)
    │
    ▼ Channel<StreamEvent>
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

Tool calls requiring approval park the stream loop on a `SyncSender<bool>`. The frontend calls `stream_tool_approval_respond`, which looks up the sender in `AppState.pending_approvals` and sends the decision.

---

## 6. Dependency Graph

Arrows point from the dependent module to the module it depends on.

```
┌─────────────────────────────────────────────────────────┐
│                    main.rs / lib.rs                      │
└────┬──────────┬──────────┬──────────┬───────────────────┘
     │          │          │          │
     ▼          ▼          ▼          ▼
┌──────────┐ ┌─────────┐ ┌────────┐ ┌───────────┐
│commands/ │ │ sidecar/ │ │search/ │ │ startup.rs│
│ (15 mod) │ │         │ │        │ └───────────┘
└────┬─────┘ └──┬──────┘ └───┬────┘
     │          │             │
     ▼          │             │
┌─────────┐    │  ┌──────────────────┐
│  repo/  │    │  │     state.rs     │
│ (10 mod)│    │  │  (AppState, 9    │
│         │    │  │    fields)       │
└────┬────┘    │  └────┬─────────────┘
     │         │       │
     │         │       │ (commands/ receives State<AppState>)
     │         │       │
     ▼         ▼       ▼
┌─────────────────────────────────┐     ┌──────────┐
│          domain/                │     │ error.rs │
│          (30 modules)           │◄────│(OrqaErr) │
│                                 │     └──────────┘
│ artifact*  enforcement*         │          ▲
│ governance* lessons             │          │
│ message    paths                │     (all modules
│ process_gates process_state     │
│ skill_injector workflow_tracker │
│ project*                        │      depend on
│ provider_event session*         │      error.rs)
│ settings   setup                │
│ stream_loop  system_prompt      │
│ time_utils  tool_executor       │
└─────────────────────────────────┘
                          ┌──────────┐
                          │  db.rs   │
                          │(init_db) │
                          └──────────┘
                          ┌──────────┐
                          │watcher.rs│
                          └──────────┘
```

### Dependency Rules

1. **`error.rs`** — depends on thiserror, serde, rusqlite, serde_json, std::io.
2. **`db.rs`** — depends on rusqlite and `error.rs`.
3. **`domain/`** — most modules depend only on serde and `error.rs`. `tool_executor.rs` and `stream_loop.rs` are exceptions that take `AppState` as a parameter.
4. **`repo/`** — depends on `domain/`, `error.rs`, rusqlite. File-based repos use std::fs.
5. **`commands/`** — depends on `domain/`, `repo/`, `state.rs`, `error.rs`, `sidecar/`, `search/`, `startup.rs`.
6. **`sidecar/`** — depends on `domain/` (for StreamEvent), `error.rs`, std::process.
7. **`search/`** — depends on `domain/`, `error.rs`, duckdb, ort (ONNX Runtime), tokenizers.
8. **`watcher.rs`** — depends on notify, `error.rs`, std::sync.
9. **`startup.rs`** — depends on std only.
10. **`main.rs` / `lib.rs`** — depends on everything.

---

## 7. Tauri Plugins (6)

```rust
.plugin(tauri_plugin_fs::init())
.plugin(tauri_plugin_shell::init())
.plugin(tauri_plugin_store::Builder::default().build())
.plugin(tauri_plugin_window_state::Builder::default().build())
.plugin(tauri_plugin_dialog::init())
.plugin(tauri_plugin_notification::init())
```

`tauri-plugin-sql` is NOT used — database access goes through `rusqlite` directly via `db.rs`.

---

## Related Documents

- IPC Commands — full command catalog with signatures
- [AD-001](AD-001) — thick backend principle
- [AD-003](AD-003) — error propagation via Result + thiserror
- [AD-014](AD-014) — repository pattern

---
