---
id: "TASK-d1b856b5"
type: "task"
title: "Fix post-restructure path references in docs"
description: "Update all documentation files that reference pre-restructure paths."
status: archived
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
assignee: "AGENT-e5dd38e4"
acceptance:
  - "No doc file references `src-tauri/` without `backend/` prefix"
  - "No doc file references `persistence/` directory (should be `repo/`)"
  - "grep for both patterns returns zero results"
relationships:
  - target: "EPIC-2bf6887a"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Fix path references across `.orqa/documentation/` that still point to pre-monorepo-restructure locations.

## How

1. Grep for `src-tauri/` without `backend/` prefix across `.orqa/`
2. Grep for `persistence/` references
3. Update all occurrences to correct paths
4. Verify no broken references remain

## Verification

- `grep -r 'src-tauri/' .orqa/ | grep -v 'backend/src-tauri'` returns empty
- `grep -r 'persistence/' .orqa/documentation/` returns empty
