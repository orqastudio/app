---
id: "EPIC-b3c2e8f5"
type: epic
title: "Devtools evolution — grouped issue monitoring with source-resolved stacks"
description: "Evolve the OrqaStudio devtools from a raw log firehose into a Muscula-inspired issue monitoring surface. Events are fingerprinted and deduplicated into Issue groups. Structured fields drive filtering, tracing, and AI-assisted root cause analysis across the full OrqaStudio stack (app webview, daemon, MCP, LSP, ONNX, connector, CLI, Vite). Source map resolution is moved to step 3 — before the event drawer and AI explain — because unresolved stack frames make those features operate on noise rather than signal."
status: captured
priority: P1
created: "2026-04-05"
updated: "2026-04-05"
horizon: next
relationships:
  - target: "PILLAR-c9e0a695"
    type: "grounded"
    rationale: "Clarity Through Structure: the devtools surface makes hidden runtime behaviour visible, grouped, and traceable."
  - target: "EPIC-a1b2c3d4"
    type: "depends-on"
    rationale: "OrqaDev must exist as a Tauri app with ingest infrastructure before this evolution epic can land."
  - target: "EPIC-1f0fcb54"
    type: "depends-on"
    rationale: "Semantic lego blocks must be in place — all new views use ORQA components only, no raw HTML."
---

# Devtools Evolution — Grouped Issue Monitoring

## Context

EPIC-a1b2c3d4 builds the OrqaDev app: pub/sub event delivery, persistence, virtualised log viewer, and process diagnostics. This epic takes that foundation and evolves the UX from a raw stream toward a structured monitoring surface. The model is Muscula — event fingerprinting, issue grouping, deduplication — but scoped to a single-project dev tool with no team management or alerting.

**What is explicitly out of scope:**

- Multi-project switching
- Team assignments, comments, user management
- Email / Slack / webhook alerting
- Uptime monitoring (daemon health widget covers it)

## Information Architecture

Navigation panel after this epic:

| Entry | Status | Notes |
|---|---|---|
| Issues | NEW | Grouped view, primary entry |
| Stream | RENAMED | Current Logs tab |
| Trace | NEW | Correlation-id timeline |
| Processes | EXISTING | Lifecycle / health |
| Metrics | EXISTING | Perf gauges |
| Help | EXISTING | |

## Sequenced Delivery

### Why source maps are step 3, not step 5

The original sequencing put source maps at position 5. That was reconsidered: the event drawer (step 4), AI explain button (step 5), and trace view (step 6) all render stack frames. Without source-resolved stacks, those features display minified bundle offsets and Rust address ranges — not useful. Source maps are a prerequisite for the user-facing value of everything that follows them. They are moved to step 3, immediately after the Issues tab UI that motivates the need.

### Tasks

1. **Ingest-side fingerprinting and issue grouping**

   Add event fingerprinting on the ingest path: derive a canonical fingerprint from `(component, level, message_template, stack_top)`. Strip dynamic tokens (IDs, timestamps, counts) from message before computing template. Maintain an in-memory `IssueGroup` record per fingerprint with: `fingerprint`, `title`, `component`, `level`, `first_seen`, `last_seen`, `count`, `sparkline_buckets` (24 × 1-hour buckets), and a ring-buffer of the 50 most recent event IDs. Update the group incrementally on each matching ingest — no full re-sort. Persist groups to the devtools database alongside raw events. Expose a query API: `list_groups(sort, filter)` and `get_group(fingerprint)`.

2. **Issues tab UI**

   Add an Issues entry to the devtools navigation panel. Render a virtualized list of issue groups, sortable by: last_seen (default), count, severity, component. Each row shows: severity badge, title (derived from message template), component pill, count, last_seen timestamp, and a Sparkline. Clicking a row opens the event drawer (initially a placeholder — wired fully in step 4). Sub-100ms render target: list must not block on full re-render when a new duplicate arrives.

3. **Source map pipeline** *(prerequisite for steps 4, 5, 6)*

   Establish source-resolved stack frames for every OrqaStudio stack component before building the event drawer and AI explain features. Three sub-tasks:

   - **Frontend (app, devtools, Vite):** Vite source maps already exist at build time. Wire the devtools ingest path to apply source map transformation to JS stack frames before storing them. The rendered frame must show `file.ts:line:col` not a bundle offset.
   - **Daemon (Rust):** Rust binaries must be built with `debug = true` in `[profile.dev]`. At event-capture time (not render time), symbolicate backtraces using the debug symbols so the stored event already contains resolved `file.rs:line` frames. This avoids shipping DWARF data to the renderer.
   - **Sidecars (connector, CLI scripts):** Script-based components must attach `{ file, line }` metadata at each log call site. Establish a logging helper that captures `import.meta` or `__filename` + a stack-frame offset at the call site, and includes this in the structured event payload.

   Acceptance: every event from every component, when rendered in the devtools, shows a human-readable source location for the top stack frame.

4. **Event context drawer**

   Right-side panel within the Issues and Stream views. Opens when clicking any event group row or individual stream event. Tabs:
   - **Stack** — resolved stack frames (source file + line, now available from step 3). Syntax-highlighted. Click a frame to copy the path.
   - **Context** — all structured fields (component, level, session_id, request_id, correlation_id, custom_fields) in a key-value table.
   - **Related** — other events in the same group (for Issues view) or other events with the same correlation_id (for Stream view).
   - **Raw** — raw JSON of the event payload.

   Keyboard: arrow keys move between events while the drawer is open. Drawer close with Escape. List position is preserved — the list does not scroll or re-render on open/close.

5. **AI explain button**

   Button in the event drawer toolbar: "Explain with AI". On click, builds a prompt from the current event's structured fields — component, error message, resolved stack trace (top 5 frames), correlation_id siblings (up to 3), and custom_fields — and opens the orchestrator chat view with that prompt pre-loaded. The user can edit the prompt before sending. No automatic send. The prompt template is configurable (stored in devtools config, not hardcoded).

6. **Trace view**

   New navigation entry: Trace. Groups events by `correlation_id` into a timeline. Renders as a horizontal timeline with one swim-lane per component. Each event is a node on the lane at its timestamp. Clicking a node opens the event drawer. Selecting a correlation_id from the Stream or Issues view navigates to the Trace view filtered to that id. Correlation ids must be propagated across all IPC boundaries (app → daemon → MCP / LSP / ONNX / connector) — audit existing IPC call sites and add forwarding where missing.

7. **Sparklines in issue group rows**

   Wire the existing `<Sparkline>` primitive to the per-group `sparkline_buckets` histogram stored in step 1. Each row in the Issues list shows a 24-hour occurrence sparkline. The sparkline updates in real time as new matching events arrive — the bucket update is part of the ingest path from step 1, so this is a rendering wiring task only.

## Constraints

- All new views use `@orqastudio/svelte-components` components only — no raw HTML in the devtools app.
- Virtualized tables throughout: Issues list, Stream list, event drawer Related tab.
- Sub-100ms render target for all list views.
- Structured event fields (component, level, message_template, stack, correlation_id) are the contract — no free-text scraping.
