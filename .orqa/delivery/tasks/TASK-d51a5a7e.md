---
id: TASK-d51a5a7e
title: Move product/ → about/ (11 files) + update project.json
description: Rename the documentation chapter from product/ to about/ by moving all 11 files via git mv. Update project.json to replace the product key with the about key and correct the path.
status: completed
priority: P1
scoring:
  impact: 3
  urgency: 4
  complexity: 2
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - All 11 product/ files moved to about/ via git mv
  - "project.json updated (product key renamed to about, path updated)"
  - All files scannable in new location via artifact browser
  - No broken artifact references to old product/ paths
relationships:
  - target: EPIC-2a6e2567
    type: delivers
    rationale: First migration phase of the documentation reorganisation
  - target: TASK-28a2989f
    type: depended-on-by
---
