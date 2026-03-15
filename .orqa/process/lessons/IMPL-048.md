---
id: IMPL-048
title: "Enforcement epics must close the loop — run tooling, act on output, create follow-up work"
description: "EPIC-061 (principle enforcement foundations) was planned with 5 phases building enforcement tooling but no phase to actually run the tooling, review output, and create work to address findings. An enforcement epic that builds tools but doesn't use them is incomplete. Planning methodology should require a 'close the loop' phase for any epic that produces enforcement or audit tooling."
status: active
created: "2026-03-13"
updated: "2026-03-13"
maturity: observation
recurrence: 1
relationships:
  - target: RULE-022
    type: observes
    rationale: "RULE-022 defines plan structure requirements but doesn't mandate a loop-closure phase for enforcement/tooling epics"
  - target: EPIC-061
    type: informed-by
    rationale: "Discovered when EPIC-061 was missing a phase to run its own tooling and act on results"
  - target: PILLAR-002
    type: observes
    rationale: "Learning Through Reflection — the epic builds reflection tooling but didn't include the reflection step itself"
  - target: IMPL-049
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IMPL-049"
---
## Pattern

When planning an epic that builds enforcement or audit tooling, the natural tendency is to stop at "tooling is built and works." But tooling that isn't run and acted upon is inert. The gap audit tool (TASK-374) would detect future *principle* enforcement gaps, but no tooling catches *planning* incompleteness — specifically, the absence of a "close the loop" phase.

## Fix

Two changes needed:
1. **Planning methodology**: Update the `planning` skill or [RULE-022](RULE-022) to require that any epic producing enforcement/audit tooling includes a final phase that runs the tooling, reviews output, and creates follow-up work (epics or tasks) to address findings.
2. **Automated gap audit scope**: The gap audit tool (TASK-374) should check not just principle enforcement but also whether enforcement epics have loop-closure tasks. This is a second-order check: "does the tooling that checks tooling also check itself?"
