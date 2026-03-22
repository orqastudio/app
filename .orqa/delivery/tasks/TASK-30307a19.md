---
id: TASK-30307a19
title: Create orqa-plugin-development skill (new project + seed data approach)
description: "Create a skill that guides the AI in building OrqaStudio plugins, always in a standalone project with seed data, using the Artifact Graph SDK."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-1dab5ebe
acceptance:
  - Skill instructs AI to always create plugins in a new standalone project
  - Skill includes seed data generation guidance (mock .orqa/ directory)
  - Skill references the Artifact Graph SDK documentation
  - "Skill covers the four plugin layers (built-in, official, community, user)"
  - Skill explains how to install a local plugin via file path
  - Skill references IDEA-b77e2955 for future distribution architecture
relationships:
  - target: EPIC-0a8a5e72
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-db618792
    type: depends-on
  - target: TASK-12eec0f3
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
