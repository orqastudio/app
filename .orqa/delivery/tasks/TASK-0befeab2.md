---
id: "TASK-0befeab2"
type: "task"
title: "Lesson promotion pipeline"
description: "Implements the self-learning loop that creates lesson entries, tracks recurrence counts, and promotes repeated patterns into rules or skills at a configurable threshold."
status: archived
created: 2026-03-05T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
acceptance:
  - "IMPL entries created and tracked"
  - "Recurrence count incremented on match"
  - "Promotion triggered at configurable threshold"
  - "Lessons viewable in UI"
relationships:
  - target: "EPIC-63ff87da"
    type: "delivers"
    rationale: "Task belongs to this epic"
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
