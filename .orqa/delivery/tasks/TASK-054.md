---
id: TASK-f1ada1f5
title: Create project setup skills
description: "Write the four setup skills that replace templates for project initialisation: base scaffolding, folder inference, agentic config migration, and the software project type preset."
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-1dab5ebe
acceptance:
  - KNOW-e0dec720 skill created (universal scaffolding — .orqa/ structure
  - canon rules
  - canon skills)
  - KNOW-82ceb1bd skill created (reads folder
  - produces project profile YAML)
  - KNOW-0fd23e0b skill created (reads existing agentic config
  - maps to OrqaStudio)
  - KNOW-819789ab skill created (software development governance preset)
  - Each skill follows SKILL.md format with proper frontmatter
  - KNOW-e0dec720 knows how to create .orqa/ directory structure
  - KNOW-82ceb1bd knows file patterns for languages
  - frameworks
  - existing governance
  - KNOW-0fd23e0b knows config formats for Claude Code
  - Cursor
  - Copilot
  - Aider
  - KNOW-819789ab knows worktree rules
  - code quality
  - testing standards
relationships:
  - target: EPIC-be023ed2
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-3cb01b3f
    type: depends-on
  - target: TASK-0d8b4a06
    type: depended-on-by
---
## Reference

- [AD-6f0dea5e](AD-6f0dea5e) defines the four setup skills and their responsibilities
- [AD-6f0dea5e](AD-6f0dea5e) Section "The Four Setup Skills" has detailed specs for each

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
