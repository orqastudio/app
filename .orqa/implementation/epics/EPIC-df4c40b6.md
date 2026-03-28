---
id: "EPIC-df4c40b6"
type: "epic"
title: "Dashboard redesign — layout, widgets, and plugin extensibility"
description: "Redesign the dashboard from a vertical card stack to an information-dense layout. Architecture must support drag-and-drop positioning and plugin-provided custom widgets."
status: archived
priority: "P1"
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
deadline: null
horizon: "active"
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

UAT round 2 found the dashboard is a column of cards, not a dashboard. Health trends are underutilised, "all clear" wastes space, and refresh/re-index are duplicated. The knowledge pipeline flow model needs rethinking (IDEA-c8b70949).

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

- [TASK-12e23349](TASK-12e23349): Fix rescan stale data — refresh graph before integrity scan
- [TASK-a5b75216](TASK-a5b75216): Pipeline health: collapse "all clear", remove Refresh button, auto-rescan after refresh
- [TASK-1382054e](TASK-1382054e): Dashboard grid layout system (extensible for drag-drop and plugin widgets)
- [TASK-777c0715](TASK-777c0715): Health trend widget redesign — more prominent, better integration with grid
- [TASK-b1bb8fe5](TASK-b1bb8fe5): Knowledge pipeline flow model rethink (IDEA-c8b70949)

## Out of Scope

- Drag-and-drop implementation (architecture supports it, not built yet)
- Plugin widget registration (architecture supports it, not built yet)
- Notification system (EPIC-5a5e3c6c)
