---
id: TASK-257
title: Full integration test of reorganised repository
description: "Run all quality gates, dev lifecycle, and production build to verify the reorganisation is complete and correct."
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - make check passes (all quality gates)
  - make dev starts and runs correctly
  - make build produces a production build
  - No stale path references remain in codebase
  - Watcher scoping verified (Vite only reloads on ui/src/ changes)
relationships:
  - target: EPIC-056
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-256
    type: depends-on
  - target: TASK-346
    type: depended-on-by
---

## What

End-to-end verification that the reorganised repository works correctly.

## How

1. `make check` — all linting, type checking, tests
2. `make dev` — verify Vite + Tauri start, dashboard works
3. `make kill`
4. `make build` — production build succeeds
5. Search for any remaining stale path references

## Verification

- [ ] `make check` passes
- [ ] `make dev` + `make kill` cycle works
- [ ] `make build` succeeds
- [ ] No stale paths found via grep
