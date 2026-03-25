---
id: "EPIC-469add1c"
type: "epic"
title: "Artifact viewer redesign — layout, relationships, and graph enrichment"
description: "Redesign the artifact viewer information hierarchy, relationships panel, and pipeline stepper. Enrich graph nodes with metadata for display. The largest systemic theme from UAT round 2."
status: "completed"
priority: "P1"
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 5
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
deadline: null
horizon: "active"
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
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

- [TASK-3c586ee4](TASK-3c586ee4): Enrich graph nodes with status, title, priority as first-class fields
- [TASK-ece42049](TASK-ece42049): Reorder artifact viewer layout — actions needed, pipeline, metadata, acceptance, body
- [TASK-cd960062](TASK-cd960062): Reusable pipeline stepper component with configurable stages and visual refresh
- [TASK-efb94956](TASK-efb94956): Relationships panel — equal columns, overflow toggle, status dots on chips
- [TASK-98447dc5](TASK-98447dc5): Relationships graph visualization view (node-link diagram grouped by edge type)
- [TASK-bbd43489](TASK-bbd43489): Migrate scope fields to relationships array (rules + skills schemas)
- [TASK-da473493](TASK-da473493): Add documents/documented-by relationship types + body-text edge extraction
- [TASK-b3314c36](TASK-b3314c36): Field display improvements — badges, checkbox icons, display order
- [TASK-c560b894](TASK-c560b894): Configurable relationship chip display per type in project settings
- [TASK-088a0d6d](TASK-088a0d6d): Actions needed icon indicator in artifact list view + epics without tasks
- [TASK-d335dc27](TASK-d335dc27): Migrate epic/milestone and task/epic references to relationship types
- [TASK-734104a2](TASK-734104a2): Surface prioritisation criteria and require justification on epics/tasks

## Out of Scope

- Dashboard redesign (EPIC-df4c40b6)
- Notification system (EPIC-5a5e3c6c)
- Dynamic table components in markdown (EPIC-2633218e)
- Roadmap kanban view (EPIC-952ff2c9)