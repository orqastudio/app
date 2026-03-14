---
id: EPIC-038
title: UX Polish Sprint
description: "UX improvements identified during early dogfooding: streaming fix, output truncation, tool call grouping, auto-naming sessions, custom titlebar."
status: done
priority: P2
created: "2026-03-06"
updated: "2026-03-09"
milestone: MS-001
horizon: null
pillars:
  - PILLAR-001
research-refs:
  - RES-023
docs-required: []
docs-produced: []
scoring:
  user-value: 4
  pillar-alignment: 3
  dependency-weight: 3
  effort: 3
  risk: 2
  score: 15
---
## Implementation Design

### Fixes
1. **Streaming** — Proper NDJSON line buffering with partial message handling
2. **Output truncation** — Tool outputs beyond 500 chars collapsed with "Show more"
3. **Tool grouping** — Consecutive same-tool calls grouped into summary card
4. **Auto-naming** — After first assistant response, session auto-titled via LLM
5. **Custom titlebar** — Branded titlebar with window controls, project name, session indicator
6. **Error display** — Fix silent error swallowing in settings persistence

## Git Evidence

- `0aab794` — Fix error swallowing and settings persistence
- `7a954d9` — Streaming fix, output truncation, tool grouping, auto-naming, custom titlebar

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
