---
id: IMPL-034
title: The orchestrator should periodically zoom out and sanity-check scope
description: "During extended design discussions, the orchestrator can lose sight of the bigger picture — accumulating decisions, observations, and tasks without pausing to verify the whole still makes sense. A periodic zoom-out sanity check should be automatic, not user-prompted."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships:
  - target: RULE-028
    type: informs
    rationale: "Systems thinking requires seeing the whole — the orchestrator was in detail mode without checking the system view"
  - target: IMPL-026
    type: informed-by
    rationale: "Same pattern as batching decisions — accumulating without pausing to assess"
  - target: IMPL-035
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-035"
  - target: IMPL-040
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-040"
---
## Pattern

This session has produced 17 observations (IMPL-018 through IMPL-034), 2 ideas (IDEA-069, 070), 8 tasks, a research document, and numerous design decisions. At no point did the orchestrator pause unprompted to ask: "Are we still on track? Has the scope drifted? Is the epic still coherent?"

The user had to prompt the zoom-out. This should be automatic — after N decisions or N observations, the orchestrator should pause and present a coherence check: here's what we've decided, here's the current scope, here's what's changed, does this still make sense?

## Fix

Not yet determined. Possible triggers for automatic zoom-out:
1. After every N observations logged (e.g., every 5)
2. After scope-changing decisions (new ideas, epic boundary changes)
3. At natural conversation boundaries (topic shift, decision batch complete)
4. Time-based (every 30 minutes of active discussion)
