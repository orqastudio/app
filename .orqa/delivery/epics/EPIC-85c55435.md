---
id: EPIC-85c55435
title: Roadmap kanban view
description: "Replace the static roadmap documentation page with a dynamic kanban board view under Process. Milestone-level board with drill-down into epics and tasks, all data from the graph."
status: completed
priority: P2
scoring:
  impact: 4
  urgency: 2
  complexity: 3
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
deadline: null
horizon: next
relationships:
  - target: MS-654badde
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-7fcdc743
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-15e676d7
    type: delivered-by
    rationale: Epic contains this task
  - target: IMPL-86f05cc5
    type: cautioned-by
---
## Context

The roadmap is currently a manually maintained markdown page. It should be an inferred dynamic view showing milestones as kanban columns with epic cards, drillable to task level.

## Tasks

- [TASK-7fcdc743](TASK-7fcdc743): Roadmap kanban view — milestone columns, epic cards, task drill-down
- [TASK-15e676d7](TASK-15e676d7): Register roadmap view under Process navigation (replace static doc)

## Out of Scope

- Drag-and-drop priority reordering (future — needs write-back to artifacts)
- Sprint/iteration planning (future)
