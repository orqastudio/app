---
id: EPIC-072
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
  - target: EPIC-067
    type: informed-by
    rationale: Graph enrichment from EPIC-067 provides the data model for kanban cards
  - target: EPIC-063
    type: informed-by
    rationale: UAT round 2 finding F29
  - target: IMPL-065
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-065
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-456
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-457
    type: delivered-by
    rationale: Epic contains this task
  - target: EPIC-067
    type: depends-on
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-003
    type: grounded-by
  - target: PILLAR-001
    type: grounded-by
---
## Context

The roadmap is currently a manually maintained markdown page. It should be an inferred dynamic view showing milestones as kanban columns with epic cards, drillable to task level.

## Tasks

- [TASK-456](TASK-456): Roadmap kanban view — milestone columns, epic cards, task drill-down
- [TASK-457](TASK-457): Register roadmap view under Process navigation (replace static doc)

## Out of Scope

- Drag-and-drop priority reordering (future — needs write-back to artifacts)
- Sprint/iteration planning (future)
