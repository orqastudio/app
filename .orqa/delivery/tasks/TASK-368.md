---
id: TASK-368
title: Recurrence auto-tracking and promotion readiness detection
description: Add tooling to auto-track lesson recurrence from review output and detect lessons ready for promotion
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - "Tooling auto-increments recurrence when review output matches existing lessons and surfaces lessons with recurrence >= 2 that lack promotion"
relationships:
  - target: EPIC-061
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-362
    type: depends-on
  - target: TASK-374
    type: depended-on-by
  - target: TASK-375
    type: depended-on-by
---

## What

Automate the learning loop's recurrence tracking and promotion readiness detection.

## How

Extend verify-pipeline-integrity.mjs or create new tooling to scan review agent output for failure patterns matching existing lessons, auto-increment recurrence, and surface promotion-ready lessons.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 3.

## Lessons

No new lessons.
