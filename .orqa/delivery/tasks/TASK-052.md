---
id: TASK-052
title: Remove old software-specific agents
description: Delete the 14 old agent files that have been merged into universal roles. Update all cross-references in rules, skills, epics, and documentation that mention old agent names.
status: done
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-045
depends-on:
  - TASK-051
  - TASK-053
assignee: AGENT-003
skills:
  - SKILL-011
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
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
