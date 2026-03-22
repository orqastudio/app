---
id: TASK-76849c1d
title: Governance artifact alignment for dogfooding
description: "Aligns governance artifacts with the live codebase in preparation for dogfood use, fixing hook paths, removing debug logging, and eliminating unsafe type annotations."
status: completed
created: 2026-03-05
updated: 2026-03-09
assignee: AGENT-cc255bc8
acceptance:
  - Hook paths use $CLAUDE_PROJECT_DIR
  - Governance artifacts match codebase state
  - Frontend debug logging removed
  - any types fixed
relationships:
  - target: EPIC-c0ab7529
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-428a887f
    type: depended-on-by
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
