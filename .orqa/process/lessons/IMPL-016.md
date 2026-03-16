---
id: IMPL-016
title: Orchestrator deferred scoped deliverable instead of implementing
description: "The orchestrator marked an epic as nearly complete while deferring a task's implementation work to a future epic, violating RULE-019."
status: active
created: 2026-03-12
updated: 2026-03-12
maturity: understanding
recurrence: 1
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "Scope integrity prevents structural gaps in deliverables"
  - target: "RULE-019"
    type: "informs"
    rationale: "Lesson documents a concrete compliance failure of RULE-019 (no deferred deliverables)"
  - target: RULE-019
    type: informs
---

## Pattern

During [EPIC-057](EPIC-057), the orchestrator reported [TASK-270](TASK-270) (AD-032 SQLite governance table removal) as documentation-only and deferred the actual table removal to a future epic. This violated [RULE-019](RULE-019) (no deferred deliverables) — the task's acceptance criteria explicitly included removing the tables, not just documenting the violation.

The root cause: the orchestrator treated "document the gap" as equivalent to "resolve the gap" when the acceptance criteria clearly required resolution.

## Fix

1. Always read acceptance criteria literally — if it says "removed from SQLite", the tables must be removed
2. If a task's implementation scope is larger than expected, report the scope to the user and ask for direction — do not silently reduce scope
3. [RULE-019](RULE-019) already covers this pattern; the violation was a compliance failure, not a missing rule
