---
id: TASK-021
title: Lesson promotion pipeline
status: done
epic: EPIC-037
description: Implements the self-learning loop that creates lesson entries, tracks recurrence counts, and promotes repeated patterns into rules or skills at a configurable threshold.
created: 2026-03-05
updated: 2026-03-09
assignee: backend-engineer
skills: [orqa-governance, orqa-ipc-patterns]
scope: [src-tauri/src/domain/, ui/lib/components/]
acceptance: [IMPL entries created and tracked, Recurrence count incremented on match, Promotion triggered at configurable threshold, Lessons viewable in UI]
---

## What

Implement the lesson promotion pipeline: create IMPL entries, track recurrence,
promote to rules/skills at threshold.

## Outcome

Pipeline implemented with config-driven recurrence threshold in project.json.
Lessons viewable and promotable through the app. Git commit: `ebabb95`.
