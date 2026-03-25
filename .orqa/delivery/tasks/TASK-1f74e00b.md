---
id: TASK-1f74e00b
type: task
title: Wire all new checks into pre-commit hook
description: "Integrate all new linter, hook, and tooling checks from Phase 2 into the pre-commit hook staged-file paths"
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - All Phase 2 checks run as part of the pre-commit hook based on staged file paths
relationships:
  - target: EPIC-a60f5b6b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-6a07cfc9
    type: depends-on
  - target: TASK-da07ad35
    type: depends-on
  - target: TASK-31f82835
    type: depends-on
  - target: TASK-a034493b
    type: depends-on
  - target: TASK-9471304a
    type: depended-on-by
  - target: TASK-79ff025c
    type: depended-on-by
---

## What

Wire all new enforcement checks into the pre-commit hook so they run automatically on relevant staged files.

## How

Update the pre-commit hook to invoke the new ESLint rules, clippy checks, hook validations, and tooling checks based on which files are staged.

## Verification

Completed as part of [EPIC-a60f5b6b](EPIC-a60f5b6b) Phase 2.

## Lessons

No new lessons.