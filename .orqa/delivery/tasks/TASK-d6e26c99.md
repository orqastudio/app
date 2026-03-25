---
id: "TASK-d6e26c99"
type: "task"
title: "Pipeline stage transition health checks"
description: "Build pipeline health checks that detect stuck observations, accepted ADs without skills, skills without rules, and rules without verification"
status: "completed"
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
acceptance:
  - "Pipeline health check reports stuck observations, missing skill coverage for ADs, missing rule coverage for skills, and missing verification for rules"
relationships:
  - target: "EPIC-a60f5b6b"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-445e8155"
    type: "depends-on"
---

## What

Build stage transition health checks for the knowledge maturity pipeline.

## How

Create a pipeline-health check tool that scans for stuck observations, accepted ADs without corresponding skills, skills without corresponding rules, and rules without verification mechanisms.

## Verification

Completed as part of [EPIC-a60f5b6b](EPIC-a60f5b6b) Phase 3.

## Lessons

No new lessons.