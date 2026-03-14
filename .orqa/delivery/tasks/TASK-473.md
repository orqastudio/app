---
id: TASK-473
title: "Roadmap hierarchical drill-down with breadcrumbs"
description: "Build roadmap drill-down navigation: top level Now/Next/Later/Completed+Archived with milestones → click milestone for epic kanban by status → click epic for task kanban. Breadcrumb bar, collapsible done columns, drag-drop between columns, configurable grouping/sorting."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on:
  - TASK-472
assignee: null
skills:
  - SKILL-030
  - SKILL-042
  - SKILL-016
acceptance:
  - Top level shows Now/Next/Later columns + Completed+Archived fourth column
  - Milestones shown as cards with progress indicator and critical item insights
  - Click milestone → kanban of epics by status with breadcrumb
  - Click epic → kanban of tasks by status with breadcrumb
  - Done column collapsed to thin bar with count, expandable
  - Drag-drop between columns changes the relevant field assignment
  - Column grouping/sorting configurable per drill-down level
relationships:
  - target: EPIC-073
    type: delivers
    rationale: Roadmap drill-down (F5)
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
