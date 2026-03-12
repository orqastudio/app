---
id: TASK-250
title: "Unified logging — dev log panel + production error surfacing"
description: "Two-tier logging: (1) in dev mode, stream info-level logs from all sources to an OrqaDev log panel; (2) in all modes, surface errors from backend, sidecar, frontend, and file watchers in the app UI."
status: todo
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-055
acceptance:
  - "DEV MODE: Rust tracing output (info level and above) appears in the OrqaDev log panel in real-time"
  - "DEV MODE: Sidecar stderr output appears in the OrqaDev log panel"
  - "DEV MODE: Frontend console.log/warn/error captured and forwarded to the log panel"
  - "DEV MODE: Vite and Cargo build errors surfaced in the log panel"
  - "DEV MODE: File watcher events appear in the log panel"
  - "DEV MODE: Log panel is filterable by source and severity"
  - "DEV MODE: Log panel only present in dev builds — stripped from production"
  - "ALL MODES: Rust errors (error level) emit events the frontend can display"
  - "ALL MODES: Sidecar errors forwarded and displayed in the app UI"
  - "ALL MODES: Frontend uncaught errors (onerror, unhandledrejection) displayed in the app UI"
  - "ALL MODES: File watcher errors displayed in the app UI"
  - "Log entries have source tags (rust, sidecar, frontend, vite, cargo) and severity levels"
---

## What

Two distinct concerns that share infrastructure:

**Concern 1 — Dev logging (dev mode only):** In dev mode, stream info-level logs from all
sources into a unified OrqaDev log panel. This is for the developer (or orchestrating agent)
to see what's happening across the entire system while building the app. The dev log panel
and info-level streaming are stripped from production builds entirely.

**Concern 2 — Error surfacing (all modes):** In both dev and production, errors from any
source (backend, sidecar, frontend, file watcher) should be surfaced in the app UI. Users
and developers need to see when things go wrong — silent error swallowing is a bug.

### Current state

- **Rust backend**: `tracing` crate imported with ~104 macro calls but no subscriber —
  all logs silently drop (including errors)
- **Sidecar**: diagnostic output goes to `process.stderr.write()` — invisible to the app
- **Frontend**: almost no logging (`console.warn` in 2 places) — errors swallowed silently
- **File watcher**: errors logged via `tracing::warn!` (which drops without a subscriber)
- **Build errors**: Vite and Cargo errors only visible in the terminal running `make dev`

## How

### 1. Backend: Wire up tracing subscriber (both modes)

- Add `tracing-subscriber` to `Cargo.toml`
- **Production**: configure subscriber at `error` level only, emitting `app-error` Tauri events
- **Dev mode**: configure subscriber at `info` level, emitting both:
  - `app-error` events (for the error UI — same as production)
  - `dev-log` events (for the dev log panel — all info+ entries)
  - Also write to stderr for terminal visibility
- Each log event carries: `timestamp`, `level`, `source` (module path), `message`

### 2. Sidecar: Forward stderr to backend (both modes)

- The sidecar manager already spawns the sidecar process
- Capture sidecar stderr and:
  - **All modes**: parse for error patterns → emit `app-error` events
  - **Dev mode**: also emit every line as `dev-log` with `source: "sidecar"`

### 3. Frontend: Error capture (both modes) + console capture (dev only)

- **All modes**: listen for `window.onerror` and `window.onunhandledrejection` → forward
  to an error store that displays in the app UI
- **Dev mode only**: monkey-patch `console.warn`, `console.error`, `console.log` → forward
  to the dev log store. Strip this patching from production builds.

### 4. Build errors: Dev controller integration (dev mode only)

- The dev controller (`scripts/dev.mjs`) manages Vite and Cargo processes
- Parse their stderr for error patterns → forward to the Tauri app as `dev-log` events
- This is inherently dev-only — the controller doesn't exist in production

### 5. Error display UI (both modes)

- Create an error notification/toast system that listens to `app-error` events
- Shows errors inline (toast or status bar indicator) without requiring a special panel
- Works in production — lightweight, non-intrusive

### 6. OrqaDev log panel (dev mode only)

- Create a `DevLogPanel` component that:
  - Listens to `dev-log` Tauri events
  - Displays log entries in a scrollable, auto-tailing list
  - Shows source tag, severity icon, timestamp, message
  - Supports filtering by source (rust/sidecar/frontend/vite/cargo) and severity
- Only compiled into dev builds — use `import.meta.env.DEV` to gate the component
- Integrate into the app layout (collapsible bottom panel or separate compose view)

## Verification

### Dev mode
- [ ] Rust tracing output (info level and above) appears in the OrqaDev log panel
- [ ] Sidecar stderr output appears in the OrqaDev log panel
- [ ] Frontend console.log/warn/error captured and forwarded to the log panel
- [ ] Vite and Cargo build errors surfaced in the log panel
- [ ] File watcher events appear in the log panel
- [ ] Log panel is filterable by source and severity
- [ ] Log panel only present in dev builds — stripped from production

### All modes (dev + production)
- [ ] Rust errors (error level) emit events the frontend can display
- [ ] Sidecar errors forwarded and displayed in the app UI
- [ ] Frontend uncaught errors (onerror, unhandledrejection) displayed in the app UI
- [ ] File watcher errors displayed in the app UI
- [ ] Log entries have source tags and severity levels
