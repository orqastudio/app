---
id: EPIC-070
title: Dynamic artifact tables and schema validation enhancements
description: "Injectable dynamic tables in markdown (tasks table on epics, epics on milestones), body template validation, and schema evolution (new relationship types, personas)."
status: completed
priority: P2
created: 2026-03-14
updated: 2026-03-14
deadline: null
horizon: next
relationships:
  - target: EPIC-067
    type: informed-by
    rationale: Graph enrichment from EPIC-067 enables dynamic table data
  - target: EPIC-063
    type: informed-by
    rationale: UAT round 2 findings drive schema and validation work
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-447
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-448
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-449
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-450
    type: delivered-by
    rationale: Epic contains this task
  - target: EPIC-067
    type: depends-on
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-003
    type: grounded-by
---
## Context

UAT round 2 identified the need for dynamic graph-driven tables in markdown rendering (e.g. tasks table on epics showing live status), body template validation with required headings, and potential new artifact types (personas).

## Implementation Design

### Phase 1: Body template validation
- Define required headings per artifact type in schema bodyTemplate
- Validate non-empty content under required headings
- Add to integrity validator

### Phase 2: Dynamic artifact tables
- New markdown extension: `:::tasks` or similar syntax triggers a graph query
- Renders as a live table with status, title, priority from graph nodes
- Supported contexts: tasks-on-epic, epics-on-milestone, children-of-any-type

### Phase 3: Schema evolution
- Personas as potential new artifact type (needs research)
- Any additional relationship types identified during EPIC-067

## Tasks

- [TASK-447](TASK-447): Body template validation — required headings with non-empty content in integrity validator
- [TASK-448](TASK-448): Dynamic artifact table component — graph-queried, injectable into markdown
- [TASK-449](TASK-449): Research personas as a top-level artifact type
- [TASK-450](TASK-450): Acceptance criteria audit — thorough backfill across all tasks

## Out of Scope

- Full markdown editor (future)
- Artifact creation/editing through the UI (future)
