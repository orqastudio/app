---
id: "TASK-6b0459ea"
title: "Automated gap audit tool (repeatable RES-dd35bbf0)"
description: "Build a repeatable version of the RES-dd35bbf0 audit as tooling that scans rules, ADs, lessons, and pipeline stages for enforcement gaps"
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
acceptance:
  - "Gap audit tool scans all rules, ADs, lessons, and pipeline stages and outputs a prioritized gap report"
relationships:
  - target: "EPIC-4e6e9eae"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-4017a3de"
    type: "depends-on"
  - target: "TASK-2ecb94ed"
    type: "depends-on"
  - target: "TASK-530bb45d"
    type: "depends-on"
  - target: "TASK-7cf80542"
    type: "depended-on-by"
  - target: "TASK-247b6ce4"
    type: "depended-on-by"
---
## What

Build a repeatable automated gap audit tool that replaces the manual [RES-dd35bbf0](RES-dd35bbf0) audit.

## How

Extend verify-pipeline-integrity.mjs or create a new script that scans all rules for enforcement mechanism, all ADs for enforcement chain completeness, all lessons for promotion status, and pipeline stage transitions for gaps. Output a prioritized gap report.

## Verification

Completed as part of [EPIC-4e6e9eae](EPIC-4e6e9eae) Phase 7.

## Lessons

No new lessons.
