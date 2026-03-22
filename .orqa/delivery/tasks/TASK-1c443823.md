---
id: TASK-1c443823
title: Implement governance recommendations
description: Built the recommendation engine that generates structured suggestions based on governance coverage analysis.
status: completed
created: 2026-03-02
updated: 2026-03-02
acceptance:
  - Recommendations are generated from analysis results
  - "Each recommendation has priority, category, and actionable text"
  - Recommendations persist across sessions
relationships:
  - target: EPIC-b1b3f5db
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-51007152
    type: depended-on-by
---
## What

Built the recommendation engine that translates coverage gaps into structured, prioritized suggestions and persists them to SQLite.

## How

Implemented a mapping from coverage gap types to recommendation templates with priority and category assignments. Recommendations are persisted via the recommendations repository and linked to the analysis record.

## Verification

Recommendations are generated from analysis results, each carries priority and category, and they persist correctly across app restarts.
