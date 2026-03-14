---
id: TASK-485
title: "Move architecture/ → development/ and reference/ (18 files) + assign missing DOC IDs"
description: "Split the architecture/ chapter: move 13 files to development/ and 5 files to reference/. Assign DOC IDs to core-architecture.md and plugin-architecture.md which currently lack them. Remove the architecture key from project.json."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-075
depends-on:
  - TASK-484
acceptance:
  - 13 architecture files moved to development/ via git mv
  - 5 architecture files moved to reference/ via git mv
  - core-architecture.md and plugin-architecture.md assigned DOC IDs in frontmatter
  - project.json updated (architecture key removed, development and reference keys updated)
  - No broken references to old architecture/ paths
relationships:
  - target: EPIC-075
    type: delivers
    rationale: Architecture chapter migration phase of the documentation reorganisation
  - target: EPIC-075
    type: belongs-to
    rationale: Task belongs to this epic
---
