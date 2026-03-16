---
id: TASK-363
title: "ESLint rules: component purity, tooltip usage, reusable components, alias detection, root cleanliness"
description: "Add ESLint rules to mechanically enforce component purity, tooltip usage, reusable component patterns, alias detection, and root directory cleanliness"
status: completed
created: 2026-03-13
updated: 2026-03-13
acceptance:
  - "ESLint rules exist and catch violations for component purity, tooltip usage, reusable components, alias detection, and root cleanliness"
relationships:
  - target: EPIC-061
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-367
    type: depended-on-by
  - target: TASK-375
    type: depended-on-by
---

## What

Create ESLint rules to mechanically enforce five previously self-compliance-only rules.

## How

Add custom ESLint rules or configure existing ones for each pattern: no invoke() in $lib/components/, no title= on interactive elements, no inline empty/loading/error patterns, no duplicate keys in unions/maps, and root directory content restrictions.

## Verification

Completed as part of [EPIC-061](EPIC-061) Phase 2.

## Lessons

No new lessons.
