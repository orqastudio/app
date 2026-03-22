---
id: TASK-3f426c14
title: Backfill missing description fields across all artifact types
description: "Add description frontmatter to 106 artifacts (ideas, decisions, lessons, epics, tasks) that predate the required field"
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - All artifacts pass schema validation (0 errors from validate-artifacts script)
  - "Every idea, decision, lesson, epic, and task has a non-empty description field"
  - "Descriptions are concise summaries, not duplicates of the title"
relationships:
  - target: EPIC-6f2d06d4
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-94149697
    type: depended-on-by
---

## What

Schema validation found 106 artifacts missing the required `description` field:

| Type | Count | Directory |
|------|-------|-----------|
| Ideas | 38 | `.orqa/delivery/ideas/` |
| Decisions | 32 | `.orqa/process/decisions/` |
| Tasks | 17 | `.orqa/delivery/tasks/` |
| Lessons | 15 | `.orqa/process/lessons/` |
| Epics | 4 | `.orqa/delivery/epics/` |

These artifacts were created before `description` became a required schema field.

## How

1. For each artifact type, read the title and body to derive a one-line description
2. Add `description: "..."` to frontmatter in the correct property order position
3. Run the validation script to confirm 0 errors remain

A scripted approach may work for some (e.g., copying title as description), but most will need a human-readable summary derived from the body content.

## Verification

- Run `node /tmp/validate-artifacts.mjs` (or equivalent) — 0 errors across all 476+ artifacts
- Spot-check 5 artifacts per type to confirm descriptions are meaningful, not just title copies
