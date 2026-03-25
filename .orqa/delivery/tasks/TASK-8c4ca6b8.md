---
id: "TASK-8c4ca6b8"
type: "task"
title: "Remove old software-specific agents"
description: "Delete the 14 old agent files that have been merged into universal roles. Update all cross-references in rules, skills, epics, and documentation that mention old agent names."
status: "completed"
created: 2026-03-09T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
assignee: "AGENT-4c94fe14"
acceptance:
  - "14 old agent files deleted (backend-engineer"
  - "frontend-engineer"
  - "data-engineer"
  - "devops-engineer"
  - "systems-architect"
  - "test-engineer"
  - "code-reviewer"
  - "qa-tester"
  - "ux-reviewer"
  - "security-engineer"
  - "debugger"
  - "refactor-agent"
  - "agent-maintainer"
  - "documentation-writer)"
  - "No broken references to old agent names in rules"
  - "No broken references to old agent names in skills"
  - "No broken references to old agent names in orchestrator.md"
  - "All references updated to use universal role names"
relationships:
  - target: "EPIC-7394ba2a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-0a4a9172"
    type: "depends-on"
  - target: "TASK-4023ac04"
    type: "depends-on"
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.