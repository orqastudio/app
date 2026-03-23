---
id: EPIC-58ba6d53
type: epic
title: "Artifact viewer redesign — layout, relationships, and graph enrichment"
description: "Redesign the artifact viewer information hierarchy, relationships panel, and pipeline stepper. Enrich graph nodes with metadata for display. The largest systemic theme from UAT round 2."
status: completed
priority: P1
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 5
created: 2026-03-14
updated: 2026-03-14
deadline: null
horizon: active
relationships:
  - target: TASK-e9219bfd
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-1f8f15b9
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-d5e090bf
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-41df15c4
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-c744d86a
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-0905e285
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-2830a2da
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-173d2618
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-e0165bca
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-a00f73cf
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-c6514b53
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-47a4b670
    type: delivered-by
    rationale: Epic contains this task
  - target: IMPL-3fbabea8
    type: cautioned-by
  - target: IMPL-2ba0fc33
    type: cautioned-by
  - target: IMPL-e5ddb8f2
    type: cautioned-by
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
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
- New relationship types: `informs`/`informed-by`
- Body-text artifact references become graph edges

### Phase 4: Field display improvements

- Maturity as badge, above recurrence
- Category and version as badges
- Boolean fields (user-invocable) as checkbox icons
- Relationship chip display configurable per type in project settings

## Tasks

- [TASK-e9219bfd](TASK-e9219bfd): Enrich graph nodes with status, title, priority as first-class fields
- [TASK-1f8f15b9](TASK-1f8f15b9): Reorder artifact viewer layout — actions needed, pipeline, metadata, acceptance, body
- [TASK-d5e090bf](TASK-d5e090bf): Reusable pipeline stepper component with configurable stages and visual refresh
- [TASK-41df15c4](TASK-41df15c4): Relationships panel — equal columns, overflow toggle, status dots on chips
- [TASK-c744d86a](TASK-c744d86a): Relationships graph visualization view (node-link diagram grouped by edge type)
- [TASK-0905e285](TASK-0905e285): Migrate scope fields to relationships array (rules + skills schemas)
- [TASK-2830a2da](TASK-2830a2da): Add documents/documented-by relationship types + body-text edge extraction
- [TASK-173d2618](TASK-173d2618): Field display improvements — badges, checkbox icons, display order
- [TASK-e0165bca](TASK-e0165bca): Configurable relationship chip display per type in project settings
- [TASK-a00f73cf](TASK-a00f73cf): Actions needed icon indicator in artifact list view + epics without tasks
- [TASK-c6514b53](TASK-c6514b53): Migrate epic/milestone and task/epic references to relationship types
- [TASK-47a4b670](TASK-47a4b670): Surface prioritisation criteria and require justification on epics/tasks

## Out of Scope

- Dashboard redesign (EPIC-6e774e50)
- Notification system (EPIC-2649e450)
- Dynamic table components in markdown (EPIC-f684378f)
- Roadmap kanban view (EPIC-7d587280)