---
id: TASK-019
title: Governance artifact alignment for dogfooding
description: Aligns governance artifacts with the live codebase in preparation for dogfood use, fixing hook paths, removing debug logging, and eliminating unsafe type annotations.
status: done
created: "2026-03-05"
updated: "2026-03-09"
epic: EPIC-037
assignee: AGENT-002
skills:
  - SKILL-011
acceptance:
  - Hook paths use $CLAUDE_PROJECT_DIR
  - Governance artifacts match codebase state
  - Frontend debug logging removed
  - any types fixed
---
## What

Align governance artifacts with the running codebase to prepare for dogfooding.
Fix frontend audit findings (debug logging, any types).

## Outcome

Governance artifacts updated, hook paths fixed, frontend cleaned. Git commits:
`1481f00`, `08a74bf`, `5b2d50a`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
