---
id: "TASK-3b119193"
title: "Reconcile EPIC-6787bb93"
description: "Standing reconciliation task — verify epic body accuracy: task table, pillars, docs-produced, scope."
status: "blocked"
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "Epic task table lists ALL tasks created during the epic"
  - "Epic pillars array reflects all pillars served"
  - "Epic docs-produced list matches actual documentation created/updated"
  - "Epic scope section accurately reflects what was in/out of scope"
relationships:
  - target: "EPIC-6787bb93"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-db9be55f"
    type: "depends-on"
  - target: "TASK-e6d10f6f"
    type: "depends-on"
  - target: "TASK-d05c56ea"
    type: "depends-on"
  - target: "TASK-cc68b8df"
    type: "depends-on"
  - target: "TASK-5e1e41de"
    type: "depends-on"
  - target: "TASK-2e1df475"
    type: "depends-on"
  - target: "TASK-21b461ea"
    type: "depends-on"
---
## What

Standing reconciliation task for [EPIC-6787bb93](EPIC-6787bb93). Ensures the epic body stays accurate as work evolves.

## Verification

- Epic body task table matches actual tasks with `epic: [EPIC-6787bb93](EPIC-6787bb93)`
- Pillars array is accurate
- docs-produced entries exist on disk

## Lessons

- Backfilled per [RULE-7b770593](RULE-7b770593) epic reconciliation requirement
