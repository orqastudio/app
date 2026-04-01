---
id: EPIC-a1b2c3d4
type: epic
title: "OrqaDev: Comprehensive Dev Tooling & Logging Platform"
description: "Transform the dev dashboard into a dedicated Tauri companion app with comprehensive logging, pub/sub event delivery, persistence, component storybook, and virtualised log viewer."
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

Replace the current `tools/debug/dev-dashboard.html` with a dedicated Tauri companion app (OrqaDev) that serves as the one-stop dev tooling support app for OrqaStudio development.

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

1. Design event schema (structured log format)
2. Implement pub/sub event bus (Rust, daemon-side)
3. Add persistence layer (SQLite initially, migrate later if needed)
4. Scaffold OrqaDev Tauri app
5. Build virtualised log table component
6. Implement log filtering/search UI
7. Add process management view
8. Integrate Storybook rendering
9. Add frontend console forwarding
10. Wire all processes to the event bus
11. Fix dev controller startup order (dashboard first)
12. Add comprehensive logging across all processes (per research findings)

## Dependencies

- Logging research findings (`.state/investigation/24-logging-*.md`)
- `@orqastudio/svelte-components` package
- Tauri multi-window or separate app architecture decision

## Success Criteria

- All process output visible in OrqaDev from first compile to running app
- Log entries filterable by source, level, category, time range
- No lost logs — guaranteed delivery from all sources
- Sub-100ms render for 10,000+ log entries (virtualised)
- Storybook accessible without a separate dev server
