---
id: IDEA-1cbb9dc4
type: idea
title: Unified logger library with SSE stream output and debug levels
description: "Create a logger library for each language (Rust, TypeScript/Svelte, Bun/sidecar) that logs to console/stdout AND sends events to the dev controller SSE stream. Use log levels (debug, info, warn, error) so debug-level logs used during development stay in code rather than being cleaned up — they are filtered by level at the viewing interface. Removes the coding standard requiring cleanup of debugging logs."
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: later
research-needed:
  - "How does the dev controller SSE stream currently work? Can it accept structured log events?"
  - "What logging patterns exist today in Rust (tracing/log), TS (console.log), and sidecar?"
  - "What log viewer interfaces would consume these streams (terminal, app dev panel, external tools)?"
  - "How should log levels be configured per-context (dev vs production vs debug)?"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

## Motivation

Debug logging is currently treated as temporary code that must be cleaned up before commit. This creates friction:

1. Developers add useful debug logs, gain insight, then must delete them
2. The same debugging logs get re-added next time the area is investigated
3. Valuable diagnostic context is permanently lost

Instead, debug logs should be permanent infrastructure with level-based filtering. A `debug`-level log stays in the codebase permanently but only appears when the viewer requests debug-level output. This aligns with observability best practices — instrumentation is always-on, visibility is configurable.

The additional value: piping log output to the dev controller SSE stream means logs from all three runtimes (Rust, Vite/Svelte, Bun/sidecar) can be viewed in a single unified stream, correlated by timestamp. This is especially valuable when debugging cross-boundary issues (sidecar → Rust → frontend).

## Sketch

**Per-language libraries:**

- **Rust**: Wrapper around `tracing` with an SSE subscriber that sends structured events to the controller
- **TypeScript/Svelte**: Logger class that wraps console methods and sends to SSE via fetch/EventSource
- **Bun/sidecar**: Logger that writes to stdout (captured by controller) and optionally sends structured events

**Level model:**

| Level | When to use | Default visibility |
|-------|------------|-------------------|
| `error` | Something failed, user-impacting | Always shown |
| `warn` | Unexpected but recoverable | Shown in dev |
| `info` | Key lifecycle events, state changes | Shown in dev |
| `debug` | Investigation, data inspection, flow tracing | Hidden by default, shown when requested |
| `trace` | Extremely verbose, per-iteration logging | Hidden, only for targeted debugging |

**Coding standards change:** Remove the "no debug logs in committed code" standard. Replace with: "Use debug-level logging for diagnostic output. All logs must use the logger library, not raw console.log/println."

**Dev controller integration:** The controller already manages SSE for hot reload signals. Extend the SSE stream to carry structured log events with: timestamp, level, source (rust/vite/sidecar), module, message, optional structured data.