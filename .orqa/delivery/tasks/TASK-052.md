---
id: TASK-052
title: Remove old software-specific agents
description: "Delete the 14 old agent files that have been merged into universal roles. Update all cross-references in rules, skills, epics, and documentation that mention old agent names."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - 14 old agent files deleted (backend-engineer
  - frontend-engineer
  - data-engineer
  - devops-engineer
  - systems-architect
  - test-engineer
  - code-reviewer
  - qa-tester
  - ux-reviewer
  - security-engineer
  - debugger
  - refactor-agent
  - agent-maintainer
  - documentation-writer)
  - No broken references to old agent names in rules
  - No broken references to old agent names in skills
  - No broken references to old agent names in orchestrator.md
  - All references updated to use universal role names
relationships:
  - target: EPIC-045
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-051
    type: depends-on
  - target: TASK-053
    type: depends-on
  - target: SKILL-011
    type: grounded-by
  - target: TASK-055
    type: depended-on-by
  - target: TASK-056
    type: depended-on-by
  - target: TASK-335
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
