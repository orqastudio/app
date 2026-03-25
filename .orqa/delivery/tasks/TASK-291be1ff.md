---
id: TASK-291be1ff
type: task
title: Cognitive load indicators
description: "Build tooling that detects excessive session complexity and surfaces warnings about too many open files, uncommitted changes, or interleaved tasks"
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - "Tooling detects and warns when a session has accumulated too much complexity (open files, uncommitted changes, interleaved tasks)"
relationships:
  - target: EPIC-a60f5b6b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-9471304a
    type: depended-on-by
  - target: TASK-79ff025c
    type: depended-on-by
---

## What

Build cognitive load indicator tooling for Pillar 3 (Purpose Through Continuity).

## How

Create tooling that monitors session complexity signals (open file count, uncommitted change count, interleaved task count) and surfaces warnings to the user.

## Verification

Completed as part of [EPIC-a60f5b6b](EPIC-a60f5b6b) Phase 6.

## Lessons

No new lessons.