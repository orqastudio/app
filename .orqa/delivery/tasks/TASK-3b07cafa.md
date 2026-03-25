---
id: TASK-3b07cafa
type: task
title: Update rules to reference pillar artifacts generically
description: Update rules to reference pillar artifacts generically
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-4c94fe14
acceptance:
  - vision-alignment.md references pillar artifacts by path
  - not hardcoded names
  - pillar-alignment-docs.md reads pillar titles from artifacts directory
  - "Rules enforce \"serve at least one active pillar\" generically"
  - No hardcoded pillar names remain in enforcement rules
relationships:
  - target: EPIC-0e8860dd
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-bf8bf526
    type: depends-on
  - target: TASK-677a65d4
    type: depended-on-by
  - target: TASK-65903e58
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.