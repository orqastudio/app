---
id: IMPL-022
title: Epics with all tasks done but not marked complete should surface in the UI
description: "When all tasks under an epic reach status:done but the epic itself remains in-progress (because it awaits human gate approval), the system should proactively surface this to the user via a dashboard or notification. Otherwise the epic sits in limbo with no visibility."
status: completed
created: 2026-03-13
updated: 2026-03-13
maturity: understanding
recurrence: 1
relationships:
  - target: IMPL-021
    type: informed-by
    rationale: "The open-item tracking gap includes the related problem of epic completion visibility — both are about surfacing state that currently exists only in agent context"
  - target: RULE-004
    type: informs
    rationale: "Artifact lifecycle defines epic completion gates but has no mechanism for surfacing epics that are ready for the gate check"
  - target: RULE-004
    type: informed-by
    rationale: "Lesson promoted to RULE-004 — epic readiness surfacing requirement added to artifact lifecycle"
  - target: RULE-004
    type: informed-by
    rationale: "RULE-004 codified the epic completion visibility requirement first observed in this lesson"
---

## Pattern

An epic has 21 tasks, all marked `done`. The epic itself is `in-progress` because the human gate hasn't been passed. But there's no dashboard, notification, or status indicator that tells the user "this epic has all tasks completed and is awaiting your review."

The user has to manually check epic status and cross-reference task statuses to discover this. In a project with multiple active epics, this creates invisible bottlenecks.

## Fix

Automatic surfacing (user-approved via RES-052). When all tasks under an epic have status: done but the epic itself is not marked done, the system surfaces it to the user for review. This prevents epics from staying perpetually open by lack of attention. Combined with the human gate on epic completion (IMPL-021), this creates a pull mechanism: system nudges, user decides.

## Triage

Promoted — epic readiness surfacing added to [RULE-004](RULE-004). Human gate ensures epics don't languish when all tasks done.
