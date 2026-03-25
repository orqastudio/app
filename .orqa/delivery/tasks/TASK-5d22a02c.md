---
id: "TASK-5d22a02c"
type: "task"
title: "Move process/ ui/ wireframes/ → target chapters (17 files)"
description: "Migrate the remaining three chapters to their target locations: 6 process files to guide/about/development/reference, 6 ui files to reference/, and 5 wireframe files to reference/wireframes/. Remove the process, ui, and wireframes keys from project.json and add the reference key."
status: "completed"
priority: "P1"
scoring:
  impact: 3
  urgency: 4
  complexity: 3
  dependencies: 3
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
acceptance:
  - "6 process files moved to their target chapters (guide, about, development, or reference) via git mv"
  - "6 ui files moved to reference/ via git mv"
  - "5 wireframe files moved to reference/wireframes/ via git mv"
  - "project.json updated (process, ui, wireframes keys removed; reference key added)"
  - "No broken references to old process/, ui/, or wireframes/ paths"
relationships:
  - target: "EPIC-4d60940b"
    type: "delivers"
    rationale: "Process, UI, and wireframes chapter migration phase of the documentation reorganisation"
  - target: "TASK-99839c77"
    type: "depends-on"
---