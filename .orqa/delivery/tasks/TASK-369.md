---
id: TASK-369
title: Pipeline stage transition health checks
description: "Build pipeline health checks that detect stuck observations, accepted ADs without skills, skills without rules, and rules without verification"
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - "Pipeline health check reports stuck observations, missing skill coverage for ADs, missing rule coverage for skills, and missing verification for rules"
relationships:
  - target: EPIC-061
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-362
    type: depends-on
  - target: TASK-374
    type: depended-on-by
  - target: TASK-375
    type: depended-on-by
  - target: TASK-376
    type: depended-on-by
---

## What

Build stage transition health checks for the knowledge maturity pipeline.

## How

Create a pipeline-health check tool that scans for stuck observations, accepted ADs without corresponding skills, skills without corresponding rules, and rules without verification mechanisms.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 3.

## Lessons

No new lessons.
