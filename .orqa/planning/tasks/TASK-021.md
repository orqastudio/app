---
id: TASK-021
title: Lesson promotion pipeline
description: Implements the self-learning loop that creates lesson entries, tracks recurrence counts, and promotes repeated patterns into rules or skills at a configurable threshold.
status: done
scope:
  - src-tauri/src/domain/
  - ui/lib/components/
created: "2026-03-05"
updated: "2026-03-09"
epic: EPIC-037
assignee: AGENT-002
skills:
  - SKILL-011
  - SKILL-012
acceptance:
  - IMPL entries created and tracked
  - Recurrence count incremented on match
  - Promotion triggered at configurable threshold
  - Lessons viewable in UI
---
## What

Implement the lesson promotion pipeline: create IMPL entries, track recurrence,
promote to rules/skills at threshold.

## Outcome

Pipeline implemented with config-driven recurrence threshold in project.json.
Lessons viewable and promotable through the app. Git commit: `ebabb95`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
