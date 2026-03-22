---
id: TASK-614a6a45
title: "Frontend: config-driven navigation and fix app hang"
description: Replace all hardcoded navigation constants with config-driven derivation. This fixes the app hang (caused by mismatch between scanner output and hardcoded navigation expectations) and enables dynamic artifact types.
status: completed
created: 2026-03-08
updated: 2026-03-08
assignee: AGENT-cc255bc8
acceptance:
  - TypeScript types for ArtifactEntry / ArtifactTypeConfig added
  - Navigation store derives all state from config (no hardcoded constants)
  - STATIC_GROUP_SUB_CATEGORIES
  - SUB_CATEGORY_LABELS
  - ARTIFACT_ACTIVITIES deleted
  - ActivityView and ActivityGroup changed to string (dynamic from config)
  - ActivityBar renders from config
  - NavSubPanel renders group children from config
  - "No $derived(() => ...) patterns (all use $derived.by or $derived)"
  - App starts without hanging
  - Navigating to any artifact group shows correct file tree
  - npm run check passes
relationships:
  - target: EPIC-4bbc3439
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-abf6025d
    type: depends-on
  - target: TASK-dd1484aa
    type: depends-on
  - target: TASK-88ccf38a
    type: depended-on-by
---
## What

The navigation store has ~100 lines of hardcoded constants that define what
artifact types exist, what labels they have, and how they're grouped. All of
this should come from the `artifacts` config loaded via the project store.

## What Gets Deleted

- `STATIC_GROUP_SUB_CATEGORIES` constant
- `GROUP_SUB_CATEGORIES` export
- `SUB_CATEGORY_LABELS` constant
- `ARTIFACT_ACTIVITIES` constant
- `ACTIVITIES_WITH_NAV_PANEL` constant
- `COMING_SOON_ACTIVITIES` constant
- `ActivityView` string literal union → `string`
- `ActivityGroup` string literal union → `string`

## What Gets Added

- `ArtifactEntry` / `ArtifactTypeConfig` types in `project.ts`
- Getters on navigation store that derive from config:
  - `allArtifactKeys` — flat list of all type keys
  - `groupKeys` — keys of entries with children
  - `getLabelForKey(key)` — label lookup
  - `isArtifactActivity(view)` — dynamic check
  - `getChildrenForGroup(key)` — sub-categories

## Critical Fix

All `$derived(() => ...)` patterns must use `$derived.by(() => ...)`.
The arrow-function-stored-as-value pattern breaks Svelte 5 reactivity
and is the proximate cause of the app hang.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
