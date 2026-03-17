---
id: TASK-065
title: Update pre-commit-reminder hook with commit prompt
description: Update pre-commit-reminder hook with commit prompt
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - Stop hook checks for uncommitted changes
  - If uncommitted changes exist
  - prompts to commit before ending session
  - Suggests logical commit groupings based on changed file paths
relationships:
  - target: EPIC-047
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-011
    type: informed-by
  - target: TASK-068
    type: depended-on-by
  - target: TASK-337
    type: depended-on-by
  - target: SKILL-011
    type: informed-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
