---
id: EPIC-071
title: "Documentation improvements — content, ordering, rendering"
description: "Fix docs navigation (no status for docs), populate Guide section, audit doc ordering for reading flow, add mermaid/PlantUML rendering, and review doc-to-artifact relationships."
status: completed
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
deadline: null
horizon: next
relationships:
  - target: EPIC-063
    type: informed-by
    rationale: UAT round 2 documentation findings
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-451
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-452
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-453
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-454
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-455
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-002
    type: grounded-by
---
## Context

UAT round 2 found documentation navigation shows status (irrelevant for docs), Guide section is empty, doc ordering is arbitrary, and the markdown renderer lacks diagram support. Documentation pages also need proper graph relationships.

## Tasks

- [TASK-451](TASK-451): Fix docs nav — show top-level categories instead of status
- [TASK-452](TASK-452): Populate Guide section — icon, move appropriate articles, add SDK docs
- [TASK-453](TASK-453): Audit and reorder documentation for structured reading flow
- [TASK-454](TASK-454): Mermaid and PlantUML rendering in markdown, themed to match app
- [TASK-455](TASK-455): Documentation relationship audit — add documents/documented-by edges

## Out of Scope

- Documentation editing UI (future)
