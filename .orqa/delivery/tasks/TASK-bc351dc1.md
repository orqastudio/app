---
id: "TASK-bc351dc1"
type: "task"
title: "Create project setup skills"
description: "Write the four setup skills that replace templates for project initialisation: base scaffolding, folder inference, agentic config migration, and the software project type preset."
status: "completed"
created: "2026-03-09"
updated: "2026-03-09"
assignee: "AGENT-4c94fe14"
acceptance:
  - "KNOW-2876afc7 skill created (universal scaffolding — .orqa/ structure"
  - "canon rules"
  - "canon skills)"
  - "KNOW-03421ec0 skill created (reads folder"
  - "produces project profile YAML)"
  - "KNOW-4a58e7dd skill created (reads existing agentic config"
  - "maps to OrqaStudio)"
  - "KNOW-d03337ac skill created (software development governance preset)"
  - "Each skill follows SKILL.md format with proper frontmatter"
  - "KNOW-2876afc7 knows how to create .orqa/ directory structure"
  - "KNOW-03421ec0 knows file patterns for languages"
  - "frameworks"
  - "existing governance"
  - "KNOW-4a58e7dd knows config formats for Claude Code"
  - "Cursor"
  - "Copilot"
  - "Aider"
  - "KNOW-d03337ac knows worktree rules"
  - "code quality"
  - "testing standards"
relationships:
  - target: "EPIC-7394ba2a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-8c0c77b0"
    type: "depends-on"
---
## Reference

- [AD-26b0eb9f](AD-26b0eb9f) defines the four setup skills and their responsibilities
- [AD-26b0eb9f](AD-26b0eb9f) Section "The Four Setup Skills" has detailed specs for each

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.