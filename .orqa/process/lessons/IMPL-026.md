---
id: IMPL-026
title: "Present decisions to the user one at a time, not batched"
description: "When surfacing multiple decisions for user input, presenting them all at once creates cognitive overload. The user has to read and hold context for all decisions before responding to any. Present one decision at a time, wait for the response, then move to the next."
status: active
created: "2026-03-13"
updated: "2026-03-13"
maturity: observation
recurrence: 1
relationships:
  - target: RULE-015
    type: observes
    rationale: "Honest reporting includes respecting the user's capacity to process information — a wall of decisions is not honest communication, it's a context dump"
  - target: PILLAR-001
    type: observes
    rationale: "Clarity Through Structure — structuring decisions sequentially is clearer than presenting them all at once"
  - target: IMPL-041
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-041"
  - target: IMPL-035
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-035"
  - target: IMPL-034
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-034"
  - target: IMPL-040
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-040"
---
## Pattern

The orchestrator identified 4 decisions with sub-decisions (7 total decision points) and presented them all in a single message. The user had to ask "can we do one at a time please."

This is the same anti-pattern as batching implementation without commits — accumulating output that should be delivered incrementally. Decisions have dependencies (the scope boundary decision depends on the outcomes of the other three), so batching them also hides the natural sequence.

## Fix

When presenting decisions to the user:

1. Present one decision at a time with its context
2. Wait for the user's response before presenting the next
3. If decisions have dependencies, present them in dependency order
4. Summarise the decision queue ("4 decisions to discuss") so the user knows the scope, but only expand one at a time
