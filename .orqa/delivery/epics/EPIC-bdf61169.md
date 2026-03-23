---
id: "EPIC-bdf61169"
type: epic
title: "UX Polish Sprint"
description: "UX improvements identified during early dogfooding: streaming fix, output truncation, tool call grouping, auto-naming sessions, custom titlebar."
status: "completed"
priority: "P2"
created: "2026-03-06"
updated: "2026-03-09"
horizon: null
scoring:
  impact: 3
  urgency: 3
  complexity: 2
  dependencies: 2
relationships:
  - target: "RES-a602a623"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-a602a623"
  - target: "TASK-faffc209"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-16a4b326"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
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