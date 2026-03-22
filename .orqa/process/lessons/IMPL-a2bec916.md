---

id: IMPL-a2bec916
type: lesson
title: The orchestrator should periodically zoom out and sanity-check scope
description: "During extended design discussions, the orchestrator can lose sight of the bigger picture — accumulating decisions, observations, and tasks without pausing to verify the whole still makes sense. A periodic zoom-out sanity check should be automatic, not user-prompted."
status: active
created: 2026-03-13
updated: 2026-03-13
maturity: observation
recurrence: 1
relationships: []
---

## Pattern

This session has produced 17 observations (IMPL-c306b136 through IMPL-a2bec916), 2 ideas (IDEA-1c7ebe74, 070), 8 tasks, a research document, and numerous design decisions. At no point did the orchestrator pause unprompted to ask: "Are we still on track? Has the scope drifted? Is the epic still coherent?"

The user had to prompt the zoom-out. This should be automatic — after N decisions or N observations, the orchestrator should pause and present a coherence check: here's what we've decided, here's the current scope, here's what's changed, does this still make sense?

## Fix

Not yet determined. Possible triggers for automatic zoom-out:
1. After every N observations logged (e.g., every 5)
2. After scope-changing decisions (new ideas, epic boundary changes)
3. At natural conversation boundaries (topic shift, decision batch complete)
4. Time-based (every 30 minutes of active discussion)
