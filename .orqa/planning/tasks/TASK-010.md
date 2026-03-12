---
id: TASK-010
title: Add Skills Field to Task Schema
description: "Add the skills field to the task frontmatter schema in both the tasks README and the artifact-framework documentation. This enables full traceability: plan → task → agent → skills → implementation."
status: done
created: "2026-03-08"
updated: "2026-03-08"
epic: EPIC-033
assignee: AGENT-007
skills:
  - SKILL-003
  - SKILL-011
acceptance:
  - Task frontmatter schema includes skills field (string array)
  - artifact-framework.md task schema updated with skills field
  - Field documented with purpose (traceability from plan to implementation)
  - Example task shown with assignee + skills combination
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
