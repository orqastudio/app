---
id: "TASK-c12c22b1"
type: "task"
title: "Integrate integrity checks into pre-commit hook"
description: "Add staged-file mode to verify-links.mjs and verify-pipeline-integrity.mjs. Add both to .githooks/pre-commit in the ORQA_CHANGED block. Broken links and missing inverses on staged files block the commit."
status: archived
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
assignee: null
docs: []
acceptance:
  - "verify-links.mjs accepts a --staged flag that checks only staged .orqa/ files"
  - "verify-pipeline-integrity.mjs accepts a --staged flag that checks only staged .orqa/ files"
  - "Both are called from .githooks/pre-commit when .orqa/ files are staged"
  - "Broken links on staged files block the commit"
  - "Missing bidirectional inverses on staged files block the commit"
rule-overrides: []
relationships:
  - target: "EPIC-88f359b0"
    type: "delivers"
    rationale: "Task belongs to this epic"
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
