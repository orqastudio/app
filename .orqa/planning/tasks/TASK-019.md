---
id: TASK-019
title: Governance artifact alignment for dogfooding
status: done
epic: EPIC-037
description: Aligns governance artifacts with the live codebase in preparation for dogfood use, fixing hook paths, removing debug logging, and eliminating unsafe type annotations.
created: 2026-03-05
updated: 2026-03-09
assignee: backend-engineer
skills: [orqa-governance]
scope: [.orqa/, src-tauri/src/commands/]
acceptance: [Hook paths use $CLAUDE_PROJECT_DIR, Governance artifacts match codebase state, Frontend debug logging removed, any types fixed]
---

## What

Align governance artifacts with the running codebase to prepare for dogfooding.
Fix frontend audit findings (debug logging, any types).

## Outcome

Governance artifacts updated, hook paths fixed, frontend cleaned. Git commits:
`1481f00`, `08a74bf`, `5b2d50a`.
