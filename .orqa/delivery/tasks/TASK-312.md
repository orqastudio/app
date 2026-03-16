---
id: TASK-312
title: Reconcile EPIC-005
description: "Standing reconciliation task — verify epic body accuracy: task table, pillars, docs-produced, scope."
status: blocked
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - Epic task table lists ALL tasks created during the epic
  - Epic pillars array reflects all pillars served
  - Epic docs-produced list matches actual documentation created/updated
  - Epic scope section accurately reflects what was in/out of scope
relationships:
  - target: EPIC-005
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-164
    type: depends-on
  - target: TASK-165
    type: depends-on
  - target: TASK-166
    type: depends-on
  - target: TASK-167
    type: depends-on
  - target: TASK-168
    type: depends-on
  - target: TASK-169
    type: depends-on
  - target: TASK-170
    type: depends-on
---
## What

Standing reconciliation task for [EPIC-005](EPIC-005). Ensures the epic body stays accurate as work evolves.

## Verification

- Epic body task table matches actual tasks with `epic: [EPIC-005](EPIC-005)`
- Pillars array is accurate
- docs-produced entries exist on disk

## Lessons

- Backfilled per [RULE-004](RULE-004) epic reconciliation requirement
