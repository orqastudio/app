---
id: "TASK-36a4b6c8"
type: "task"
title: "Add Skills Field to Task Schema"
description: "Add the skills field to the task frontmatter schema in both the tasks README and the artifact-framework documentation. This enables full traceability: plan → task → agent → skills → implementation."
status: archived
created: 2026-03-08T00:00:00.000Z
updated: 2026-03-08T00:00:00.000Z
assignee: "AGENT-bbad3d30"
acceptance:
  - "Task frontmatter schema includes skills field (string array)"
  - "artifact-framework.md task schema updated with skills field"
  - "Field documented with purpose (traceability from plan to implementation)"
  - "Example task shown with assignee + skills combination"
relationships:
  - target: "EPIC-57dd7d4c"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

The task YAML frontmatter should include:

```yaml
assignee: backend-engineer
skills:
  - chunkhound
  - orqa-ipc-patterns
```

This creates a traceable chain:

- **Plan** defines what needs doing
- **Task** specifies who does it and what knowledge they need
- **Agent** loads those skills before starting
- **Implementation** is done with the right context

Update:

1. `tasks/README.md` — add `skills` to the frontmatter schema
2. `artifact-framework.md` — add `skills` to the task type definition

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
