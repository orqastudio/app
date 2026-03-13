---
id: TASK-315
title: "Reconcile EPIC-025"
description: "Standing reconciliation task — verify epic body accuracy: task table, pillars, docs-produced, scope."
status: done
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-025
depends-on: ["TASK-094", "TASK-095", "TASK-096", "TASK-097", "TASK-098"]
acceptance:
  - "Epic task table lists ALL tasks created during the epic"
  - "Epic pillars array reflects all pillars served"
  - "Epic docs-produced list matches actual documentation created/updated"
  - "Epic scope section accurately reflects what was in/out of scope"
---

## What

Standing reconciliation task for EPIC-025. Ensures the epic body stays accurate as work evolves.

## Verification

- Epic body task table matches actual tasks with `epic: EPIC-025`
- Pillars array is accurate
- docs-produced entries exist on disk

## Lessons

- Backfilled per RULE-004 epic reconciliation requirement
