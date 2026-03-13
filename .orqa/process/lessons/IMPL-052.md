---
id: IMPL-052
title: "Orchestrator stops to ask permission when not blocked — breaks flow"
description: "After completing Phase 1 of EPIC-061, the orchestrator asked 'shall I continue?' instead of just continuing. It was not blocked — no dependency gate, no user decision needed, no ambiguity. Asking permission when unblocked wastes the user's time and breaks momentum."
status: recurring
created: "2026-03-13"
updated: "2026-03-13"
maturity: observation
recurrence: 2
relationships:
  - target: RULE-004
    type: observes
    rationale: "Task dependency gates define when work is blocked. If no gate is blocking, work should continue."
  - target: IMPL-049
    type: informed-by
    rationale: "Same pattern of unnecessary user interaction — the orchestrator defers when it should act"
---

## Pattern

After completing a task, the orchestrator asks "shall I continue?" or "ready for the next phase?" when there is no blocking dependency, no ambiguity, and no user decision needed. This breaks flow and forces the user to type "yes" for no reason. The user already approved the epic scope — the orchestrator has standing authorization to execute the plan.

## Fix

The orchestrator should only stop and ask when:
1. A dependency gate is not met
2. A user decision is needed (scope change, ambiguity, trade-off)
3. Work is complete (epic done)

If none of these apply, continue to the next task. The user said "are you blocked?" — which is the correct test. If not blocked, keep going.
