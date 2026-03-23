---
id: TASK-b1dc0073
type: task
title: Migrate rule/skill scope fields to relationship types
description: "Migrate the scope field on rules and skills from a standalone frontmatter array to scoped-to/scoped-by relationship types on the graph. Update schemas, existing artifacts, and any code that reads the scope field."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Rule schema no longer has standalone scope field
  - Skill schema no longer has standalone scope field
  - All existing scope entries migrated to scoped-to relationships with inverses on targets
  - Backend code that reads scope field updated to read relationships instead
  - Frontend code that displays scope updated
  - make verify passes clean
relationships:
  - target: EPIC-b2ca1ea3
    type: delivers
    rationale: Schema migration (F14, F15)
---