---
id: "EPIC-c08f0b83"
type: epic
title: "Roadmap kanban view"
description: "Replace the static roadmap documentation page with a dynamic kanban board view under Process. Milestone-level board with drill-down into epics and tasks, all data from the graph."
status: archived
priority: "P2"
scoring:
  impact: 4
  urgency: 2
  complexity: 3
  dependencies: 1
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
deadline: null
horizon: "next"
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

The roadmap is currently a manually maintained markdown page. It should be an inferred dynamic view showing milestones as kanban columns with epic cards, drillable to task level.

## Tasks

- [TASK-c08f0b83](TASK-c08f0b83): Roadmap kanban view — milestone columns, epic cards, task drill-down
- [TASK-d5971d0d](TASK-d5971d0d): Register roadmap view under Process navigation (replace static doc)

## Out of Scope

- Drag-and-drop priority reordering (future — needs write-back to artifacts)
- Sprint/iteration planning (future)
