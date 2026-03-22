---
id: TASK-aadbf15a
title: Unified logging — OrqaDev dashboard + production error surfacing
description: "Two-tier logging: (1) in dev mode, stream info-level logs from backend, sidecar, frontend, and file watchers to the existing OrqaDev web dashboard (port 3001); (2) in all modes, surface errors in the app UI."
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - "DEV MODE: Rust tracing output (info level and above) streamed to OrqaDev dashboard via SSE"
  - "DEV MODE: Sidecar stderr output appears in OrqaDev dashboard log stream"
  - "DEV MODE: Frontend console.log/warn/error forwarded to OrqaDev dashboard"
  - "DEV MODE: File watcher events appear in OrqaDev dashboard"
  - "DEV MODE: OrqaDev dashboard filters include new sources (rust-tracing, sidecar, frontend, watcher)"
  - "DEV MODE: No dev logging infrastructure included in production builds"
  - "ALL MODES: Rust errors (error level) emit events the frontend can display"
  - "ALL MODES: Sidecar errors forwarded and displayed in the app UI"
  - "ALL MODES: Frontend uncaught errors (onerror, unhandledrejection) displayed in the app UI"
  - "ALL MODES: File watcher errors displayed in the app UI"
  - Log entries have source tags and severity levels
relationships:
  - target: EPIC-5f9fcf48
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-389af55e
    type: depended-on-by
---

## What

Two distinct concerns that share infrastructure:

**Concern 1 — Dev logging (dev mode only):** Stream info-level logs from all sources to the
**existing OrqaDev web dashboard** (`debugger/dev-dashboard.html` served on port 3001 by the
dev controller `debugger/dev.mjs`). The dashboard already captures Vite and Cargo stdout/stderr
with source-tagged SSE events. This task extends it to also surface Rust tracing output,
sidecar diagnostics, frontend console output, and file watcher events. No new UI is created
inside the app — the OrqaDev dashboard IS the dev logging surface.

**Concern 2 — Error surfacing (all modes):** In both dev and production, errors from any
source (backend, sidecar, frontend, file watcher) should be surfaced in the app UI. Users
need to see when things go wrong — silent error swallowing is a bug.

### Current state

- **OrqaDev dashboard** (`localhost:3001`): already captures Vite and Cargo process
  stdout/stderr via SSE, with source filters (ctrl/vite/rust) and process controls
- **Rust backend**: `tracing` crate imported with ~104 macro calls but no subscriber —
  all logs silently drop (including errors)
- **Sidecar**: diagnostic output goes to `process.stderr.write()` — visible in terminal
  but not in OrqaDev dashboard or app
- **Frontend**: almost no logging (`console.warn` in 2 places) — errors swallowed silently
- **File watcher**: errors logged via `tracing::warn!` (drops without a subscriber)

### Existing infrastructure (what we build on)

| Component | File | What It Does |
|-----------|------|-------------|
| Dev controller | `debugger/dev.mjs` | HTTP + SSE server on port 3001, manages Vite + Cargo processes |
| Dashboard UI | `debugger/dev-dashboard.html` | Real-time log viewer with source filters and process controls |
| SSE protocol | `GET /events` | Streams `log` events (`{ source, text, error }`) and `status` events |
| Process control | `POST /command/{cmd}` | start, restart-tauri, restart-vite, restart, stop |

## How

### 1. Backend: Wire up tracing subscriber

- Add `tracing-subscriber` to `Cargo.toml`
- **All modes**: configure subscriber that emits `app-error` Tauri events for error-level logs
- **Dev mode**: additionally write info+ logs to stderr (the dev controller already captures
  Cargo process stderr and broadcasts it via SSE — so tracing output will flow through to
  the OrqaDev dashboard automatically)
- Each log event carries: `timestamp`, `level`, `source` (module path), `message`

### 2. Sidecar: Forward stderr to dev controller + app

- The sidecar manager (`backend/src-tauri/src/sidecar/manager.rs`) spawns the sidecar process
- Capture sidecar stderr in the Rust backend:
  - **All modes**: parse for error patterns → emit `app-error` Tauri events
  - **Dev mode**: forward lines to the dev controller's SSE stream (either by writing to
    the Tauri process stderr where the controller captures it, or by POSTing to the
    controller's HTTP endpoint). Add `sidecar` as a new source filter in the dashboard.

### 3. Frontend: Error capture (all modes) + console forwarding (dev only)

- **All modes**: listen for `window.onerror` and `window.onunhandledrejection` → forward
  to an error store that displays in the app UI
- **Dev mode only**: monkey-patch `console.warn`, `console.error`, `console.log` →
  forward to the dev controller via a lightweight HTTP POST or WebSocket to `localhost:3001`.
  Add `frontend` as a new source filter in the dashboard. Gate this behind
  `import.meta.env.DEV` so it's stripped from production.

### 4. File watcher: Already covered

File watcher uses `tracing::warn!` — once the tracing subscriber is wired (step 1), watcher
events flow through automatically. No additional work needed.

### 5. OrqaDev dashboard updates (dev mode only)

- Add new source filter buttons: `sidecar`, `frontend`, `watcher` (alongside existing
  `ctrl`, `vite`, `rust`)
- Optionally add severity-level filtering (info/warn/error) if tracing output includes
  structured levels
- No changes to the dashboard architecture — it already handles SSE log events with
  source tags

### 6. Error display in app UI (all modes)

- Create an error notification/toast system that listens to `app-error` Tauri events
- Shows errors inline (toast or status bar indicator) — lightweight, non-intrusive
- Works in production — no dependency on the dev controller or dashboard

## Verification

### Dev mode
- [ ] Rust tracing output (info+) visible in OrqaDev dashboard at localhost:3001
- [ ] Sidecar stderr output visible in OrqaDev dashboard
- [ ] Frontend console.log/warn/error forwarded to OrqaDev dashboard
- [ ] File watcher events visible in OrqaDev dashboard
- [ ] Dashboard has source filters for all sources (ctrl, vite, rust, sidecar, frontend)
- [ ] No dev logging infrastructure included in production builds

### All modes (dev + production)
- [ ] Rust errors (error level) surfaced in the app UI
- [ ] Sidecar errors surfaced in the app UI
- [ ] Frontend uncaught errors (onerror, unhandledrejection) surfaced in the app UI
- [ ] File watcher errors surfaced in the app UI
- [ ] Error display is non-intrusive (toast/status bar, not a modal)
