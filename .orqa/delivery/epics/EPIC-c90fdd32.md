---
id: "EPIC-c90fdd32"
type: "epic"
title: "Artifact viewer enhancements"
description: "Enhance the artifact viewer with unified relationships display, actions needed inference, pipeline position stepper, horizon display, and acceptance criteria checkboxes. Originated from UAT Theme D findings."
status: "completed"
priority: "P1"
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
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

During UAT of the dashboard and artifact viewer (EPIC-f7055984), Theme D identified 8 findings related to artifact viewer enhancements. These were substantial enough to warrant their own epic rather than being a task within the UAT fixes epic.

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

- [TASK-d6982377](TASK-d6982377): Implement all artifact viewer enhancements (done)

## Out of Scope

- Auto-embed child artifacts (milestones→epics, epics→tasks) — requires backend graph queries
- Acceptance criteria state persistence (checked/unchecked tracking in schema) — needs schema design