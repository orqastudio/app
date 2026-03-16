---
id: TASK-219
title: End-to-end test — task with docs/skills triggers correct context injection
description: Verify that a task with docs and skills fields causes the plugin to inject the correct documentation and skills into agent context.
status: completed
created: 2026-03-12
updated: 2026-03-12
docs: []
acceptance:
  - Docs listed in task.docs are injected into agent context
  - Skills listed in task.skills are loaded
  - Epic context from task.epic is available
  - No duplicate injections
relationships:
  - target: EPIC-053
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-214
    type: depends-on
  - target: TASK-215
    type: depends-on
  - target: SKILL-020
    type: grounded-by
  - target: SKILL-018
    type: grounded-by
  - target: TASK-343
    type: depended-on-by
---
## What

Verify the full graph-based injection pipeline works end-to-end: a task with `docs` and `skills` fields causes the plugin to automatically inject the correct context.

## How

1. Create a test task with specific docs and skills fields
2. Start a session referencing that task
3. Verify the plugin reads and injects the docs
4. Verify the plugin loads the skills
5. Trigger a second injection for the same paths — verify dedup works

## Verification

- Docs from task.docs appear in agent context
- Skills from task.skills are loaded
- Epic context from task.epic is available
- No duplicate injections in the same session
