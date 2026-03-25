---
id: EPIC-560cf78c
type: epic
title: Developer Experience Polish
description: "Quality-of-life improvements for dogfooding, including project-local database, build splash window, and system prompt templates."
status: captured
priority: P3
created: 2026-03-07
updated: 2026-03-07
horizon: next
scoring:
  impact: 2
  urgency: 1
  complexity: 2
  dependencies: 1
relationships:
  - target: TASK-092c5cdc
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-8ccbd213
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-d3085ce2
    type: delivered-by
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
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