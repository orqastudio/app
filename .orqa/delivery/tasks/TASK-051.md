---
id: TASK-051
title: Create universal agent definitions
description: "Write the 4 new universal role agent files (researcher, planner, implementer, reviewer), rename documentation-writer to writer, and broaden designer to cover experience/interface/structure design beyond just UI."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - researcher.md exists with universal investigation role definition
  - planner.md exists with universal approach design role definition
  - implementer.md exists with universal building role definition
  - reviewer.md exists with universal quality verification role definition
  - writer.md exists (renamed from documentation-writer.md)
  - designer.md broadened beyond UI-only to experience/interface/structure
  - All new agents include skills list
  - required reading
  - ownership boundaries
  - Claude Code subagent_type mapping documented per role
relationships:
  - target: EPIC-045
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-057
    type: depends-on
  - target: TASK-052
    type: depended-on-by
  - target: TASK-053
    type: depended-on-by
  - target: TASK-335
    type: depended-on-by
---
## Reference

- [AD-029](AD-029) defines the 7 universal roles and migration path
- universal-roles-ownership.md has the ownership boundaries and subagent mapping
- Orchestrator already restructured (Section 1 + 2) — not in scope here

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
