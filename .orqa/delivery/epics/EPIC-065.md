---
id: EPIC-065
title: Artifact viewer enhancements
description: "Enhance the artifact viewer with unified relationships display, actions needed inference, pipeline position stepper, horizon display, and acceptance criteria checkboxes. Originated from UAT Theme D findings."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
deadline: null
horizon: active
relationships:
  - target: EPIC-063
    type: informed-by
    rationale: UAT Theme D findings from EPIC-063 drove this epic's scope
  - target: EPIC-060
    type: informs
    rationale: Enhances artifact viewer built in EPIC-060
  - target: EPIC-067
    type: informs
    rationale: Auto-generated inverse of informs relationship from EPIC-067
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-406
    type: delivered-by
    rationale: Epic contains this task
  - target: EPIC-060
    type: depends-on
  - target: PILLAR-001
    type: informed-by
  - target: DOC-057
    type: informs
  - target: PILLAR-001
    type: grounded-by
---
## Context

During UAT of the dashboard and artifact viewer (EPIC-063), Theme D identified 8 findings related to artifact viewer enhancements. These were substantial enough to warrant their own epic rather than being a task within the UAT fixes epic.

## Implementation Design

### Components Created

- **PipelineStepper** — Horizontal lifecycle stepper showing all stages with current stage highlighted
- **ActionsNeeded** — Amber box inferring next actions from artifact status, hidden for terminal states
- **AcceptanceCriteria** — Display-only checklist for task acceptance items

### Components Modified

- **ArtifactViewer** — Integrated PipelineStepper, ActionsNeeded, AcceptanceCriteria
- **FrontmatterHeader** — Horizon as chip field, acceptance skipped from metadata
- **ReferencesPanel** — Grouped relationships by type
- **RelationshipsList** — SvelteMap for lint compliance

## Tasks

- [TASK-406](TASK-406): Implement all artifact viewer enhancements (done)

## Out of Scope

- Auto-embed child artifacts (milestones→epics, epics→tasks) — requires backend graph queries
- Acceptance criteria state persistence (checked/unchecked tracking in schema) — needs schema design
