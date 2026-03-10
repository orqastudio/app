---
id: EPIC-010
title: Developer Experience Polish
description: Quality-of-life improvements for dogfooding, including project-local database, build splash window, and system prompt templates.
status: draft
priority: P3
created: "2026-03-07"
updated: "2026-03-07"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs: []
docs-required:
  - DOC-009
docs-produced:
  - DOC-009
  - DOC-001
scoring:
  pillar: 2
  impact: 2
  dependency: 1
  effort: 2
  score: 4.5
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
