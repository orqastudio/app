---
id: TASK-081
title: Create orqa-plugin-development skill (new project + seed data approach)
description: "Create a skill that guides the AI in building OrqaStudio plugins, always in a standalone project with seed data, using the Artifact Graph SDK."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-003
acceptance:
  - Skill instructs AI to always create plugins in a new standalone project
  - Skill includes seed data generation guidance (mock .orqa/ directory)
  - Skill references the Artifact Graph SDK documentation
  - "Skill covers the four plugin layers (built-in, official, community, user)"
  - Skill explains how to install a local plugin via file path
  - Skill references IDEA-038 for future distribution architecture
relationships:
  - target: EPIC-048
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-080
    type: depends-on
  - target: TASK-338
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
