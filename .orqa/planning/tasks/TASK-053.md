---
id: TASK-053
title: Extract domain skills from old agents
description: Read each old software-specific agent and extract its domain knowledge into a standalone skill file. These skills will be loaded into universal roles at runtime based on project context.
status: done
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-045
depends-on:
  - TASK-057
  - TASK-051
assignee: AGENT-003
skills:
  - SKILL-011
  - SKILL-029
scope:
  - .orqa/team/skills/
acceptance:
  - diagnostic-methodology skill created (from debugger)
  - restructuring-methodology skill created (from refactor-agent)
  - security-audit skill created (from security-engineer)
  - governance-maintenance skill created (from agent-maintainer)
  - code-quality-review skill created (from code-reviewer)
  - qa-verification skill created (from qa-tester)
  - ux-compliance-review skill created (from ux-reviewer)
  - test-engineering skill created (from test-engineer)
  - architectural-evaluation skill created (from systems-architect)
  - Each skill captures the domain knowledge from its source agent
  - Each skill follows the SKILL.md format with proper frontmatter
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
