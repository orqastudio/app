---
id: TASK-054
title: Create project setup skills
description: "Write the four setup skills that replace templates for project initialisation: base scaffolding, folder inference, agentic config migration, and the software project type preset."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - SKILL-023 skill created (universal scaffolding — .orqa/ structure
  - canon rules
  - canon skills)
  - SKILL-021 skill created (reads folder
  - produces project profile YAML)
  - SKILL-022 skill created (reads existing agentic config
  - maps to OrqaStudio)
  - SKILL-024 skill created (software development governance preset)
  - Each skill follows SKILL.md format with proper frontmatter
  - SKILL-023 knows how to create .orqa/ directory structure
  - SKILL-021 knows file patterns for languages
  - frameworks
  - existing governance
  - SKILL-022 knows config formats for Claude Code
  - Cursor
  - Copilot
  - Aider
  - SKILL-024 knows worktree rules
  - code quality
  - testing standards
relationships:
  - target: EPIC-045
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-057
    type: depends-on
  - target: SKILL-011
    type: grounded-by
  - target: SKILL-029
    type: grounded-by
  - target: TASK-335
    type: depended-on-by
---
## Reference

- [AD-030](AD-030) defines the four setup skills and their responsibilities
- [AD-030](AD-030) Section "The Four Setup Skills" has detailed specs for each

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
