---
id: TASK-486
title: Move process/ ui/ wireframes/ → target chapters (17 files)
description: "Migrate the remaining three chapters to their target locations: 6 process files to guide/about/development/reference, 6 ui files to reference/, and 5 wireframe files to reference/wireframes/. Remove the process, ui, and wireframes keys from project.json and add the reference key."
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - "6 process files moved to their target chapters (guide, about, development, or reference) via git mv"
  - 6 ui files moved to reference/ via git mv
  - 5 wireframe files moved to reference/wireframes/ via git mv
  - "project.json updated (process, ui, wireframes keys removed; reference key added)"
  - "No broken references to old process/, ui/, or wireframes/ paths"
relationships:
  - target: EPIC-075
    type: delivers
    rationale: Process, UI, and wireframes chapter migration phase of the documentation reorganisation
  - target: TASK-485
    type: depends-on
  - target: TASK-487
    type: depended-on-by
  - target: TASK-488
    type: depended-on-by
  - target: TASK-489
    type: depended-on-by
---
