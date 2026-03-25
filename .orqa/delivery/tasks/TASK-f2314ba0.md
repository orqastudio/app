---
id: "TASK-f2314ba0"
type: "task"
title: "Create artifact audit skill"
description: "Create a reusable skill that captures the methodology, checklists, and patterns for auditing .orqa/ artifacts — enabling future audits to be systematic and repeatable without rediscovering the process each time."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
acceptance:
  - "SKILL.md exists in .orqa/process/skills/orqa-artifact-audit/"
  - "Skill covers all artifact types in .orqa/"
  - "Skill includes verification checklists that an agent can follow"
  - "Skill is referenced in relevant agent definitions (reviewer, orchestrator)"
relationships:
  - target: "EPIC-5aa11e2f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-1637bc63"
    type: "depends-on"
  - target: "TASK-d6030100"
    type: "depends-on"
  - target: "TASK-88e72cc1"
    type: "depends-on"
  - target: "TASK-81b11647"
    type: "depends-on"
  - target: "TASK-3109164e"
    type: "depends-on"
---
## What

Create a skill that captures the full artifact audit methodology so future audits are systematic, repeatable, and don't require rediscovering the process.

## How

1. Review the audit process from [EPIC-d45b4dfd](EPIC-d45b4dfd) (planning artifacts) and [EPIC-5aa11e2f](EPIC-5aa11e2f) (team/enforcement)
2. Extract the common patterns: status verification, cross-reference checks, path consistency, codebase alignment
3. Organize into per-artifact-type checklists
4. Write as a SKILL.md with clear sections for each audit dimension
5. Add to relevant agent `skills:` lists

## Verification

- Skill file exists and follows SKILL.md format
- A reviewer agent loading this skill can execute a full audit without additional context
- All artifact types are covered