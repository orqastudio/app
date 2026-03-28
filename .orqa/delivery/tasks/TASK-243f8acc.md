---
id: "TASK-243f8acc"
type: "task"
title: "Implement Stop hook (replaces pre-commit-reminder.sh)"
description: "Plugin Stop hook replaces the shell-script pre-commit reminder with a structured hook."
status: archived
created: 2026-03-11T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
docs: []
acceptance:
  - "Stop hook fires when agent is about to stop"
  - "Hook provides pre-commit checklist as additionalContext"
  - "Hook replaces .claude/hooks/pre-commit-reminder.sh functionality"
relationships:
  - target: "EPIC-9a1eba3f"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-15cb18ee"
    type: "depends-on"
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
