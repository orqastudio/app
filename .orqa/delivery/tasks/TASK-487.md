---
id: TASK-487
title: "Sweep body-text path references across all .orqa/ files"
description: "After all file moves are complete, scan every .orqa/ file for body-text references to old documentation paths and update them to the new locations. Covers all 152 known references identified during the reorganisation planning."
status: done
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-075
depends-on:
  - TASK-486
acceptance:
  - All 152 body-text path references updated to new locations
  - Zero stale documentation paths remain in any .orqa/ file
  - Verified by grep for old path patterns (product/, architecture/, process/, ui/, wireframes/)
relationships:
  - target: EPIC-075
    type: delivers
    rationale: Reference sweep to complete the documentation reorganisation
  - target: EPIC-075
    type: belongs-to
    rationale: Task belongs to this epic
---
