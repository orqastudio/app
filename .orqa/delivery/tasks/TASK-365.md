---
id: TASK-365
title: "Hook checks: status transitions, config-disk consistency, pillar alignment, historical preservation, E2E completeness"
description: "Add pre-commit hook checks for status transition validation, config-disk consistency, pillar alignment sections, historical preservation, and end-to-end completeness"
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - "Pre-commit hook validates status transitions, config-disk consistency, pillar alignment, historical preservation, and E2E completeness for staged files"
relationships:
  - target: EPIC-061
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-367
    type: depended-on-by
  - target: TASK-375
    type: depended-on-by
---

## What

Extend the pre-commit hook with five new validation checks for previously self-compliance-only rules.

## How

Add check functions to the pre-commit hook chain for each pattern: invalid status transitions, project.json paths not matching disk, missing pillar alignment sections in docs, deletion of research/task files, and missing TS interfaces for new Tauri commands.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 2.

## Lessons

No new lessons.
