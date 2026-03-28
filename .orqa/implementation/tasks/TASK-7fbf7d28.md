---
id: "TASK-7fbf7d28"
type: "task"
title: "Pre-commit hook enforcement verification"
description: "Verify that the pre-commit hook catches all documented coding standard violations. Test each enforcement path and fix any gaps."
status: ready
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
acceptance:
  - "Pre-commit hook tested with intentional violations for each documented standard"
  - "Every documented standard that should be caught by pre-commit is caught"
  - "Any gaps between documented standards and hook enforcement are fixed"
  - "Hook runs the correct subset of checks based on staged file types"
  - "Hook does not silently pass when it should fail"
relationships:
  - target: "EPIC-e24086ed"
    type: "delivers"
  - target: "TASK-4a9f0681"
    type: "depends-on"
---

## What

Verify the pre-commit hook (`.githooks/pre-commit`) is a reliable enforcement gate for all documented coding standards per RULE-0be7765e (error-ownership) and RULE-42d17086 (tooling-ecosystem).

## How

1. Read `.githooks/pre-commit` and map which checks run for which file types
2. For each documented standard in coding-standards.md:
   - Determine if it should be caught by pre-commit
   - Create a deliberate violation in a test file
   - Stage the file and run the hook
   - Verify the hook catches the violation
3. Test edge cases:
   - Mixed file types staged (Rust + TypeScript)
   - Only documentation files staged (should skip code checks)
   - Schema validation on `.orqa/*.md` files
4. Fix any gaps found

## Verification

1. Every testable standard has been verified against the hook
2. Gap report shows zero unresolved enforcement gaps
3. `make check` passes after any hook modifications
