---
id: TASK-182
title: Implement Stop hook (replaces pre-commit-reminder.sh)
description: Plugin Stop hook replaces the shell-script pre-commit reminder with a structured hook.
status: completed
created: 2026-03-11
updated: 2026-03-12
assignee: AGENT-002
docs: []
acceptance:
  - Stop hook fires when agent is about to stop
  - Hook provides pre-commit checklist as additionalContext
  - Hook replaces .claude/hooks/pre-commit-reminder.sh functionality
relationships:
  - target: EPIC-050
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-177
    type: depends-on
  - target: SKILL-020
    type: grounded-by
  - target: TASK-185
    type: depended-on-by
  - target: TASK-340
    type: depended-on-by
---

## What

The Stop hook replaces the shell-script pre-commit reminder. When the agent is
about to stop, it injects a checklist reminding the agent to commit, update
session state, and clean up.

## How

1. Create `hooks/stop.md` hook definition
2. On Stop event, build pre-commit checklist from governance rules
3. Return checklist as additionalContext

## Verification

- Agent stop shows pre-commit checklist
- Removing `.claude/hooks/pre-commit-reminder.sh` doesn't lose functionality
