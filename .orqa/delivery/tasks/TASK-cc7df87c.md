---
id: TASK-cc7df87c
type: task
title: Update rust-modules.md module tree
description: Bring the Rust module tree documentation in line with current codebase structure.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-bbad3d30
acceptance:
  - "Module tree matches `ls -R backend/src-tauri/src/` output"
  - skill_injector.rs listed in domain module section
  - All paths use backend/src-tauri/ prefix
relationships:
  - target: EPIC-2bf6887a
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-d1b856b5
    type: depends-on
  - target: TASK-f50f84f8
    type: depended-on-by
  - target: TASK-e850a474
    type: depended-on-by
---

## What

Update `.orqa/documentation/development/rust-modules.md` to reflect the actual module structure.

## How

1. Read current codebase structure
2. Compare against documented module tree
3. Add missing modules (skill_injector.rs, any others)
4. Fix all path prefixes

## Verification

Every module in `backend/src-tauri/src/` appears in the doc. No phantom modules listed.