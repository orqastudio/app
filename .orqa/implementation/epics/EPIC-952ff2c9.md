---
id: "EPIC-952ff2c9"
type: epic
title: "Documentation improvements — content, ordering, rendering"
description: "Fix docs navigation (no status for docs), populate Guide section, audit doc ordering for reading flow, add mermaid/PlantUML rendering, and review doc-to-artifact relationships."
status: archived
priority: "P2"
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 1
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
deadline: null
horizon: "next"
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

## Context

UAT round 2 found documentation navigation shows status (irrelevant for docs), Guide section is empty, doc ordering is arbitrary, and the markdown renderer lacks diagram support. Documentation pages also need proper graph relationships.

## Tasks

- [TASK-7f45cdfa](TASK-7f45cdfa): Fix docs nav — show top-level categories instead of status
- [TASK-7202f1b4](TASK-7202f1b4): Populate Guide section — icon, move appropriate articles, add SDK docs
- [TASK-85c8d2fb](TASK-85c8d2fb): Audit and reorder documentation for structured reading flow
- [TASK-84f08739](TASK-84f08739): Mermaid and PlantUML rendering in markdown, themed to match app
- [TASK-fcd3d5c6](TASK-fcd3d5c6): Documentation relationship audit — add documents/documented-by edges

## Out of Scope

- Documentation editing UI (future)
