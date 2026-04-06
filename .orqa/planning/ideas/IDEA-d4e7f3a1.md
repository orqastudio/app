---
id: "IDEA-d4e7f3a1"
type: planning-idea
title: "Devtools as Muscula-inspired structured event monitoring — grouped issues over raw firehose"
description: "Evolve the OrqaStudio devtools from a raw log firehose into a grouped issue monitor, drawing on Muscula's UX model. Events are fingerprinted and deduplicated into Issue groups. Structured fields (component, level, message template, stack, correlation id) drive filtering, tracing, and AI-assisted root cause analysis across the full OrqaStudio stack."
status: active
created: "2026-04-05"
updated: "2026-04-05"
horizon: active
relationships:
  - target: "PILLAR-c9e0a695"
    type: "grounded"
    rationale: "The core problem is visibility: hidden runtime errors buried in a log stream. Grouping, fingerprinting, and source-resolved stack traces make system behavior visible and structured."
  - target: "EPIC-b3c2e8f5"
    type: "realises"
    rationale: "This idea is realised through the devtools evolution epic."
---

## Motivation

The current devtools Logs tab is a raw event stream — useful for moment-to-moment observation but provides no signal about what is broken, how often, or across which components. When an error occurs across multiple stack layers (app webview → daemon → MCP server → LSP server → ONNX worker → connector → CLI), there is no view that connects the dots.

The goal is to move from firehose to insight: grouped issues, causal traces, and source-resolved stack frames that show exactly where in the codebase each problem originated.

## Problem Statement

The OrqaStudio stack spans 8 components — app webview, daemon, MCP server, LSP server, ONNX worker, Claude Code connector, CLI, and Vite dev server. Errors in any of these produce events. The current view shows all events in sequence, with no deduplication, no grouping, and no way to walk the causal chain when one user action touches several components.

Result: a single reproducible error produces tens of identical log entries, the important ones lost in noise, and stack traces pointing to minified bundle line numbers rather than source files.

## Sketch

The devtools evolves into a monitoring surface with a structured information architecture:

**Navigation:**

- **Issues** — grouped view (primary entry, new)
- **Stream** — the existing firehose (renamed from Logs)
- **Trace** — correlation-id timeline (new)
- **Processes** — existing lifecycle/health panel
- **Metrics** — existing perf gauges
- **Help** — existing

**Issue grouping:**
Events with the same fingerprint `(component, level, message_template, stack_top)` are deduplicated into a single Issue group. Each group carries: count, first_seen, last_seen, affected components, severity, and a 24-hour occurrence sparkline.

**Structured event fields:**
Every event carries typed metadata — component, level, message, message_template, stack, request_id, session_id, correlation_id, custom_fields. Filters expose every field.

**Per-event context drawer:**
Click any event → right-side drawer with Stack / Context / Related / Raw tabs. List focus is preserved so you can arrow through events without losing your place.

**AI explain:**
Every event in the drawer has a one-click action that opens the orchestrator chat with event context (component, error, stack trace, recent siblings, structured fields) pre-loaded in the prompt.

**Trace view:**
Events sharing a correlation_id are grouped into a timeline across components — swim-lane per component — so you can walk the causal chain of a single user action.

**Source maps (load-bearing):**
Stack traces must resolve to real source file + line numbers for every stack component:

- Frontend bundles: Vite source maps (already exist, must reach devtools renderer)
- Daemon: Rust binaries built with debuginfo, backtraces symbolicated at event-capture time
- Sidecars: script-based components emit file + line at log call site

This is not a nice-to-have. Without source-resolved stacks, the Issues tab, the event drawer, and the AI explain feature are all operating on noise, not signal.

**Performance:**

- Sub-100ms rendering target for all views
- Virtualized tables
- Incremental grouping index maintained on the ingest path — no full re-sort on duplicate arrival

## What This Does NOT Include

These are explicitly out of scope (deviations from the Muscula model):

- Multi-project switching — this is a single-project dev tool
- Team assignments, comments, user management — devtools is personal
- Email / Slack / webhook alerting — toaster covers real-time notification
- Uptime monitoring — daemon health widget already covers this

## Research Needed

- How does fingerprinting handle parameterized messages? (e.g. "Failed to load artifact IDEA-xxxxxxxx" — should the ID be elided from the template before fingerprinting?)
- Correlation id propagation: which IPC boundaries currently forward the id and which drop it?
- Source map delivery path: how do Vite source maps reach the devtools process at runtime?
