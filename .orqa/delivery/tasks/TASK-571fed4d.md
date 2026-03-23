---
id: TASK-571fed4d
type: task
title: Tool call display polish
description: "Replaces raw tool call JSON with user-friendly presentation: human-readable names, Lucide icons, parameter summaries, and grouping of consecutive same-tool calls."
status: completed
created: 2026-03-04
updated: 2026-03-09
assignee: AGENT-cc255bc8
acceptance:
  - "Tool calls display friendly names (Read → \"Reading file\")"
  - Each tool type has a Lucide icon
  - Consecutive same-tool calls grouped into summary
  - Collapsible detail view for tool input/output
relationships:
  - target: EPIC-a2cfc2b4
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-ebe49b1a
    type: depended-on-by
---
## What

Replace raw tool call JSON display with user-friendly presentation: human-readable
names, icons, parameter summaries, and grouping.

## Outcome

Implemented in `tool-display.ts` utility and tool components. Git commit: `b0ee670`.
Later enhanced with output truncation and improved grouping in `7a954d9`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.