---
id: TASK-484
title: "Move product/ → about/ (11 files) + update project.json"
description: "Rename the documentation chapter from product/ to about/ by moving all 11 files via git mv. Update project.json to replace the product key with the about key and correct the path."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-075
depends-on: []
acceptance:
  - All 11 product/ files moved to about/ via git mv
  - project.json updated (product key renamed to about, path updated)
  - All files scannable in new location via artifact browser
  - No broken artifact references to old product/ paths
relationships:
  - target: EPIC-075
    type: delivers
    rationale: First migration phase of the documentation reorganisation
  - target: EPIC-075
    type: belongs-to
    rationale: Task belongs to this epic
---
