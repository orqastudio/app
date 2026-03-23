---
id: TASK-b6e9df91
type: task
title: Audit all agent definitions for accuracy
description: "Verify every agent YAML definition in .orqa/process/agents/ has correct skills lists, valid required reading paths, accurate role descriptions, and no stale references."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - All agent skills references point to existing skill directories
  - All Required Reading paths resolve to existing files
  - "No references to deprecated concepts (plans, Forge, decisions.md index)"
  - Role descriptions align with AD-774cc3d0
relationships:
  - target: EPIC-4a7aeacb
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-1ac4d16f
    type: depended-on-by
  - target: TASK-77b6e5bd
    type: depended-on-by
  - target: TASK-4a8c3c6a
    type: depended-on-by
  - target: TASK-ec136ce9
    type: depended-on-by
---
## What

Systematic audit of all agent definition files to ensure internal consistency and accuracy against the current codebase and governance framework.

## How

1. List all `.md` files in `.orqa/process/agents/`
2. For each agent, read and verify skills, required reading, role description
3. Cross-reference with `.orqa/process/skills/` directory contents
4. Fix any broken references or stale content

## Verification

- `grep -r "plans\|Forge\|decisions\.md" .orqa/process/agents/` returns no results
- Every skill in every agent's `skills:` list has a matching directory in `.orqa/process/skills/`
- Every Required Reading path exists on disk