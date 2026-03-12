---
id: TASK-256
title: "Move dev controller to debugger/"
description: "Relocate scripts/dev.mjs and scripts/dev-dashboard.html to debugger/ and update Makefile and internal paths."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-056
depends-on:
  - TASK-255
acceptance:
  - "scripts/dev.mjs moved to debugger/dev.mjs"
  - "scripts/dev-dashboard.html moved to debugger/dev-dashboard.html"
  - "Makefile dev controller references updated (9 changes)"
  - "DASHBOARD_HTML path in dev.mjs updated"
  - "make dev starts successfully"
  - "scripts/ directory removed if empty"
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
