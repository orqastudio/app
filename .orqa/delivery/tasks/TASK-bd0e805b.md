---
id: TASK-bd0e805b
title: "Implement /orqa, /orqa:rules, /orqa:status commands"
description: Plugin slash commands for governance interaction in Claude Code.
status: completed
created: 2026-03-11
updated: 2026-03-14
assignee: AGENT-cc255bc8
docs: []
acceptance:
  - "/orqa shows governance summary (active rules, recent violations, health)"
  - "/orqa:rules lists all active rules with enforcement status"
  - "/orqa:status shows governance health (rule coverage, broken refs, schema compliance)"
  - Commands are discoverable via Claude Code skill system
relationships:
  - target: EPIC-3a8ad459
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-0b584382
    type: depends-on
  - target: TASK-11cf4c1d
    type: depended-on-by
  - target: TASK-fa39671d
    type: depended-on-by
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
