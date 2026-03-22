---
id: TASK-eb73e082
title: Sweep body-text path references across all .orqa/ files
description: "After all file moves are complete, scan every .orqa/ file for body-text references to old documentation paths and update them to the new locations. Covers all 152 known references identified during the reorganisation planning."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 5
  complexity: 2
  dependencies: 4
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - All 152 body-text path references updated to new locations
  - Zero stale documentation paths remain in any .orqa/ file
  - "Verified by grep for old path patterns (product/, architecture/, process/, ui/, wireframes/)"
relationships:
  - target: EPIC-2a6e2567
    type: delivers
    rationale: Reference sweep to complete the documentation reorganisation
  - target: TASK-96db8fb0
    type: depends-on
---
