---
id: TASK-474
title: "Migrate rule/skill scope fields to relationship types"
description: "Migrate the scope field on rules and skills from a standalone frontmatter array to scoped-to/scoped-by relationship types on the graph. Update schemas, existing artifacts, and any code that reads the scope field."
status: done
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on: []
assignee: null
skills:
  - SKILL-011
  - SKILL-039
  - SKILL-050
acceptance:
  - Rule schema no longer has standalone scope field
  - Skill schema no longer has standalone scope field
  - All existing scope entries migrated to scoped-to relationships with inverses on targets
  - Backend code that reads scope field updated to read relationships instead
  - Frontend code that displays scope updated
  - make verify passes clean
relationships:
  - target: EPIC-073
    type: delivers
    rationale: Schema migration (F14, F15)
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
