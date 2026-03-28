---
id: TASK-9dc457ee
type: task
title: "ESCALATION: Promote lesson IMPL-0c9a5882 (recurrence 3)"
description: Lesson IMPL-0c9a5882 has recurrence 3 but status is "review" — needs promoting to a rule
status: captured
priority: critical
created: 2026-03-22
updated: 2026-03-22
relationships:
  - target: IMPL-0c9a5882
    type: addresses
    rationale: Escalation task for lesson with recurrence 3
  - target: EPIC-2867fe9a
    type: delivers
    rationale: Escalation task linked to active epic
---

## What

Lesson IMPL-0c9a5882 has recurrence 3 but status is "review" — needs promoting to a rule

## Why

The lesson must be promoted to a rule so it is mechanically enforced. Recurrence >= 3 means this pattern is established and will continue without a rule.

## Acceptance

- [ ] Rule created and linked to lesson IMPL-0c9a5882 via promoted-to relationship
- [ ] Recurrence does not increase in the next session
