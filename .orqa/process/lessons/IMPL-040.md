---
id: IMPL-040
title: "Decisions scrolled out of view by automated work must be resurfaced"
description: "When the orchestrator presents a decision to the user and then launches background agents whose output scrolls the decision out of view, the user loses context. The orchestrator must re-present pending decisions after automated work completes, not assume the user remembers what was asked."
status: promoted
created: "2026-03-13"
updated: "2026-03-13"
maturity: understanding
recurrence: 1
relationships:
  - target: IMPL-026
    type: informed-by
    rationale: "IMPL-026 is about presenting decisions one at a time — this observation adds that decisions must also survive being scrolled away by background work"
  - target: IMPL-034
    type: informed-by
    rationale: "Periodic zoom-outs are related — both are about maintaining user orientation during extended sessions"
  - target: RULE-004
    type: observes
    rationale: "The human gate on epic completion requires user attention — decisions that scroll away undermine this gate"
  - target: TASK-310
    type: enforces
    rationale: "TASK-310 implemented memory-based AD injection ensuring decisions persist across context compaction"
  - target: TASK-310
    type: grounded-by
    rationale: "Promoted to this task which implemented memory-based AD injection for decision persistence"
---

## Pattern

The orchestrator asked the user a pillar design decision (extend existing pillars vs create PILLAR-003). Before the user could respond, background agents completed and their notification output scrolled the question out of view. The user had to explicitly ask for the decision to be resurfaced. In a design discussion skill (IDEA-072), pending decisions should be tracked and re-presented after interruptions.

## Fix

Not yet determined. Possible approaches:
1. Track pending decisions in session state and re-present after background agent completions
2. Design discussion skill (IDEA-072) maintains a "pending decisions" queue
3. Pin important questions in the UI so they don't scroll away
4. Session tasklist (IDEA-074) could track pending decisions as a category

## Triage

Resolved by [TASK-310](TASK-310) — unimplemented ADs maintained as memory entries, surviving context compaction. Decisions no longer lost when scrolled out of view.
