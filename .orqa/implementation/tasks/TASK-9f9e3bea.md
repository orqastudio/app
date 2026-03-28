---
id: "TASK-9f9e3bea"
type: "task"
title: "Plugin extends graph on artifact creation"
description: "Add PostToolUse logic that detects when new artifacts are created and prompts for proper graph relationships (docs, skills, epic, depends-on fields)."
status: archived
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
docs:
  - "DOC-28344cd7"
acceptance:
  - "New artifacts without relationship fields trigger a warning"
  - "Warnings suggest specific relationships to add"
  - "Does not block — advisory only"
  - "Helps maintain graph integrity passively"
relationships:
  - target: "EPIC-f079c196"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

The self-building loop requires that new artifacts are created with proper relationships. This PostToolUse hook gently enforces graph integrity by warning when an artifact is created without the relationships that make it discoverable.

Example: Creating a new TASK without `docs` or `skills` fields triggers: "This task has no docs or skills edges. Consider adding documentation and skill references so agents get the right context during implementation."

## How

1. PostToolUse hook fires after Write/Edit to `.orqa/` paths
2. Parse the written file's frontmatter
3. Check for expected relationship fields based on artifact type
4. If fields are missing, return `additionalContext` with suggestions

## Verification

- Creating a task without docs/skills triggers a suggestion
- Creating an epic without research-refs triggers a suggestion
- Advisory only — does not block the tool call
