---
id: TASK-199f5d5a
type: task
title: Backfill docs field on existing tasks from git history
description: "Analyse git history and task scope to populate the docs field on existing todo tasks, creating graph edges from tasks to the documentation that should be loaded during implementation."
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - All todo tasks have a docs field (may be empty array if no docs apply)
  - docs entries point to existing documentation files
  - Documentation selection matches task scope and epic context
relationships:
  - target: EPIC-dc1e3e4b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-d8813639
    type: depends-on
  - target: TASK-0c6ac8d8
    type: depended-on-by
  - target: TASK-dd9c8538
    type: depended-on-by
---

## What

Populate the `docs` field on existing tasks so that agents implementing those tasks automatically receive the right documentation context via graph traversal.

## How

1. For each todo task, read its `scope` and `epic` fields
2. Determine relevant documentation based on:
   - Epic's `docs-required` and `docs-produced`
   - File paths in scope (e.g., `backend/src-tauri/` → architecture/rust-modules.md, architecture/ipc-commands.md)
   - Git history showing which docs were read before similar past implementations
3. Add `docs:` array to task frontmatter

## Verification

- All todo tasks have a `docs` field (array of strings or empty array)
- Referenced documentation paths exist on disk
- Tasks touching `backend/src-tauri/` reference coding-standards.md
- Tasks touching `.orqa/` reference artifact-framework.md where relevant