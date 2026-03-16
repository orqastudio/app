---
id: EPIC-010
title: Developer Experience Polish
description: "Quality-of-life improvements for dogfooding, including project-local database, build splash window, and system prompt templates."
status: captured
priority: P3
created: 2026-03-07
updated: 2026-03-07
horizon: next
scoring: null
relationships:
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-161
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-162
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
  - target: TASK-069
    type: delivered-by
---
## Why P3

Quality of life improvements for dogfooding. Not blocking but make daily use more pleasant.

## Tasks

- [ ] Project-local database — move SQLite from `app_data_dir` to `.orqa/orqa.db` so session history travels with the project
- [ ] Build splash window — small branded window during `make dev` compilation
- [ ] Custom system prompt templates — pre-built prompts for common scenarios (dogfooding, greenfield, legacy)

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
