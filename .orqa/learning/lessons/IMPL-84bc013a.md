---
id: "IMPL-84bc013a"
type: lesson
title: "Epic body drifts from actual work — no standing mechanism to maintain consistency"
description: "When tasks are added, pillars updated, or scope evolves mid-epic, the epic body (task table, pillars array, docs-produced) isn't updated in sync. This causes incomplete epic bodies at closure that need manual reconciliation. A standing reconciliation task per epic would force ongoing maintenance."
status: completed
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships:
  - type: cautions
    target: EPIC-bbac7876
    rationale: "Epic body drifts from actual work — cautions delivery pipeline consistency"
---

## Pattern

During [EPIC-88f359b0](EPIC-88f359b0), work emerged mid-epic (TASK-cb213c0d for AD memory injection, [PILLAR-a6a4bbbb](PILLAR-a6a4bbbb) creation). These were completed as tasks but the epic body wasn't updated — the task table was missing [TASK-cb213c0d](TASK-cb213c0d) and the pillars array didn't include [PILLAR-a6a4bbbb](PILLAR-a6a4bbbb). This was only caught during the manual completion review. The pattern: epic bodies drift as work evolves, and nothing enforces sync until someone notices at the end.

## Fix

Create a standing "Reconcile EPIC-NNN" task automatically when any epic is created. This task:

1. Cannot be marked done until the epic is ready to close
2. Has acceptance criteria that check epic body accuracy (task table, pillars, docs-produced, scope)
3. Depends on all other tasks in the epic (always last)
4. Forces the orchestrator to maintain epic body consistency throughout the epic lifecycle, not just at closure
