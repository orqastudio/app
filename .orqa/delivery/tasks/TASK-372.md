---
id: TASK-372
title: "Research trigger: orchestrator creates RES-NNN before investigation"
description: "Update orchestrator behavior to recognize investigation-class requests and create RES-NNN artifacts before delegating research"
status: done
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-061
depends-on: []
acceptance:
  - "Orchestrator creates RES-NNN artifacts before delegating investigation-class requests to researcher agents"
---

## What

Ensure the orchestrator creates research artifacts before delegating investigation work.

## How

Update orchestrator rules or skills to recognize investigation-class requests and create RES-NNN artifacts in .orqa/planning/research/ before delegating to a researcher agent.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 4.

## Lessons

No new lessons.
