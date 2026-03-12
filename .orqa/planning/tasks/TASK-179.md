---
id: TASK-179
title: Implement agent and skill loading from .orqa/team/
description: Plugin loads agent definitions and skills directly from .orqa/team/, replacing .claude/ symlinks.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-050
depends-on:
  - TASK-177
  - TASK-173
assignee: AGENT-002
docs: []
skills:
  - orqa-plugin-development
  - orqa-governance
scope:
  - orqa-plugin/hooks/
  - orqa-plugin/agents/
  - orqa-plugin/skills/
acceptance:
  - Plugin exposes .orqa/team/agents/ as plugin agents directory
  - Plugin exposes .orqa/team/skills/ as plugin skills directory
  - Agent capabilities are resolved to CLI tool names per RULE-034
  - Skills are discoverable via Claude Code skill system
  - No .claude/ symlinks required for agent or skill loading
---

## What

The plugin becomes the bridge between `.orqa/team/` and Claude Code, replacing
the symlink architecture. It reads agent definitions and skills directly and
exposes them to Claude Code's discovery system.

## How

1. Configure plugin to expose `.orqa/team/agents/` as the agents directory
2. Configure plugin to expose `.orqa/team/skills/` as the skills directory
3. In SubagentStart hook, resolve agent capabilities to CLI tools (TASK-175)
4. Verify Claude Code discovers agents and skills without symlinks

## Verification

- `/agents` shows agents from `.orqa/team/agents/`
- Skills are loadable via `/skill-name`
- Removing `.claude/agents/` and `.claude/skills/` symlinks doesn't break anything
