# EPIC-a1b2c3d4: OrqaDev Task Decomposition

> Atomic task list for "OrqaDev: Comprehensive Logging & Developer Tools"
> Generated: 2026-03-31
> Total tasks: 42
> Phases: 4 (Quick Wins, Comprehensive Logging, Event Infrastructure, OrqaDev App)

---

## Phase 1: Quick Wins (unblock dogfooding)

These tasks have NO blockers and can start immediately. Tasks 01-03 are fully parallel.
Task 04 depends on 02. Task 05 depends on 01.

---

### TASK-01: Move dashboard spawn to top of startController()
**Blocked by:** none
**Files:**
- `cli/src/commands/dev.ts` (edit ~lines 387-560)
**Acceptance Criteria:**
- [ ] Dashboard process spawns BEFORE daemon start (before line 423's `runDaemonCommand`)
- [ ] `if (!dashboardProc)` guard preserved to prevent double-spawning on restart
- [ ] Dashboard PID is written to control file at all stages (starting, building, running)
- [ ] Dashboard stdio changed from `"ignore"` to append-mode log fd so dashboard errors are captured in `.state/dev-controller.log`
- [ ] `orqa dev` run shows dashboard URL in terminal before "Starting daemon..." message
**Instructions:**
In `startController()`, move the dashboard spawn block (currently at lines 534-557) to immediately after the `writeControlFile(root, { state: "starting", ... })` call at line 416. Open `.state/dev-controller.log` in append mode (`fs.openSync(..., "a")`) and pass the fd as `stdio: ["ignore", logFd, logFd]` to the dashboard spawn. Keep the browser-open `setTimeout` as-is. Update the three `writeControlFile` calls (starting, building, running) to include `dashboard: dashboardProc?.pid ?? null`.
**ZERO DEFERRALS:** The dashboard MUST be listening before any other process starts. Do not leave the old spawn location in place.

---

### TASK-02: Fix errorStore.initBrowserHandlers() — never called
**Blocked by:** none
**Files:**
- `app/src/lib/components/layout/AppLayout.svelte` (edit ~line 53)
**Acceptance Criteria:**
- [ ] `errorStore.initBrowserHandlers()` is called in `onMount` alongside `errorStore.initialize()`
- [ ] `window.onerror` handler fires and adds error to `errorStore.errors` (manual test: `setTimeout(() => { throw new Error("test"); }, 1000)` in console)
- [ ] `window.onunhandledrejection` handler fires and adds error to `errorStore.errors`
**Instructions:**
In `AppLayout.svelte`, add `errorStore.initBrowserHandlers()` on the line after `errorStore.initialize()` inside the `onMount` callback (line 53). The method already exists in `errors.svelte.ts:83` with proper idempotency guards.
**ZERO DEFERRALS:** This is a one-line addition. No partial fix.

---

### TASK-03: Fix duplicate SDK log forwarding
**Blocked by:** none
**Files:**
- `app/src/lib/utils/dev-console.ts` (edit)
**Acceptance Criteria:**
- [ ] Only ONE mechanism forwards SDK log entries to `http://localhost:10130/log` (not two)
- [ ] The logger's built-in `forwardToDashboard()` in `libs/logger/src/index.ts:58` remains as the sole forwarding path
- [ ] `dev-console.ts` no longer calls `subscribeToLogs(forwardEntry)` — remove the subscriber-based forwarding
- [ ] `initDevConsole()` still calls `setLogLevel("debug")` in dev mode
- [ ] Verify by checking network tab: each SDK log entry produces exactly one POST to `/log`
**Instructions:**
In `dev-console.ts`, remove the `subscribeToLogs(forwardEntry)` call and the `forwardEntry` function. The logger itself already forwards via `forwardToDashboard()` on every `emit()`. Keep `initDevConsole()` as a function that calls `setLogLevel("debug")` — this is still needed. Remove the now-unused `DEV_LOG_URL` constant and `forwardEntry` function.
**ZERO DEFERRALS:** Both the subscriber removal and the verification are required.

---

### TASK-04: Add browser console.log/warn/error intercept in dev mode
**Blocked by:** TASK-03 (must remove duplicate forwarding first)
**Files:**
- `app/src/lib/utils/dev-console.ts` (edit)
**Acceptance Criteria:**
- [ ] In dev mode (`import.meta.env.DEV`), `console.log`, `console.warn`, `console.error`, and `console.debug` are monkey-patched to also forward to the dashboard
- [ ] Original console methods still work (output appears in browser DevTools)
- [ ] Forwarded entries have `source: "console"` and correct level mapping (log->info, warn->warn, error->error, debug->debug)
- [ ] Non-dev builds are unaffected (no monkey-patching occurs)
- [ ] Object arguments are JSON-stringified in the forwarded message
**Instructions:**
In `initDevConsole()`, after `setLogLevel("debug")`, add a console intercept loop. For each method in `["log", "warn", "error", "debug"]`, save the original via `const original = console[method].bind(console)`, then replace `console[method]` with a wrapper that calls `original(...args)` and then sends a `sendBeacon` POST to `DEV_LOG_URL` with `{ level, source: "console", message }`. Use the same `sendBeacon`/`fetch` pattern already in the logger. Guard everything under `import.meta.env.DEV`.
**ZERO DEFERRALS:** All four console methods must be intercepted. Do not skip `console.debug`.

---

### TASK-05: Forward process stdout/stderr from dev controller to dashboard SSE
**Blocked by:** TASK-01 (dashboard must be running first)
**Files:**
- `tools/debug/dev.mjs` (edit)
**Acceptance Criteria:**
- [ ] Dashboard tails `.state/dev-controller.log` using `fs.createReadStream` with `{ start: initialSize, encoding: "utf-8" }` and re-opens on `fs.watchFile` trigger
- [ ] New lines from the log file are broadcast as `controller-log` SSE events with `{ source, text, level }` fields
- [ ] Source is extracted from prefixed lines (e.g., `[search]`, `[app]`, `[tsc:sdk]`) — fallback to `"ctrl"` for unprefixed lines
- [ ] Dashboard HTML renders `controller-log` events in the log view (same rendering as `log` events)
- [ ] Log file truncation (new `orqa dev` run) is detected and tail resets to position 0
**Instructions:**
In `dev.mjs`, after the server starts listening, open `.state/dev-controller.log` with `fs.createReadStream`. Track the current file size. On new data, split into lines, extract the `[source]` prefix via regex, and `sseBroadcast("controller-log", { source, text, level: "info" })`. Use `fs.watchFile` (or `fs.watch`) to detect file changes and re-open if the file was truncated (new size < last known size). In `dev-dashboard.html`, add a handler for the `controller-log` event that renders it using the same `addLine()` function as `log` events.
**ZERO DEFERRALS:** Must handle both the tailing and the truncation-reset case.

---

## Phase 2: Comprehensive Logging

Tasks 06-13 add structured logging to the four major components. All Phase 2 tasks
depend only on Phase 1 completion (no inter-dependencies within Phase 2). Tasks 06-13
can ALL run in parallel.

---

### TASK-06: Engine timing spans — graph build, validate, index, embed
**Blocked by:** none (Phase 1 not required for engine logging)
**Files:**
- `engine/validation/src/graph.rs` (edit `build_artifact_graph`)
- `engine/validation/src/checks/mod.rs` (edit `run_all` or equivalent integrity check fn)
- `engine/search/src/engine.rs` (edit `index`, `embed_chunks`)
**Acceptance Criteria:**
- [ ] `build_artifact_graph` logs `tracing::info!` at completion with `elapsed_ms` and `node_count` fields
- [ ] Integrity check entry function logs `tracing::info!` at completion with `elapsed_ms`, `check_count`, `error_count`, `warning_count`
- [ ] `index` logs `tracing::info!` at completion with `elapsed_ms`, `chunk_count`, `file_count`
- [ ] `embed_chunks` logs `tracing::info!` at completion with `elapsed_ms`, `batch_count`, `total_chunks`
- [ ] All timing uses `std::time::Instant::now()` at function entry, `elapsed().as_millis()` at exit
**Instructions:**
Add `use std::time::Instant;` to each file. At the top of each function, add `let start = Instant::now();`. At the function's return point, add `tracing::info!(subsystem = "engine", elapsed_ms = start.elapsed().as_millis() as u64, <metric_fields>, "[engine] <function_name> completed");`. For `build_artifact_graph`, count nodes from the returned graph. For integrity checks, count from the returned results. For `index`/`embed_chunks`, count from the processed items.
**ZERO DEFERRALS:** All four functions must have timing. Do not skip embed_chunks.

---

### TASK-07: Engine error surfacing — silent I/O and parse failures
**Blocked by:** none
**Files:**
- `engine/validation/src/graph.rs` (edit `walk_directory` ~2 locations)
- `engine/artifact/src/reader.rs` (edit ~3 locations)
- `engine/plugin/src/discovery.rs` (edit `scan_plugins` ~2 locations)
**Acceptance Criteria:**
- [ ] `walk_directory`: `read_dir` failure logs `tracing::warn!` with path before returning Ok (2 locations in graph.rs)
- [ ] `walk_directory`: YAML parse `unwrap_or(Value::Null)` replaced with `match` that logs `tracing::warn!` with file path on parse error
- [ ] `artifact_scan_tree` in reader.rs: all `read_to_string(...).unwrap_or_default()` calls replaced with `match` + `tracing::warn!` on error (2-3 locations)
- [ ] `artifact_scan_tree` in reader.rs: YAML parse `unwrap_or(Value::Null)` replaced with `match` + `tracing::warn!` on error
- [ ] `scan_plugins`: `read_to_string` failure logs `tracing::warn!` with manifest path
- [ ] `scan_plugins`: `serde_json::from_str` failure logs `tracing::warn!` with plugin path and error
- [ ] Plugin discovery logs `tracing::info!` with discovered plugin count at end of `scan_plugins`
**Instructions:**
In each file, replace `unwrap_or_default()` and `unwrap_or(Value::Null)` patterns with `match` blocks. On the error arm, emit `tracing::warn!(path = %path.display(), error = %e, "[engine] ...")` then use the default value. For `scan_plugins`, add a final `tracing::info!(count = plugins.len(), "[plugins] discovered N plugins")` after the scan loop.
**ZERO DEFERRALS:** Every silent failure pattern identified in the research must be addressed. Do not skip reader.rs.

---

### TASK-08: Engine metrics — prompt builder token estimates and plugin install logging
**Blocked by:** none
**Files:**
- `engine/prompt/src/builder.rs` (edit `build_system_prompt`)
- `engine/plugin/src/installer.rs` (edit `install_from_path`, `install_from_github`)
- `engine/prompt/src/builder.rs` (edit `read_rules` silent skip)
**Acceptance Criteria:**
- [ ] `build_system_prompt` logs `tracing::debug!` with rule_count, knowledge_count, agent_definition_count, and estimated_tokens (char_count / 4)
- [ ] `install_from_path` logs `tracing::info!` with plugin name and version on success
- [ ] `install_from_github` logs `tracing::info!` with plugin name, version, and resolved ref on success
- [ ] `read_rules` logs `tracing::warn!` when a rule file cannot be read (currently silently skips)
- [ ] Constraint violation in installer logs `tracing::warn!` with which specific constraint failed
**Instructions:**
In `build_system_prompt`, after assembling the prompt, count rules, knowledge items, and agent definitions from the assembled sections. Compute `estimated_tokens = prompt_text.len() / 4`. Log all fields at `debug` level. In `installer.rs`, add `tracing::info!` calls after successful install operations. In `read_rules`, replace silent `continue` on file read error with `tracing::warn!` + `continue`.
**ZERO DEFERRALS:** Token estimate logging is critical for P5 enforcement. Do not skip it.

---

### TASK-09: Daemon handler timing — prompt, knowledge, context, parse endpoints
**Blocked by:** none
**Files:**
- `daemon/src/prompt.rs` (edit `prompt_handler`)
- `daemon/src/knowledge.rs` (edit `knowledge_handler`, `get_semantic_knowledge`)
- `daemon/src/context.rs` (edit `context_handler`)
- `daemon/src/parse.rs` (edit `parse_handler`)
- `daemon/src/compact_context.rs` (edit `compact_context_handler`)
**Acceptance Criteria:**
- [ ] `prompt_handler` logs entry with request fields and exit with `elapsed_ms`, `prompt_type`, `method`, `tokens`, `budget`
- [ ] `knowledge_handler` logs entry and exit with `elapsed_ms`, `role`, `declared_count`, `semantic_count`
- [ ] `get_semantic_knowledge` logs `tracing::debug!` at each early-return point explaining which condition caused the return
- [ ] `context_handler` logs `tracing::debug!` with rule_count and workflow_count found
- [ ] `parse_handler`: promote `should_warn=true` case from `debug!` to `info!`
- [ ] `compact_context_handler` logs exit with `elapsed_ms`; logs `tracing::warn!` when project_path does not exist
**Instructions:**
Add `use std::time::Instant;` at the top of each file. Add `let start = Instant::now();` at the beginning of each handler. Add `tracing::info!(subsystem = "...", elapsed_ms = ..., ...)` at the return point. For `get_semantic_knowledge`, add `tracing::debug!` before each early `return None` or `return vec![]` explaining which condition triggered. In `parse_handler`, change `debug!` to `info!` when `should_warn` is true.
**ZERO DEFERRALS:** All five handlers must have timing. The `should_warn` promotion is required.

---

### TASK-10: Daemon lifecycle logging — config, ONNX, subprocess, watcher
**Blocked by:** none
**Files:**
- `daemon/src/config.rs` (edit `DaemonConfig::load`)
- `daemon/src/knowledge.rs` (edit `resolve_model_dir`)
- `daemon/src/logging.rs` (edit `init`)
- `daemon/src/subprocess.rs` (edit `check_status`)
- `daemon/src/watcher.rs` (edit `invoke_generator`, `handle_events`)
**Acceptance Criteria:**
- [ ] `DaemonConfig::load` logs `tracing::warn!` when orqa.toml exists but cannot be parsed; logs `tracing::info!` with effective config values (min_score, max_semantic, etc.)
- [ ] `resolve_model_dir` logs `tracing::info!` with model path when found; logs `tracing::info!` with "ONNX model not found" when not found
- [ ] `logging::init` logs `tracing::info!` after subscriber installation with log file path, TTY mode, and effective log level
- [ ] `check_status` crash path logs a crash counter field (e.g., incrementing a counter in SubprocessManager)
- [ ] `invoke_generator` logs `tracing::info!` with `elapsed_ms` around `cmd.output()`
- [ ] `handle_events` logs `tracing::debug!` with changed file paths that triggered rebuild
**Instructions:**
For `DaemonConfig::load`: after attempting to read/parse orqa.toml, add a `match` around the parse result — `Ok` logs values at info, `Err` logs warning with path + error. For `resolve_model_dir`: add `info!` at the found/not-found branches. For `logging::init`: add `info!` after `tracing::subscriber::set_global_default`. For `check_status`: add a `crash_count: u32` field to `SubprocessManager`, increment on crash, include in log. For `invoke_generator`: wrap `cmd.output()` with `Instant` timing. For `handle_events`: log the `changed_paths` list at debug level.
**ZERO DEFERRALS:** The ONNX model resolution logging is the single highest-impact gap. Do not skip it.

---

### TASK-11: Daemon subsystem field consistency — mcp, lsp, health
**Blocked by:** none
**Files:**
- `daemon/src/mcp.rs` (edit `start_mcp`)
- `daemon/src/lsp.rs` (edit `start_lsp`)
- `daemon/src/health.rs` (edit `start`)
- `daemon/src/session_start.rs` (edit `session_start_handler`, `check_graph_integrity`, `check_installation`)
**Acceptance Criteria:**
- [ ] All `tracing::info!` and `tracing::warn!` calls in `mcp.rs` include `subsystem = "mcp"` field
- [ ] All `tracing::info!` and `tracing::warn!` calls in `lsp.rs` include `subsystem = "lsp"` field
- [ ] All `tracing::info!` and `tracing::warn!` calls in `health.rs` include `subsystem = "health"` field
- [ ] `start_mcp` logs resolved port before spawn attempt
- [ ] `start_lsp` logs resolved port before spawn attempt
- [ ] `session_start_handler` logs `tracing::info!` at entry with project_path
- [ ] `check_graph_integrity` logs `tracing::warn!` for malformed JSON files (not just response body)
- [ ] `check_installation` logs `tracing::warn!` for missing installation items
**Instructions:**
Add `subsystem = "mcp"` to all tracing calls in mcp.rs. Same for lsp.rs with `subsystem = "lsp"`. Add `subsystem = "health"` to health.rs. Add `tracing::info!(subsystem = "mcp", port = mcp_port, ...)` before the spawn in start_mcp. Same pattern for start_lsp. In session_start_handler, add `tracing::info!(subsystem = "session", project_path = %..., ...)` at the top. In check_graph_integrity, log malformed JSON paths as `tracing::warn!`. In check_installation, log missing items as `tracing::warn!`.
**ZERO DEFERRALS:** Every subsystem field and entry log must be added.

---

### TASK-12: Tauri IPC logging — command entry/exit, sidecar, sessions, plugins
**Blocked by:** none
**Files:**
- `app/src-tauri/src/commands/project_commands.rs` (edit `project_open`)
- `app/src-tauri/src/commands/session_commands.rs` (edit `session_create`, `session_end`, `session_delete`)
- `app/src-tauri/src/commands/plugin_commands.rs` (edit `plugin_install_local`, `plugin_install_github`, `plugin_uninstall`)
- `app/src-tauri/src/sidecar/manager.rs` (edit `spawn`, `kill`, `restart`)
- `app/src-tauri/src/commands/graph_commands.rs` (edit `get_or_build_graph`, `refresh_artifact_graph`, `apply_auto_fixes`, `update_artifact_field`)
**Acceptance Criteria:**
- [ ] `project_open` logs `tracing::info!` at entry with path and at exit with project ID
- [ ] `session_create`, `session_end`, `session_delete` each log `tracing::info!` with session_id
- [ ] `plugin_install_local`, `plugin_install_github` log `tracing::info!` with plugin name on success
- [ ] `plugin_uninstall` logs `tracing::info!` with plugin name
- [ ] `SidecarManager::spawn` logs `tracing::info!` with PID and command after successful spawn
- [ ] `SidecarManager::kill` and `restart` log `tracing::info!` at entry
- [ ] `get_or_build_graph` logs `tracing::info!` on cache miss with node_count and elapsed_ms
- [ ] `refresh_artifact_graph` logs `tracing::info!` at entry and exit with elapsed_ms and node_count
- [ ] `apply_auto_fixes` logs `tracing::info!` with count of applied fixes
- [ ] `update_artifact_field` logs `tracing::debug!` with artifact_id and field name
**Instructions:**
Add `use std::time::Instant;` where timing is needed. For each command, add `tracing::info!` or `tracing::debug!` at the appropriate points. Use `tracing::info!` for lifecycle events (project open, session create, plugin install) and `tracing::debug!` for high-frequency operations (field updates). For graph operations, wrap with `Instant::now()` for timing.
**ZERO DEFERRALS:** All listed commands must have logging. Do not skip the sidecar manager.

---

### TASK-13: Tauri IPC logging — streaming, CLI tools, status transitions, enforcement
**Blocked by:** none
**Files:**
- `app/src-tauri/src/commands/stream_commands.rs` (edit `stream_send_message`, `stream_stop`, `stream_tool_approval_respond`)
- `app/src-tauri/src/commands/cli_tool_commands.rs` (edit `run_cli_tool`)
- `app/src-tauri/src/commands/status_transition_commands.rs` (edit `apply_status_transition`)
- `app/src-tauri/src/commands/enforcement_commands.rs` (edit `enforcement_rules_reload`, `governance_scan`)
- `app/src-tauri/src/commands/sidecar_commands.rs` (edit `ensure_sidecar_running`, `sidecar_restart`)
**Acceptance Criteria:**
- [ ] `stream_send_message` logs `tracing::debug!` at entry with session_id and content length; logs `tracing::info!` at exit with elapsed_ms
- [ ] `stream_stop` logs `tracing::info!` with session_id
- [ ] `stream_tool_approval_respond` logs `tracing::info!` with tool_call_id and approved/denied
- [ ] `run_cli_tool` logs `tracing::info!` at entry with tool key and at exit with exit status and elapsed_ms
- [ ] `apply_status_transition` logs `tracing::info!` with artifact_id and new status
- [ ] `enforcement_rules_reload` promoted from `tracing::debug!` to `tracing::info!` (explicit user action)
- [ ] `governance_scan` logs `tracing::info!` with count of governance files found
- [ ] `ensure_sidecar_running` logs `tracing::debug!` for fast path (already connected) and `tracing::info!` for slow path (spawning)
- [ ] `sidecar_restart` logs `tracing::info!` at entry
**Instructions:**
Add appropriate `tracing::info!` and `tracing::debug!` calls at the specified locations. For `stream_send_message`, add timing with `Instant`. For `run_cli_tool`, add timing. For `enforcement_rules_reload`, change `tracing::debug!` to `tracing::info!`. For `governance_scan`, log the count from the returned results.
**ZERO DEFERRALS:** Streaming command logging is critical for debugging agent interactions. Do not skip it.

---

### TASK-14: Frontend SDK logging — graph init, IPC timing, navigation, streams
**Blocked by:** none
**Files:**
- `libs/sdk/src/ipc/invoke.ts` (edit `invoke`)
- `libs/sdk/src/graph/artifact-graph.svelte.ts` (edit `initialize`, `_fetchAll`, `refresh`)
- `libs/sdk/src/stores/navigation.svelte.ts` (edit `setActivity`)
- `libs/sdk/src/stores/conversation.svelte.ts` (edit `handleStreamEvent`)
- `libs/sdk/src/stores/project.svelte.ts` (edit `loadActiveProject`, `openProject`)
- `libs/sdk/src/stores/session.svelte.ts` (edit `createSession`, `restoreSession`)
**Acceptance Criteria:**
- [ ] `invoke()` wrapper measures `performance.now()` before and after `tauriInvoke`, logs via SDK logger with cmd name and duration_ms
- [ ] `invoke()` catch path logs error with cmd name and error message before re-throwing
- [ ] `initialize()` logs start with projectPath and completion with elapsed_ms and node_count
- [ ] `_fetchAll()` logs completion with elapsed_ms and total node count
- [ ] `refresh()` logs completion with elapsed_ms
- [ ] `setActivity()` logs from/to activity keys
- [ ] `handleStreamEvent` for `stream_start` logs model and message_id
- [ ] `handleStreamEvent` for `turn_complete` logs elapsed_ms since stream_start
- [ ] `handleStreamEvent` for `stream_error` logs error message
- [ ] `loadActiveProject` and `openProject` log at entry and exit with project path
- [ ] `createSession` and `restoreSession` log with session_id
**Instructions:**
In `invoke.ts`, wrap the `tauriInvoke` call with `performance.now()` timing and emit via `logger("ipc").perf(cmd, duration)` on success and `logger("ipc").error(cmd + " failed", error)` on catch. In graph SDK, use `logger("graph")` for timing logs. In navigation, use `logger("navigation")` for activity changes. In conversation store, track `streamStartTime` and compute elapsed on `turn_complete`. In project/session stores, use `logger("project")` and `logger("session")`.
**ZERO DEFERRALS:** IPC invoke timing is the highest-impact frontend gap. Do not skip it.

---

### TASK-15: Frontend lifecycle logging — app boot, setup, settings, daemon health
**Blocked by:** none
**Files:**
- `app/src/routes/+layout.svelte` (edit module init)
- `app/src/lib/components/layout/AppLayout.svelte` (edit onMount)
- `libs/sdk/src/stores/setup.svelte.ts` (edit `checkSetupStatus`)
- `libs/sdk/src/stores/settings.svelte.ts` (edit `initialize`, `refreshSidecarStatus`, `refreshDaemonHealth`)
- `libs/sdk/src/plugins/plugin-registry.svelte.ts` (edit `register`)
**Acceptance Criteria:**
- [ ] `+layout.svelte` logs app boot start timestamp via SDK logger at module init
- [ ] `AppLayout.svelte:onMount` logs "app shell mounted" via SDK logger
- [ ] `checkSetupStatus` logs result (complete or current step)
- [ ] `settings.initialize()` logs initial theme, model, fontSize
- [ ] `refreshSidecarStatus` logs state transitions (new state, pid)
- [ ] `refreshDaemonHealth` logs state transitions (new state, artifact count)
- [ ] `plugin-registry.register()` logs plugin name, schema count, relationship count, view count
**Instructions:**
Use `logger("lifecycle")` for boot/mount events, `logger("setup")` for setup status, `logger("settings")` for settings init, `logger("plugins")` for plugin registration. Import from `@orqastudio/sdk` (which re-exports from `@orqastudio/logger`). Each log should be a single `log.info(...)` or `log.debug(...)` call.
**ZERO DEFERRALS:** All listed lifecycle points must have logging. Do not skip plugin registration logging.

---

### TASK-16: Frontend worker error forwarding
**Blocked by:** none
**Files:**
- `app/src/lib/workers/graph-layout.worker.ts` (edit error handling)
- consumer of graph-layout worker (edit to forward worker errors)
**Acceptance Criteria:**
- [ ] `graph-layout.worker.ts` posts error messages back to main thread via `postMessage({ type: "error", message: ... })`
- [ ] Main-thread worker consumer receives `type: "error"` messages and logs them via `logger("graph-layout-worker").error(message)`
- [ ] Worker errors appear in the dashboard log stream
**Instructions:**
In the worker, wrap the `runLayout` function's `console.error` call to also `postMessage({ type: "error", message: errorMessage })`. On the main thread where the worker is consumed, add an `onmessage` handler that checks for `type: "error"` and logs via the SDK logger.
**ZERO DEFERRALS:** Both the worker-side postMessage and the main-thread handler are required.

---

## Phase 3: Event Infrastructure

Phase 3 tasks build the structured event system. They depend on Phase 2 logging being
in place so that event sources exist. Tasks 17-19 are sequential (schema -> bus -> persistence).
Task 20 can run in parallel with 19.

---

### TASK-17: Design structured event schema
**Blocked by:** TASK-06 through TASK-16 (Phase 2 logging in place)
**Files:**
- `engine/types/src/lib.rs` or new `engine/types/src/event.rs` (create)
- `daemon/src/events.rs` (create — event types for daemon-side use)
**Acceptance Criteria:**
- [ ] `LogEvent` struct defined with fields: `id` (u64), `timestamp` (i64 ms), `level` (enum: debug/info/warn/error/perf), `source` (String), `category` (String), `message` (String), `metadata` (serde_json::Value), `session_id` (Option<String>)
- [ ] `EventLevel` enum with Display impl
- [ ] `EventSource` enum with variants for each process: Daemon, App, Frontend, DevController, MCP, LSP, Search, Worker
- [ ] All types derive `Serialize`, `Deserialize`, `Clone`, `Debug`
- [ ] Types are re-exported from `engine/types` for use by all Rust crates
**Instructions:**
Create `engine/types/src/event.rs` with the event types. Add `pub mod event;` to `engine/types/src/lib.rs`. Use `serde_json::Value` for metadata to keep it flexible. The `category` field allows sub-classification (e.g., "timing", "lifecycle", "ipc", "file-watch"). Keep the schema flat — no nested types.
**ZERO DEFERRALS:** The schema must be complete and usable by the bus implementation.

---

### TASK-18: Implement pub/sub event bus (Rust, daemon-side)
**Blocked by:** TASK-17
**Files:**
- `daemon/src/event_bus.rs` (create)
- `daemon/src/main.rs` (edit — wire bus into startup)
**Acceptance Criteria:**
- [ ] `EventBus` struct with `publish(event: LogEvent)`, `subscribe() -> Receiver<LogEvent>`, and `shutdown()` methods
- [ ] Uses `tokio::sync::broadcast` channel with configurable buffer size (default 10,000)
- [ ] `publish` is non-blocking — if buffer is full, oldest events are dropped with a warn log
- [ ] Subscribers receive events via `tokio::sync::broadcast::Receiver`
- [ ] `EventBus` is created in `main.rs::run()` and passed to handlers via shared state
- [ ] Health endpoint exposes event bus stats: total_published, total_dropped, subscriber_count
- [ ] Bus is wrapped in `Arc<EventBus>` for safe sharing across tasks
**Instructions:**
Create `event_bus.rs` with an `EventBus` struct wrapping `tokio::sync::broadcast::Sender<LogEvent>`. The `publish` method sends on the channel; on `SendError` (no receivers), it increments a dropped counter. The `subscribe` method returns `sender.subscribe()`. Wire the bus into `run()` in main.rs by creating it early and passing an `Arc<EventBus>` to the health handler state. Update the health response to include bus stats.
**ZERO DEFERRALS:** The bus must be functional and wired into the daemon startup.

---

### TASK-19: Add SQLite persistence layer for events
**Blocked by:** TASK-18
**Files:**
- `daemon/src/event_store.rs` (create)
- `daemon/src/main.rs` (edit — start persistence subscriber)
**Acceptance Criteria:**
- [ ] SQLite database created at `.state/events.db` with a `log_events` table: `id INTEGER PRIMARY KEY, timestamp INTEGER, level TEXT, source TEXT, category TEXT, message TEXT, metadata TEXT, session_id TEXT`
- [ ] `EventStore` struct with `insert(event: &LogEvent)`, `query(filters: EventFilter) -> Vec<LogEvent>`, and `purge(older_than: i64)` methods
- [ ] `EventFilter` struct with optional fields: level, source, category, after_timestamp, before_timestamp, search_text (LIKE on message), limit
- [ ] A background tokio task subscribes to the event bus and inserts events into SQLite
- [ ] Retention policy: auto-purge events older than 7 days on daemon startup
- [ ] Batch inserts: accumulate up to 100 events or 500ms before flushing to SQLite
- [ ] Query API exposed as HTTP endpoint on the health server: `GET /events?source=...&level=...&after=...&limit=...`
**Instructions:**
Create `event_store.rs` with an `EventStore` that opens `.state/events.db` via `rusqlite`. Run `CREATE TABLE IF NOT EXISTS` on init. The background subscriber task receives events from the broadcast channel, batches them, and flushes periodically. Add a `GET /events` route to the health server that delegates to `EventStore::query`.
**ZERO DEFERRALS:** Both insert and query must work. The HTTP query endpoint is required.

---

### TASK-20: Wire all daemon logging to the event bus
**Blocked by:** TASK-18
**Files:**
- `daemon/src/logging.rs` (edit — add a tracing layer that publishes to event bus)
- `daemon/src/main.rs` (edit — pass bus to logging init)
**Acceptance Criteria:**
- [ ] A custom `tracing_subscriber::Layer` that converts tracing events to `LogEvent` and publishes to the event bus
- [ ] The layer extracts `subsystem`, `elapsed_ms`, and other structured fields from tracing events
- [ ] The layer is added to the existing subscriber stack in `logging::init`
- [ ] All existing `tracing::info!` / `tracing::warn!` / `tracing::debug!` calls automatically flow through the event bus without code changes
- [ ] The layer maps tracing levels: TRACE->debug, DEBUG->debug, INFO->info, WARN->warn, ERROR->error
**Instructions:**
In `logging.rs`, create a struct `EventBusLayer` implementing `tracing_subscriber::Layer<S>`. In `on_event`, extract the message and structured fields, construct a `LogEvent`, and call `bus.publish(event)`. Modify `init()` to accept an `Option<Arc<EventBus>>` and add the layer to the subscriber stack when present. In `main.rs`, create the event bus before calling `logging::init` and pass it in.
**ZERO DEFERRALS:** The layer must capture all existing tracing output. Do not require individual call sites to be modified.

---

### TASK-21: Wire Tauri app logging to the event bus via HTTP
**Blocked by:** TASK-18, TASK-19
**Files:**
- `app/src-tauri/src/logging.rs` (edit — add HTTP forwarder layer)
**Acceptance Criteria:**
- [ ] A `tracing_subscriber::Layer` in the app that forwards events to daemon's event bus via HTTP POST to `http://localhost:{daemon_port}/events`
- [ ] The layer is non-blocking: uses `tokio::spawn` for HTTP calls so tracing is never delayed
- [ ] Events are batched: accumulate up to 50 events or 200ms before sending
- [ ] If the daemon is unreachable, events are silently dropped (fire-and-forget)
- [ ] The daemon health server has a `POST /events` endpoint that accepts a batch of `LogEvent` and publishes each to the bus
**Instructions:**
In the app's `logging.rs`, create an `EventForwarderLayer` that accumulates events in a `tokio::sync::mpsc` channel. A background task drains the channel, batches events, and POSTs them to the daemon. Add `POST /events` to the daemon's health server in `health.rs` that deserializes the batch and publishes each event to the bus.
**ZERO DEFERRALS:** Both the app-side forwarder and the daemon-side receiver endpoint are required.

---

### TASK-22: Wire frontend/dev-controller logging to the event bus
**Blocked by:** TASK-18, TASK-19
**Files:**
- `tools/debug/dev.mjs` (edit — forward received logs to daemon event bus)
- `libs/logger/src/index.ts` (edit — forward to daemon event bus endpoint in addition to dashboard)
**Acceptance Criteria:**
- [ ] Dashboard server (`dev.mjs`) forwards all received `/log` POST entries to daemon's `POST /events` endpoint
- [ ] Frontend logger (`index.ts`) forwards to daemon's event bus endpoint (`http://localhost:{daemon_port}/events`) in addition to dashboard
- [ ] Dev controller process output (tailed from log file) is also forwarded to daemon's event bus
- [ ] Each forwarded event includes correct `source` field (Frontend, DevController, etc.)
**Instructions:**
In `dev.mjs`, after parsing the `/log` POST body, also forward to `http://localhost:{daemon_port}/events` via fetch. In the logger, add a second `forwardToDaemonBus()` call alongside `forwardToDashboard()`. For dev controller output, the dashboard already tails the log file (from TASK-05) — add a forwarding step there too.
**ZERO DEFERRALS:** All three sources (frontend SDK, dashboard received logs, controller output) must flow to the event bus.

---

## Phase 4: OrqaDev App

Phase 4 builds the companion app. Tasks 23-30 are sequential for the scaffold and core,
then 31-37 can partially parallelize for views. Tasks 38-42 are integration/polish.

---

### TASK-23: Scaffold OrqaDev as a separate Tauri app
**Blocked by:** TASK-18 (event bus must exist for the app to consume)
**Files:**
- `devtools/src-tauri/Cargo.toml` (create)
- `devtools/src-tauri/src/main.rs` (create)
- `devtools/src-tauri/src/lib.rs` (create)
- `devtools/src-tauri/tauri.conf.json` (create)
- `devtools/src-tauri/build.rs` (create)
- `devtools/src/app.html` (create)
- `devtools/src/routes/+layout.svelte` (create)
- `devtools/package.json` (create)
- `devtools/vite.config.ts` (create)
- `devtools/svelte.config.js` (create)
- `Cargo.toml` (edit — add devtools to workspace members)
**Acceptance Criteria:**
- [ ] `devtools/` directory contains a complete Tauri 2 app scaffold
- [ ] `cargo build -p orqa-devtools` compiles successfully
- [ ] App window opens and renders an empty shell with OrqaDev title
- [ ] Uses `@orqastudio/svelte-components` as a dependency
- [ ] Window is 1200x800 default, resizable, with title "OrqaDev"
- [ ] App has dark theme matching main app's design tokens
- [ ] `devtools/src-tauri/Cargo.toml` depends on `engine/types` (for event types)
**Instructions:**
Create a minimal Tauri 2 app in `devtools/`. Follow the same structure as `app/src-tauri/` but stripped down. The Rust side only needs: tauri setup, a window, and IPC commands for event queries. The frontend uses SvelteKit with the shared component library. Add `"devtools/src-tauri"` to the workspace `members` in the root `Cargo.toml`. Configure `tauri.conf.json` with `identifier: "studio.orqa.devtools"`, window dimensions, and the `custom-protocol` feature.
**ZERO DEFERRALS:** The app must compile, launch, and render. A blank window with correct styling is acceptable.

---

### TASK-24: OrqaDev dual-launch — standalone and from main app toolbar
**Blocked by:** TASK-23
**Files:**
- `cli/src/commands/dev.ts` (edit — launch devtools before other processes)
- `app/src-tauri/src/commands/` — new `devtools_commands.rs` (create)
- `app/src-tauri/src/lib.rs` (edit — register devtools command)
- `libs/svelte-components/src/connected/toolbar/Toolbar.svelte` (edit — add devtools button)
**Acceptance Criteria:**
- [ ] `orqa dev` launches OrqaDev FIRST (before daemon, search, app) so build output is captured
- [ ] OrqaDev binary is found via `findBin("orqa-devtools")`
- [ ] Main app toolbar has a "DevTools" button that launches OrqaDev as a sidecar process
- [ ] If OrqaDev is already running, the toolbar button brings its window to focus instead of launching a second instance
- [ ] `devtools_commands.rs` exposes `launch_devtools` and `is_devtools_running` IPC commands
**Instructions:**
In `dev.ts`, add OrqaDev spawn as step 0 in `startController()`, before daemon start. Use `spawnManaged` with the `orqa-devtools` binary. In `devtools_commands.rs`, implement `launch_devtools` that spawns the binary and `is_devtools_running` that checks if the process is alive. In `Toolbar.svelte`, add a button that calls `invoke("launch_devtools")`. Use a wrench or bug icon from lucide-svelte.
**ZERO DEFERRALS:** Both launch paths (CLI and toolbar) must work. Do not implement only one.

---

### TASK-25: OrqaDev event consumer — SSE client connecting to daemon event bus
**Blocked by:** TASK-23, TASK-19
**Files:**
- `devtools/src-tauri/src/events.rs` (create)
- `devtools/src-tauri/src/lib.rs` (edit — register event commands)
**Acceptance Criteria:**
- [ ] OrqaDev connects to daemon's `GET /events/stream` SSE endpoint on startup
- [ ] Received events are stored in a ring buffer (max 50,000 entries)
- [ ] IPC command `devtools_get_events(filter)` returns filtered events from the buffer
- [ ] IPC command `devtools_subscribe_events()` starts a Tauri event stream that pushes new events to the frontend
- [ ] Daemon health server has a `GET /events/stream` endpoint that sends `LogEvent` as SSE
- [ ] Connection auto-reconnects on disconnect with exponential backoff (1s, 2s, 4s, max 30s)
**Instructions:**
Add `GET /events/stream` to the daemon's health server — subscribe to the event bus and write events as SSE. In OrqaDev's `events.rs`, spawn a tokio task that connects to this SSE endpoint and stores events in a `RwLock<VecDeque<LogEvent>>`. Expose IPC commands for the frontend to query the buffer and subscribe to live updates via Tauri events.
**ZERO DEFERRALS:** Both the daemon SSE endpoint and the OrqaDev consumer must work.

---

### TASK-26: OrqaDev navigation shell — tabs for Logs, Processes, Storybook, Metrics
**Blocked by:** TASK-23
**Files:**
- `devtools/src/routes/+layout.svelte` (edit)
- `devtools/src/lib/components/DevToolsShell.svelte` (create)
- `devtools/src/lib/stores/devtools-navigation.svelte.ts` (create)
**Acceptance Criteria:**
- [ ] Navigation tabs at the top: Logs (default), Processes, Storybook, Metrics
- [ ] Active tab is highlighted with the app's accent color
- [ ] Tab state is managed by a Svelte 5 `$state` store
- [ ] Shell includes a status bar at the bottom showing: daemon connection status, event count, buffer usage
- [ ] Shell uses `@orqastudio/svelte-components` tabs and status components
**Instructions:**
Create `DevToolsShell.svelte` with a `SimpleTabs` component from the shared library. Create a navigation store with `activeTab: "logs" | "processes" | "storybook" | "metrics"`. The status bar at the bottom shows connection info. Wire the `+layout.svelte` to render the shell with a content slot.
**ZERO DEFERRALS:** All four tab labels must be present. Only the Logs tab needs content in this task.

---

### TASK-27: Build virtualised log table component
**Blocked by:** TASK-25, TASK-26
**Files:**
- `devtools/src/lib/components/logs/LogTable.svelte` (create)
- `devtools/src/lib/components/logs/LogRow.svelte` (create)
- `devtools/src/lib/stores/log-store.svelte.ts` (create)
- `devtools/package.json` (edit — add @tanstack/svelte-table dependency)
**Acceptance Criteria:**
- [ ] Log table renders 10,000+ entries without jank (virtualised rows, only visible rows in DOM)
- [ ] Columns: timestamp (HH:MM:SS.mmm), level (color-coded badge), source, category, message
- [ ] Rows are color-coded by level: debug=dim, info=default, warn=yellow, error=red, perf=blue
- [ ] Clicking a row expands to show full metadata JSON with syntax highlighting
- [ ] Table auto-scrolls to bottom for new entries (with a "scroll lock" toggle to pause auto-scroll)
- [ ] Log store receives events from the Tauri event subscription and maintains the display buffer
**Instructions:**
Add `@tanstack/svelte-table` to devtools' dependencies. Create `LogTable.svelte` using TanStack's virtualiser for row virtualisation. Create `LogRow.svelte` for individual row rendering with level-based color coding. Create `log-store.svelte.ts` that subscribes to the Tauri event channel and maintains a `$state` array of log entries. Use `requestAnimationFrame` for batching UI updates.
**ZERO DEFERRALS:** Virtualisation is non-negotiable — direct DOM rendering of 10K rows is unacceptable.

---

### TASK-28: Implement log filtering and search UI
**Blocked by:** TASK-27
**Files:**
- `devtools/src/lib/components/logs/LogFilters.svelte` (create)
- `devtools/src/lib/stores/log-store.svelte.ts` (edit — add filter logic)
**Acceptance Criteria:**
- [ ] Source filter: multi-select dropdown populated from observed sources
- [ ] Level filter: checkbox group for debug/info/warn/error/perf
- [ ] Category filter: multi-select dropdown populated from observed categories
- [ ] Time range filter: start/end datetime pickers
- [ ] Full-text search: input that filters messages by substring match
- [ ] Filters are composable: all active filters apply simultaneously (AND logic)
- [ ] Filter state persists in the log store (not lost on tab switch)
- [ ] Clear all filters button
**Instructions:**
Create `LogFilters.svelte` as a horizontal bar above the log table. Use `SelectMenu` from the shared components for source/category dropdowns. Use checkboxes for level filter. Use `SearchInput` for full-text search. Wire filters into the log store's derived state so filtered entries update reactively.
**ZERO DEFERRALS:** All five filter types must work. Do not implement only text search.

---

### TASK-29: Build process diagnostics view
**Blocked by:** TASK-25, TASK-26
**Files:**
- `devtools/src/lib/components/processes/ProcessView.svelte` (create)
- `devtools/src/lib/components/processes/ProcessCard.svelte` (create)
- `devtools/src-tauri/src/process_status.rs` (create)
**Acceptance Criteria:**
- [ ] Process view shows status cards for: Daemon, MCP server, LSP server, Search server, Sidecar (Claude)
- [ ] Each card shows: process name, status (running/stopped/crashed), PID, uptime, memory usage
- [ ] Status is polled from daemon health endpoint every 2 seconds
- [ ] Cards use color-coded status indicators: green=running, red=crashed, grey=stopped
- [ ] Clicking a card shows recent log entries filtered to that source
- [ ] IPC command `devtools_process_status` fetches status from daemon and returns structured data
**Instructions:**
Create `process_status.rs` with an IPC command that calls the daemon's health endpoint and parses process statuses. Create `ProcessView.svelte` with a grid of `ProcessCard.svelte` components. Each card uses the `DashboardCard` from shared components. Poll status every 2 seconds using `setInterval` in `onMount`.
**ZERO DEFERRALS:** All five process types must have cards. Do not show only daemon status.

---

### TASK-30: Integrate Storybook rendering view
**Blocked by:** TASK-26
**Files:**
- `devtools/src/lib/components/storybook/StorybookView.svelte` (create)
**Acceptance Criteria:**
- [ ] Storybook view embeds a webview/iframe pointing to `http://localhost:6006` (Storybook dev server)
- [ ] When Storybook is not running, shows a message with instructions to start it (`cd libs/svelte-components && npm run storybook`)
- [ ] Connection status indicator shows whether Storybook server is reachable
- [ ] View is wrapped in the shell's content area with no additional chrome
**Instructions:**
Create `StorybookView.svelte` with an iframe pointing to the Storybook dev server URL. Poll the URL with a HEAD request every 5 seconds to determine if Storybook is running. Show an `EmptyState` component from shared library when not reachable. This is a lightweight integration — Storybook is a separate dev server, not embedded.
**ZERO DEFERRALS:** The iframe integration and the "not running" fallback are both required.

---

### TASK-31: Add metrics/performance view
**Blocked by:** TASK-25, TASK-26
**Files:**
- `devtools/src/lib/components/metrics/MetricsView.svelte` (create)
- `devtools/src/lib/components/metrics/TimingChart.svelte` (create)
- `devtools/src/lib/stores/metrics-store.svelte.ts` (create)
**Acceptance Criteria:**
- [ ] Metrics view shows performance graphs for: graph build time, prompt generation time, IPC call durations, search index time
- [ ] Data sourced from `perf`-level events in the event stream
- [ ] Each metric shows: current value, min, max, average, sparkline of last 100 values
- [ ] Uses `Sparkline` component from shared library for mini graphs
- [ ] Error rate panel: errors per minute over last 30 minutes
- [ ] Metrics store aggregates perf events by category and maintains running statistics
**Instructions:**
Create `metrics-store.svelte.ts` that subscribes to the log store, filters for `perf`-level events, and computes running stats per category. Create `MetricsView.svelte` with a dashboard grid of `MetricCell` and `Sparkline` components from the shared library. Create `TimingChart.svelte` for the detailed timing distribution view.
**ZERO DEFERRALS:** At minimum four metric categories must be displayed. The sparklines must render.

---

### TASK-32: Configure log levels — info in production, debug in dev
**Blocked by:** TASK-20, TASK-21
**Files:**
- `daemon/src/logging.rs` (edit)
- `app/src-tauri/src/logging.rs` (edit)
- `libs/logger/src/index.ts` (edit)
**Acceptance Criteria:**
- [ ] Daemon: default log level is `info`; when `ORQA_DEV=true` env var is set, level is `debug`
- [ ] Tauri app: default log level is `info` for the event bus layer; `debug` in dev builds
- [ ] Frontend logger: default level is `info`; `initDevConsole()` sets to `debug`
- [ ] Log level is configurable via `orqa.toml` for daemon (`log_level = "debug"`)
- [ ] The event bus receives ALL events regardless of console log level (filtering is display-side)
**Instructions:**
In daemon `logging.rs`, read `ORQA_DEV` env var and `orqa.toml` `log_level` field to set the tracing filter. The event bus layer should use `LevelFilter::TRACE` to capture everything. The console/file layer uses the configured level. In app `logging.rs`, set the event forwarder layer to `TRACE` and the console layer to the appropriate level. In the frontend logger, the `minLevel` only affects console output; `forwardToDashboard` and `forwardToDaemonBus` always send.
**ZERO DEFERRALS:** The event bus must receive all events. Level filtering is display-side only.

---

### TASK-33: OrqaDev event query from SQLite history
**Blocked by:** TASK-19, TASK-25
**Files:**
- `devtools/src-tauri/src/events.rs` (edit — add history query command)
- `devtools/src/lib/stores/log-store.svelte.ts` (edit — add history loading)
**Acceptance Criteria:**
- [ ] IPC command `devtools_query_history(filter)` queries daemon's `GET /events` HTTP endpoint
- [ ] History results are merged into the log store's buffer with deduplication by event id
- [ ] "Load earlier" button in the log table loads events before the oldest visible event
- [ ] Time range filter can query historical events (not just live buffer)
- [ ] History query returns max 1000 events per request with pagination support
**Instructions:**
In `events.rs`, add a `devtools_query_history` command that calls daemon's `GET /events?...` endpoint with filter params. In the log store, add a `loadHistory(before: timestamp)` method that calls the IPC command and prepends results to the buffer. Add a "Load earlier" button to the log table header that triggers history loading.
**ZERO DEFERRALS:** Both the query command and the UI integration are required.

---

### TASK-34: Dashboard replacement — deprecate dev.mjs in favour of OrqaDev
**Blocked by:** TASK-27, TASK-28, TASK-29
**Files:**
- `tools/debug/dev.mjs` (edit — add deprecation notice, keep functional)
- `cli/src/commands/dev.ts` (edit — spawn OrqaDev instead of dashboard by default)
**Acceptance Criteria:**
- [ ] `orqa dev` spawns OrqaDev instead of `dev.mjs` as the default dashboard
- [ ] `orqa dev --legacy-dashboard` flag still spawns `dev.mjs` as fallback
- [ ] `dev.mjs` prints a deprecation notice on startup: "This dashboard is deprecated. OrqaDev is the new developer tools app."
- [ ] Dashboard POST /log endpoint remains functional (frontend logger still sends there)
- [ ] OrqaDev window opens automatically on `orqa dev` startup
**Instructions:**
In `dev.ts`, change the default dashboard spawn from `dev.mjs` to the `orqa-devtools` binary. Add a `--legacy-dashboard` flag that preserves the old behavior. In `dev.mjs`, add a console deprecation notice. Keep all existing functionality in `dev.mjs` — it's a fallback, not deleted.
**ZERO DEFERRALS:** Both the new default and the legacy flag must work.

---

### TASK-35: SQLite event retention management
**Blocked by:** TASK-19
**Files:**
- `daemon/src/event_store.rs` (edit — retention policy)
- `daemon/src/config.rs` (edit — add retention config)
**Acceptance Criteria:**
- [ ] `DaemonConfig` has `event_retention_days: u32` field (default: 7)
- [ ] On daemon startup, `EventStore::purge(older_than)` runs to remove expired events
- [ ] A background task runs purge every 6 hours
- [ ] Purge logs `tracing::info!` with count of deleted events
- [ ] `orqa.toml` supports `event_retention_days = 14` configuration
**Instructions:**
Add `event_retention_days` to `DaemonConfig`. In `main.rs::run()`, call `event_store.purge()` after creating the store. Spawn a background tokio task with `tokio::time::interval(Duration::from_secs(6 * 3600))` that calls purge periodically.
**ZERO DEFERRALS:** The startup purge and the periodic purge are both required.

---

### TASK-36: End-to-end integration test — log from frontend to OrqaDev
**Blocked by:** TASK-22, TASK-25, TASK-27
**Files:**
- `devtools/__tests__/integration/event-flow.test.ts` (create)
**Acceptance Criteria:**
- [ ] Test starts daemon, starts OrqaDev, emits a log event from a mock frontend source
- [ ] Verifies the event appears in OrqaDev's event buffer via IPC query
- [ ] Verifies the event is persisted in SQLite via daemon's HTTP query endpoint
- [ ] Verifies event has correct timestamp, level, source, and message
- [ ] Test cleans up all processes on completion
**Instructions:**
Create an integration test that spawns the daemon (using `orqa daemon start`), sends a POST to the dashboard `/log` endpoint with a test event, queries the daemon's `/events` endpoint to verify persistence, and queries OrqaDev's event buffer. Use `child_process.spawn` for daemon lifecycle. Use `fetch` for HTTP assertions.
**ZERO DEFERRALS:** The test must verify the complete event flow, not just individual components.

---

### TASK-37: OrqaDev process auto-discovery from daemon health
**Blocked by:** TASK-29
**Files:**
- `daemon/src/health.rs` (edit — add detailed process info to health response)
- `devtools/src/lib/components/processes/ProcessCard.svelte` (edit — show detailed info)
**Acceptance Criteria:**
- [ ] Daemon health endpoint returns per-process info: name, pid, status, uptime_seconds, binary_path
- [ ] OrqaDev process view auto-discovers processes from the health endpoint (no hardcoded list)
- [ ] New processes added to the daemon (e.g., future ONNX server) automatically appear in OrqaDev
- [ ] Process cards show binary path on hover/expand
**Instructions:**
Extend the daemon health response to include a `processes` array with detailed info per managed subprocess. Read from `SubprocessManager` state. In OrqaDev, dynamically generate process cards from the health response instead of a hardcoded list.
**ZERO DEFERRALS:** Auto-discovery is required — do not hardcode process names in OrqaDev.

---

### TASK-38: OrqaDev window state persistence
**Blocked by:** TASK-23
**Files:**
- `devtools/src-tauri/Cargo.toml` (edit — add tauri-plugin-window-state)
- `devtools/src-tauri/src/lib.rs` (edit — register plugin)
**Acceptance Criteria:**
- [ ] OrqaDev window position and size persist across app restarts
- [ ] Active tab selection persists across restarts (stored in local storage)
- [ ] Filter state in the log view persists across restarts (stored in local storage)
**Instructions:**
Add `tauri-plugin-window-state` to devtools dependencies. Register it in `setup_app`. For tab and filter persistence, use browser `localStorage` in the Svelte stores — save on change, restore on init.
**ZERO DEFERRALS:** All three persistence targets (window, tab, filters) must work.

---

### TASK-39: OrqaDev keyboard shortcuts
**Blocked by:** TASK-27, TASK-28
**Files:**
- `devtools/src/routes/+layout.svelte` (edit — global keydown handler)
**Acceptance Criteria:**
- [ ] `Ctrl+F` / `Cmd+F` focuses the full-text search input in log view
- [ ] `Ctrl+L` / `Cmd+L` clears the log display
- [ ] `Ctrl+1` through `Ctrl+4` switch tabs (Logs, Processes, Storybook, Metrics)
- [ ] `Escape` clears active filters
- [ ] Shortcuts only fire when no input/textarea is focused (except Ctrl+F in log view)
**Instructions:**
Add a `keydown` event listener in the layout. Check `e.ctrlKey || e.metaKey` for modified shortcuts. Use `e.target` to check if an input is focused. Dispatch to the appropriate store action for each shortcut.
**ZERO DEFERRALS:** All five shortcuts must work.

---

### TASK-40: OrqaDev connection resilience
**Blocked by:** TASK-25
**Files:**
- `devtools/src-tauri/src/events.rs` (edit — connection management)
- `devtools/src/lib/stores/devtools-navigation.svelte.ts` (edit — connection status display)
**Acceptance Criteria:**
- [ ] OrqaDev reconnects to daemon SSE stream automatically on disconnect
- [ ] Exponential backoff: 1s, 2s, 4s, 8s, 16s, max 30s between reconnection attempts
- [ ] On reconnect, events missed during disconnect are loaded from SQLite history
- [ ] Connection status shown in status bar: "Connected", "Reconnecting (attempt N)", "Disconnected"
- [ ] When daemon is not running, OrqaDev shows a "Waiting for daemon..." state (not an error)
**Instructions:**
In `events.rs`, add reconnection logic to the SSE consumer task. Track the last event timestamp and on reconnect, query `/events?after=<last_timestamp>` to fill the gap. Emit Tauri events for connection state changes. In the navigation store, track connection state and display it in the status bar.
**ZERO DEFERRALS:** Both reconnection and gap-fill from history are required.

---

### TASK-41: OrqaDev export and share functionality
**Blocked by:** TASK-28
**Files:**
- `devtools/src/lib/components/logs/LogExport.svelte` (create)
**Acceptance Criteria:**
- [ ] "Export" button in the log toolbar exports currently filtered logs as JSON file
- [ ] Export includes all fields: timestamp, level, source, category, message, metadata
- [ ] File is saved via Tauri's save dialog with default name `orqadev-logs-{date}.json`
- [ ] Export respects current filters (only exports what's visible)
- [ ] "Copy to clipboard" option for individual log entries (right-click or button)
**Instructions:**
Create `LogExport.svelte` with an export button. Use `@tauri-apps/plugin-dialog` for the save dialog and `@tauri-apps/plugin-fs` for writing the file. Format as pretty-printed JSON array. Add a copy-to-clipboard handler for individual rows via `navigator.clipboard.writeText`.
**ZERO DEFERRALS:** Both file export and clipboard copy must work.

---

### TASK-42: OrqaDev documentation and help panel
**Blocked by:** TASK-26
**Files:**
- `devtools/src/lib/components/help/HelpPanel.svelte` (create)
**Acceptance Criteria:**
- [ ] Help panel accessible via `?` keyboard shortcut or a help icon in the toolbar
- [ ] Shows keyboard shortcuts reference
- [ ] Shows event schema reference (field names and types)
- [ ] Shows filter syntax guide
- [ ] Panel slides in from the right side, closable with Escape or clicking outside
**Instructions:**
Create `HelpPanel.svelte` as a slide-out panel from the right. Use hardcoded content (not fetched from files). Include sections for shortcuts, event schema, and filter syntax. Use the shared `Dialog` component or a custom slide-out panel.
**ZERO DEFERRALS:** All three content sections must be present.

---

## Summary

| Phase | Tasks | Parallel Slots | Dependencies |
|-------|-------|----------------|--------------|
| Phase 1: Quick Wins | 01-05 | 3 parallel (01,02,03), then 04 after 03, 05 after 01 | None |
| Phase 2: Comprehensive Logging | 06-16 | All 11 tasks fully parallel | Phase 1 only for TASK-17 |
| Phase 3: Event Infrastructure | 17-22 | 17 serial, 18 after 17, 19+20 parallel after 18, 21+22 parallel after 18+19 | Phase 2 for TASK-17 |
| Phase 4: OrqaDev App | 23-42 | See dependency graph below | Phase 3 for TASK-23 |

### Phase 4 Dependency Graph

```
TASK-23 (scaffold)
  ├── TASK-24 (dual launch) ── TASK-34 (dashboard replacement)
  ├── TASK-25 (event consumer)
  │     ├── TASK-27 (log table) ── TASK-28 (filters) ── TASK-39 (shortcuts)
  │     │                                             ── TASK-41 (export)
  │     ├── TASK-29 (processes) ── TASK-37 (auto-discovery)
  │     ├── TASK-31 (metrics)
  │     └── TASK-33 (history query)
  ├── TASK-26 (nav shell)
  │     ├── TASK-27, TASK-29, TASK-30 (storybook), TASK-31
  │     └── TASK-42 (help panel)
  ├── TASK-38 (window state)
  └── TASK-40 (connection resilience)

TASK-36 (integration test) ── depends on TASK-22, TASK-25, TASK-27
TASK-34 (dashboard replacement) ── depends on TASK-27, TASK-28, TASK-29
TASK-35 (retention) ── depends on TASK-19 only
```

### Maximum Parallelism by Phase

- **Phase 1**: 3 agents max (tasks 01, 02, 03)
- **Phase 2**: 11 agents max (tasks 06-16 all independent)
- **Phase 3**: 2 agents max (task 19+20 parallel, then 21+22 parallel)
- **Phase 4**: 4-5 agents max (tasks 25+26+38 parallel after 23, then multiple view tasks)
