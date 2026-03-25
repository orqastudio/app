---
id: TASK-8e9ca15d
type: task
title: Skills content audit (48 skills)
description: "Audit all 48 skills: fix layer classifications, resolve overlaps, establish naming convention (orqa-* = project-specific), remove stale content references, verify category accuracy."
status: completed
created: 2026-03-13
updated: 2026-03-13
assignee: null
docs: []
acceptance:
  - All skills have correct layer classification
  - No overlapping skills without documented justification
  - "Naming convention enforced: orqa-* = project-specific, no prefix = portable"
  - No stale content references
  - Category accuracy verified for all skills
rule-overrides: []
relationships:
  - target: EPIC-88f359b0
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-d16b7868
    type: depends-on
  - target: TASK-f717d20c
    type: depended-on-by
  - target: TASK-d2b54d2f
    type: depended-on-by
---

## What

Content audit of all 48 skills for correct classification and content alignment.

## How

1. List all skills with their current layer and category
2. For each: is the layer correct? (core vs project)
3. For each: does the name follow convention? (orqa-* for project-specific)
4. Identify overlaps between skills → resolve or document justification
5. Remove references to files/paths that no longer exist
6. Verify category (methodology/domain/tool) matches content

## Verification

- Skills audit report showing all changes made
- No layer misclassifications remain
- Naming convention consistently applied