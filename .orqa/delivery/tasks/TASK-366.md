---
id: TASK-366
title: "Tooling checks: skill portability, capability-not-tools, persistence boundaries"
description: "Add verification tooling for skill portability, provider-agnostic capabilities, and data persistence boundary enforcement"
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - "Tooling scans core skills for project-specific paths, checks agent definitions use capabilities not tools, and detects governance data in SQLite or conversation data in files"
relationships:
  - target: EPIC-061
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-367
    type: depended-on-by
  - target: TASK-375
    type: depended-on-by
---

## What

Create verification tooling for three rules that need custom scanning rather than linter or hook enforcement.

## How

Build scripts or extend existing tools to scan for skill portability violations, agent definition format compliance, and persistence boundary violations.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 2.

## Lessons

No new lessons.
