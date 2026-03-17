---
id: IMPL-042
title: Epic body drifts from actual work — no standing mechanism to maintain consistency
description: "When tasks are added, pillars updated, or scope evolves mid-epic, the epic body (task table, pillars array, docs-produced) isn't updated in sync. This causes incomplete epic bodies at closure that need manual reconciliation. A standing reconciliation task per epic would force ongoing maintenance."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: RULE-004
    type: informs
    rationale: "RULE-004 governs epic lifecycle but doesn't enforce epic body maintenance during the epic"
  - target: RULE-004
    type: informed-by
    rationale: "Lesson promoted to RULE-004 — standing reconciliation task requirement added to epic lifecycle"
  - target: RULE-004
    type: informed-by
    rationale: "RULE-004 codified the epic body reconciliation task requirement first observed in this lesson"
---

## Pattern

During [EPIC-059](EPIC-059), work emerged mid-epic (TASK-310 for AD memory injection, [PILLAR-003](PILLAR-003) creation). These were completed as tasks but the epic body wasn't updated — the task table was missing [TASK-310](TASK-310) and the pillars array didn't include [PILLAR-003](PILLAR-003). This was only caught during the manual completion review. The pattern: epic bodies drift as work evolves, and nothing enforces sync until someone notices at the end.

## Fix

Create a standing "Reconcile EPIC-NNN" task automatically when any epic is created. This task:
1. Cannot be marked done until the epic is ready to close
2. Has acceptance criteria that check epic body accuracy (task table, pillars, docs-produced, scope)
3. Depends on all other tasks in the epic (always last)
4. Forces the orchestrator to maintain epic body consistency throughout the epic lifecycle, not just at closure
