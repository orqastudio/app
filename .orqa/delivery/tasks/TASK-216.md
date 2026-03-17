---
id: TASK-216
title: Plugin extends graph on artifact creation
description: "Add PostToolUse logic that detects when new artifacts are created and prompts for proper graph relationships (docs, skills, epic, depends-on fields)."
status: completed
created: 2026-03-12
updated: 2026-03-12
docs:
  - DOC-036
acceptance:
  - New artifacts without relationship fields trigger a warning
  - Warnings suggest specific relationships to add
  - Does not block — advisory only
  - Helps maintain graph integrity passively
relationships:
  - target: EPIC-053
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-020
    type: informed-by
  - target: SKILL-011
    type: informed-by
  - target: TASK-343
    type: depended-on-by
  - target: SKILL-011
    type: informed-by
  - target: SKILL-020
    type: informed-by
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
