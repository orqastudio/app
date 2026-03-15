---
id: EPIC-067
title: Artifact viewer redesign — layout, relationships, and graph enrichment
description: Redesign the artifact viewer information hierarchy, relationships panel, and pipeline stepper. Enrich graph nodes with metadata for display. The largest systemic theme from UAT round 2.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
deadline: null
milestone: MS-001
horizon: active
pillars:
  - PILLAR-001
  - PILLAR-003
depends-on: []
research-refs: []
docs-required: []
docs-produced: []
relationships:
  - target: EPIC-065
    type: informed-by
    rationale: First pass at artifact viewer enhancements revealed deeper layout and relationship issues
  - target: EPIC-063
    type: informed-by
    rationale: UAT round 2 findings drive this epic's scope
  - target: EPIC-072
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-072
  - target: IMPL-064
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-064
  - target: EPIC-070
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-070
  - target: IMPL-058
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-058
  - target: IMPL-059
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from IMPL-059
  - target: MS-001
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-429
    type: contains
    rationale: Epic contains this task
  - target: TASK-430
    type: contains
    rationale: Epic contains this task
  - target: TASK-431
    type: contains
    rationale: Epic contains this task
  - target: TASK-432
    type: contains
    rationale: Epic contains this task
  - target: TASK-433
    type: contains
    rationale: Epic contains this task
  - target: TASK-434
    type: contains
    rationale: Epic contains this task
  - target: TASK-435
    type: contains
    rationale: Epic contains this task
  - target: TASK-436
    type: contains
    rationale: Epic contains this task
  - target: TASK-437
    type: contains
    rationale: Epic contains this task
  - target: TASK-438
    type: contains
    rationale: Epic contains this task
  - target: TASK-458
    type: contains
    rationale: Epic contains this task
  - target: TASK-459
    type: contains
    rationale: Epic contains this task
  - target: TASK-436
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-436"
  - target: TASK-431
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-431"
  - target: TASK-430
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-430"
  - target: TASK-437
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-437"
  - target: TASK-459
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-459"
  - target: TASK-433
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-433"
  - target: TASK-458
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-458"
  - target: TASK-434
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-434"
  - target: TASK-435
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-435"
  - target: TASK-432
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-432"
  - target: TASK-429
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-429"
  - target: TASK-438
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-438"
  - target: EPIC-073
    type: informs
    rationale: "Auto-generated inverse of informs relationship from EPIC-073"
---
## Context

UAT round 2 produced 30 findings. Themes C, D, and E converge on the artifact viewer — wrong information hierarchy, relationships panel needing redesign, and graph nodes lacking metadata for display. These are interdependent and must be addressed together.

## Implementation Design

### Phase 1: Graph node enrichment (Theme E foundation)

Enrich ArtifactNode in the Rust graph builder to carry status, title, description, and priority as first-class fields (not buried in frontmatter JSON). This unblocks relationship chips with status dots, dynamic tables, and actions-needed inference from the graph.

### Phase 2: Artifact viewer layout (Theme C)

Reorder the artifact viewer:
1. Actions needed (top — most actionable)
2. Pipeline stepper (configurable stages, reusable component)
3. Title + metadata box
4. Acceptance criteria (tasks only, before body content)
5. Body content (markdown)
6. Relationships panel

### Phase 3: Relationships panel redesign (Theme D)

- Equal column widths for label/value
- One row per relationship type with "..." overflow toggle
- Relationship chips show: configurable display (title or id), status dot, click-to-navigate
- Graph visualization view alongside list view (focused artifact at centre, nodes grouped by edge type)
- Migrate `scope` fields to relationships array (rules + skills)
- New relationship types: `documents`/`documented-by`
- Body-text artifact references become graph edges

### Phase 4: Field display improvements

- Maturity as badge, above recurrence
- Category and version as badges
- Boolean fields (user-invocable) as checkbox icons
- Relationship chip display configurable per type in project settings

## Tasks

- [TASK-429](TASK-429): Enrich graph nodes with status, title, priority as first-class fields
- [TASK-430](TASK-430): Reorder artifact viewer layout — actions needed, pipeline, metadata, acceptance, body
- [TASK-431](TASK-431): Reusable pipeline stepper component with configurable stages and visual refresh
- [TASK-432](TASK-432): Relationships panel — equal columns, overflow toggle, status dots on chips
- [TASK-433](TASK-433): Relationships graph visualization view (node-link diagram grouped by edge type)
- [TASK-434](TASK-434): Migrate scope fields to relationships array (rules + skills schemas)
- [TASK-435](TASK-435): Add documents/documented-by relationship types + body-text edge extraction
- [TASK-436](TASK-436): Field display improvements — badges, checkbox icons, display order
- [TASK-437](TASK-437): Configurable relationship chip display per type in project settings
- [TASK-438](TASK-438): Actions needed icon indicator in artifact list view + epics without tasks
- [TASK-458](TASK-458): Migrate epic/milestone and task/epic references to relationship types
- [TASK-459](TASK-459): Surface prioritisation criteria and require justification on epics/tasks

## Out of Scope

- Dashboard redesign (EPIC-068)
- Notification system (EPIC-069)
- Dynamic table components in markdown (EPIC-070)
- Roadmap kanban view (EPIC-071)
