---
id: TASK-089
title: Create artifact audit skill
description: "Create a reusable skill that captures the methodology, checklists, and patterns for auditing .orqa/ artifacts — enabling future audits to be systematic and repeatable without rediscovering the process each time."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - SKILL.md exists in .orqa/process/skills/orqa-artifact-audit/
  - Skill covers all artifact types in .orqa/
  - Skill includes verification checklists that an agent can follow
  - "Skill is referenced in relevant agent definitions (reviewer, orchestrator)"
relationships:
  - target: EPIC-049
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-084
    type: depends-on
  - target: TASK-085
    type: depends-on
  - target: TASK-086
    type: depends-on
  - target: TASK-087
    type: depends-on
  - target: TASK-088
    type: depends-on
  - target: TASK-339
    type: depended-on-by
---
## What

Create a skill that captures the full artifact audit methodology so future audits are systematic, repeatable, and don't require rediscovering the process.

## How

1. Review the audit process from [EPIC-048](EPIC-048) (planning artifacts) and [EPIC-049](EPIC-049) (team/enforcement)
2. Extract the common patterns: status verification, cross-reference checks, path consistency, codebase alignment
3. Organize into per-artifact-type checklists
4. Write as a SKILL.md with clear sections for each audit dimension
5. Add to relevant agent `skills:` lists

## Verification

- Skill file exists and follows SKILL.md format
- A reviewer agent loading this skill can execute a full audit without additional context
- All artifact types are covered
