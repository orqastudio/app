---
id: TASK-084
title: Audit all agent definitions for accuracy
description: Verify every agent YAML definition in .orqa/team/agents/ has correct skills lists, valid required reading paths, accurate role descriptions, and no stale references.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Read every agent file in .orqa/team/agents/
  - Verify skills list references exist in .orqa/team/skills/
  - Verify Required Reading paths resolve to existing files
  - Check role descriptions match AD-029 universal role model
  - Fix stale references (old names, deprecated concepts, wrong paths)
  - Verify delegation instructions match available subagent types
acceptance:
  - All agent skills references point to existing skill directories
  - All Required Reading paths resolve to existing files
  - No references to deprecated concepts (plans, Forge, decisions.md index)
  - Role descriptions align with AD-029
---
## What

Systematic audit of all agent definition files to ensure internal consistency and accuracy against the current codebase and governance framework.

## How

1. List all `.md` files in `.orqa/team/agents/`
2. For each agent, read and verify skills, required reading, role description
3. Cross-reference with `.orqa/team/skills/` directory contents
4. Fix any broken references or stale content

## Verification

- `grep -r "plans\|Forge\|decisions\.md" .orqa/team/agents/` returns no results
- Every skill in every agent's `skills:` list has a matching directory in `.orqa/team/skills/`
- Every Required Reading path exists on disk
