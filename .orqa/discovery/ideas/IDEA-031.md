---
id: IDEA-031
title: Auto-Refresh on File Changes
description: Watch the .orqa/ directory for file changes and automatically invalidate the artifact navigation tree and viewer cache so the UI stays current without requiring a hard refresh.
status: captured
created: 2026-03-07
updated: 2026-03-13
horizon: next
research-needed:
  - Evaluate Tauri fs watch plugin vs notify crate for file watching
  - Determine debounce strategy for bulk file changes
relationships:
  - target: PILLAR-001
    type: grounded
  - target: PERSONA-001
    type: benefits
---
## Motivation

When artifact files change on disk (e.g., during bulk data quality
updates), the UI shows stale data until the user hard-refreshes.
The backend reads fresh from disk on every command, but the frontend
caches the navigation tree in `artifactStore.navTree` and only loads
it once.

## Current State

- Backend: No cache — reads disk fresh every invocation
- Frontend: `artifactStore.navTree` cached, loaded once on startup
- `invalidateNavTree()` and `invalidateContent()` methods exist but nothing calls them
- `tauri-plugin-fs` with `watch` feature is enabled but unused

## Proposed Approach

1. Backend: Watch `.orqa/` with Tauri fs plugin, emit event on change
2. Frontend: Listen for event, call `invalidateNavTree()` + `invalidateContent()`
3. Debounce: Batch rapid changes (e.g., 500ms window) to avoid excessive reloads
