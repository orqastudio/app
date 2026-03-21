---
id: "TASK-8b482a0a"
title: "Extract remaining command domain logic"
description: "Applies the thin-handler pattern established in TASK-320eb399 to the setup, governance, and artifact command files, moving all business logic into dedicated domain and repository modules."
status: "completed"
created: "2026-03-07"
updated: "2026-03-09"
assignee: "AGENT-cc255bc8"
acceptance:
  - "All command files follow thin-handler pattern"
  - "Domain logic in domain/ modules"
  - "Data access in repo/ modules"
relationships:
  - target: "EPIC-897bbe8f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-fca01488"
    type: "depended-on-by"
---
## What

Apply the domain service extraction pattern (established in [TASK-320eb399](TASK-320eb399)) to the
remaining command files: setup, governance, and artifact commands.

## Outcome

All command files now follow the thin-command → domain service → repository
pattern. Git commits: `35b6f76`, `e55dd76`, `8750420`, `c60b181`, `e7d4d99`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
