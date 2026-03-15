---
id: EPIC-065
title: Artifact viewer enhancements
description: Enhance the artifact viewer with unified relationships display, actions needed inference, pipeline position stepper, horizon display, and acceptance criteria checkboxes. Originated from UAT Theme D findings.
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
deadline: null
milestone: MS-001
horizon: active
pillars:
  - PILLAR-001
depends-on:
  - EPIC-060
research-refs: []
docs-required: []
docs-produced: []
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
    type: belongs-to
    rationale: Epic belongs to this milestone
  - target: TASK-406
    type: contains
    rationale: Epic contains this task
  - target: TASK-406
    type: delivered-by
    rationale: "Auto-generated inverse of delivered-by relationship from TASK-406"
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
