---
id: TASK-300
title: Update all path references across codebase
description: "After directory moves, every hardcoded path in Rust, TypeScript, skills, rules, agents, documentation must be updated. Use verify-links to confirm zero broken references."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - Zero broken path references in .orqa/ artifacts
  - Zero broken path references in Rust source code
  - Zero broken path references in TypeScript source code
  - verify-links.mjs reports zero issues
  - Pre-commit hook references correct paths
rule-overrides: []
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-292
    type: depends-on
  - target: TASK-349
    type: depended-on-by
---

## What

Final sweep to update every path reference across the entire codebase after directory reorganization.

## How

1. Run `node tools/verify-links.mjs` to find remaining broken references
2. Search Rust code for old paths (.orqa/process/, .orqa/process/, .orqa/delivery/)
3. Search TypeScript code for old paths
4. Search all .orqa/ artifacts for old paths
5. Fix each reference
6. Re-run verify-links until clean

## Verification

- `make verify` passes clean
- grep for old paths returns zero results
- All symlinks resolve correctly
