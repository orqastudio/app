---
id: TASK-c08f0b83
type: task
title: Roadmap kanban view
description: "Create a dynamic kanban board showing milestones as columns with epic cards, with drill-down capability."
status: completed
priority: P2
scoring:
  impact: 4
  urgency: 2
  complexity: 4
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Dynamic kanban board showing milestones as columns with epic cards
  - Drill-down into milestone shows epics
  - Drill-down into epic shows tasks
  - All data from graph
relationships:
  - target: EPIC-c08f0b83
    type: delivers
    rationale: Kanban view provides spatial overview of roadmap progress
  - target: TASK-3c586ee4
    type: depends-on
  - target: TASK-d5971d0d
    type: depended-on-by
---

## Scope

Create new RoadmapView.svelte with a kanban layout. Milestones render as columns, epics as cards within columns. Clicking a milestone drills down to show its epics. Clicking an epic drills down to show its tasks. All data sourced from the artifact graph.