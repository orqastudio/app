---
id: IMPL-90f9ae97
type: lesson
title: Orchestrator must trace artifacts to their usage contexts without prompting
description: "When a new artifact or insight is captured, the orchestrator did not trace where it should be applied — the user had to prompt the connection. Every new artifact should trigger automatic impact tracing: where does this get used?"
status: promoted
created: 2026-03-21
updated: 2026-03-21
maturity: pattern
recurrence: 3
relationships:
  - target: RULE-67b91c13
    type: promoted-to
    rationale: Promoted after recurrence 3 in a single session — orchestrator observed connections without acting on them three times
---

## Pattern

When the dogfood readiness signal (IMPL-073) was captured and recorded in session state, the orchestrator did not trace where it should be applied. The user had to explicitly prompt the connection to the dogfood milestone (MS-001). The gap: artifact captured, impact not traced.

The correct behaviour is: every new artifact or insight should trigger an automatic question — "where does this get used?" — and the orchestrator should update all affected artifacts (milestones, rules, pillars, epics) without being asked.

## Root Cause

The orchestrator applied the *recording* step of systems thinking (capture the lesson) but not the *tracing* step (propagate second-order effects). Systems thinking requires both. Capturing a lesson without tracing its usage contexts is incomplete systems thinking.

This is distinct from failing to capture the lesson at all. The artifact existed. The connection did not.

## Significance

This is a correction, not an observation — which means per IMPL-073's own logic, the orchestrator's systems thinking process is not yet mature at the tracing step. The learning loop is working (lessons are captured) but the propagation step is missing.

The thinking mode injection or session start protocol should include an explicit prompt: after any new artifact is created, ask "where does this get used?" and update all affected artifacts before considering the task complete.

## Suggested Promotion

If recurrence >= 2, add to the orchestrator's session start protocol or thinking mode template: "For every new artifact created (lesson, rule, decision, knowledge), ask: where does this get used? Trace to milestones, pillars, rules, and epics. Update all affected artifacts without waiting to be prompted."

Consider also adding a process gate: a task that produces a new artifact is not complete until affected artifacts have been updated or the orchestrator has explicitly confirmed no affected artifacts exist.