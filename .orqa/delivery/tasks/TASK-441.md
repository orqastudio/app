---
id: TASK-441
title: Dashboard grid layout system
description: Replace the vertical card stack dashboard with a CSS grid layout that supports future drag-drop and plugin widgets.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Dashboard uses a grid layout instead of vertical card stack
  - Architecture supports future drag-drop and plugin widgets
  - Widgets can span columns/rows
relationships:
  - target: EPIC-068
    type: delivers
    rationale: Grid layout is the foundation for a more useful dashboard
  - target: TASK-442
    type: depended-on-by
---

## Scope

Redesign ProjectDashboard.svelte with CSS grid. Create a widget slot system that allows widgets to declare their grid span. Maintain existing widget functionality in the new layout.
