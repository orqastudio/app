---
id: TASK-053
title: Extract domain skills from old agents
description: Read each old software-specific agent and extract its domain knowledge into a standalone skill file. These skills will be loaded into universal roles at runtime based on project context.
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - SKILL-006 skill created (from debugger)
  - SKILL-026 skill created (from refactor-agent)
  - SKILL-028 skill created (from security-engineer)
  - SKILL-007 skill created (from agent-maintainer)
  - SKILL-004 skill created (from code-reviewer)
  - SKILL-025 skill created (from qa-tester)
  - SKILL-036 skill created (from ux-reviewer)
  - SKILL-033 skill created (from test-engineer)
  - SKILL-001 skill created (from systems-architect)
  - Each skill captures the domain knowledge from its source agent
  - Each skill follows the SKILL.md format with proper frontmatter
relationships:
  - target: EPIC-045
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-057
    type: depends-on
  - target: TASK-051
    type: depends-on
  - target: SKILL-011
    type: grounded-by
  - target: SKILL-029
    type: grounded-by
  - target: TASK-052
    type: depended-on-by
  - target: TASK-056
    type: depended-on-by
  - target: TASK-335
    type: depended-on-by
---
## Reference

- [AD-029](AD-029) migration table shows which agents become which skills
- universal-roles-ownership.md has the extraction mapping

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
