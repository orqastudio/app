---
id: TASK-168
title: "Frontend: Client-side sorting, filtering, and grouping logic"
description: Implement the ArtifactViewState store integration and client-side sort/filter/group logic that transforms the DocNode array based on user selections.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-005
depends-on:
  - TASK-167
acceptance:
  - Filtering by any enum field works correctly
  - Sorting by title, created, updated, and type-specific fields works
  - Grouping produces collapsible sections with correct ordering
  - Defaults load from _navigation.json on first navigation to a type
  - User overrides persist in the store per type key within the session
  - Tree rendering is removed
  - make typecheck passes
---


## What

The reactive logic that connects the toolbar controls to the artifact list rendering. When the user changes sort/filter/group settings, the list updates immediately via client-side transformations.

## How

1. Add `viewStates: Record<string, ArtifactViewState>` to NavigationStore
2. On `navigateToPath`, initialise the view state from `_navigation.json` defaults if not already set
3. Create utility functions in a new `ui/src/lib/utils/artifact-view.ts`:
   - `applyFilters(nodes, filters)` — filter DocNode array by frontmatter values
   - `applySort(nodes, sort)` — sort DocNode array by field with direction
   - `applyGrouping(nodes, groupField, groupOrder, filterableFields)` — partition into ordered groups
4. In ArtifactNav, replace tree/flat rendering with a single path: filtered → sorted → optionally grouped
5. Render groups with collapsible headers (chevron toggle, uppercase label)
6. When layout mode is active, render curated sections instead

## Verification

- [ ] `make typecheck` passes
- [ ] Filtering removes non-matching items from the list
- [ ] Sorting reorders items correctly for each field type
- [ ] Grouping creates collapsible sections in the correct order
- [ ] Changing artifact type loads the correct defaults
- [ ] Tree rendering code is removed
