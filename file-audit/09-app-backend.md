# File Audit: app/src-tauri/ (Rust Backend)

## Question

Catalogue every file in `app/src-tauri/` (excluding `target/`) with file-level detail on all source files. Identify engine boundary (Tauri backend vs separate crate) and classify files as Tauri glue vs engine business logic.

## Findings

### Engine Boundary Summary

**Separate library crates (extracted engines):**
- `orqa-validation` (`../../libs/validation`) -- artifact graph validation, integrity checks, relationship schemas. Used via `artifact_graph.rs` and `integrity_engine.rs` (thin wrappers).
- `orqa-search` (`../../libs/search`) -- chunking, embedding, storage, search engine. Used via `src/search/mod.rs` (thin re-export).

**Business logic embedded IN the Tauri backend (not extracted):**
- Enforcement engine (783 lines) -- regex-based rule evaluation for file/bash/scan/lint events
- Status transitions (880 lines) -- config-driven engine with 5 named conditions
- Stream loop (1042 lines) -- core streaming infrastructure, tool approval, enforcement integration
- Tool executor (1140 lines) -- 11 tool implementations + enforcement hooks
- Process gates (548 lines) -- 5 gates (understand-first, docs-before-code, plan-before-build, evidence-before-done, learn-after-doing)
- Knowledge injector (477 lines) -- ONNX embeddings, cosine similarity matching
- Governance scanner (422 lines) -- scans 6 governance areas in .orqa/
- Hook manager (415 lines) -- git hook dispatcher generation
- Setup/auth (428 lines) -- Claude CLI detection, version parsing, auth checking
- Project scanner (396 lines) -- recursive walk to depth 10, language/framework detection
- Workflow tracker (406 lines) -- session activity tracking (files read/written, searches, docs)
- Artifact reader (874 lines) -- config-driven navigation tree scanner
- Artifact core (687 lines) -- ID generation, frontmatter parsing, artifact types
- Process state (287 lines) -- session process state tracking tool calls
- System prompt builder (270 lines) -- constructs system prompts from rules + knowledge + CLAUDE.md

**Tauri glue code:**
- 19 command modules (58 total commands registered)
- State management (`state.rs`)
- File system watcher (`watcher.rs`)
- Sidecar lifecycle management (`sidecar/`)
- Server spawners (`servers/`)
- Database init + migrations (`db.rs`)
- Logging/error infrastructure (`logging.rs`, `error.rs`)
- Plugin system (`plugins/`)
- Repository layer (`repo/`)

---

### Configuration Files

| File | Lines | Purpose |
|------|-------|---------|
| `Cargo.toml` | 59 | Package config. Name: `orqa-studio` v0.1.4-dev, edition 2021. Key deps: tauri 2, rusqlite (bundled), tokio (full), reqwest, serde, serde_json (preserve_order), serde_yaml, regex, notify 8, notify-debouncer-full, sha2, tar, flate2, tracing, rand, glob, base64, dirs-next, orqa-search (path), orqa-validation (path). Dev dep: tempfile. Lints from workspace. Features: default = custom-protocol. |
| `build.rs` | 3 | Single call to `tauri_build::build()`. |
| `clippy.toml` | 7 | `too-many-lines-threshold = 50`. |
| `tauri.conf.json` | 41 | Product "OrqaStudio" v0.1.0, identifier `com.orqa.studio`. Dev port 10420. Frontend dist: `../build`. Window: 1280x800, min 720x480, decorations: false, resizable. CSP configured (default-src self, script-src self, style-src self+unsafe-inline, img-src self+data+asset, font-src self+data, connect-src self+ipc). Bundle targets: all. |
| `capabilities/default.json` | 20 | Permissions for main window: core:default, store:default, window-state:default, dialog:default:open, window:default + allow-start-dragging + allow-set-size, shell:default:open. |

---

### Entry Points

#### `src/main.rs` (52 lines) -- Binary entry point
- Parses CLI flags: `--mcp` and `--lsp`
- If `--mcp`: runs `servers::mcp::run_mcp_server()` (spawns orqa-mcp-server binary over stdio)
- If `--lsp`: runs `servers::lsp::run_lsp_server()` (spawns orqa-lsp-server binary over stdio)
- Otherwise: normal Tauri app with `run_app()`
- `daemon_port()` reads `ORQA_PORT_BASE` env var (default 9120) and adds 1 for IPC
- Classification: **Tauri glue**

#### `src/lib.rs` (268 lines) -- Library root
- 15 module declarations: `cli_tools`, `commands`, `db`, `domain`, `error`, `hooks`, `logging`, `plugins`, `repo`, `search`, `servers`, `sidecar`, `startup`, `state`, `watcher`
- `build_app_state()`: constructs `AppState` with DB, SidecarManager, SearchEngine, CliToolRunner, EnforcementEngine, StartupTracker, SessionState
- `setup_database()`: calls `db::init_db()` to run migrations
- `register_plugins()`: registers 6 Tauri plugins (fs with watch, shell, store, window-state, dialog, notification)
- `run_app()`: builds Tauri app, registers 58 commands, starts IPC socket server, spawns file watcher, async search engine init, async enforcement loading
- 58 commands registered across artifact, CLI tool, daemon, enforcement, graph, hook, lesson, message, plugin, project, project settings, search, session, settings, setup, sidecar, status transition, stream categories
- Classification: **Tauri glue**

---

### Core Infrastructure

#### `src/state.rs` (127 lines) -- Application state
- `AppState` struct with 8 sub-structs:
  - `DbState` -- `Mutex<rusqlite::Connection>`
  - `SidecarState` -- `SidecarManager`
  - `SearchState` -- `SearchEngine`
  - `StartupState` -- `StartupTracker`
  - `EnforcementState` -- `Mutex<Option<EnforcementEngine>>`
  - `SessionState` -- `Mutex<Option<String>>` (current session ID)
  - `ArtifactState` -- `Mutex<Option<String>>` (current artifact ID)
  - `CliToolState` -- `CliToolRunner`
- Classification: **Tauri glue**

#### `src/error.rs` (231 lines) -- Error types
- `OrqaError` enum with 10 variants: `Database`, `Io`, `NotFound`, `Validation`, `Sidecar`, `Search`, `InvalidInput`, `NetworkError`, `ExternalTool`, `StreamError`
- Implements `Display`, `std::error::Error`, `Serialize`, `From<rusqlite::Error>`, `From<std::io::Error>`
- Classification: **Tauri glue**

#### `src/db.rs` (245 lines) -- Database initialization
- `init_db(path)` opens SQLite, sets WAL mode PRAGMAs (`journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`, `busy_timeout=5000`)
- Runs migrations 001-010 (skipping 006) tracked in `schema_migrations` table
- Migrations 005 and 010 contain additional ALTER TABLE logic in Rust code (column additions with existence checks)
- `init_memory_db()` for tests -- in-memory SQLite with same migrations
- 7 tests covering migration, idempotency, WAL mode, memory DB
- Classification: **Tauri glue**

#### `src/logging.rs` (149 lines) -- Logging infrastructure
- Two-tier tracing subscriber: console (compact, auto-color) + Tauri event emission
- `TauriEventLayer` emits structured log events to frontend via `app://log` event
- `emit_app_error()` helper sends `app://app-error` events
- Respects `RUST_LOG` env var, defaults to `info`
- Classification: **Tauri glue**

#### `src/startup.rs` (94 lines) -- Startup tracking
- `StartupTracker` with `Mutex<Vec<StartupTask>>`
- Tracks long-running init tasks: `start_task()`, `complete_task()`, `fail_task()`, `get_tasks()`
- `StartupTask` has name, status (Pending/Running/Completed/Failed), started_at, completed_at, error
- Classification: **Tauri glue**

#### `src/watcher.rs` (248 lines) -- File system watcher
- `start_watcher()` watches `.orqa/` directory for changes with 500ms debounce
- Uses `notify-debouncer-full` crate
- On change: emits `orqa://fs-change` event to frontend, then triggers `sidecar_commands::ensure_sidecar_running` and rescans graph context via `graph_commands`
- Ignores `.git/` and `node_modules/` paths
- Thread-spawned, non-blocking
- Classification: **Tauri glue**

---

### Commands (src/commands/) -- 19 modules

#### `src/commands/mod.rs` -- Module declarations
- Re-exports 19 command sub-modules
- Classification: **Tauri glue**

#### `src/commands/helpers.rs` -- Command utilities
- Helper functions shared across command modules
- Classification: **Tauri glue**

#### `src/commands/artifact_commands.rs` -- Artifact CRUD
- Tauri commands for creating, reading, updating, deleting artifacts
- Delegates to domain layer (`artifact.rs`, `artifact_fs.rs`)
- Classification: **Tauri glue**

#### `src/commands/cli_tool_commands.rs` -- CLI tool management
- Commands for listing registered CLI tools, running them, getting statuses
- Delegates to `CliToolRunner`
- Classification: **Tauri glue**

#### `src/commands/daemon_commands.rs` -- Daemon control
- Commands for daemon lifecycle management
- Classification: **Tauri glue**

#### `src/commands/enforcement_commands.rs` -- Enforcement rules
- Commands for loading, listing, and querying enforcement rules
- Delegates to enforcement engine
- Classification: **Tauri glue**

#### `src/commands/graph_commands.rs` (567 lines) -- Graph operations (LARGEST command file)
- Commands for graph queries, validation, health checks, relationship management
- Delegates to `artifact_graph.rs` which delegates to `orqa_validation` crate
- Classification: **Tauri glue** (but substantial orchestration logic)

#### `src/commands/hook_commands.rs` -- Git hook management
- Commands for hook generation and management
- Delegates to hooks::manager
- Classification: **Tauri glue**

#### `src/commands/lesson_commands.rs` -- Lesson CRUD
- Commands for creating, listing, updating lessons
- Delegates to `lesson_repo`
- Classification: **Tauri glue**

#### `src/commands/message_commands.rs` -- Message CRUD
- Commands for creating, listing, searching messages
- Uses FTS5 search
- Delegates to `message_repo`
- Classification: **Tauri glue**

#### `src/commands/plugin_commands.rs` -- Plugin install/uninstall/registry
- Commands for installing (local + GitHub), uninstalling, listing plugins
- Includes collision detection integration
- Delegates to plugins subsystem
- Classification: **Tauri glue**

#### `src/commands/project_commands.rs` -- Project CRUD
- Commands for creating, listing, scanning projects
- Delegates to `project_repo` and `project_scanner`
- Classification: **Tauri glue**

#### `src/commands/project_settings_commands.rs` -- Project settings
- Commands for reading/writing project.json settings
- Delegates to `project_settings_repo`
- Classification: **Tauri glue**

#### `src/commands/search_commands.rs` -- Search operations
- Commands for semantic search, regex search, code research
- Delegates to `orqa_search` via search module
- Classification: **Tauri glue**

#### `src/commands/session_commands.rs` -- Session management
- Commands for creating, listing, ending sessions
- Delegates to `session_repo`
- Classification: **Tauri glue**

#### `src/commands/settings_commands.rs` -- App settings
- Commands for getting/setting application preferences
- Delegates to `settings_repo`
- Classification: **Tauri glue**

#### `src/commands/setup_commands.rs` -- Claude CLI setup
- Commands for detecting Claude CLI, checking auth
- Delegates to `domain::setup`
- Classification: **Tauri glue**

#### `src/commands/sidecar_commands.rs` (177 lines) -- Sidecar lifecycle
- `sidecar_status` -- queries SidecarManager status
- `sidecar_restart` -- kills and respawns sidecar
- `ensure_sidecar_running` -- auto-start if not connected
- `resolve_sidecar()` -- reads `sidecar-config.json` (written by plugin system), falls back to test echo sidecar
- `SidecarConfig` struct: runtime, entrypoint, args
- 4 tests: status not started, resolve fallback, config deserialization, config with args
- Classification: **Tauri glue**

#### `src/commands/status_transition_commands.rs` -- Status transitions
- Commands for triggering and querying status transitions
- Delegates to `domain::status_transitions`
- Classification: **Tauri glue**

#### `src/commands/stream_commands.rs` -- Streaming messages
- Commands for sending messages, cancelling streams, handling tool approval
- Delegates to `domain::stream_loop`
- Classification: **Tauri glue**

---

### Domain Logic (src/domain/) -- 34 modules

#### `src/domain/mod.rs` -- Module declarations
- Re-exports all 34 domain sub-modules
- Classification: **Domain infrastructure**

#### `src/domain/artifact.rs` (687 lines) -- Core artifact types
- `Artifact` struct: id, title, type_key, status, frontmatter fields, body, metadata, filesystem info
- `ArtifactMetadata`: created, modified, tags, links, relationships (HashMap), custom_fields
- `ArtifactType` enum: Task, Epic, Principle, Decision, Lesson, Pattern, Agent, Rule, Skill, Knowledge, Workflow, Custom
- `parse_artifact(content)` -- YAML frontmatter parser, extracts relationships from body links
- `generate_id(type_key)` -- prefixed ID generation (e.g., TASK-abcd1234)
- `normalize_relationships()` -- deduplicates relationship entries
- Tests for frontmatter parsing, relationship extraction, ID generation, edge cases
- Classification: **Engine business logic**

#### `src/domain/artifact_fs.rs` (301 lines) -- Artifact filesystem operations
- `write_artifact_file()` -- serialises artifact to YAML frontmatter + markdown body
- `scan_directory()` -- reads all `.md` files from a directory, parses each as artifact
- `read_artifact_file()` -- reads single file, returns parsed artifact
- `delete_artifact_file()` -- removes file from disk
- Classification: **Engine business logic**

#### `src/domain/artifact_reader.rs` (874 lines) -- Navigation tree scanner
- `NavigationNode` struct: id, label, type, path, children, status, artifact_type
- `scan_navigation_tree(project_root)` -- builds navigation tree from project.json `artifacts` config array
- Config-driven: each entry in project.json `artifacts` array defines a directory to scan, its artifact type, and display settings
- Handles nested directories, sorting, icon resolution
- Classification: **Engine business logic**

#### `src/domain/artifact_graph.rs` (196 lines) -- Graph bridge
- Thin wrapper over `orqa_validation` crate
- `build_graph()` -- constructs validation graph from scanned artifacts
- `validate_graph()` -- runs integrity checks via library
- Re-exports `ValidationResult`, `ValidationIssue` from `orqa_validation`
- Classification: **Bridge to extracted engine** (`orqa_validation`)

#### `src/domain/config_loader.rs` (94 lines) -- Centralised config loading
- `load_project_settings(project_root)` -- reads `.orqa/project.json`
- Returns `Option<ProjectSettings>` -- None if file doesn't exist
- Used by discovery, paths, and settings modules
- Classification: **Engine business logic**

#### `src/domain/enforcement.rs` (113 lines) -- Enforcement types
- `EventType` enum: File, Bash, Scan, Lint
- `RuleAction` enum: Block, Warn, Inject
- `Verdict` enum: Allow, Block(reason), Warn(reason), Inject(content)
- `EnforcementRule` struct: name, event_type, patterns, action, message, inject_content
- `EnforcementEntry` struct: rule with file path metadata
- Classification: **Engine business logic**

#### `src/domain/enforcement_engine.rs` (783 lines) -- Enforcement engine
- `CompiledEntry` struct: pre-compiled `Regex` patterns for performance
- `EnforcementEngine` struct with `Vec<CompiledEntry>` + inverse map cache
- `load(project_root)` -- discovers rules from `.orqa/process/rules/` + plugin directories
- `evaluate_file(path, content)` -- tests file content against File-type rules
- `evaluate_bash(command)` -- tests bash commands against Bash-type rules
- `evaluate_scan(diagnostics)` -- tests scan output against Scan-type rules
- `evaluate_lint(output)` -- tests lint output against Lint-type rules
- Pattern matching uses `regex::Regex` with case-insensitive flag option
- Returns `Vec<Verdict>` per evaluation
- Extensive tests (10+)
- Classification: **Engine business logic** (NOT extracted)

#### `src/domain/enforcement_parser.rs` (340 lines) -- Enforcement rule parser
- Parses YAML frontmatter from `.md` rule files
- Extracts: name, event, patterns (array), action, message, inject_content
- Supports `---` delimited frontmatter
- Validation: required fields, valid event types, valid actions
- 8 tests
- Classification: **Engine business logic**

#### `src/domain/enforcement_violation.rs` (24 lines) -- Violation struct
- `EnforcementViolation` struct: rule_name, violation_type, message, file_path, line_number, timestamp
- Classification: **Engine business logic**

#### `src/domain/governance.rs` (43 lines) -- Governance types
- `GovernanceScanResult` struct: areas, coverage_ratio, total_files, total_expected
- `GovernanceArea` struct: name, path, files_found, expected_minimum, coverage
- Classification: **Engine business logic**

#### `src/domain/governance_scanner.rs` (422 lines) -- Governance area scanner
- Scans 6 governance areas in `.orqa/`:
  - principles (`discovery/principles/`)
  - decisions (`process/decisions/`)
  - agents (`process/agents/`)
  - rules (`process/rules/`)
  - skills (`process/skills/`)
  - workflows (`process/workflows/`)
- Each area has an expected minimum file count
- Calculates coverage ratio
- 6 tests
- Classification: **Engine business logic** (NOT extracted)

#### `src/domain/health_snapshot.rs` (48 lines) -- Health metrics
- `HealthSnapshot` struct with 14 metrics: total_artifacts, total_relationships, orphan_count, violation_count, coverage_ratio, process_score, avg_staleness_days, active_sessions, total_sessions, total_messages, principle_count, decision_count, agent_count, lesson_count
- Classification: **Engine business logic**

#### `src/domain/integrity_engine.rs` (101 lines) -- Integrity bridge
- Bridge to `orqa_validation` crate
- Re-exports: `RelationshipSchema`, `ValidationContext`, `IntegrityResult`, `IntegrityIssue`, `IssueSeverity`
- `build_validation_context()` -- constructs context from project settings
- Classification: **Bridge to extracted engine** (`orqa_validation`)

#### `src/domain/knowledge_injector.rs` (477 lines) -- Knowledge injection
- `KnowledgeMatch` struct: file_path, chunk, similarity, title
- `KnowledgeInjector` struct with ONNX model embeddings
- `load(knowledge_dir)` -- reads `.md` files, chunks them, computes embeddings
- `match_prompt(prompt, threshold, max_results)` -- cosine similarity search against embedded knowledge chunks
- Uses `orqa_search::embedder` for ONNX embedding computation
- Threshold default: 0.3 cosine similarity
- 3 tests
- Classification: **Engine business logic** (NOT extracted, but uses `orqa_search` for embedding)

#### `src/domain/lessons.rs` (252 lines) -- Lesson management
- `Lesson` struct: id, title, body, tags, recurrence_count, source_session
- `parse_lesson(content)` -- YAML frontmatter parser
- `render_lesson(lesson)` -- serialises back to frontmatter + body
- Classification: **Engine business logic**

#### `src/domain/message.rs` (192 lines) -- Message types
- `Message` struct with 14 fields: id, session_id, role, content, content_type, turn_index, tool_use_id, tool_name, tool_input, tool_result, is_error, stream_status, thinking, created_at
- `MessageRole` enum: User, Assistant, System, ToolUse, ToolResult
- `ContentType` enum: Text, ToolUse, ToolResult, Thinking, Image
- `StreamStatus` enum: Streaming, Complete, Error, Cancelled
- Classification: **Engine business logic**

#### `src/domain/paths.rs` (242 lines) -- Config-driven path cache
- `ProjectPaths` struct: config-driven path resolution from project.json `artifacts` array
- `from_project_settings()` -- builds path cache from project.json
- `resolve_dir(type_key)` -- returns directory path for a given artifact type
- Falls back to default paths when project.json is missing
- 4 tests
- Classification: **Engine business logic**

#### `src/domain/platform_config.rs` (251 lines) -- Platform config (compile-time)
- `PLATFORM_JSON` embedded at compile time from `libs/types/src/platform/core.json` via `include_str!`
- `PlatformConfig` struct: artifact_types, relationships, semantics
- `RelationshipDef` struct: key, inverse, label, from, to, description, semantic, constraints
- `ConstraintsDef` struct: required, min_count, max_count, require_inverse, status_rules
- `StatusRuleDef` struct: evaluate, condition, statuses, proposed_status, description
- `SemanticDef` struct: description, keys
- `ArtifactTypeDef` struct: key, label, icon, id_prefix
- `PLATFORM: LazyLock<PlatformConfig>` -- parsed once on first access
- `build_inverse_map(rels)` -- builds key<->inverse HashMap
- `build_merged_inverse_map(project_relationships)` -- merges platform + project relationships
- `keys_for_semantic(semantic)` -- returns relationship keys for a semantic category
- `has_semantic(key, semantic)` -- checks if a relationship key belongs to a semantic
- 6 tests
- Classification: **Engine business logic**

#### `src/domain/process_gates.rs` (548 lines) -- Process gates engine
- 5 gates: `understand-first`, `docs-before-code`, `plan-before-build`, `evidence-before-done`, `learn-after-doing`
- `GateResult` struct: gate_name, passed, message, required_actions
- `evaluate_process_gates(process_state)` -- evaluates all gates against session state
- Each gate has specific conditions based on workflow tracker data (files read, docs consulted, etc.)
- 10+ tests
- Classification: **Engine business logic** (NOT extracted)

#### `src/domain/process_state.rs` (287 lines) -- Session process state
- `SessionProcessState` struct: tracks tool calls, file operations, search queries during a session
- `ToolCallRecord` struct: tool_name, timestamp, success
- Aggregation methods: `files_read_count()`, `files_written_count()`, `searches_count()`, `docs_consulted()`
- Used by process gates for evaluation
- Classification: **Engine business logic**

#### `src/domain/project.rs` (119 lines) -- Project types
- `Project` struct: id, name, path, description, detected_stack, created_at, updated_at
- `ProjectSummary` struct: id, name, path, artifact_count
- `DetectedStack` struct: languages, frameworks, package_managers
- `ScanResult` struct: artifacts_found, relationships_found, issues
- Classification: **Engine business logic**

#### `src/domain/project_scanner.rs` (396 lines) -- Project scanner
- Recursive directory walk to depth 10
- Detects 12 languages: Rust, TypeScript, JavaScript, Python, Go, Java, C#, Ruby, Swift, Kotlin, PHP, Dart
- Detects 9 frameworks: React, Vue, Angular, Svelte, Next.js, Django, Flask, Spring, Rails
- Detects 6 package managers: npm, yarn, pnpm, pip, cargo, go modules
- Skips: node_modules, .git, target, dist, build, __pycache__, .venv
- 4 tests
- Classification: **Engine business logic**

#### `src/domain/project_settings.rs` (251 lines) -- Project settings types
- Re-exports from `orqa_validation`: `ProjectSettings`, `ArtifactConfig`, `RelationshipConfig`
- Local types: `ProjectRelationshipConfig`, `ProjectPluginConfig`, `NavigationConfig`
- `ProjectPluginConfig`: installed, enabled, path, version, repo
- Classification: **Engine business logic** (partially re-exports from `orqa_validation`)

#### `src/domain/provider_event.rs` (288 lines) -- Stream event types
- `StreamEvent` enum with 16 variants:
  - `StreamStart`, `TextDelta`, `ThinkingDelta`, `ToolUseStart`, `ToolInputDelta`
  - `ToolResult`, `BlockComplete`, `TurnComplete`, `StreamError`, `StreamCancelled`
  - `HealthOk`, `SummaryResult`, `ToolExecute`, `ToolApprovalRequest`, `SessionInitialized`
  - `EnforcementViolation`
- Each variant carries relevant data (content, tool info, error details)
- Classification: **Engine business logic**

#### `src/domain/session.rs` (128 lines) -- Session types
- `Session` struct: id, project_id, title, status, provider_session_id, created_at, updated_at, ended_at, total_input_tokens, total_output_tokens
- `SessionSummary` struct: id, title, status, message_count, created_at
- `SessionStatus` enum: Active, Ended, Cancelled, Error
- Classification: **Engine business logic**

#### `src/domain/session_title.rs` (116 lines) -- Auto-title generation
- `auto_title_session()` -- sends GenerateSummary to sidecar, uses response as session title
- Falls back to first user message truncated to 50 chars
- Classification: **Engine business logic**

#### `src/domain/settings.rs` (171 lines) -- Settings types
- `ResolvedTheme` struct: theme settings with override cascade
- `SidecarStatus` struct: state, pid, uptime_seconds, cli_detected, cli_version, error_message
- `SidecarState` enum: NotStarted, Starting, Running, Error, Stopped
- Classification: **Engine business logic**

#### `src/domain/setup.rs` (428 lines) -- Claude CLI setup
- `detect_claude_cli()` -- searches PATH for `claude` binary
- `check_claude_version(path)` -- parses version output with regex
- `check_claude_auth(path)` -- verifies authentication status
- `reauthenticate(path)` -- triggers re-auth flow
- `SetupStatus` struct: cli_found, cli_path, cli_version, authenticated, auth_method
- Supports multiple install locations (global npm, homebrew, cargo)
- 5 tests
- Classification: **Engine business logic**

#### `src/domain/status_transitions.rs` (880 lines) -- Status transition engine
- `ProposedTransition` struct: artifact_id, from_status, to_status, reason, auto_apply, rule_description
- `evaluate_transitions(artifact, relationships, rules)` -- evaluates 5 named conditions:
  1. `all_children_status` -- all children have one of the specified statuses
  2. `any_child_status` -- at least one child has a specified status
  3. `no_children_status` -- no children have a specified status
  4. `parent_status` -- parent has a specified status
  5. `relationship_count` -- relationship count meets threshold
- Config-driven from `statusRules` in relationship definitions
- 15+ tests
- Classification: **Engine business logic** (NOT extracted)

#### `src/domain/stream_loop.rs` (1042 lines) -- Core streaming infrastructure
- `run_stream_loop(sidecar, session, messages)` -- main loop: sends message to sidecar, processes streaming responses, emits events to frontend
- `translate_response(sidecar_response)` -- converts sidecar responses to stream events
- `handle_tool_execute(tool_execute, state)` -- dispatches tool execution requests
- `handle_tool_approval(approval_request, state)` -- handles tool approval flow (auto-approve read-only tools, prompt for write tools)
- Enforcement integration: evaluates enforcement rules before/after tool execution
- Session process state tracking during stream
- 5+ tests
- Classification: **Engine business logic** (NOT extracted, largest single file)

#### `src/domain/system_prompt.rs` (270 lines) -- System prompt builder
- `build_system_prompt(project_root, knowledge_matches)` -- constructs prompt from:
  - CLAUDE.md file content
  - Enforcement rules (formatted as instructions)
  - Knowledge injection results (matched chunks)
  - Project context (name, path, detected stack)
- Template-based assembly with section markers
- Classification: **Engine business logic**

#### `src/domain/time_utils.rs` (272 lines) -- Calendar utilities
- `is_leap_year(year)` -- leap year check
- `days_in_month(year, month)` -- days in a given month
- `format_unix_timestamp(seconds)` -- formats Unix timestamp to ISO 8601
- `parse_iso_date(date_string)` -- parses ISO 8601 date string
- No external date/time crate (no chrono)
- 10+ tests
- Classification: **Engine business logic**

#### `src/domain/tool_executor.rs` (1140 lines) -- Tool dispatch
- 11 tool implementations:
  1. `read_file` -- reads file content (MAX_TOOL_OUTPUT_CHARS = 100,000)
  2. `write_file` -- writes file with enforcement check
  3. `edit_file` -- string replacement edit with enforcement check
  4. `bash` -- spawns shell command with 120s timeout, enforcement check
  5. `glob` -- file pattern matching
  6. `grep` -- regex content search
  7. `search_regex` -- regex search (separate from grep)
  8. `search_semantic` -- semantic search via orqa_search
  9. `code_research` -- combined search strategy
  10. `load_knowledge` -- knowledge injection query
  11. `list_directory` -- directory listing
- Enforcement integration: `evaluate_file` before write, `evaluate_bash` before shell
- Returns `ToolOutput` struct with content and metadata
- 8+ tests
- Classification: **Engine business logic** (NOT extracted, second largest file)

#### `src/domain/workflow_tracker.rs` (406 lines) -- Session activity tracker
- `WorkflowTracker` struct: tracks during a session:
  - Files read (with timestamps)
  - Files written (with timestamps)
  - Searches performed (queries + results count)
  - Docs consulted (documentation files read)
  - Verification steps (test runs, lint checks)
- `add_file_read()`, `add_file_write()`, `add_search()`, `add_doc_consulted()`, `add_verification()`
- Used by process gates to evaluate gate conditions
- 5 tests
- Classification: **Engine business logic**

---

### Plugin System (src/plugins/) -- 6 modules

#### `src/plugins/mod.rs` (6 lines) -- Module declarations
- 6 sub-modules: collision, discovery, installer, lockfile, manifest, registry
- Classification: **Plugin infrastructure**

#### `src/plugins/collision.rs` (188 lines) -- Collision detection
- `KeyCollision` struct: key, existing_source, new_source, semantic_match (bool)
- `detect_relationship_collisions(incoming, existing_core, existing_plugins)` -- checks new plugin relationships against core.json + already-installed plugins
- `semantic_match` flag: true when different keys serve same intent (e.g., both "implements" and "realises")
- 4 tests
- Classification: **Plugin infrastructure**

#### `src/plugins/discovery.rs` (70 lines) -- Plugin discovery
- `DiscoveredPlugin` struct: name, version, display_name, description, path, source
- `scan_plugins(project_root)` -- reads project.json, returns only plugins with `installed: true` AND `enabled: true`
- No fallback directory scanning
- 1 test
- Classification: **Plugin infrastructure**

#### `src/plugins/installer.rs` (317 lines) -- Plugin installation
- `install_from_path(source, target)` -- local plugin install (copy directory)
- `install_from_github(repo, target)` -- async, downloads latest release tar.gz, verifies SHA256, extracts
- `InstallResult` struct: name, version, collisions (Vec<KeyCollision>)
- `uninstall(plugin_dir, lockfile_path)` -- removes directory + lockfile entry
- `copy_dir_all()` -- recursive directory copy
- `extract_tar_gz()` -- tar.gz extraction with flate2
- `fetch_latest_tag(owner, repo)` -- GitHub API call for latest release tag
- 3 tests
- Classification: **Plugin infrastructure**

#### `src/plugins/lockfile.rs` (92 lines) -- Plugin lockfile
- `LockEntry` struct: name, version, repo, sha256, installed_at
- `Lockfile` struct with version (always 1) and entries (Vec<LockEntry>)
- `read_lockfile(path)` / `write_lockfile(path, lockfile)` for `plugins.lock.json`
- 2 tests
- Classification: **Plugin infrastructure**

#### `src/plugins/manifest.rs` (145 lines) -- Plugin manifest
- `PluginManifest` struct: name, version, display_name, description, provides, merge_decisions
- `PluginProvides` struct: schemas, views, widgets, relationships, sidecar, cli_tools, tools (legacy), hooks
- `MergeDecision` struct: key, action (replace/skip/rename), rename_to
- `read_manifest(plugin_dir)` -- reads `orqa-plugin.json`
- `validate_manifest(manifest)` -- checks required fields, valid version format
- 3 tests
- Classification: **Plugin infrastructure**

#### `src/plugins/registry.rs` (159 lines) -- Plugin registry
- `RegistryEntry` struct: name, version, description, repo, tags, official (bool)
- `RegistryCatalog` struct: entries (Vec<RegistryEntry>), updated_at
- `RegistryCache` struct with `Mutex<Option<(Instant, RegistryCatalog)>>` -- 1-hour TTL cache
- Fetches from hardcoded GitHub raw URLs (official + community registries)
- `fetch_registry()` -- async, combines official + community entries
- 2 tests
- Classification: **Plugin infrastructure**

---

### Sidecar (src/sidecar/) -- 3 modules

#### `src/sidecar/mod.rs` (3 lines) -- Module declarations
- 3 sub-modules: manager, protocol, types
- Classification: **Tauri glue**

#### `src/sidecar/manager.rs` (333 lines) -- Sidecar process manager
- `SidecarManager` struct with 6 `Mutex`-guarded fields: child (process), stdin, stdout reader, state, start_time, pid
- `spawn(command, args)` -- starts child process with stdin/stdout piped, stderr forwarded to error logging thread
- `send(request)` -- serialises `SidecarRequest` to NDJSON, writes to stdin
- `read_line()` -- reads single NDJSON line from stdout
- `kill()` -- terminates child process
- `restart(command, args)` -- kill + spawn
- `is_connected()` -- checks if child process is alive
- `status()` -- returns `SidecarStatus` with state, PID, uptime
- Stderr thread: detects error patterns (`error`, `Error`, `ERROR`, `panic`, `PANIC`) and emits `app://app-error` events
- 4 tests
- Classification: **Tauri glue**

#### `src/sidecar/protocol.rs` (160 lines) -- NDJSON wire format
- `to_ndjson(value)` -- serialises to single-line JSON + newline
- `from_ndjson(line)` -- deserialises from JSON line
- Handles edge cases: empty lines, trailing whitespace, invalid JSON
- 6 tests
- Classification: **Tauri glue**

#### `src/sidecar/types.rs` (558 lines) -- Sidecar message types
- `SidecarRequest` enum (6 variants):
  - `SendMessage` -- with model, system prompt, messages, max_tokens
  - `CancelStream`
  - `GenerateSummary` -- with text to summarise
  - `HealthCheck`
  - `ToolResult` -- with tool_use_id, content, is_error
  - `ToolApproval` -- with tool_use_id, approved (bool)
- `SidecarResponse` enum (15 variants):
  - `StreamStart` -- with session_id
  - `TextDelta` -- with text
  - `ThinkingDelta` -- with text
  - `ToolUseStart` -- with id, name
  - `ToolInputDelta` -- with text (partial JSON)
  - `ToolResult` -- with id, content
  - `BlockComplete` -- with block_type
  - `TurnComplete` -- with stop_reason, usage
  - `StreamError` -- with error, code
  - `StreamCancelled`
  - `HealthOk` -- with version
  - `SummaryResult` -- with summary
  - `ToolExecute` -- with id, name, input (JSON)
  - `ToolApprovalRequest` -- with id, name, input (JSON)
  - `SessionInitialized` -- with session_id
- Serialization uses `type` field discriminant with camelCase variant names
- 10+ tests for round-trip serialization
- Classification: **Tauri glue** (protocol types)

---

### Servers (src/servers/) -- 3 modules + helper

#### `src/servers/mod.rs` (52 lines) -- Module declarations + helper
- 3 sub-modules: ipc_socket, lsp, mcp
- `find_server_binary(name)` -- locates pre-built server binaries (orqa-mcp-server, orqa-lsp-server) in expected paths relative to the executable
- Classification: **Tauri glue**

#### `src/servers/ipc_socket.rs` (225 lines) -- TCP IPC server
- `start_ipc_listener(port)` -- TCP listener on specified port
- Port written to well-known file: `{data_dir}/com.orqastudio.app/ipc.port` (via `dirs_next::data_dir()`)
- Connection handler reads first line as protocol header:
  - `"MCP"` -> spawns bridge to `orqa-mcp-server` binary
  - `"LSP"` -> spawns bridge to `orqa-lsp-server` binary
- Bridge pipes TCP socket stdin/stdout to binary stdin/stdout
- 2 tests
- Classification: **Tauri glue**

#### `src/servers/lsp.rs` (43 lines) -- LSP server spawner
- `run_lsp_server()` -- finds and spawns `orqa-lsp-server` binary over stdio
- Inherits stdin/stdout for direct stdio communication
- Classification: **Tauri glue**

#### `src/servers/mcp.rs` (43 lines) -- MCP server spawner
- `run_mcp_server()` -- finds and spawns `orqa-mcp-server` binary over stdio
- Inherits stdin/stdout for direct stdio communication
- Classification: **Tauri glue**

---

### Search (src/search/)

#### `src/search/mod.rs` (11 lines) -- Search re-exports
- Thin re-export of `orqa_search` crate: `chunker`, `embedder`, `store`, `types`, `SearchEngine`
- Classification: **Bridge to extracted engine** (`orqa_search`)

---

### Hooks (src/hooks/)

#### `src/hooks/mod.rs` (1 line) -- Module declaration
- `pub mod manager`
- Classification: **Hook infrastructure**

#### `src/hooks/manager.rs` (415 lines) -- Hook dispatcher generation
- `RegisteredHook` struct: plugin, event (git hook name), script_path, priority
- `HookGenerationResult` struct: hooks_generated, events_covered, errors
- `read_hook_registry(project_root)` -- reads `plugin-hooks.json` (written by frontend when plugins with `provides.hooks` are loaded)
- `generate_dispatchers(project_root, registry)` -- generates `.githooks/` shell scripts:
  - Groups registered hooks by git event (pre-commit, commit-msg, etc.)
  - Writes dispatcher script that calls each plugin's hook script in priority order
  - Scripts marked with `GENERATED_MARKER` comment for idempotent regeneration
- 5 tests
- Classification: **Engine business logic**

---

### CLI Tools (src/cli_tools/)

#### `src/cli_tools/mod.rs` (1 line) -- Module declaration
- `pub mod runner`
- Classification: **CLI tool infrastructure**

#### `src/cli_tools/runner.rs` (299 lines) -- CLI tool runner
- `RegisteredCliTool` struct: plugin, key, label, icon, runtime, entrypoint, args, category
- `CliToolResult` struct: plugin, tool_key, exit_code, stdout, stderr, duration_ms, completed_at
- `CliToolStatus` struct: tool_key, plugin, label, success, last_run, last_duration_ms, summary
- `CliToolRunner` struct with `Mutex<HashMap<String, CliToolResult>>` cache
- `read_cli_tool_registry(project_root)` -- reads `plugin-cli-tools.json` (or legacy `plugin-tools.json`)
- `registered_cli_tools()` -- returns all registered tools
- `run(tool, project_root)` -- spawns tool as child process:
  - Runtime "node": `node <entrypoint> [args]`
  - Runtime "system": `<entrypoint> [args]`
  - Category "integrity": appends project_root as argument
  - Captures stdout/stderr, measures duration, caches result
- `statuses(project_root)` -- returns status of all tools with last-run info
- 5 tests
- Classification: **Engine business logic**

---

### Repository Layer (src/repo/) -- 10 modules

#### `src/repo/mod.rs` (11 lines) -- Module declarations
- 10 sub-modules: enforcement_rules_repo, health_snapshot_repo, lesson_repo, message_repo, project_repo, session_repo, settings_repo, theme_repo, project_settings_repo, violations_repo
- Classification: **Data access layer**

#### `src/repo/enforcement_rules_repo.rs` (129 lines) -- Enforcement rules loading
- `load_rules(dir)` -- reads all `*.md` files from a directory, parses YAML frontmatter
- Returns `Vec<EnforcementEntry>` sorted by name
- Classification: **Data access layer**

#### `src/repo/health_snapshot_repo.rs` (220 lines) -- Health snapshots
- `create(conn, snapshot)` -- inserts into health_snapshots table (14 columns)
- `get(conn, id)` -- get by ID
- `get_recent(conn, limit)` -- get most recent N snapshots
- 3 tests
- Classification: **Data access layer**

#### `src/repo/lesson_repo.rs` (371 lines) -- Lessons (file-based)
- File-based storage (NOT SQLite)
- `list(project_root)` -- reads all lessons from resolved directory
- `get(project_root, id)` -- reads single lesson by ID
- `create(project_root, lesson)` -- writes lesson file, generates next ID via IMPL-NNN pattern
- `increment_recurrence(project_root, id)` -- increments recurrence counter
- Uses `ProjectPaths` for directory resolution
- 5 tests
- Classification: **Data access layer**

#### `src/repo/message_repo.rs` (436 lines) -- Messages (SQLite + FTS5)
- `create(conn, message)` -- inserts message, FTS5 trigger auto-indexes
- `create_tool_message(conn, message)` -- specialised for tool use/result messages
- `list(conn, session_id)` -- all messages for a session, ordered by turn_index
- `search(conn, query)` -- FTS5 full-text search with porter tokenizer, snippet highlighting
- `next_turn_index(conn, session_id)` -- max turn_index + 1
- `update_content(conn, id, content)` -- updates message content
- `update_stream_status(conn, id, status)` -- updates streaming status
- 6 tests
- Classification: **Data access layer**

#### `src/repo/project_repo.rs` (289 lines) -- Projects (SQLite)
- `create(conn, project)` -- inserts with unique path constraint
- `get(conn, id)` -- get by ID
- `get_by_path(conn, path)` -- get by filesystem path
- `get_active(conn)` -- get the most recently updated project
- `list(conn)` -- all projects ordered by updated_at DESC
- `touch_updated_at(conn, id)` -- updates timestamp
- `update_detected_stack(conn, id, stack)` -- updates detected languages/frameworks
- 5 tests
- Classification: **Data access layer**

#### `src/repo/session_repo.rs` (444 lines) -- Sessions (SQLite)
- `create(conn, session)` -- inserts session
- `get(conn, id)` -- get by ID
- `list(conn, project_id, status_filter, limit, offset)` -- paginated listing with optional status filter
- `update_title(conn, id, title)` -- updates title
- `auto_update_title(conn, id, title)` -- updates only if `title_manually_set` is false
- `end_session(conn, id)` -- sets status to Ended, records ended_at
- `delete(conn, id)` -- deletes session + cascades
- `update_token_usage(conn, id, input_tokens, output_tokens)` -- accumulates token counts
- `update_provider_session_id(conn, id, provider_id)` -- updates external session reference
- 8 tests
- Classification: **Data access layer**

#### `src/repo/settings_repo.rs` (173 lines) -- Settings (SQLite key-value)
- `get(conn, scope, key)` -- get setting by scope + key
- `set(conn, scope, key, value)` -- upsert setting
- `get_all(conn, scope)` -- all settings for a scope
- Scoped key-value storage (scope examples: "app", "project:{id}")
- 4 tests
- Classification: **Data access layer**

#### `src/repo/theme_repo.rs` (268 lines) -- Themes (SQLite)
- `get_themes(conn)` -- all theme definitions
- `get_overrides(conn, project_id)` -- project-specific theme overrides
- `set_override(conn, project_id, key, value)` -- upsert theme override
- `clear_overrides(conn, project_id)` -- removes all overrides for a project
- Cascade on project delete
- 5 tests
- Classification: **Data access layer**

#### `src/repo/project_settings_repo.rs` (124 lines) -- Project settings (file-based)
- `read(project_root)` -- delegates to `config_loader::load_project_settings`
- `write(project_root, settings)` -- writes `.orqa/project.json`
- File-based, not SQLite
- 2 tests
- Classification: **Data access layer**

#### `src/repo/violations_repo.rs` (145 lines) -- Violations (SQLite)
- `list_for_project(conn, project_id, limit)` -- lists violations with optional limit, ordered most recent first
- Classification: **Data access layer**

---

### SQL Migrations (migrations/)

| File | Lines | Purpose |
|------|-------|---------|
| `001_initial_schema.sql` | 162 | Core tables: projects, sessions, messages, artifacts (later dropped), settings, project_themes, project_theme_overrides. FTS5 on messages (porter tokenizer) with INSERT/DELETE triggers. FTS5 on artifacts (later dropped). |
| `002_governance_bootstrap.sql` | 41 | governance_analyses, governance_recommendations tables (both later dropped). |
| `003_enforcement.sql` | 16 | enforcement_violations table: rule_name, violation_type, message, file_path, line_number, project_id, session_id, created_at. |
| `004_sdk_session_id.sql` | 12 | ALTER TABLE sessions ADD COLUMN sdk_session_id (later renamed to provider_session_id in code). |
| `005_title_manually_set.sql` | 6 | Comment only -- actual ALTER TABLE sessions ADD COLUMN title_manually_set is done in Rust code. |
| `007_drop_governance_tables.sql` | 3 | DROP governance_recommendations, governance_analyses (tables from 002). |
| `008_health_snapshots.sql` | 17 | health_snapshots table: id, project_id, 14 metric columns, created_at. |
| `009_drop_artifacts_table.sql` | 3 | DROP artifacts_fts, artifacts (artifacts moved to file-based storage). |
| `010_health_snapshots_extended.sql` | 7 | Placeholder -- actual column additions done in Rust code. |

Note: Migration 006 is skipped (does not exist).

---

### Test Sidecar

#### `test-sidecar/echo.cjs` (90 lines) -- NDJSON echo sidecar for testing
- Node.js script using NDJSON protocol over stdin/stdout
- Handles 3 message types:
  - `health_check` -> responds with `{ type: "health_ok", version: "0.1.0-test" }`
  - `send_message` -> fake streaming: emits stream_start, text_delta ("Echo: " + first message content), turn_complete
  - `cancel_stream` -> responds with `{ type: "stream_cancelled" }`
  - `generate_summary` -> responds with `{ type: "summary_result", summary: "Test summary" }`
- Classification: **Test fixture**

---

### Generated Files (gen/schemas/)

| File | Purpose |
|------|---------|
| `acl-manifests.json` | Tauri ACL manifests (auto-generated by tauri-build) |
| `capabilities.json` | Tauri capabilities schema (auto-generated) |
| `desktop-schema.json` | Tauri desktop configuration schema (auto-generated) |
| `windows-schema.json` | Tauri windows configuration schema (auto-generated) |

All generated files are produced by `tauri_build::build()` in `build.rs`. Do not edit manually.

---

### Icons (icons/)

17 icon files for cross-platform packaging:

| File | Purpose |
|------|---------|
| `32x32.png` | Small icon (Linux/Windows) |
| `128x128.png` | Medium icon (Linux) |
| `128x128@2x.png` | Retina medium icon (macOS) |
| `icon.icns` | macOS icon bundle |
| `icon.ico` | Windows icon |
| `icon.png` | Base icon |
| `Square*Logo*.png` (x11) | Windows Store / UWP tiles (various sizes: 30x30, 44x44, 71x71, 89x89, 107x107, 142x142, 150x150, 284x284, 310x310 + scale variants) |
| `StoreLogo.png` | Windows Store listing icon |

---

### File Count Summary

| Category | Files | Total Lines (approx) |
|----------|-------|---------------------|
| Configuration | 4 | 127 |
| Entry points | 2 | 320 |
| Core infrastructure | 5 | 964 |
| Commands | 20 | ~2,500 |
| Domain logic | 35 | ~12,400 |
| Plugins | 7 | 977 |
| Sidecar | 4 | 1,054 |
| Servers | 4 | 363 |
| Search | 1 | 11 |
| Hooks | 2 | 416 |
| CLI Tools | 2 | 300 |
| Repository | 11 | ~2,600 |
| Migrations | 9 | 267 |
| Test sidecar | 1 | 90 |
| Generated schemas | 4 | N/A (auto-generated) |
| Icons | 17 | N/A (binary) |
| **Total** | **128** | **~22,400** |

---

## Recommendations

(None -- this is a factual inventory per the task scope.)

## Open Questions

None. All files in `app/src-tauri/` (excluding `target/`) have been catalogued.
