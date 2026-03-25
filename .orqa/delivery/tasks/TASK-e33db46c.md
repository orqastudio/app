---
id: TASK-e33db46c
type: task
title: Move pillars to process/ and rename planning to delivery
description: "Move pillars from planning/ to process/. Rename planning/ to delivery/. Update project.json, all path references."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - .orqa/principles/pillars/ exists with all pillar files
  - ".orqa/delivery/ exists with ideas, research, milestones, epics, tasks"
  - .orqa/delivery/ directory no longer exists
  - project.json paths updated
rule-overrides:
  - "rule: RULE-63cc16ad"
relationships:
  - target: EPIC-88f359b0
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-6aa8e1f1
    type: depends-on
  - target: TASK-c4b77719
    type: depended-on-by
  - target: TASK-d29cb6b9
    type: depended-on-by
  - target: TASK-d2b54d2f
    type: depended-on-by
---

## What

Move pillars to `process/` (they're pipeline artifacts, not delivery items). Rename `planning/` to `delivery/` to reflect its actual purpose.

## How

1. `git mv .orqa/principles/pillars/ .orqa/principles/pillars/`
2. `git mv .orqa/delivery/ .orqa/delivery/`
3. Update `project.json` artifact paths
4. Search and update all references

## Verification

- All files accessible at new paths
- `project.json` paths resolve correctly
- No references to old `.orqa/delivery/` paths remain