---
id: TASK-184
title: Implement /orqa, /orqa:rules, /orqa:status commands
description: Plugin slash commands for governance interaction in Claude Code.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-050
depends-on:
  - TASK-178
assignee: AGENT-002
docs: []
skills:
  - orqa-plugin-development
  - orqa-governance
scope:
  - orqa-plugin/commands/
acceptance:
  - /orqa shows governance summary (active rules, recent violations, health)
  - /orqa:rules lists all active rules with enforcement status
  - /orqa:status shows governance health (rule coverage, broken refs, schema compliance)
  - Commands are discoverable via Claude Code skill system
---

## What

Slash commands give the user direct governance interaction from within Claude Code.

## How

1. Create `commands/orqa.md` — main governance summary
2. Create `commands/orqa-rules.md` — list active rules with enforcement status
3. Create `commands/orqa-status.md` — governance health check
4. Each command reads from `.orqa/` and presents structured output

## Verification

- `/orqa` returns governance summary
- `/orqa:rules` lists rules with enforcement status markers
- `/orqa:status` reports health metrics
