---
id: TASK-025
title: Extract remaining command domain logic
description: "Applies the thin-handler pattern established in TASK-024 to the setup, governance, and artifact command files, moving all business logic into dedicated domain and repository modules."
status: completed
created: 2026-03-07
updated: 2026-03-09
assignee: AGENT-002
acceptance:
  - All command files follow thin-handler pattern
  - Domain logic in domain/ modules
  - Data access in repo/ modules
relationships:
  - target: EPIC-039
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-329
    type: depended-on-by
---
## What

Apply the domain service extraction pattern (established in [TASK-024](TASK-024)) to the
remaining command files: setup, governance, and artifact commands.

## Outcome

All command files now follow the thin-command → domain service → repository
pattern. Git commits: `35b6f76`, `e55dd76`, `8750420`, `c60b181`, `e7d4d99`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
