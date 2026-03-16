---
id: TASK-214
title: Plugin reads task docs and skills fields for context injection
description: "Update the companion plugin to read task docs and skills fields from YAML frontmatter, loading referenced documentation and skills into agent context automatically."
status: completed
created: 2026-03-12
updated: 2026-03-12
docs: []
acceptance:
  - Plugin loads docs listed in task.docs into agent context
  - Plugin loads skills listed in task.skills
  - No duplicate injection within a session
  - Works with existing plugin infrastructure from EPIC-050
relationships:
  - target: EPIC-053
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-208
    type: depends-on
  - target: SKILL-020
    type: grounded-by
  - target: SKILL-011
    type: grounded-by
  - target: TASK-219
    type: depended-on-by
  - target: TASK-220
    type: depended-on-by
  - target: TASK-343
    type: depended-on-by
---
## What

The companion plugin becomes the primary consumer of graph edges. When an agent starts working on a task, the plugin reads the task's `docs` and `skills` fields and automatically injects that context — no hardcoded injection table needed.

## How

1. On SessionStart, check session state for current task ID
2. Read the task's frontmatter from `.orqa/delivery/tasks/TASK-NNN.md`
3. For each entry in `docs`: read the file and inject as `systemMessage`
4. For each entry in `skills`: load the skill via the plugin's skill mechanism
5. Follow `task.epic` → read epic for design context
6. Track what's been injected to prevent duplicates

## Verification

- Plugin reads task frontmatter docs and skills arrays
- Referenced docs are injected as system context
- Referenced skills are loaded into agent context
- No duplicate injection in the same session
