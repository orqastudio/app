---
id: "TASK-77bfa7e1"
type: "task"
title: "Add docs and skills fields to task schema"
description: "Extend the task schema with optional docs (array of documentation paths) and skills (array of skill names) fields that create graph edges from tasks to their implementation context."
status: archived
created: "2026-03-12"
updated: "2026-03-12"
acceptance:
  - "Task schema accepts `docs` and `skills` arrays"
  - "Existing tasks without these fields still validate"
  - "Schema validator passes with and without the new fields"
  - "Artifact framework documentation reflects the new fields"
relationships:
  - target: "EPIC-f079c196"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Add two new optional fields to the task YAML frontmatter schema:

- **`docs`**: Array of documentation file paths that should be loaded into agent context during implementation of this task
- **`skills`**: Array of skill names that should be loaded into agent context during implementation

These fields create explicit graph edges from tasks to their implementation context, replacing the hardcoded injection table in the orchestrator prompt.

## How

1. Edit `.orqa/implementation/tasks/schema.json` — add both fields as optional string arrays
2. Update `.orqa/documentation/about/artifact-framework.md` — add fields to Task schema
3. Verify existing tasks still validate with `node .githooks/validate-schema.mjs`

## Verification

- Schema validator accepts tasks with and without docs/skills fields
- Existing task [TASK-30045ad8](TASK-30045ad8) validates without docs/skills
- Artifact framework docs updated with new field descriptions
