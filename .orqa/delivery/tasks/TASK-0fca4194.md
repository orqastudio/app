---
id: TASK-0fca4194
type: task
title: Update documentation paths for directory reorganisation
description: Find-replace all path references in .orqa/ documentation to reflect the new directory structure.
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - All src-tauri/ references updated to backend/src-tauri/
  - All sidecar/ references updated to sidecars/claude-agentsdk-sidecar/
  - All scripts/dev.mjs references updated to debugger/dev.mjs
  - All ui/lib/ references updated to ui/src/lib/
  - All ui/routes/ references updated to ui/src/routes/
  - Spot-checked for correctness
relationships:
  - target: EPIC-7b039d05
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-0e401567
    type: depended-on-by
  - target: TASK-03741ecb
    type: depended-on-by
---

## What

Update ~140 path references across ~40 documentation files in `.orqa/` to reflect the
new directory structure before any code moves happen.

## How

Automated find-replace on all `.md` files in `.orqa/`:
- `src-tauri/` → `backend/src-tauri/`
- `sidecar/` → `sidecars/claude-agentsdk-sidecar/`
- `scripts/dev.mjs` → `debugger/dev.mjs`
- `scripts/dev-dashboard.html` → `debugger/dev-dashboard.html`
- `ui/lib/` → `ui/src/lib/`
- `ui/routes/` → `ui/src/routes/`
- `ui/app.css` → `ui/src/app.css`
- `ui/app.html` → `ui/src/app.html`

Spot-check results for false positives (e.g., artifact references that shouldn't change).

## Verification

- [ ] No stale path references remain in `.orqa/` docs
- [ ] No false positives introduced