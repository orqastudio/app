---
id: TASK-259
title: Update rust-modules.md module tree
description: Bring the Rust module tree documentation in line with current codebase structure.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-007
acceptance:
  - "Module tree matches `ls -R backend/src-tauri/src/` output"
  - skill_injector.rs listed in domain module section
  - All paths use backend/src-tauri/ prefix
relationships:
  - target: EPIC-057
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-258
    type: depends-on
  - target: TASK-266
    type: depended-on-by
  - target: TASK-347
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
