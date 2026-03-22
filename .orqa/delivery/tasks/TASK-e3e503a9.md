---
id: TASK-e3e503a9
title: Update rust-modules.md module tree
description: Bring the Rust module tree documentation in line with current codebase structure.
status: completed
created: 2026-03-12
updated: 2026-03-12
assignee: AGENT-ec1b3785
acceptance:
  - "Module tree matches `ls -R backend/src-tauri/src/` output"
  - skill_injector.rs listed in domain module section
  - All paths use backend/src-tauri/ prefix
relationships:
  - target: EPIC-4726cb3b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-caa1dd3c
    type: depends-on
  - target: TASK-76e79dba
    type: depended-on-by
  - target: TASK-91bc09f9
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
