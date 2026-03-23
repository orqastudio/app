---
id: TASK-96db8fb0
type: task
title: Move process/ ui/ wireframes/ → target chapters (17 files)
description: "Migrate the remaining three chapters to their target locations: 6 process files to guide/about/development/reference, 6 ui files to reference/, and 5 wireframe files to reference/wireframes/. Remove the process, ui, and wireframes keys from project.json and add the reference key."
status: completed
priority: P1
scoring:
  impact: 3
  urgency: 4
  complexity: 3
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - "6 process files moved to their target chapters (guide, about, development, or reference) via git mv"
  - 6 ui files moved to reference/ via git mv
  - 5 wireframe files moved to reference/wireframes/ via git mv
  - "project.json updated (process, ui, wireframes keys removed; reference key added)"
  - "No broken references to old process/, ui/, or wireframes/ paths"
relationships:
  - target: EPIC-2a6e2567
    type: delivers
    rationale: Process, UI, and wireframes chapter migration phase of the documentation reorganisation
  - target: TASK-28a2989f
    type: depends-on
  - target: TASK-eb73e082
    type: depended-on-by
  - target: TASK-700bb275
    type: depended-on-by
  - target: TASK-bad2c95f
    type: depended-on-by
---