---
id: EPIC-ed09464b
title: Configuration-driven delivery pipeline
description: "Replace hardcoded delivery artifact types (milestones, epics, tasks) with project-configurable type definitions in project.json. The code becomes a generic enforcement engine for whatever delivery pipeline the project defines. Core artifacts (ideas, research, rules, lessons, decisions, skills, agents, pillars) remain hardcoded as firmware."
status: completed
priority: P1
scoring:
  impact: 5
  urgency: 4
  complexity: 4
  dependencies: 4
created: 2026-03-15
updated: 2026-03-15
deadline: null
horizon: active
relationships:
  - target: AD-c8535011
    type: driven-by
    rationale: Implements the config-driven delivery pipeline decision
  - target: MS-eea45fa8
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-bc24af0b
    type: delivered-by
  - target: TASK-fcb7ddc4
    type: delivered-by
  - target: TASK-c2db174d
    type: delivered-by
  - target: TASK-a5606ccd
    type: delivered-by
  - target: TASK-26acd796
    type: delivered-by
---

## Context

Currently the codebase hardcodes knowledge of delivery artifact types:
- Rust integrity checks reference "epic", "milestone", "task" field names directly
- Roadmap components have hardcoded EPIC_COLUMNS, TASK_COLUMNS
- Parent-child relationships (task→epic→milestone) are hardcoded
- The StatusKanban groups by hardcoded status values

Per AD-c8535011, the delivery pipeline should be configurable. A software project defines milestones/epics/tasks. A different project might define phases/workstreams/actions.

## Implementation Design

### Phase 1: Define delivery type configuration schema

Add a `delivery` section to `project.json`:
```json
{
  "delivery": {
    "types": [
      {
        "key": "milestone",
        "label": "Milestone",
        "path": ".orqa/delivery/milestones",
        "parent": null
      },
      {
        "key": "epic",
        "label": "Epic",
        "path": ".orqa/delivery/epics",
        "parent": { "type": "milestone", "field": "milestone" }
      },
      {
        "key": "task",
        "label": "Task",
        "path": ".orqa/delivery/tasks",
        "parent": { "type": "epic", "field": "epic" }
      }
    ]
  }
}
```

### Phase 2: Make integrity checks config-driven

Replace hardcoded field references in `artifact_graph.rs` with lookups against the delivery type config. The parent-child consistency check reads the hierarchy from config, not from hardcoded "epic"/"milestone" strings.

### Phase 3: Make roadmap components config-driven

Roadmap reads delivery types from config to determine:
- Which type is the top level (milestones)
- Which type is the second level (epics)
- Which type is the third level (tasks)
- Column definitions derived from configured statuses

### Phase 4: Validate current artifacts against configuration

Ensure all existing delivery artifacts conform to the configured type definitions.

## Tasks

- [x] [TASK-bc24af0b](TASK-bc24af0b): Define delivery type schema in project.json and Rust/TS types
- [x] [TASK-fcb7ddc4](TASK-fcb7ddc4): Replace hardcoded parent-child field references in integrity checks
- [x] [TASK-c2db174d](TASK-c2db174d): Make roadmap components read type hierarchy from config
- [ ] [TASK-a5606ccd](TASK-a5606ccd): Validate existing artifacts against delivery type configuration
- [ ] [TASK-26acd796](TASK-26acd796): Project settings UI for managing delivery types and state machine
