---
id: TASK-10ab6c5c
type: task
title: "ESCALATION: Strengthen enforcement for lesson IMPL-90f9ae97 (recurrence 3)"
description: Lesson IMPL-90f9ae97 has recurrence 3 and status "promoted" but no associated rule found — check promoted-to relationship
status: captured
priority: critical
created: 2026-03-22
updated: 2026-03-22
relationships:
  - target: IMPL-90f9ae97
    type: addresses
    rationale: Escalation task for lesson with recurrence 3
  - target: EPIC-2362adfc
    type: delivers
    rationale: Escalation task linked to active epic
---
## What

Lesson IMPL-90f9ae97 has recurrence 3 and status "promoted" but no associated rule found — check promoted-to relationship

## Why

The rule exists but enforcement is insufficient — recurrence continues post-promotion. Strengthening enforcement means adding mechanical checks (lint rules, hooks, or gates) that catch violations before they reach production.

## Acceptance

- [ ] enforcement_updated date added to associated rule and lesson recurrence reset to 0
- [ ] Recurrence does not increase in the next session