---
id: TASK-367
title: Wire all new checks into pre-commit hook
description: "Integrate all new linter, hook, and tooling checks from Phase 2 into the pre-commit hook staged-file paths"
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - All Phase 2 checks run as part of the pre-commit hook based on staged file paths
relationships:
  - target: EPIC-061
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-363
    type: depends-on
  - target: TASK-364
    type: depends-on
  - target: TASK-365
    type: depends-on
  - target: TASK-366
    type: depends-on
  - target: TASK-375
    type: depended-on-by
  - target: TASK-376
    type: depended-on-by
---

## What

Wire all new enforcement checks into the pre-commit hook so they run automatically on relevant staged files.

## How

Update the pre-commit hook to invoke the new ESLint rules, clippy checks, hook validations, and tooling checks based on which files are staged.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 2.

## Lessons

No new lessons.
