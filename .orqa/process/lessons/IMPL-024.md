---
id: IMPL-024
title: Lessons learned should be recorded on task completion artifacts
description: "When a task is completed, any observations logged or recurrence incremented during that task should be recorded in the task artifact itself. This makes the learning visible to the user as part of the completion statement, not buried in conversation history."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: TASK-308
    type: yielded-by
---
## Pattern

Currently, task artifacts have three body sections: What, How, Verification. When a task is completed, the agent updates `status: done` but doesn't record what was learned during implementation.

The user has to read conversation history to discover what observations were logged, what existing lessons had recurrence incremented, or what surprises occurred. This information is ephemeral — lost when the context window compacts.

If the task artifact itself recorded "Lessons: created [IMPL-017](IMPL-017) (stale paths), incremented [IMPL-003](IMPL-003) recurrence to 3", the learning loop becomes visible and auditable from the artifact graph alone.

## Fix

Required "Lessons" body section on task artifacts (user-approved via RES-052). Added to task schema bodyTemplate. Format:

```markdown
## Lessons

- Created [IMPL-018](IMPL-018): Hardcoded paths should be configurable
- Updated [IMPL-003](IMPL-003): recurrence 2 → 3
- None — straightforward implementation
```

"None — straightforward" is valid. Decreasing lesson frequency over time is a signal the pipeline is working.

## Triage

Promoted — task schema now requires a Lessons section in every task body. Ensures lessons are recorded at task completion.
