---
id: TASK-054
title: Create project setup skills
description: "Write the four setup skills that replace templates for project initialisation: base scaffolding, folder inference, agentic config migration, and the software project type preset."
status: done
scope:
  - .orqa/team/skills/
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-045
depends-on:
  - TASK-057
assignee: AGENT-003
skills:
  - SKILL-011
  - SKILL-029
acceptance:
  - project-setup skill created (universal scaffolding — .orqa/ structure
  - canon rules
  - canon skills)
  - project-inference skill created (reads folder
  - produces project profile YAML)
  - project-migration skill created (reads existing agentic config
  - maps to OrqaStudio)
  - project-type-software skill created (software development governance preset)
  - Each skill follows SKILL.md format with proper frontmatter
  - project-setup knows how to create .orqa/ directory structure
  - project-inference knows file patterns for languages
  - frameworks
  - existing governance
  - project-migration knows config formats for Claude Code
  - Cursor
  - Copilot
  - Aider
  - project-type-software knows worktree rules
  - code quality
  - testing standards
---
## Reference

- AD-030 defines the four setup skills and their responsibilities
- AD-030 Section "The Four Setup Skills" has detailed specs for each

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
