---
id: TASK-167
title: "Frontend: ArtifactToolbar with sort dropdown and filter popover"
description: "Replace SearchInput in ArtifactNav with an icon-based toolbar containing sort (DropdownMenu) and filter (Popover) controls, dynamically generated from schema metadata."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - Toolbar renders at h-10 matching NavSubPanel header height
  - Sort dropdown shows options dynamically from NavType sortable_fields
  - Filter popover shows sections dynamically from NavType filterable_fields
  - Active indicators appear when sort/filter differs from defaults
  - Clear-all resets to _navigation.json defaults
  - make typecheck and make lint-frontend pass
relationships:
  - target: EPIC-005
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-166
    type: depends-on
  - target: TASK-168
    type: depended-on-by
  - target: TASK-312
    type: depended-on-by
---


## What

The core navigation toolbar that replaces the current text filter with rich sort and filter controls. This is the primary interaction point for artifact browsing.

## How

1. Create `ArtifactToolbar.svelte` in `ui/src/lib/components/navigation/` — `h-10 flex items-center gap-1 px-2` with two ghost icon buttons
2. Create `ArtifactSortDropdown.svelte` — uses shadcn DropdownMenu with RadioGroup for sort selection and group-by section
3. Create `ArtifactFilterPopover.svelte` — uses shadcn Popover with dynamically generated checkbox sections
4. Wire to ArtifactViewState in the navigation store
5. Replace SearchInput in ArtifactNav.svelte with the new toolbar

## Verification

- [ ] `make typecheck` passes
- [ ] `make lint-frontend` passes
- [ ] Toolbar visually matches h-10 height of breadcrumb and NavSubPanel headers
- [ ] Sort/filter options are generated from schema, not hardcoded
- [ ] Filter sections only appear for fields that exist in the current type's schema
