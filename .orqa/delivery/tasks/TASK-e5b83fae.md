---
id: TASK-e5b83fae
type: task
title: Wire all new checks into pre-commit hook
description: "Integrate all new linter, hook, and tooling checks from Phase 2 into the pre-commit hook staged-file paths"
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - All Phase 2 checks run as part of the pre-commit hook based on staged file paths
relationships:
  - target: EPIC-4e6e9eae
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-2284f3e6
    type: depends-on
  - target: TASK-8f18cf73
    type: depends-on
  - target: TASK-fc7f41ca
    type: depends-on
  - target: TASK-ab350828
    type: depends-on
  - target: TASK-7cf80542
    type: depended-on-by
  - target: TASK-247b6ce4
    type: depended-on-by
---

## What

Wire all new enforcement checks into the pre-commit hook so they run automatically on relevant staged files.

## How

Update the pre-commit hook to invoke the new ESLint rules, clippy checks, hook validations, and tooling checks based on which files are staged.

## Verification

Completed as part of [EPIC-4e6e9eae](EPIC-4e6e9eae) Phase 2.

## Lessons

No new lessons.