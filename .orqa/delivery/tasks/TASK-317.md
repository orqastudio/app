---
id: TASK-317
title: Reconcile EPIC-027
description: "Standing reconciliation task — verify epic body accuracy: task table, pillars, docs-produced, scope."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - Epic task table lists ALL tasks created during the epic
  - Epic pillars array reflects all pillars served
  - Epic docs-produced list matches actual documentation created/updated
  - Epic scope section accurately reflects what was in/out of scope
relationships:
  - target: EPIC-027
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-103
    type: depends-on
  - target: TASK-104
    type: depends-on
  - target: TASK-105
    type: depends-on
  - target: TASK-106
    type: depends-on
  - target: TASK-107
    type: depends-on
---

## What

Standing reconciliation task for [EPIC-027](EPIC-027). Ensures the epic body stays accurate as work evolves.

## Verification

- Epic body task table matches actual tasks with `epic: [EPIC-027](EPIC-027)`
- Pillars array is accurate
- docs-produced entries exist on disk

## Lessons

- Backfilled per [RULE-004](RULE-004) epic reconciliation requirement
