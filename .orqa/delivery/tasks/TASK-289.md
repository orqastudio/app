---
id: TASK-289
title: "Move pillars to process/ and rename planning to delivery"
description: "Move pillars from planning/ to process/. Rename planning/ to delivery/. Update project.json, all path references."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: [TASK-286]
assignee: null
docs: []
skills: []
acceptance:
  - ".orqa/process/pillars/ exists with all pillar files"
  - ".orqa/delivery/ exists with ideas, research, milestones, epics, tasks"
  - ".orqa/delivery/ directory no longer exists"
  - "project.json paths updated"
rule-overrides:
  - rule: RULE-003
    reason: "Artifact paths are being reorganized — intermediate state will have mismatches"
---

## What

Move pillars to `process/` (they're pipeline artifacts, not delivery items). Rename `planning/` to `delivery/` to reflect its actual purpose.

## How

1. `git mv .orqa/process/pillars/ .orqa/process/pillars/`
2. `git mv .orqa/delivery/ .orqa/delivery/`
3. Update `project.json` artifact paths
4. Search and update all references

## Verification

- All files accessible at new paths
- `project.json` paths resolve correctly
- No references to old `.orqa/delivery/` paths remain
