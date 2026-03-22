---
id: IMPL-e5ddb8f2
title: Priority without justification is noise
description: "A P1 label without reasoning is an assertion, not a decision. Every priority assignment should have visible justification against defined criteria so stakeholders can evaluate and challenge it."
status: active
created: 2026-03-14
updated: 2026-03-14
recurrence: 1
maturity: understanding
relationships:
  - target: EPIC-58ba6d53
    type: cautions
    rationale: "TASK-47a4b670 surfaces prioritisation criteria and requires justification"
---

## Pattern

Epics have a `priority` field (P1/P2/P3) and an optional `scoring` field for rationale. But scoring is freeform, not validated, and not surfaced in the artifact viewer. The user has to open the file to see why something is P1. Without visible justification, priority becomes a habit — everything new is P1.

## Fix

Surface prioritisation criteria in the UI alongside the priority badge. Flag priority-without-justification as an action needed. The integrity validator should warn on missing scoring for prioritised artifacts.
