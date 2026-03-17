---
id: TASK-306
title: Replace hardcoded path constants with runtime config cache (IMPL-018)
description: "Remove paths.rs constants and all hardcoded .orqa/ paths. Load project.json once at startup, build a ProjectPaths struct, pass it through the call chain. Decision: Option C from RES-052, approved by user."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - "paths.rs removed or reduced to only truly structural constants (ORQA_DIR, SETTINGS_FILE)"
  - ProjectPaths struct built from project.json at startup
  - All modules that previously used path constants use ProjectPaths instead
  - "project_scanner.rs, artifact_fs.rs, and delivery workflow code all read from config"
  - "make lint-backend && make test-rust pass"
  - IMPL-018 maturity updated to understanding
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-349
    type: depended-on-by
  - target: IMPL-018
    type: informed-by
---
## What

Eliminate the constant/config duality by making `project.json` the single source of truth for all `.orqa/` paths at runtime. Load config once, cache in a struct, pass through the call chain.

## How

1. Map all consumers of `paths.rs` constants
2. Map all consumers of `project.json` artifact paths
3. Design `ProjectPaths` struct that unifies both
4. Load and cache at app startup
5. Thread through service constructors
6. Remove `paths.rs` constants (keep only `ORQA_DIR` and `SETTINGS_FILE` as bootstrap constants needed to find the config file itself)
7. Update [IMPL-018](IMPL-018) maturity to understanding

## Verification

- `grep -r "paths::" backend/src-tauri/src/` shows only ORQA_DIR and SETTINGS_FILE usage
- All path resolution traces back to project.json
- make check passes
