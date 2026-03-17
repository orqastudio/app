---
id: TASK-056
title: Update rules for universal roles
description: "Update agent-delegation.md and all other rules that reference old software-specific agent names to use the new universal role names (Implementer, Reviewer, etc.) and skill-based delegation."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - agent-delegation.md rewritten for 7 universal roles instead of 16 agents
  - "Delegation table uses role + skill pattern (e.g. \"Implementer + backend skills\")"
  - Resource safety table updated for universal roles
  - skill-enforcement.md updated with skills mapped to universal roles
  - lessons-learned.md references updated (code-reviewer → Reviewer + code-quality skill)
  - honest-reporting.md references updated
  - No remaining references to deleted agent names in any rule file
relationships:
  - target: EPIC-045
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-052
    type: depends-on
  - target: TASK-053
    type: depends-on
  - target: SKILL-011
    type: informed-by
  - target: TASK-055
    type: depended-on-by
  - target: TASK-335
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
