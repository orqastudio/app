---
id: TASK-184
title: "Implement /orqa, /orqa:rules, /orqa:status commands"
description: Plugin slash commands for governance interaction in Claude Code.
status: completed
created: 2026-03-11
updated: 2026-03-14
assignee: AGENT-002
docs: []
acceptance:
  - "/orqa shows governance summary (active rules, recent violations, health)"
  - "/orqa:rules lists all active rules with enforcement status"
  - "/orqa:status shows governance health (rule coverage, broken refs, schema compliance)"
  - Commands are discoverable via Claude Code skill system
relationships:
  - target: EPIC-050
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-178
    type: depends-on
  - target: TASK-185
    type: depended-on-by
  - target: TASK-340
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
