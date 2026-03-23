---
id: "IMPL-38fcae1c"
type: lesson
title: "Epics are goal-completion units — never split for progress optics"
description: "Epics exist to achieve a goal. Until that goal is met to the best of current understanding, the epic isn't complete. Big epics should not be split just to feel like progress is being made. Scope is a user decision — the orchestrator should never unilaterally split an epic."
status: "active"
created: "2026-03-13"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships: []
---
## Pattern

The orchestrator asked whether [EPIC-4e6e9eae](EPIC-4e6e9eae) should be split because of its size (28 tasks, 8 phases). The implicit assumption was that smaller epics are better. But epic size is not the relevant measure — goal completeness is. An epic that achieves half its goal is not "two epics done" — it's one incomplete epic.

## Fix

Update [RULE-7b770593](RULE-7b770593) or the `planning` skill to state explicitly:
- Epics are goal-completion units, not progress-tracking units
- Epic scope is determined by the goal, not by a task count threshold
- Splitting an epic is a user decision, never an orchestrator suggestion driven by size alone
- An epic is complete when its goal is met, not when a comfortable number of tasks are done