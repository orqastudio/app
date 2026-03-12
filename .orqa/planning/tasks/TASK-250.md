---
id: TASK-250
title: "Unified dev logging — stream all info/error logs to OrqaDev Compose UI"
description: "In dev mode, surface info-level logs from Rust backend, Svelte frontend, sidecar, and file watchers in a single OrqaDev log panel. Include build errors, runtime errors, and browser console errors."
status: todo
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-055
acceptance:
  - "Rust tracing output (info level and above) appears in the OrqaDev log panel in real-time"
  - "Sidecar stderr output appears in the OrqaDev log panel"
  - "Frontend console.warn/error calls are captured and forwarded to the log panel"
  - "Vite build errors and Cargo build errors are surfaced in the log panel"
  - "File watcher events appear in the log panel"
  - "Log entries have source tags (rust, sidecar, frontend, vite, cargo) and severity levels"
  - "Log panel is only active in dev mode — not present in production builds"
  - "Logs are filterable by source and severity"
---

## What

Currently logging is fragmented:
- **Rust backend**: `tracing` crate is imported with ~104 macro calls but no subscriber is
  configured — all logs silently drop
- **Sidecar**: diagnostic output goes to `process.stderr.write()` — visible only if you
  watch the sidecar process stderr manually
- **Frontend**: almost no logging (`console.warn` in 2 places) — errors swallowed silently
- **File watcher**: errors logged via `tracing::warn!` (which drops without a subscriber)
- **Build errors**: Vite and Cargo errors only visible in the terminal running `make dev`

In dev mode, all of these should stream into a unified log panel in the app UI so the
developer (or the orchestrating agent) can see what's happening across the entire system.

## How

### 1. Backend: Wire up tracing subscriber

- Add `tracing-subscriber` to `Cargo.toml`
- In dev mode, configure a subscriber that:
  - Writes to stderr (for terminal visibility)
  - Emits structured log events via a Tauri event channel (`dev-log`)
  - Filters at `info` level by default (configurable)
- Each log event carries: `timestamp`, `level`, `source` (module path), `message`

### 2. Sidecar: Forward stderr to backend

- The sidecar manager (`src-tauri/src/sidecar/manager.rs`) already spawns the sidecar process
- Capture sidecar stderr and emit each line as a `dev-log` event with `source: "sidecar"`
- Parse sidecar output for error patterns to tag severity

### 3. Frontend: Capture console output

- In dev mode, monkey-patch `console.warn`, `console.error`, `console.log` (info level)
- Forward captured entries to a Svelte store
- Also listen for `window.onerror` and `window.onunhandledrejection` for uncaught errors
- Tag entries with `source: "frontend"`

### 4. Build errors: Capture from dev controller

- The dev controller (`scripts/dev.mjs`) manages Vite and Cargo processes
- In dev mode, parse their stderr for error patterns
- Forward build errors to the Tauri app via a mechanism TBD (could be a file watch on
  a dev log file, or IPC if the controller is a sidecar)

### 5. OrqaDev Compose UI

- Create a `DevLogPanel` component that:
  - Listens to `dev-log` Tauri events
  - Displays log entries in a scrollable, auto-tailing list
  - Shows source tag, severity icon, timestamp, message
  - Supports filtering by source (rust/sidecar/frontend/vite/cargo) and severity
  - Only mounts when `dev` mode is detected (not in production builds)
- Integrate into the app layout (collapsible bottom panel or separate compose view)

## Verification

- [ ] Rust tracing output (info level and above) appears in the OrqaDev log panel in real-time
- [ ] Sidecar stderr output appears in the OrqaDev log panel
- [ ] Frontend console.warn/error calls are captured and forwarded to the log panel
- [ ] Vite build errors and Cargo build errors are surfaced in the log panel
- [ ] File watcher events appear in the log panel
- [ ] Log entries have source tags (rust, sidecar, frontend, vite, cargo) and severity levels
- [ ] Log panel is only active in dev mode — not present in production builds
- [ ] Logs are filterable by source and severity
