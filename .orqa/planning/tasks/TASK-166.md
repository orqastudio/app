---
id: TASK-166
title: "Frontend: TypeScript types for schema metadata and navigation config"
description: Add TypeScript interfaces matching the new Rust types for FilterableField, SortableField, NavigationConfig, and extend DocNode/NavType interfaces.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-005
depends-on:
  - TASK-165
scope:
  - Add FilterableField, SortableField interfaces to nav-tree.ts
  - Add NavigationConfig, NavigationDefaults, NavigationLayout, SortConfig interfaces
  - Extend DocNode with frontmatter field
  - Extend NavType with filterable_fields, sortable_fields, navigation_config
  - Add ArtifactViewState interface for per-type sort/filter/group state
acceptance:
  - All TypeScript interfaces match Rust struct shapes exactly
  - make check-frontend passes
---

## What

The frontend needs TypeScript types that match the new Rust backend types so the IPC boundary is type-safe.

## How

1. Add new interfaces to `ui/lib/types/nav-tree.ts`
2. Extend existing `DocNode` and `NavType` interfaces with the new fields
3. Add `ArtifactViewState` interface for the navigation store

## Verification

- [ ] `make check-frontend` passes
- [ ] Types match Rust struct shapes (field names, types, optionality)
