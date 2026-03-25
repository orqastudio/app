---
id: TASK-dde309b0
type: task
title: "Frontend: TypeScript types for schema metadata and navigation config"
description: "Add TypeScript interfaces matching the new Rust types for FilterableField, SortableField, NavigationConfig, and extend DocNode/NavType interfaces."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - All TypeScript interfaces match Rust struct shapes exactly
  - make typecheck passes
relationships:
  - target: EPIC-9ddef7f9
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-764410d7
    type: depends-on
  - target: TASK-99af06ab
    type: depended-on-by
  - target: TASK-58e87849
    type: depended-on-by
---


## What

The frontend needs TypeScript types that match the new Rust backend types so the IPC boundary is type-safe.

## How

1. Add new interfaces to `ui/src/lib/types/nav-tree.ts`
2. Extend existing `DocNode` and `NavType` interfaces with the new fields
3. Add `ArtifactViewState` interface for the navigation store

## Verification

- [ ] `make typecheck` passes
- [ ] Types match Rust struct shapes (field names, types, optionality)