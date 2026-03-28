---
id: "TASK-39c5ac3f"
type: "task"
title: "Update rules for universal roles"
description: "Update agent-delegation.md and all other rules that reference old software-specific agent names to use the new universal role names (Implementer, Reviewer, etc.) and skill-based delegation."
status: archived
created: 2026-03-09T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
acceptance:
  - "agent-delegation.md rewritten for 7 universal roles instead of 16 agents"
  - "Delegation table uses role + skill pattern (e.g. \"Implementer + backend skills\")"
  - "Resource safety table updated for universal roles"
  - "skill-enforcement.md updated with skills mapped to universal roles"
  - "lessons-learned.md references updated (code-reviewer → Reviewer + code-quality skill)"
  - "honest-reporting.md references updated"
  - "No remaining references to deleted agent names in any rule file"
relationships:
  - target: "EPIC-7394ba2a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-8c4ca6b8"
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
