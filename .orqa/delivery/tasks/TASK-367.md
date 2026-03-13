---
id: TASK-367
title: "Wire all new checks into pre-commit hook"
description: "Integrate all new linter, hook, and tooling checks from Phase 2 into the pre-commit hook staged-file paths"
status: done
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-061
depends-on:
  - TASK-363
  - TASK-364
  - TASK-365
  - TASK-366
acceptance:
  - "All Phase 2 checks run as part of the pre-commit hook based on staged file paths"
---

## What

Wire all new enforcement checks into the pre-commit hook so they run automatically on relevant staged files.

## How

Update the pre-commit hook to invoke the new ESLint rules, clippy checks, hook validations, and tooling checks based on which files are staged.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 2.

## Lessons

No new lessons.
