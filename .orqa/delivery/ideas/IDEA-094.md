---
id: IDEA-094
title: "Dev controller log streaming to devtool — agents react to build/runtime errors"
description: "Capture Vite HMR errors, Rust compile errors, and runtime warnings from the dev controller and surface them in the devtool as a subscribable stream. Agents can monitor the stream and react to errors automatically."
status: captured
created: "2026-03-14"
updated: "2026-03-14"
pillars:
  - PILLAR-001
  - PILLAR-003
milestone: null
horizon: next
research-needed:
  - "How does the dev controller currently capture stdout/stderr from Vite and Tauri?"
  - "What structured error format should the stream use?"
  - "How do agents subscribe to and react to the error stream?"
  - "Should errors trigger automatic fix attempts or just surface for human review?"
promoted-to: null
spun-off-from: null
relationships: []
---

## Motivation

During UAT, multiple Svelte compile errors (`{@const}` placement) only surfaced at Vite HMR runtime — not during `svelte-check` or TypeScript compilation. The agents that wrote the code had no way to see these errors because the dev controller's output wasn't available to them.

If the dev controller forwarded structured error events from Vite and cargo to a subscribable stream, agents could:
1. Detect compile errors immediately after writing code
2. Auto-fix common patterns (like `{@const}` placement)
3. Avoid multiple back-and-forth cycles between user reporting and agent fixing

The devtool already exists — this is about feeding it the right data and giving agents a way to consume it.
