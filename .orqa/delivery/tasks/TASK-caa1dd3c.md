---
id: TASK-caa1dd3c
type: task
title: Fix post-restructure path references in docs
description: Update all documentation files that reference pre-restructure paths.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-cc255bc8
acceptance:
  - "No doc file references `src-tauri/` without `backend/` prefix"
  - "No doc file references `persistence/` directory (should be `repo/`)"
  - grep for both patterns returns zero results
relationships:
  - target: EPIC-4726cb3b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-e3e503a9
    type: depended-on-by
  - target: TASK-76e79dba
    type: depended-on-by
  - target: TASK-91bc09f9
    type: depended-on-by
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