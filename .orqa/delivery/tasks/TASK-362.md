---
id: TASK-362
title: "Extend pipeline integrity tool with enforcement chain checks"
description: "Add checks for AD enforcement gaps, promoted lesson targets, and rule-AD reference consistency to verify-pipeline-integrity.mjs"
status: done
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-061
depends-on:
  - TASK-350
  - TASK-351
acceptance:
  - "verify-pipeline-integrity.mjs checks for accepted ADs without enforcement relationships, promoted lessons without promoted-to targets, and rules referencing ADs without enforces relationships"
---

## What

Extend the pipeline integrity verification tool with enforcement chain completeness checks.

## How

Add new check functions to verify-pipeline-integrity.mjs covering AD enforcement gaps, lesson promotion targets, and rule-AD reference consistency.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 1.

## Lessons

No new lessons.
