---
id: "IDEA-d9e70a44"
type: discovery-idea
title: "OrqaDev Companion App + In-App Debug Overlay"
description: "Replace the dev dashboard HTML page with a pure SSE endpoint, build a dedicated OrqaDev companion app that consumes the feed, and add an optional in-app debug overlay that subscribes to the same stream."
status: captured
created: "2026-03-12"
updated: "2026-03-13"
horizon: "next"
research-needed:
  - "Tech stack for companion app (Tauri mini-app? Electron? Web app with persistent tab?)"
  - "SSE endpoint design — structured event schema, log levels, source tags, metadata"
  - "In-app debug overlay UX — toggle mechanism, panel placement, performance impact"
  - "Whether companion app should also expose process controls (start/restart/stop) or just observe"
relationships:
  - target: "PILLAR-c9e0a695"
    type: "grounded"
  - target: "PERSONA-c4afd86b"
    type: "benefits"
---
## Motivation

The current OrqaDev dashboard (`debugger/dev-dashboard.html`) bundles the UI and the SSE server together. Separating these concerns — a pure SSE endpoint as the data layer, with multiple consumers — enables richer tooling without coupling to a single HTML page.

## Sketch

### Architecture

```
Dev Controller (debugger/dev.mjs)
  └─ SSE endpoint: GET /events
       ├─ OrqaDev Companion App (dedicated window, richer UI)
       ├─ In-app debug overlay (toggle-on inside OrqaStudio itself)
       └─ Any other consumer (CLI tools, test harnesses, agent dashboards)
```

### Three pieces

1. **SSE endpoint** — the dev controller already serves SSE at `GET /events`. Strip the
   HTML dashboard to just the endpoint (or keep the HTML as a lightweight fallback). Define
   a structured event schema with log levels, source tags, timestamps, and metadata.

2. **OrqaDev Companion App** — a separate app (likely a small Tauri app or persistent web
   app) that connects to the SSE endpoint and provides a rich debugging experience: log
   filtering, search, process controls, performance graphs, error aggregation.

3. **In-app debug overlay** — a toggleable UI inside OrqaStudio itself that subscribes to
   the same SSE feed. Activated via a dev menu or keyboard shortcut. Useful when you want
   to see logs without switching windows. Only available when the dev controller is running.

### Key principle

The SSE stream is the single source of truth for all dev-time observability. Everything
subscribes to the same feed. [TASK-36924b1c](TASK-36924b1c) (routing all logging to the stream) is the prerequisite
— this idea builds on top of that foundation.