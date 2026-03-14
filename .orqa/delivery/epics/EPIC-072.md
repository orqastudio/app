---

id: EPIC-072
title: "Roadmap kanban view"
description: "Replace the static roadmap documentation page with a dynamic kanban board view under Process. Milestone-level board with drill-down into epics and tasks, all data from the graph."
status: draft
priority: P2
created: "2026-03-14"
updated: "2026-03-14"
deadline: null
milestone: MS-001
horizon: next
pillars:
  - PILLAR-001
  - PILLAR-003
depends-on:
  - EPIC-067
research-refs: []
docs-required: []
docs-produced: []
relationships:
  - target: EPIC-067
    type: informed-by
    rationale: "Graph enrichment from EPIC-067 provides the data model for kanban cards"
  - target: EPIC-063
    type: informed-by
    rationale: "UAT round 2 finding F29"
  - target: IMPL-065
    type: informed-by
    rationale: "Auto-generated inverse of informed-by relationship from IMPL-065"
---
## Context

The roadmap is currently a manually maintained markdown page. It should be an inferred dynamic view showing milestones as kanban columns with epic cards, drillable to task level.

## Tasks

- [TASK-456](TASK-456): Roadmap kanban view — milestone columns, epic cards, task drill-down
- [TASK-457](TASK-457): Register roadmap view under Process navigation (replace static doc)

## Out of Scope

- Drag-and-drop priority reordering (future — needs write-back to artifacts)
- Sprint/iteration planning (future)
