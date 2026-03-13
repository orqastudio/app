---
id: TASK-374
title: "Automated gap audit tool (repeatable RES-054)"
description: "Build a repeatable version of the RES-054 audit as tooling that scans rules, ADs, lessons, and pipeline stages for enforcement gaps"
status: done
created: "2026-03-13"
updated: "2026-03-13"
epic: EPIC-061
depends-on:
  - TASK-362
  - TASK-368
  - TASK-369
acceptance:
  - "Gap audit tool scans all rules, ADs, lessons, and pipeline stages and outputs a prioritized gap report"
---

## What

Build a repeatable automated gap audit tool that replaces the manual [RES-054](RES-054) audit.

## How

Extend verify-pipeline-integrity.mjs or create a new script that scans all rules for enforcement mechanism, all ADs for enforcement chain completeness, all lessons for promotion status, and pipeline stage transitions for gaps. Output a prioritized gap report.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 7.

## Lessons

No new lessons.
