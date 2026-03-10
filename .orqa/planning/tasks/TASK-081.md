---
id: TASK-081
title: Create plugin-development skill (new project + seed data approach)
description: Create a skill that guides the AI in building OrqaStudio plugins, always in a standalone project with seed data, using the Artifact Graph SDK.
status: done
scope:
  - .orqa/team/skills/plugin-development/SKILL.md
created: "2026-03-10"
updated: "2026-03-10"
epic: EPIC-048
depends-on:
  - TASK-080
assignee: AGENT-003
skills:
  - SKILL-029
  - SKILL-011
acceptance:
  - Skill instructs AI to always create plugins in a new standalone project
  - Skill includes seed data generation guidance (mock .orqa/ directory)
  - Skill references the Artifact Graph SDK documentation
  - Skill covers the four plugin layers (built-in, official, community, user)
  - Skill explains how to install a local plugin via file path
  - Skill references IDEA-038 for future distribution architecture
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
