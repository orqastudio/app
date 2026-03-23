---
id: TASK-5fc0e2dc
type: task
title: Auto-fix engine for deterministic integrity issues
description: "Add auto-fix logic to the artifact graph that can programmatically fix missing bidirectional inverses by writing the inverse relationship to the target artifact's frontmatter. Returns a list of fixes applied."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - "apply_auto_fixes Tauri command accepts Vec<IntegrityCheck> and applies deterministic fixes"
  - "Missing inverse fix: reads target file, adds inverse relationship to frontmatter, writes file"
  - "Returns Vec<AppliedFix> describing what was changed"
  - "Only auto_fixable checks are processed, others are skipped"
  - Graph is refreshed after fixes are applied
  - make check passes
relationships:
  - target: EPIC-e37794bf
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-f6fd3161
    type: depends-on
  - target: TASK-614122c8
    type: depended-on-by
  - target: TASK-d624db8f
    type: depended-on-by
---

## What

Enable the app to fix deterministic integrity issues (missing bidirectional inverses) by writing the inverse relationship to the target artifact's YAML frontmatter.

## How

1. Define `AppliedFix` struct in `artifact_graph.rs`
2. Add `apply_fixes()` function that processes auto-fixable checks
3. For MissingInverse: parse target file YAML, add inverse relationship entry, write back
4. Add `apply_auto_fixes` Tauri command
5. Refresh the graph after applying fixes

## Verification

- `make check` passes
- Create a test with a one-directional relationship, run auto-fix, verify inverse was added

## Lessons

(none yet)