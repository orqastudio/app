---
id: TASK-214
title: Plugin reads task docs and skills fields for context injection
description: Update the companion plugin to read task docs and skills fields from YAML frontmatter, loading referenced documentation and skills into agent context automatically.
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-053
depends-on:
  - TASK-208
docs: []
skills:
  - orqa-plugin-development
  - rule-enforcement
  - orqa-governance
scope:
  - Plugin SessionStart/PreToolUse reads current task from session state
  - Parse task's docs field → read each doc and inject as context
  - Parse task's skills field → load each skill
  - Follow task.epic edge → load epic context
  - Dedup injection per session
acceptance:
  - Plugin loads docs listed in task.docs into agent context
  - Plugin loads skills listed in task.skills
  - No duplicate injection within a session
  - Works with existing plugin infrastructure from EPIC-050
---
## What

The companion plugin becomes the primary consumer of graph edges. When an agent starts working on a task, the plugin reads the task's `docs` and `skills` fields and automatically injects that context — no hardcoded injection table needed.

## How

1. On SessionStart, check session state for current task ID
2. Read the task's frontmatter from `.orqa/planning/tasks/TASK-NNN.md`
3. For each entry in `docs`: read the file and inject as `systemMessage`
4. For each entry in `skills`: load the skill via the plugin's skill mechanism
5. Follow `task.epic` → read epic for design context
6. Track what's been injected to prevent duplicates

## Verification

- Plugin reads task frontmatter docs and skills arrays
- Referenced docs are injected as system context
- Referenced skills are loaded into agent context
- No duplicate injection in the same session
