---
id: TASK-b7a65fee
type: task
title: Move dev controller to debugger/
description: Relocate scripts/dev.mjs and scripts/dev-dashboard.html to debugger/ and update Makefile and internal paths.
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - scripts/dev.mjs moved to debugger/dev.mjs
  - scripts/dev-dashboard.html moved to debugger/dev-dashboard.html
  - Makefile dev controller references updated (9 changes)
  - DASHBOARD_HTML path in dev.mjs updated
  - make dev starts successfully
  - scripts/ directory removed if empty
relationships:
  - target: EPIC-5adc6d0a
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-c0b200c5
    type: depends-on
  - target: TASK-f47db62a
    type: depended-on-by
  - target: TASK-43c0557f
    type: depended-on-by
---

## What

Move the dev controller and dashboard to their own directory.

## How

1. `mkdir debugger && git mv scripts/dev.mjs debugger/ && git mv scripts/dev-dashboard.html debugger/`
2. Update Makefile: all `node scripts/dev.mjs` → `node debugger/dev.mjs`
3. Update `debugger/dev.mjs`: DASHBOARD_HTML path
4. Remove `scripts/` if empty
5. Verify with `make dev && make kill`

## Verification

- [ ] `make dev` starts controller, Vite, and Tauri
- [ ] Dashboard accessible at localhost:3001
- [ ] `make kill` stops everything cleanly