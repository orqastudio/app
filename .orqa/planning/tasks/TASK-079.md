---
id: TASK-079
title: File watcher for .orqa/ with graph rebuild and event emission
description: Watch .orqa/ for file system changes and rebuild the artifact graph on change, emitting a full snapshot Tauri event to the frontend.
status: done
created: "2026-03-10"
updated: "2026-03-10"
epic: EPIC-048
depends-on:
  - TASK-073
assignee: AGENT-002
skills:
  - SKILL-027
  - SKILL-032
acceptance:
  - File watcher monitors .orqa/ for create, modify, delete, rename events
  - Graph rebuild is debounced (500ms)
  - Full graph snapshot emitted as artifact-graph-updated Tauri event
  - Frontend SDK receives event and refreshes local graph
  - Watcher ignores non-.md files and hidden directories
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
