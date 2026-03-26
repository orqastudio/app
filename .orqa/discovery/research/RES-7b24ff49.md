---
id: "RES-7b24ff49"
type: research
title: "Tauri v2 Research"
description: "Tauri v2's capabilities, security model, and plugin ecosystem for OrqaStudio's requirements."
status: completed
created: "2026-03-02"
updated: "2026-03-07"
relationships:
  - target: "AD-39e2fb81"
    type: "informs"
  - target: "AD-d01b9e0a"
    type: "informs"
  - target: "AD-5d0f8814"
    type: "informs"
---
**Date:** 2026-03-02 | **Status:** Complete

Research into Tauri v2's capabilities, security model, and plugin ecosystem for OrqaStudio's requirements.

---

## Open Questions

### Q1: Capability Audit

**Question:** Does Tauri v2 cover all of OrqaStudio's requirements?

**Verdict: ALL eight requirements are supported.**

| Requirement | Supported | Mechanism |
|-------------|-----------|-----------|
| File system read/write (arbitrary paths) | **Yes** | `tauri-plugin-fs` with scope config; `$HOME/**` or runtime `FsScope::allow_directory()` |
| Process spawning (git, shell, Python) | **Yes** | `tauri-plugin-shell` with scoped command allowlist |
| File watching | **Yes** | Built into `tauri-plugin-fs` via `watch()` / `watchImmediate()` (uses `notify` crate) |
| System tray | **Yes** | Core feature via `tray-icon` Cargo feature flag |
| Auto-update | **Yes** | `tauri-plugin-updater` (v2.9.0) |
| Cross-platform builds | **Yes** | Windows, macOS, Linux; `tauri-action` GitHub Action for CI/CD |
| SSE/HTTP streaming from Rust | **Yes** | `tauri::ipc::Channel<T>` for ordered streaming; `reqwest` for HTTP |
| Multi-window support | **Yes** | Core feature; `WebviewWindow::new()` or JS `WebviewWindow` API |

**Detailed notes:**

- **File system:** `tauri-plugin-fs` (v2.4.5) supports reading/writing outside the app directory. Scopes configured in `backend/src-tauri/capabilities/default.json`. Runtime scope expansion via `app_handle.fs_scope().allow_directory()`. Path traversal blocked. `tauri-plugin-persisted-scope` remembers user-selected directories across restarts.
- **Process spawning:** `tauri-plugin-shell` (v2.3.4) provides `execute()` (blocking) and `spawn()` (non-blocking with streaming stdout/stderr). Commands must be pre-declared in capabilities file with regex arg validators. Sidecar binary support included.
- **File watching:** Built into `tauri-plugin-fs` (requires `watch` Cargo feature). `watch()` for debounced events, `watchImmediate()` for immediate callbacks. Events: access, create, modify, remove. Recursive watching supported. Underlying engine is the Rust `notify` crate.
- **System tray:** Enabled via `features = ["tray-icon"]`. Supports custom icons, menus, and events (Click, DoubleClick, Enter, Move, Leave). Linux has limited event support.
- **Auto-update:** `tauri-plugin-updater` supports GitHub releases, custom endpoints, and CrabNebula Cloud. Known issue: slow Windows uninstalls during updates (5-30 min in some cases).
- **Cross-platform:** `tauri-action` builds for Windows (.msi, .exe), macOS (.dmg, .app), Linux (.deb, .AppImage). x86_64 and aarch64 supported.
- **Streaming:** `Channel<T>` is the documented recommended mechanism — faster than events, ordered delivery, type-safe on Rust side. Perfect for Claude API SSE responses.
- **Multi-window:** Stable with single-webview-per-window. Multi-webview per window is unstable/behind feature flag.

**Decision:** Tauri v2 is fully capable. No blockers. Proceed with [AD-7121ec20](AD-7121ec20) (thick backend) and [AD-4e7faf0e](AD-4e7faf0e) (IPC boundary) as designed.

---

### Q2: IPC Design

**Question:** How should the Tauri IPC boundary be designed for OrqaStudio's needs?

**Findings:**

Three IPC mechanisms available, each with a specific use case:

#### `invoke()` — Request/Response (Frontend → Backend)

Async function returning a Promise. JSON-RPC-like protocol internally. Arguments must be JSON-serializable (camelCase on JS side, snake_case on Rust side — Tauri handles conversion).

```typescript
// Frontend
const result = await invoke<Session>('create_session', { projectPath: '/some/path' });
```

```rust
// Backend
#[tauri::command]
async fn create_session(project_path: String) -> Result<Session, String> { ... }
```

**Type safety:** `invoke()` is NOT type-safe at the JS/TS boundary — returns `Promise<unknown>` by default. Must cast or use generics. Community tool [TauRPC](https://lib.rs/crates/taurpc) generates TypeScript types from Rust command signatures.

**Error handling:** Commands return `Result<T, E>` where E must implement `serde::Serialize`. Rejected promise contains serialized error.

#### `Channel<T>` — Streaming (Backend → Frontend) — RECOMMENDED FOR STREAMING

The **preferred mechanism for streaming data**. Faster than events, ordered delivery (index-based), type-safe on Rust side.

```rust
use tauri::ipc::Channel;

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
enum StreamEvent {
    Chunk { text: String },
    ToolUse { name: String, input: serde_json::Value },
    Done,
    Error { message: String },
}

#[tauri::command]
async fn stream_response(prompt: String, on_event: Channel<StreamEvent>) -> Result<(), String> {
    // Make streaming HTTP request to Claude API
    // For each SSE chunk:
    on_event.send(StreamEvent::Chunk { text: chunk_text }).unwrap();
    on_event.send(StreamEvent::Done).unwrap();
    Ok(())
}
```

```typescript
// Frontend
const channel = new Channel<StreamEvent>();
channel.onmessage = (event) => {
  if (event.event === 'Chunk') appendToUI(event.data.text);
};
await invoke('stream_response', { prompt: 'Hello', onEvent: channel });
```

#### `emit()` / `listen()` — Events (Backend → Frontend)

Fire-and-forget. Not type-safe, cannot return values. Best for simple notifications (app lifecycle, user actions). **Not designed for high-throughput or low-latency.**

#### Payload Size

No documented hard limits. ~200ms for 3MB JSON via standard IPC. For large binary data, use `tauri::ipc::Response` (bypasses JSON) or Channel with chunked delivery.

**Proposed IPC patterns for OrqaStudio:**

| Pattern | Use Case | Mechanism |
|---------|----------|-----------|
| CRUD operations (sessions, projects, settings) | One-shot queries | `invoke()` → `Result<T>` |
| Claude API streaming (tokens, tool calls) | High-throughput ordered stream | `Channel<StreamEvent>` |
| Scanner progress, git operations | Long-running progress | `Channel<ProgressEvent>` |
| File change notifications | Background events | `emit()` / `listen()` |
| App lifecycle (window close, tray click) | Simple notifications | `emit()` / `listen()` |

**Decision:** Use `invoke()` for CRUD, `Channel<T>` for streaming (Claude responses, progress), events for background notifications. Evaluate TauRPC for end-to-end type safety during Phase 1. → Extends [AD-4e7faf0e](AD-4e7faf0e).

---

### Q3: Security Model

**Question:** How should Tauri's permission system be configured for OrqaStudio?

**Findings:**

Three-layer security model:

1. **Permissions** — On/off toggles for individual commands (e.g., `fs:allow-read`)
2. **Scopes** — Parameter validation constraining what a permitted command can access (e.g., which paths)
3. **Capabilities** — Bundles of permissions+scopes assigned to specific windows

Capabilities are defined as JSON/TOML files in `backend/src-tauri/capabilities/`, compiled into the binary at build time. **Deny always takes precedence over allow.**

**Recommended capability configuration for OrqaStudio:**

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-capability",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:default",
    "fs:allow-read",
    "fs:allow-write",
    {
      "identifier": "fs:scope",
      "allow": ["$HOME/**"],
      "deny": ["$HOME/.ssh/**", "$HOME/.gnupg/**"]
    },
    {
      "identifier": "shell:allow-execute",
      "allow": [
        { "name": "git", "cmd": "git", "args": [{ "validator": ".*" }], "sidecar": false },
        { "name": "sh", "cmd": "sh", "args": ["-c", { "validator": ".*" }], "sidecar": false }
      ]
    },
    "shell:allow-spawn",
    "shell:allow-open"
  ]
}
```

**API key storage:** Use `tauri-plugin-keyring` for OS keychain integration (macOS Keychain, Windows Credential Manager, Linux Secret Service):

```typescript
import { setPassword, getPassword } from 'tauri-plugin-keyring-api';
await setPassword('orqa-studio', 'claude-api-key', apiKey);
const key = await getPassword('orqa-studio', 'claude-api-key');
```

**CSP:** Default CSP is sufficient if all API calls route through the Rust backend (recommended). Frontend only talks to backend via IPC. If frontend needs direct external access, add `connect-src` entries in `tauri.conf.json`.

**Project directory selection:** Use dialog plugin for initial project folder selection (auto-expands scope) + `tauri-plugin-persisted-scope` to remember across restarts.

**Decision:** Scoped permissions with `$HOME/**` base, sensitive path denials, pre-declared shell commands, keyring for API keys. → Needs new AD [AD-09fc4e65](AD-09fc4e65).

---

### Q4: Plugin Ecosystem

**Question:** Which Tauri plugins should OrqaStudio use?

**All plugins are official, stable, and maintained in [tauri-apps/plugins-workspace](https://github.com/tauri-apps/plugins-workspace).** Tauri core is at v2.10.2 (Feb 2026). Minimum Rust version: 1.77.2.

| Plugin | Version | Use in OrqaStudio | Notes |
|--------|---------|-------------|-------|
| `tauri-plugin-sql` | 2.3.1 | SQLite for sessions, metrics, tasks | Enable `--features sqlite`. Runs migrations. For complex queries, use `sqlx` in Rust directly. |
| `tauri-plugin-fs` | 2.4.5 | Read/write project files, watch for changes | Enable `watch` feature. Runtime scope expansion. Path traversal blocked. |
| `tauri-plugin-shell` | 2.3.4 | Git, shell commands, scanner execution | Pre-declare commands with arg validators. `spawn()` for streaming stdout. |
| `tauri-plugin-store` | 2.4.2 | App preferences, UI state, recent projects | **Not encrypted** — never use for secrets. |
| `tauri-plugin-autostart` | 2.5.1 | Optional — launch at system startup | Desktop only. Simple enable/disable API. |
| `tauri-plugin-updater` | 2.9.0 | Auto-update via GitHub Releases | Known issue: slow Windows uninstalls. |
| `tauri-plugin-window-state` | 2.4.1 | Persist window size/position | Automatic — just register the plugin. |
| `tauri-plugin-dialog` | 2.4.2 | File/folder selection, confirmation dialogs | Selected paths auto-added to fs scope. |
| `tauri-plugin-notification` | 2.3.3 | System notifications for completed tasks | Requires permission request on some platforms. |
| `tauri-plugin-keyring` | community | Store Claude API key in OS keychain | Not official but well-maintained. |
| `tauri-plugin-persisted-scope` | official | Remember file system permissions across restarts | Pairs with fs plugin. |

**Gotchas:**
- Shell plugin arg validators: `".*"` allows any arg but weakens security — tighten for production
- FS scope on first launch: no paths allowed until user selects a directory (use dialog plugin)
- SQL plugin migrations: for complex schemas, manage via `sqlx` in Rust rather than JS API
- Store plugin is for key-value preferences only, not relational data

**Decision:** Use all listed plugins. `tauri-plugin-sql` for persistence, `tauri-plugin-keyring` for API keys, `tauri-plugin-fs` with watch for file ops, `tauri-plugin-shell` for git/shell. → Confirm during Phase 1 scaffold.

---

## Summary

Tauri v2 is fully capable for OrqaStudio. No blockers, no workarounds needed. The key architectural patterns:

- **Claude streaming:** `reqwest` SSE → `Channel<T>` → Svelte store
- **File operations:** `tauri-plugin-fs` with runtime scope expansion + persisted scope
- **Git/Shell:** `tauri-plugin-shell` with `spawn()` for streaming stdout
- **API key storage:** `tauri-plugin-keyring` (OS keychain)
- **Database:** `tauri-plugin-sql` with SQLite
- **IPC pattern:** `invoke()` for CRUD, `Channel<T>` for streaming, events for notifications

## Related

- Architecture Decisions — [AD-7121ec20](AD-7121ec20), [AD-4e7faf0e](AD-4e7faf0e) confirmed viable; [AD-09fc4e65](AD-09fc4e65) (security model) needed
- [Claude Integration Research](RES-b666c725) — Streaming pipeline confirmed: reqwest → Channel → Svelte
- [Persistence Research](RES-ac474863) — tauri-plugin-sql confirmed for SQLite