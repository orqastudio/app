---
id: "TASK-6a07cfc9"
type: "task"
title: "ESLint rules: component purity, tooltip usage, reusable components, alias detection, root cleanliness"
description: "Add ESLint rules to mechanically enforce component purity, tooltip usage, reusable component patterns, alias detection, and root directory cleanliness"
status: archived
created: 2026-03-13T00:00:00.000Z
updated: 2026-03-13T00:00:00.000Z
acceptance:
  - "ESLint rules exist and catch violations for component purity, tooltip usage, reusable components, alias detection, and root cleanliness"
relationships:
  - target: "EPIC-a60f5b6b"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

Create ESLint rules to mechanically enforce five previously self-compliance-only rules.

## How

Add custom ESLint rules or configure existing ones for each pattern: no invoke() in $lib/components/, no title= on interactive elements, no inline empty/loading/error patterns, no duplicate keys in unions/maps, and root directory content restrictions.

## Verification

Completed as part of [EPIC-a60f5b6b](EPIC-a60f5b6b) Phase 2.

## Lessons

No new lessons.
