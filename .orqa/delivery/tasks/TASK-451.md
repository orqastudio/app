---
id: TASK-451
title: Fix docs nav — show categories instead of status
description: "Update documentation navigation to show top-level categories (Architecture, Product, Development, etc.) instead of status values."
status: completed
priority: P2
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "Documentation section in nav shows top-level doc categories (Architecture, Product, Development, Process, UI, Guide, etc.) not status values"
  - "Navigation component for doc-type artifacts groups by directory structure, not by status field"
  - Each category shows its README icon and label if available
  - Clicking a category expands to show the pages within it
relationships:
  - target: EPIC-071
    type: delivers
    rationale: Category-based nav matches how users think about documentation
---

## Scope

Update the navigation component for doc-type artifacts to display top-level directory categories rather than status-based grouping.
