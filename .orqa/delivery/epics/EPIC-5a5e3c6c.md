---
id: EPIC-5a5e3c6c
type: epic
title: "Notification system — toast, in-app panel, desktop"
description: "Design and implement a notification strategy covering toast messages, in-app notification panel, and desktop notifications. Determine which events use which channel."
status: completed
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
deadline: null
horizon: next
relationships:
  - target: TASK-5610fe29
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-97e1fa39
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-299639cc
    type: delivered-by
    rationale: Epic contains this task
  - target: IMPL-9177c9bd
    type: cautioned-by
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
## Context

No notification strategy exists. Auto-fix confirmations are verbose and inline. Need to decide on toast messages, in-app notification panel, desktop notifications, and which events use which channel.

## Implementation Design

TBD — needs research on:
- Toast library (sonner? shadcn toast?)
- Desktop notification API (Tauri notification plugin)
- In-app notification panel design
- Event-to-channel mapping (what goes where)

## Tasks

- [TASK-5610fe29](TASK-5610fe29): Research notification strategy — toast, panel, desktop, event mapping
- [TASK-97e1fa39](TASK-97e1fa39): Implement toast notification system
- [TASK-299639cc](TASK-299639cc): Wire auto-fix and other confirmations to toast instead of inline

## Out of Scope

- In-app notification panel (future — needs more design)
- Desktop notifications (future — needs user preference controls)