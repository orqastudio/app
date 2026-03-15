---
id: EPIC-074
title: "Dashboard redesign — narrative flow layout grounded in pillars"
description: "Replace the current metrics-dump dashboard with a narrative flow layout: milestone context at top, three pillar-aligned columns (clarity/learning/purpose), collapsible power user section. The dashboard answers what matters, not what exists."
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
deadline: null
milestone: MS-001
horizon: active
pillars:
  - PILLAR-001
  - PILLAR-002
  - PILLAR-003
depends-on: []
blocks: []
research-refs:
  - RES-064
docs-required:
  - RES-064
docs-produced: []
scoring:
  dogfood-value: 5 — dashboard is first thing the user sees every session
  user-facing: 5 — primary orientation surface
  foundation: 3 — builds on existing widget infrastructure
  complexity: 3 — refactoring existing components into new layout
  score: 4.5
relationships:
  - target: RES-064
    type: informed-by
    rationale: Dashboard research drives the design
  - target: EPIC-073
    type: informed-by
    rationale: UAT finding F1 identified the need
  - target: MS-001
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-483
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-483"
  - target: TASK-483
    type: contains
    rationale: "Auto-generated inverse of contains relationship from TASK-483"
  - target: TASK-479
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-479"
  - target: TASK-479
    type: contains
    rationale: "Auto-generated inverse of contains relationship from TASK-479"
  - target: TASK-482
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-482"
  - target: TASK-482
    type: contains
    rationale: "Auto-generated inverse of contains relationship from TASK-482"
  - target: TASK-481
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-481"
  - target: TASK-481
    type: contains
    rationale: "Auto-generated inverse of contains relationship from TASK-481"
---
## Context

The current dashboard is existing data cards in a grid. RES-064 research recommends a Narrative Flow layout that answers three questions mapping to the pillars:
1. What is the current clarity level? (graph health, structure integrity)
2. Is the system getting better? (trends, lesson velocity)
3. Are we staying true to purpose? (milestone progress, decisions, blockers)

## Implementation Design

### Layout Structure

```
┌──────────────────────────────────────────────────────────┐
│ Active Milestone Card (title, gate, P1 progress, deadline)│
├──────────────────┬──────────────────┬────────────────────┤
│ Where You Are    │ How You're       │ What's Next        │
│ (Clarity)        │ Improving        │ (Purpose)          │
│                  │ (Learning)       │                    │
│ • Graph health   │ • Error trends   │ • Pending decisions│
│ • Orphan count   │ • Lesson velocity│ • Blockers         │
│ • Broken refs    │ • Rule adoption  │ • Scope risks      │
│ • Integrity score│ • Pass rate trend│ • Next actions     │
├──────────────────┴──────────────────┴────────────────────┤
│ Knowledge Pipeline (collapsible, power users)            │
└──────────────────────────────────────────────────────────┘
```

### Widgets to Build/Refactor

1. **MilestoneContextCard** — active milestone with P1 epic progress
2. **GraphHealthWidget** — refactor current integrity widget into clarity-focused card
3. **ImprovementTrendsWidget** — refactor current HealthTrendWidget into 2x2 sparkline grid
4. **LessonVelocityWidget** — new: lesson pipeline stages with counts
5. **DecisionQueueWidget** — new: pending decisions and blockers
6. **KnowledgePipelineWidget** — refactor current PipelineWidget, make collapsible

## Tasks

- [ ] [TASK-479](TASK-479): Build MilestoneContextCard and new dashboard layout shell
- [ ] [TASK-480](TASK-480): Refactor GraphHealthWidget for clarity-focused display
- [ ] [TASK-481](TASK-481): Build ImprovementTrendsWidget (2x2 sparklines with trend arrows)
- [ ] [TASK-482](TASK-482): Build LessonVelocityWidget and DecisionQueueWidget
- [ ] [TASK-483](TASK-483): Refactor KnowledgePipelineWidget as collapsible bottom section
