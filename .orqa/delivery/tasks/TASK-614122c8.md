---
id: TASK-614122c8
title: Fix flow — auto-fix button and results display in IntegrityWidget
description: "Add a Fix button to the IntegrityWidget that calls apply_auto_fixes, shows which fixes were applied, and re-scans to show remaining issues."
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - Fix button appears when auto-fixable issues exist
  - "Clicking Fix calls apply_auto_fixes, shows results, then re-scans"
  - Applied fixes shown as a success list before the remaining issues
  - Fix button disabled when no auto-fixable issues or while applying
  - make typecheck passes
relationships:
  - target: EPIC-e37794bf
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-ee4681f4
    type: depends-on
  - target: TASK-5fc0e2dc
    type: depends-on
  - target: TASK-d624db8f
    type: depended-on-by
---

## What

Wire the Fix action in the dashboard widget to the auto-fix backend command.

## How

1. Add AppliedFix type to TypeScript types
2. Add applyAutoFixes() method to SDK
3. Add Fix button and results display to IntegrityWidget

## Verification

- `make typecheck` passes
- Fix button appears when MissingInverse issues are present
- After clicking Fix, results shown and widget re-scans

## Lessons

(none yet)
