---
id: EPIC-010
title: "Developer Experience Polish"
status: draft
priority: P3
milestone: MS-001
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 2
  impact: 2
  dependency: 1
  effort: 2
score: 4.5
roadmap-ref: "D10"
docs-required:
  - docs/architecture/project-configuration.md
docs-produced:
  - docs/architecture/project-configuration.md (update with local DB path)
  - docs/architecture/decisions.md (AD for project-local database location)
description: >
  Quality-of-life improvements for dogfooding, including project-local
  database, build splash window, and system prompt templates.
tags: [dx, polish, dogfooding]
---

## Why P3

Quality of life improvements for dogfooding. Not blocking but make daily use more pleasant.

## Tasks

- [ ] Project-local database — move SQLite from `app_data_dir` to `.orqa/orqa.db` so session history travels with the project
- [ ] Build splash window — small branded window during `make dev` compilation
- [ ] Custom system prompt templates — pre-built prompts for common scenarios (dogfooding, greenfield, legacy)
