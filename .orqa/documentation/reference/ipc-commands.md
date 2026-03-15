---
id: DOC-005
title: IPC Command Catalog
description: Catalog of all Tauri IPC commands defining the frontend-backend communication contract.
created: 2026-03-02
updated: 2026-03-10
sort: 1
relationships:
  - target: RES-007
    type: documents
    rationale: Documentation page references RES-007
  - target: AD-002
    type: documents
    rationale: Documentation page references AD-002
  - target: AD-009
    type: documents
    rationale: Documentation page references AD-009
  - target: AD-003
    type: documents
    rationale: Documentation page references AD-003
  - target: IMPL-001
    type: documents
    rationale: Documentation page references IMPL-001
  - target: EPIC-048
    type: documents
    rationale: Documentation page references EPIC-048
  - type: informed-by
    target: PILLAR-001
    rationale: This document describes architecture or practices that serve the Clarity Through Structure pillar
  - type: informed-by
    target: PILLAR-002
    rationale: This document describes features or patterns that serve the Learning Through Reflection pillar
  - type: informed-by
    target: SKILL-012
    rationale: IPC patterns skill defines the Tauri invoke contract that this command catalog implements
---

**References:** [Tauri v2 Research](RES-007), [AD-002](AD-002), [AD-009](AD-009), Rust Module Architecture

Complete catalog of `#[tauri::command]` functions. Every frontend-to-backend call crosses the IPC boundary through one of these commands. Streaming data uses `Channel<T>` [AD-009](AD-009) rather than `invoke()`.

All commands return `Result<T, OrqaError>` where `OrqaError` is a `thiserror`-derived enum serialized as `{"code": "<variant>", "message": "<detail>"}` [AD-003](AD-003).

There are currently **15 command modules** containing approximately **82 commands** in total.

---

## Error Envelope

Every command returns the same error shape on failure:

```typescript
interface OrqaError {
  code: "not_found" | "database" | "file_system" | "sidecar" | "validation" |
        "scan" | "serialization" | "permission_denied" | "search";
  message: string;
}
```

---

## 1. Project Commands (`project_commands.rs`)

| Command | Signature | Description |
|---------|-----------|-------------|
| `project_open` | `(path: String) -> Project` | Open existing directory as project; loads enforcement engine |
| `project_create` | `(name: String, parent_path: String, init_git: Option<bool>) -> Project` | Create directory with `.orqa/` scaffold, register in DB |
| `project_get` | `(project_id: i64) -> Project` | Fetch project by ID |
| `project_get_active` | `() -> Option<Project>` | Get most recently touched project |
| `project_list` | `() -> Vec<ProjectSummary>` | List all registered projects |

## 2. Session Commands (`session_commands.rs`)

| Command | Signature | Description |
|---------|-----------|-------------|
| `session_create` | `(project_id: i64, model: Option<String>, system_prompt: Option<String>) -> Session` | Create session with model and optional system prompt |
| `session_list` | `(project_id: i64, status: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Vec<SessionSummary>` | List sessions with optional status filter and pagination |
| `session_get` | `(session_id: i64) -> Session` | Fetch session by ID |
| `session_update_title` | `(session_id: i64, title: String) -> ()` | Update session display title |
| `session_end` | `(session_id: i64) -> ()` | Mark session as completed |
| `session_delete` | `(session_id: i64) -> ()` | Delete session and all messages (cascading) |

## 3. Message Commands (`message_commands.rs`)

| Command | Signature | Description |
|---------|-----------|-------------|
| `message_list` | `(session_id: i64, limit: Option<i64>, offset: Option<i64>) -> Vec<Message>` | List messages for a session with pagination |
| `message_search` | `(project_id: i64, query: String, limit: Option<i64>) -> Vec<SearchResult>` | FTS5 full-text search across project messages |

## 4. Stream Commands (`stream_commands.rs`)

These commands drive the AI conversation loop. `stream_send_message` opens a `Channel<StreamEvent>` that forwards streaming events to the frontend.

| Command | Signature | Description |
|---------|-----------|-------------|
| `stream_send_message` | `(session_id: i64, content: String, channel: Channel<StreamEvent>) -> ()` | Write user message to DB, send to sidecar, stream response via Channel |
| `stream_stop` | `(session_id: i64) -> ()` | Cancel the active stream for a session |
| `stream_tool_approval_respond` | `(tool_call_id: String, approved: bool) -> ()` | Send approval decision for a pending tool call requiring user confirmation |
| `system_prompt_preview` | `(project_id: i64, session_id: i64) -> String` | Preview the assembled system prompt for a session without sending it |

## 5. Artifact Commands (`artifact_commands.rs`)

| Command | Signature | Description |
|---------|-----------|-------------|
| `artifact_list` | `(project_id: i64, artifact_type: Option<String>) -> Vec<ArtifactSummary>` | List artifacts, optionally filtered by type |
| `artifact_get` | `(artifact_id: i64) -> Artifact` | Fetch artifact metadata and read file content from disk |
| `artifact_get_by_path` | `(project_id: i64, rel_path: String) -> Artifact` | Get artifact by project ID and relative path |
| `artifact_create` | `(project_id: i64, artifact_type: String, name: String, content: String) -> Artifact` | Write file to disk and index in DB |
| `artifact_update` | `(artifact_id: i64, content: String) -> Artifact` | Write updated content to disk and update DB hash |
| `artifact_delete` | `(artifact_id: i64) -> ()` | Delete artifact file from disk and remove DB record |
| `read_artifact` | `(rel_path: String) -> Artifact` | Read any `.orqa/` artifact by relative path (universal reader) |
| `artifact_scan_tree` | `() -> NavTree` | Scan active project and return unified navigation tree from `project.json` config |
| `artifact_watch_start` | `(project_path: String) -> ()` | Start file-system watcher on `.orqa/` with 500ms debounce; emits `artifact-changed` events |

## 6. Sidecar Commands (`sidecar_commands.rs`)

| Command | Signature | Description |
|---------|-----------|-------------|
| `sidecar_status` | `() -> SidecarStatus` | Return current sidecar process state |
| `sidecar_restart` | `() -> SidecarStatus` | Kill and respawn the sidecar process |

## 7. Settings Commands (`settings_commands.rs`)

Key-value store backed by SQLite. Scope is a string: `"app"` for global settings or `"project:<id>"` for project-level.

| Command | Signature | Description |
|---------|-----------|-------------|
| `settings_get` | `(key: String, scope: Option<String>) -> Option<Value>` | Fetch a setting by key and scope |
| `settings_set` | `(key: String, value: Value, scope: Option<String>) -> ()` | Upsert a setting |
| `settings_get_all` | `(scope: Option<String>) -> HashMap<String, Value>` | Fetch all settings for a scope |

## 8. Project Settings Commands (`project_settings_commands.rs`)

File-based project configuration stored at `.orqa/project.json`. Separate from the SQLite settings table.

| Command | Signature | Description |
|---------|-----------|-------------|
| `project_settings_read` | `(path: String) -> Option<ProjectSettings>` | Read `.orqa/project.json`; returns None if missing |
| `project_settings_write` | `(path: String, settings: ProjectSettings) -> ProjectSettings` | Write project settings to disk |
| `project_icon_upload` | `(project_path: String, source_path: String) -> String` | Copy image to `.orqa/icon.{ext}`; returns filename |
| `project_icon_read` | `(project_path: String, icon_filename: String) -> String` | Read project icon as base64 data URI |
| `project_scan` | `(path: String, excluded_paths: Option<Vec<String>>) -> ProjectScanResult` | Scan project for language, framework, and governance info |

## 9. Search Commands (`search_commands.rs`)

DuckDB-backed code search with ONNX semantic embeddings. All commands are `async`.

| Command | Signature | Description |
|---------|-----------|-------------|
| `index_codebase` | `(project_path: String, excluded_paths: Vec<String>) -> IndexStatus` | Index project files into DuckDB for code search |
| `search_regex` | `(pattern: String, path: Option<String>, max_results: Option<u32>) -> Vec<SearchResult>` | Regex search across indexed codebase |
| `search_semantic` | `(query: String, max_results: Option<u32>) -> Vec<SearchResult>` | Semantic similarity search using ONNX embeddings |
| `get_index_status` | `(project_path: String) -> IndexStatus` | Get codebase index statistics; auto-loads DB if on disk |
| `init_embedder` | `(model_dir: String) -> ()` | Initialize ONNX embedding model, downloading if needed |
| `get_startup_status` | `() -> StartupSnapshot` | Get status of all async startup tasks |

## 10. Theme Commands (`theme_commands.rs`)

| Command | Signature | Description |
|---------|-----------|-------------|
| `theme_get_project` | `(project_id: i64) -> ResolvedTheme` | Merge extracted theme tokens with user overrides |
| `theme_set_override` | `(project_id: i64, token_name: String, value_light: String, value_dark: Option<String>) -> ()` | Create or update a manual theme token override |
| `theme_clear_overrides` | `(project_id: i64) -> ()` | Clear all manual theme overrides for a project |

## 11. Setup Commands (`setup_commands.rs`)

First-run setup wizard. Checks prerequisites before the app is usable.

| Command | Signature | Description |
|---------|-----------|-------------|
| `get_setup_status` | `() -> SetupStatus` | Query setup completion state and current step statuses |
| `check_claude_cli` | `() -> ClaudeCliInfo` | Check whether Claude CLI is installed and retrieve version info |
| `check_claude_auth` | `() -> ClaudeCliInfo` | Check whether Claude CLI is authenticated |
| `reauthenticate_claude` | `() -> ClaudeCliInfo` | Trigger Claude CLI login flow |
| `check_embedding_model` | `(app_handle: AppHandle) -> SetupStepStatus` | Check whether bge-small-en-v1.5 ONNX model is downloaded |
| `complete_setup` | `() -> ()` | Mark setup as complete by storing version in settings |

## 12. Lesson Commands (`lesson_commands.rs`)

Lesson management backed by file-based storage in `.orqa/process/lessons/`. Lessons are stored as `IMPL-NNN.md` files.

| Command | Signature | Description |
|---------|-----------|-------------|
| `lessons_list` | `(project_path: String) -> Vec<Lesson>` | List all lessons from `.orqa/process/lessons/` |
| `lessons_get` | `(project_path: String, id: String) -> Lesson` | Get a single lesson by ID (e.g. `[IMPL-001](IMPL-001)`) |
| `lessons_create` | `(project_path: String, title: String, category: String, body: String) -> Lesson` | Create a new lesson with auto-assigned ID |
| `lesson_increment_recurrence` | `(project_path: String, id: String) -> Lesson` | Increment recurrence count; used by review agents |
| `lessons_scan_promotions` | `(project_path: String) -> Vec<Lesson>` | Return lessons with recurrence >= 2 that have not yet been promoted |

## 13. Governance Commands (`governance_commands.rs`)

Governance analysis using Claude. Scans filesystem for governance files, sends to Claude for analysis, persists recommendations.

| Command | Signature | Description |
|---------|-----------|-------------|
| `governance_scan` | `(project_id: i64) -> GovernanceScanResult` | Walk filesystem and collect governance files |
| `governance_analyze` | `(project_id: i64, scan_result: GovernanceScanResult) -> GovernanceAnalysis` | Send scan to Claude, parse response, persist analysis |
| `governance_analysis_get` | `(project_id: i64) -> Option<GovernanceAnalysis>` | Get the latest governance analysis for a project |
| `recommendations_list` | `(project_id: i64) -> Vec<Recommendation>` | List all recommendations for a project |
| `recommendation_update` | `(id: i64, status: String) -> Recommendation` | Update status of a recommendation (pending / approved / rejected / applied) |
| `recommendation_apply` | `(id: i64) -> Recommendation` | Write approved recommendation to disk and mark applied |
| `recommendations_apply_all` | `(project_id: i64) -> Vec<Recommendation>` | Apply all approved recommendations for a project |

## 14. Enforcement Commands (`enforcement_commands.rs`)

Rule enforcement engine. Rules are loaded from `.orqa/rules/` on project open and can be reloaded without restart.

| Command | Signature | Description |
|---------|-----------|-------------|
| `enforcement_rules_list` | `() -> Vec<EnforcementRule>` | List enforcement rules currently loaded for the active project |
| `enforcement_rules_reload` | `() -> usize` | Reload enforcement engine from `.orqa/rules/`; returns count of rules loaded |
| `enforcement_scan_governance` | `() -> Vec<ScanFinding>` | Scan governance files for violations against all `event: scan` entries |

## 15. Graph Commands (`graph_commands.rs`)

Bidirectional artifact reference graph. Built lazily from disk, cached in `AppState`, invalidated by the file watcher when `.orqa/` changes.

| Command | Signature | Description |
|---------|-----------|-------------|
| `resolve_artifact` | `(id: String) -> Option<ArtifactNode>` | Resolve artifact by ID (e.g. `[EPIC-048](EPIC-048)`) |
| `resolve_artifact_path` | `(path: String) -> Option<ArtifactNode>` | Resolve artifact by relative file path |
| `get_references_from` | `(id: String) -> Vec<ArtifactRef>` | Get all forward references (outgoing edges) from an artifact |
| `get_references_to` | `(id: String) -> Vec<ArtifactRef>` | Get all backlinks (incoming edges) to an artifact |
| `get_artifacts_by_type` | `(artifact_type: String) -> Vec<ArtifactNode>` | Get all artifact nodes of a given type (e.g. `"epic"`, `"task"`) |
| `read_artifact_content` | `(path: String) -> String` | Read raw markdown body of artifact file from disk |
| `get_graph_stats` | `() -> GraphStats` | Return summary statistics about the artifact graph |
| `refresh_artifact_graph` | `() -> ()` | Rebuild artifact graph from disk and replace cached copy |

---

## Streaming Protocol

`stream_send_message` uses `Channel<StreamEvent>` rather than `invoke()`. The frontend registers an `onChannelMessage` callback that receives `StreamEvent` values as they arrive.

```
AI Provider (SSE) → TypeScript sidecar → NDJSON stdout → Rust StreamHandler
  → DB write (buffered, ~500ms flush) + channel.send(event)
    → Tauri Channel<StreamEvent> → Svelte onChannelMessage
```

Tool calls that require approval park on a sync channel. The frontend calls `stream_tool_approval_respond` to unblock the stream loop.

---

## Related Documents

- Rust Module Architecture — module structure, AppState, dependency graph
- [AD-002](AD-002) — IPC boundary principle
- [AD-009](AD-009) — streaming via Channel\<T\>
- [AD-003](AD-003) — error propagation via Result + thiserror

---
