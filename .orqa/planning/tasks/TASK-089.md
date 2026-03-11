---
id: TASK-089
title: Create artifact audit skill
description: Create a reusable skill that captures the methodology, checklists, and patterns for auditing .orqa/ artifacts — enabling future audits to be systematic and repeatable without rediscovering the process each time.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on:
  - TASK-084
  - TASK-085
  - TASK-086
  - TASK-087
  - TASK-088
scope:
  - Encode the audit methodology used across EPIC-048 and EPIC-049 into a portable skill
  - Include checklists for each artifact type (research, epics, tasks, ideas, decisions, lessons, agents, skills, rules, hooks, pillars, milestones, READMEs)
  - Include cross-reference verification patterns
  - Include status accuracy verification patterns
  - Include path and naming consistency checks
  - Include the systemic pattern grouping approach from RES-035
acceptance:
  - SKILL.md exists in .orqa/team/skills/artifact-audit/
  - Skill covers all artifact types in .orqa/
  - Skill includes verification checklists that an agent can follow
  - Skill is referenced in relevant agent definitions (reviewer, orchestrator)
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
