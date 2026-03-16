---
id: TASK-281
title: Integrate integrity checks into pre-commit hook
description: Add staged-file mode to verify-links.mjs and verify-pipeline-integrity.mjs. Add both to .githooks/pre-commit in the ORQA_CHANGED block. Broken links and missing inverses on staged files block the commit.
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - verify-links.mjs accepts a --staged flag that checks only staged .orqa/ files
  - verify-pipeline-integrity.mjs accepts a --staged flag that checks only staged .orqa/ files
  - Both are called from .githooks/pre-commit when .orqa/ files are staged
  - Broken links on staged files block the commit
  - Missing bidirectional inverses on staged files block the commit
rule-overrides: []
relationships:
  - target: EPIC-059
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-283
    type: depended-on-by
  - target: TASK-284
    type: depended-on-by
  - target: TASK-349
    type: depended-on-by
---

## What

Add staged-file mode to the existing verification tools and wire them into the pre-commit hook.

## How

1. Modify `tools/verify-links.mjs` to accept `--staged` flag — reads `git diff --cached --name-only` and only checks those files
2. Modify `tools/verify-pipeline-integrity.mjs` to accept `--staged` flag — same approach
3. Add both to `.githooks/pre-commit` in the ORQA_CHANGED block alongside existing schema validation

## Verification

- Stage a file with a broken link, commit should fail
- Stage a file with a missing inverse, commit should fail
- Normal commits without .orqa/ changes pass without running these checks
