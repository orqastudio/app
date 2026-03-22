---
id: TASK-47a4b670
title: Surface prioritisation criteria and require justification on epics/tasks
description: Prioritisation criteria should be visible in the UI. Every epic and task with a priority value must have justification against the defined criteria as part of its content.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 3
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Prioritisation criteria from DOC-c8de180d surfaced in the artifact viewer when viewing epics/tasks
  - Epics and tasks with priority values show their justification (scoring field or dedicated section)
  - Missing justification flagged as action needed
  - Integrity validator warns on priority without justification
relationships:
  - target: EPIC-58ba6d53
    type: delivers
    rationale: Artifact viewer should surface priority reasoning
---

## Scope

Priority assessment criteria live in `.orqa/documentation/about/priority-assessment.md` (DOC-c8de180d). Currently, epics have an optional `scoring` field with freeform rationale, but this isn't enforced or surfaced prominently.

- Display scoring/justification alongside priority in the metadata box
- If an epic/task has `priority` but no `scoring` or justification content, flag as action needed
- Add check to integrity validator: priority without justification is a warning
