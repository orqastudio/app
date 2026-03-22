---
id: TASK-c2db174d
title: Make roadmap components read type hierarchy from config
description: "Refactor RoadmapView and its column definitions so they derive the three-level drill-down structure and column labels from the delivery type config in project settings, eliminating the hardcoded EPIC_COLUMNS and TASK_COLUMNS constants."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
created: 2026-03-15
updated: 2026-03-15
horizon: active
acceptance:
  - RoadmapView reads delivery types from project settings to determine 3-level drill-down
  - Column definitions per level derived from configured statuses (already done) and type labels from config
  - EPIC_COLUMNS and TASK_COLUMNS removed — replaced with config-driven column generation
  - Works identically for current project (milestone/epic/task)
relationships:
  - target: EPIC-ed09464b
    type: delivers
  - target: TASK-bc24af0b
    type: depends-on
---
## What

`RoadmapView` currently uses hardcoded `EPIC_COLUMNS` and `TASK_COLUMNS` constants to define the column structure for the second and third levels of the drill-down. This task replaces those constants with config-driven column generation derived from the `delivery.types` array in project settings, so the roadmap adapts automatically to any configured hierarchy.

## How

1. Read `projectSettings.delivery?.types` in `RoadmapView` (or its store). Sort by hierarchy depth: root type first (parent is null), then children by parent chain.
2. Derive level labels from `DeliveryTypeConfig.label` — e.g. level 1 = `"Milestone"`, level 2 = `"Epic"`, level 3 = `"Task"`.
3. Replace `EPIC_COLUMNS` and `TASK_COLUMNS` with a `buildColumns(type: DeliveryTypeConfig)` helper that generates the column definition array from the type's label and the existing status-based column logic (statuses are already config-driven and must remain so).
4. Delete the `EPIC_COLUMNS` and `TASK_COLUMNS` constant declarations entirely — no dead code.
5. Ensure `RoadmapView` renders identically for the current `milestone/epic/task` hierarchy: visual output is the acceptance bar for this task.
6. Component is still a display component: it receives `deliveryTypes: DeliveryTypeConfig[]` via props (from a parent store) rather than reading project settings directly.
7. Run `make check` — zero warnings, zero type errors.

## Verification

- `EPIC_COLUMNS` and `TASK_COLUMNS` do not appear anywhere in the frontend source after this change (search_regex confirms).
- Snapshot or component test: rendering `RoadmapView` with the standard three-type config produces the same column structure as the previous hardcoded version.
- TypeScript strict mode: no `any` types introduced in the column generation helper.
- `make check` passes with zero warnings and zero type errors.

## Lessons

(To be filled during/after implementation)
