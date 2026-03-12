---
id: TASK-166
title: "Frontend: TypeScript types for schema metadata and navigation config"
description: Add TypeScript interfaces matching the new Rust types for FilterableField, SortableField, NavigationConfig, and extend DocNode/NavType interfaces.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-005
depends-on:
  - TASK-165
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
