---
id: IMPL-040
title: Decisions scrolled out of view by automated work must be resurfaced
description: "When the orchestrator presents a decision to the user and then launches background agents whose output scrolls the decision out of view, the user loses context. The orchestrator must re-present pending decisions after automated work completes, not assume the user remembers what was asked."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: TASK-310
    type: yielded-by
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
