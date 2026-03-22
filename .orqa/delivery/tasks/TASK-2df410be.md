---
id: TASK-2df410be
title: Implement agent and skill loading from .orqa/process/
description: "Plugin loads agent definitions and skills directly from .orqa/process/, replacing .claude/ symlinks."
status: completed
created: 2026-03-11
updated: 2026-03-12
assignee: AGENT-cc255bc8
docs: []
acceptance:
  - Plugin exposes .orqa/process/agents/ as plugin agents directory
  - Plugin exposes .orqa/process/skills/ as plugin skills directory
  - Agent capabilities are resolved to CLI tool names per RULE-2f7b6a31
  - Skills are discoverable via Claude Code skill system
  - No .claude/ symlinks required for agent or skill loading
relationships:
  - target: EPIC-3a8ad459
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-869c27b5
    type: depends-on
  - target: TASK-f936b9b2
    type: depends-on
  - target: TASK-11cf4c1d
    type: depended-on-by
  - target: TASK-fa39671d
    type: depended-on-by
---

## What

The plugin becomes the bridge between `.orqa/process/` and Claude Code, replacing
the symlink architecture. It reads agent definitions and skills directly and
exposes them to Claude Code's discovery system.

## How

1. Configure plugin to expose `.orqa/process/agents/` as the agents directory
2. Configure plugin to expose `.orqa/process/skills/` as the skills directory
3. In SubagentStart hook, resolve agent capabilities to CLI tools (TASK-027139e7)
4. Verify Claude Code discovers agents and skills without symlinks

## Verification

- `/agents` shows agents from `.orqa/process/agents/`
- Skills are loadable via `/skill-name`
- Removing `.claude/agents/` and `.claude/skills/` symlinks doesn't break anything
