---
id: EPIC-013
title: "Learning Through Reflection"
status: draft
priority: P1
milestone: MS-002
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - learning-through-reflection
scoring:
  pillar: 5
  impact: 4
  dependency: 2
  effort: 3
score: 8.7
roadmap-ref: "M3"
docs-required:
  - docs/architecture/lessons.md
  - docs/ui/lesson-dashboard.md
  - .orqa/plans/ (plan required before implementation)
docs-produced:
  - .orqa/plans/ (self-learning loop plan)
  - docs/architecture/lessons.md (update with automated capture pipeline)
  - docs/ui/lesson-dashboard.md (update with trend charts)
description: >
  Build the automated lesson capture and promotion pipeline on top
  of the existing lesson management CRUD and UI.
tags: [learning, lessons, promotion]
---

## Why P1

Pillar 2 (Learning Through Reflection) — the system must get smarter with each cycle.

## Tasks

- [ ] Post-session hooks that capture lessons automatically to `.orqa/lessons/`
- [ ] Rules enforcing lesson checking before implementation
- [ ] Automated promotion suggestions when recurrence >= threshold
- [ ] Lesson dashboard with recurrence trends (LayerChart)
- [ ] Session analytics — pass/fail rates, coverage trends
