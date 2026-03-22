---
id: TASK-209f1646
title: Backfill lesson promoted-to targets (22 lessons)
description: Populate the promoted-to field for all 22 promoted lessons that had empty targets
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - All 22 promoted lessons have non-empty promoted-to fields pointing to valid rule/skill/standard targets
relationships:
  - target: EPIC-4e6e9eae
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-4017a3de
    type: depended-on-by
  - target: TASK-7cf80542
    type: depended-on-by
---

## What

Backfill promoted-to targets for 22 promoted lessons with empty fields.

## How

Trace each promoted lesson to the rule, skill, or standard it was promoted to and populate the promoted-to field.

## Verification

Completed as part of [EPIC-4e6e9eae](EPIC-4e6e9eae) Phase 1.

## Lessons

No new lessons.
