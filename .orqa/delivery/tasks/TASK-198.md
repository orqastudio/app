---
id: TASK-198
title: Implement evidence-before-done + learn-after-doing gates
description: |
  Process gates that fire at task completion without verification evidence or
  lesson documentation.
status: completed
created: 2026-03-11
updated: 2026-03-12
acceptance:
  - evidence-before-done gate fires at Stop when code was written but no make check/test ran
  - learn-after-doing gate fires at Stop when significant work done but no lessons checked
  - Gates return systemMessage reminders
relationships:
  - target: EPIC-052
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-195
    type: depends-on
  - target: TASK-207
    type: depended-on-by
  - target: TASK-342
    type: depended-on-by
---

## What

Two process gates:
- **evidence-before-done**: Fires at Stop event when the session included code
  writes but no `make check`/`make test` was run. Injects review prompt.
- **learn-after-doing**: Fires at Stop event when the session included significant
  work but no lessons were checked or created. Injects learning prompt.

## How

1. Add gate logic to Stop hook evaluation
2. Query WorkflowTracker for verification commands and lesson interactions
3. Return `systemMessage` with review/learning prompts

## Verification

- End session with code writes but no `make check` → reminder fires
- End session with code writes and `make check` run → no reminder
- End session with significant work but no lesson check → reminder fires
