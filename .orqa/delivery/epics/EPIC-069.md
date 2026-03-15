---
id: EPIC-069
title: Notification system — toast, in-app panel, desktop
description: Design and implement a notification strategy covering toast messages, in-app notification panel, and desktop notifications. Determine which events use which channel.
status: completed
priority: P2
created: 2026-03-14
updated: 2026-03-14
deadline: null
milestone: MS-001
horizon: next
pillars:
  - PILLAR-001
depends-on: []
research-refs: []
docs-required: []
docs-produced: []
relationships:
  - target: EPIC-063
    type: informed-by
    rationale: UAT round 2 identified missing notification strategy
  - target: IMPL-061
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-061
  - target: MS-001
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-444
    type: contains
    rationale: Epic contains this task
  - target: TASK-445
    type: contains
    rationale: Epic contains this task
  - target: TASK-446
    type: contains
    rationale: Epic contains this task
  - target: TASK-445
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-445"
  - target: TASK-444
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-444"
  - target: TASK-446
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-446"
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

- [TASK-444](TASK-444): Research notification strategy — toast, panel, desktop, event mapping
- [TASK-445](TASK-445): Implement toast notification system
- [TASK-446](TASK-446): Wire auto-fix and other confirmations to toast instead of inline

## Out of Scope

- In-app notification panel (future — needs more design)
- Desktop notifications (future — needs user preference controls)
