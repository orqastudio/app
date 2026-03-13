---
id: TASK-364
title: "Clippy/custom check: function size limits"
description: "Add clippy or custom check to enforce function size limits in Rust code"
status: done
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-061
depends-on: []
acceptance:
  - "Functions exceeding 50 lines are flagged by clippy or a custom check during make lint-backend"
---

## What

Configure clippy too_many_lines or a custom check to enforce function size limits.

## How

Enable and configure the clippy::too_many_lines lint with appropriate thresholds per module type.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 2.

## Lessons

No new lessons.
