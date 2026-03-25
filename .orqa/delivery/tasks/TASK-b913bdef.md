---
id: TASK-b913bdef
type: task
title: Governance artifact alignment for dogfooding
description: "Aligns governance artifacts with the live codebase in preparation for dogfood use, fixing hook paths, removing debug logging, and eliminating unsafe type annotations."
status: completed
created: 2026-03-05
updated: 2026-03-09
assignee: AGENT-e5dd38e4
acceptance:
  - Hook paths use $CLAUDE_PROJECT_DIR
  - Governance artifacts match codebase state
  - Frontend debug logging removed
  - any types fixed
relationships:
  - target: EPIC-63ff87da
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-39504e66
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