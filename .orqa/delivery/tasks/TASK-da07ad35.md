---
id: "TASK-da07ad35"
type: "task"
title: "Clippy/custom check: function size limits"
description: "Add clippy or custom check to enforce function size limits in Rust code"
status: "completed"
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
acceptance:
  - "Functions exceeding 50 lines are flagged by clippy or a custom check during make lint-backend"
relationships:
  - target: "EPIC-a60f5b6b"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Configure clippy too_many_lines or a custom check to enforce function size limits.

## How

Enable and configure the clippy::too_many_lines lint with appropriate thresholds per module type.

## Verification

Completed as part of [EPIC-a60f5b6b](EPIC-a60f5b6b) Phase 2.

## Lessons

No new lessons.