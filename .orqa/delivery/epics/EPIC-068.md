---
id: EPIC-068
title: Dashboard redesign — layout, widgets, and plugin extensibility
description: Redesign the dashboard from a vertical card stack to an information-dense layout. Architecture must support drag-and-drop positioning and plugin-provided custom widgets.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
deadline: null
milestone: MS-001
horizon: active
pillars:
  - PILLAR-001
  - PILLAR-003
depends-on:
  - EPIC-067
research-refs: []
docs-required: []
docs-produced: []
relationships:
  - target: EPIC-063
    type: informed-by
    rationale: UAT round 2 findings drive dashboard redesign
  - target: IDEA-091
    type: informed-by
    rationale: Pipeline flow model rethinking feeds into dashboard widget design
  - target: IMPL-062
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-062
  - target: IMPL-060
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-060
  - target: IMPL-063
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-063
  - target: MS-001
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-439
    type: contains
    rationale: Epic contains this task
  - target: TASK-440
    type: contains
    rationale: Epic contains this task
  - target: TASK-441
    type: contains
    rationale: Epic contains this task
  - target: TASK-442
    type: contains
    rationale: Epic contains this task
  - target: TASK-443
    type: contains
    rationale: Epic contains this task
  - target: TASK-443
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-443"
  - target: TASK-439
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-439"
  - target: TASK-442
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-442"
  - target: TASK-440
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-440"
  - target: TASK-441
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-441"
  - target: EPIC-073
    type: informs
    rationale: "Auto-generated inverse of informs relationship from EPIC-073"
---
## Context

UAT round 2 found the dashboard is a column of cards, not a dashboard. Health trends are underutilised, "all clear" wastes space, and refresh/re-index are duplicated. The knowledge pipeline flow model needs rethinking (IDEA-091).

## Implementation Design

### Design constraints
- Architecture must support drag-and-drop layout customization (not implemented this pass)
- Architecture must support plugin-provided custom widgets (not implemented this pass)
- Widget grid/layout system chosen now must accommodate both constraints later

### Phase 1: Pipeline health widget rework (Theme B)
- Fix stale data after rescan (refresh graph before scanning)
- Auto-fix confirmations → toast notifications
- "All clear" collapses to subtle indicator, expands when errors exist
- Remove duplicate Refresh button (Re-index in statusbar is sufficient)
- Rescan auto-triggers after graph refresh

### Phase 2: Dashboard layout (Theme A)
- Replace vertical card stack with information-dense grid layout
- Health trend sparklines more prominent
- Widget sizing and positioning via grid system
- Remove duplicate Re-index/Refresh buttons

## Tasks

- [TASK-439](TASK-439): Fix rescan stale data — refresh graph before integrity scan
- [TASK-440](TASK-440): Pipeline health: collapse "all clear", remove Refresh button, auto-rescan after refresh
- [TASK-441](TASK-441): Dashboard grid layout system (extensible for drag-drop and plugin widgets)
- [TASK-442](TASK-442): Health trend widget redesign — more prominent, better integration with grid
- [TASK-443](TASK-443): Knowledge pipeline flow model rethink (IDEA-091)

## Out of Scope

- Drag-and-drop implementation (architecture supports it, not built yet)
- Plugin widget registration (architecture supports it, not built yet)
- Notification system (EPIC-069)
