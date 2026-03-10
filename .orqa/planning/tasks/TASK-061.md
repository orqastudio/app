---
id: TASK-061
title: Add pillar reference field to epic/idea schemas
description: Add pillar reference field to epic/idea schemas
status: done
created: 2026-03-09
updated: 2026-03-09
epic: EPIC-046
depends-on:
  - TASK-058
assignee: orchestrator
skills:
  - orqa-governance
scope:
  - .orqa/documentation/product/artifact-framework.md
acceptance:
  - Epic schema includes optional pillars field (list of PILLAR-NNN IDs)
  - Idea schema includes optional pillars field
  - Scoring pillar-alignment dimension documented as reading from pillars field
  - Existing epics do not need backfilling (field is optional)
---