---
id: TASK-d5971d0d
type: task
title: Register roadmap view under Process navigation
description: Make the roadmap kanban view accessible under Process in navigation and handle the static roadmap.md.
status: completed
priority: P2
scoring:
  impact: 2
  urgency: 2
  complexity: 1
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Roadmap view accessible under Process in navigation
  - Static roadmap.md doc removed or deprecated
relationships:
  - target: EPIC-c08f0b83
    type: delivers
    rationale: Navigation registration makes the roadmap view discoverable
  - target: TASK-c08f0b83
    type: depends-on
---

## Scope

Update navigation configuration to register the roadmap kanban view under the Process section. Add route registration for the view. Remove or deprecate the static roadmap.md documentation page in favour of the dynamic view.