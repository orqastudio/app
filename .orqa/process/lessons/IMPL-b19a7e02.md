---
id: "IMPL-b19a7e02"
type: lesson
title: "Enforcement epics must close the loop — run tooling, act on output, create follow-up work"
description: "EPIC-a60f5b6b (principle enforcement foundations) was planned with 5 phases building enforcement tooling but no phase to actually run the tooling, review output, and create work to address findings. An enforcement epic that builds tools but doesn't use them is incomplete. Planning methodology should require a 'close the loop' phase for any epic that produces enforcement or audit tooling."
status: "active"
created: "2026-03-13"
updated: "2026-03-13"
maturity: "observation"
recurrence: 1
relationships: []
---
## Pattern

When planning an epic that builds enforcement or audit tooling, the natural tendency is to stop at "tooling is built and works." But tooling that isn't run and acted upon is inert. The gap audit tool (TASK-52101e2a) would detect future *principle* enforcement gaps, but no tooling catches *planning* incompleteness — specifically, the absence of a "close the loop" phase.

## Fix

Two changes needed:
1. **Planning methodology**: Update the `planning` skill or [RULE-dccf4226](RULE-dccf4226) to require that any epic producing enforcement/audit tooling includes a final phase that runs the tooling, reviews output, and creates follow-up work (epics or tasks) to address findings.
2. **Automated gap audit scope**: The gap audit tool (TASK-52101e2a) should check not just principle enforcement but also whether enforcement epics have loop-closure tasks. This is a second-order check: "does the tooling that checks tooling also check itself?"