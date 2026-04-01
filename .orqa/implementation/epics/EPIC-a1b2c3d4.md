---
id: EPIC-a1b2c3d4
type: epic
title: "OrqaDev: Comprehensive Logging & Developer Tools"
description: "Embed a full logging and debugging platform into the main OrqaStudio app. Pub/sub event delivery, persistence, virtualised log viewer, storybook, and process diagnostics — available in both production and dev mode via a toolbar link."
status: captured
created: "2026-04-01"
updated: "2026-04-01"
relationships:
  - target: MS-21d5096a
    type: fulfils
  - target: IDEA-d9e70a44
    type: realises
  - target: IDEA-30d6a2f9
    type: realises
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

# OrqaDev: Comprehensive Dev Tooling & Logging Platform

## Vision

Build a comprehensive logging and developer tools app that runs as a separate window, launched from the main OrqaStudio toolbar. Packaged alongside the main app in both production and dev builds — not a dev-only tool. Available anytime for live process debugging.

## Scope

### 1. Logging Infrastructure

**Pub/Sub Event Bus**
- All processes (daemon, app, MCP, LSP, search, dev controller) publish structured log events to a central event bus
- Guaranteed delivery — no dropped logs from race conditions or process crashes
- Structured event schema: timestamp, level, source, category, message, metadata

**Persistence Layer**
- Log events persisted to a database (evaluate: SQLite for simplicity vs MongoDB for query flexibility)
- Queryable history — search past sessions, filter by source/level/category
- Retention policy — auto-purge after configurable period

**Frontend Console Forwarding**
- Browser console.log/warn/error intercepted and forwarded to the event bus
- Svelte component warnings and unhandled promise rejections captured
- Source-mapped stack traces where possible

### 2. OrqaDev Companion App

**Architecture**
- Separate Tauri app consuming the same SSE/pub-sub feed as the main app
- Uses `@orqastudio/svelte-components` shared component library
- Launches FIRST before all other processes so build output is captured from the start

**Log Viewer**
- Virtualised table (tanstack-table or similar) for thousands of log entries
- Column-based filtering: source, level, category, timestamp range
- Full-text search across log messages
- Syntax-highlighted JSON metadata expansion
- Color-coded severity levels

**Navigation**
- Logs view (default) — the virtualised log table
- Processes view — status cards for each managed process with start/stop/restart
- Storybook view — embedded rendering of `@orqastudio/svelte-components` for component development
- Metrics view — performance graphs, timing distributions, error rates

### 3. Process Management

- Dashboard launches FIRST in the dev controller startup sequence
- Process stdout/stderr piped to the event bus with source tagging
- Build output (cargo compile, Vite, tsc) captured and displayed in real-time
- Process crash/restart events as first-class log entries

### 4. Storybook Integration

- Render `@orqastudio/svelte-components` in a dedicated view
- Component playground for testing pure and connected components
- Props editor for interactive component exploration

## Delivery Tasks

### Phase 1: Quick Wins (unblock dogfooding)
1. Fix dev controller startup order — dashboard launches FIRST before any process
2. Fix `errorStore.initBrowserHandlers()` — never called, window.onerror is dead
3. Fix duplicate SDK log forwarding (logger + dev-console both send)
4. Add browser console.log/warn/error intercept in dev mode → forward to dashboard
5. Forward process stdout/stderr from dev controller to dashboard SSE stream

### Phase 2: Comprehensive Logging (100+ gaps from research)
6. Engine timing spans — `build_artifact_graph`, `validate`, `index`, `embed_chunks`
7. Engine error surfacing — silent I/O failures in `walk_directory`, YAML parse failures
8. Daemon handler timing — prompt, knowledge, context endpoints + ONNX operations
9. Daemon lifecycle — config load warnings, generator timing, watcher events
10. Tauri IPC timing — every `#[tauri::command]` entry/exit with duration
11. Tauri lifecycle — session create/end, plugin install/uninstall, sidecar PID
12. Frontend SDK — graph init timing, navigation events, IPC call durations
13. Stream lifecycle — stream_start, turn_complete, stream_error with model/duration

### Phase 3: Event Infrastructure
14. Design structured event schema (timestamp, level, source, category, metadata)
15. Implement pub/sub event bus (Rust, daemon-side) with guaranteed delivery
16. Add persistence layer (SQLite initially, evaluate MongoDB for query flexibility)
17. Wire all processes to the event bus

### Phase 4: OrqaDev App (separate window, dual launch)
18. Scaffold OrqaDev as its own Tauri app (uses `@orqastudio/svelte-components`)
19. Dual launch: standalone via `orqa dev` (launched FIRST to capture build output) AND from main app toolbar button (for live debugging in production)
20. Build virtualised log table component (tanstack-table or similar)
21. Implement log filtering/search UI (source, level, category, time range, full-text)
22. Add process diagnostics view (daemon, MCP, LSP, search, watcher status)
23. Integrate Storybook rendering view for component development
24. Add metrics/performance graphs view
25. Log levels: info in production, debug in dev — infrastructure always on

## Research Findings

Detailed logging gap analysis from 5 parallel researchers:
- `.state/investigation/24-logging-daemon.md` — 26 candidates (5 HIGH)
- `.state/investigation/24-logging-app.md` — 40+ candidates (10 HIGH)
- `.state/investigation/24-logging-engine.md` — 30 candidates (11 HIGH)
- `.state/investigation/24-logging-frontend.md` — 8 critical gaps
- `.state/investigation/24-logging-devcontroller.md` — 5 architectural issues

## Dependencies

- `@orqastudio/svelte-components` package (shared with main app)
- Virtualised table library evaluation (tanstack-table vs alternatives)
- Tauri multi-window architecture for OrqaDev window

## Success Criteria

- All process output visible in OrqaDev from first compile to running app
- Log entries filterable by source, level, category, time range
- No lost logs — guaranteed delivery from all sources
- Sub-100ms render for 10,000+ log entries (virtualised)
- Storybook accessible without a separate dev server
