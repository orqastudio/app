---
id: "TASK-d5d3e417"
title: "Add make verify targets"
description: "Add make targets for full .orqa/ link verification and pipeline integrity checking."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
assignee: null
docs: []
acceptance:
  - "make verify-links runs full .orqa/ link verification"
  - "make verify-integrity runs pipeline integrity check"
  - "make verify runs both"
  - "All three targets documented in commands.md"
rule-overrides: []
relationships:
  - target: "EPIC-942c7678"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-8954343c"
    type: "depended-on-by"
  - target: "TASK-c4a7b6bb"
    type: "depended-on-by"
  - target: "TASK-cea1bc37"
    type: "depended-on-by"
---
## What

Add `make verify-links`, `make verify-integrity`, and `make verify` targets to the Makefile.

## How

1. Add targets to Makefile calling the corresponding tools scripts
2. Update `.orqa/documentation/development/commands.md` with the new targets
3. Update [RULE-c71f1c3f](RULE-c71f1c3f) command mapping table

## Verification

- `make verify-links` runs and reports results
- `make verify-integrity` runs and reports results
- `make verify` runs both
