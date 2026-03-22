---
id: TASK-8f18cf73
title: "Clippy/custom check: function size limits"
description: Add clippy or custom check to enforce function size limits in Rust code
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - Functions exceeding 50 lines are flagged by clippy or a custom check during make lint-backend
relationships:
  - target: EPIC-4e6e9eae
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-e5b83fae
    type: depended-on-by
  - target: TASK-7cf80542
    type: depended-on-by
---

## What

Configure clippy too_many_lines or a custom check to enforce function size limits.

## How

Enable and configure the clippy::too_many_lines lint with appropriate thresholds per module type.

## Verification

Completed as part of [EPIC-4e6e9eae](EPIC-4e6e9eae) Phase 2.

## Lessons

No new lessons.
