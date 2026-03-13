---
id: TASK-282
title: "Add make verify targets"
description: "Add make targets for full .orqa/ link verification and pipeline integrity checking."
status: todo
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-059
depends-on: []
assignee: null
docs: []
skills: []
acceptance:
  - "make verify-links runs full .orqa/ link verification"
  - "make verify-integrity runs pipeline integrity check"
  - "make verify runs both"
  - "All three targets documented in commands.md"
rule-overrides: []
---

## What

Add `make verify-links`, `make verify-integrity`, and `make verify` targets to the Makefile.

## How

1. Add targets to Makefile calling the corresponding tools scripts
2. Update `.orqa/documentation/development/commands.md` with the new targets
3. Update RULE-007 command mapping table

## Verification

- `make verify-links` runs and reports results
- `make verify-integrity` runs and reports results
- `make verify` runs both
