---
id: "IMPL-d3804cd0"
type: lesson
title: "Decisions scrolled out of view by automated work must be resurfaced"
description: "When the orchestrator presents a decision to the user and then launches background agents whose output scrolls the decision out of view, the user loses context. The orchestrator must re-present pending decisions after automated work completes, not assume the user remembers what was asked."
status: completed
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
maturity: "understanding"
recurrence: 1
relationships:
  - type: cautions
    target: EPIC-281f7857
    rationale: "Decisions scrolled away by background work must be resurfaced — cautions agent lifecycle"
---

## Pattern

The orchestrator asked the user a pillar design decision (extend existing pillars vs create PILLAR-a6a4bbbb). Before the user could respond, background agents completed and their notification output scrolled the question out of view. The user had to explicitly ask for the decision to be resurfaced. In a design discussion skill (IDEA-e2458c2c), pending decisions should be tracked and re-presented after interruptions.

## Fix

Not yet determined. Possible approaches:

1. Track pending decisions in session state and re-present after background agent completions
2. Design discussion skill (IDEA-e2458c2c) maintains a "pending decisions" queue
3. Pin important questions in the UI so they don't scroll away
4. Session tasklist (IDEA-cbbe02f7) could track pending decisions as a category

## Triage

Resolved by [TASK-cb213c0d](TASK-cb213c0d) — unimplemented ADs maintained as memory entries, surviving context compaction. Decisions no longer lost when scrolled out of view.
