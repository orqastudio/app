---
id: TASK-396
title: "Audit components for invoke()-to-SDK migration opportunities"
description: "Audit all Svelte components in ui/src/lib/components/ for direct invoke() calls that could be replaced with artifactGraphSDK lookups. Finding: no migration needed — only invoke() in components is artifact_watch_start (a watcher command, not a graph query)."
status: done
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-060
depends-on:
  - TASK-389
acceptance:
  - "All components in ui/src/lib/components/ audited for invoke() usage"
  - "Any graph-data invoke() calls identified for migration"
  - "Result documented"
---

## What

Audit whether any Svelte components bypass the SDK and call `invoke()` directly for data the artifact graph already provides.

## Finding

Only one `invoke()` call exists in `ui/src/lib/components/`: `artifact_watch_start` in `AppLayout.svelte`. This is a file-system watcher operation — not a graph query — so it correctly uses `invoke()` rather than the SDK.

All graph data access in components already goes through `artifactGraphSDK`. No migration needed.
