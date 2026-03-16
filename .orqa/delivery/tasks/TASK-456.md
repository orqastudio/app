---
id: TASK-456
title: Roadmap kanban view
description: "Create a dynamic kanban board showing milestones as columns with epic cards, with drill-down capability."
status: completed
priority: P2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Dynamic kanban board showing milestones as columns with epic cards
  - Drill-down into milestone shows epics
  - Drill-down into epic shows tasks
  - All data from graph
relationships:
  - target: EPIC-072
    type: delivers
    rationale: Kanban view provides spatial overview of roadmap progress
  - target: TASK-429
    type: depends-on
  - target: TASK-457
    type: depended-on-by
---

## Scope

Create new RoadmapView.svelte with a kanban layout. Milestones render as columns, epics as cards within columns. Clicking a milestone drills down to show its epics. Clicking an epic drills down to show its tasks. All data sourced from the artifact graph.
