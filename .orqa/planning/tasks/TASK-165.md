---
id: TASK-165
title: "Backend: Extend DocNode with frontmatter and NavType with schema metadata"
description: Extend the Rust DocNode to carry all scalar frontmatter fields, extract filterable/sortable fields from schema.json, and read _navigation.json per type directory.
status: todo
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-005
depends-on: []
scope:
  - Add frontmatter HashMap to DocNode in artifact_reader.rs
  - Read schema.json per artifact type directory and extract FilterableField (enum properties) and SortableField (date/string properties)
  - Add filterable_fields, sortable_fields, and navigation_config to NavType
  - Read _navigation.json from each type directory if present
  - Define NavigationConfig, NavigationDefaults, NavigationLayout, SortConfig Rust types
  - Include group_order in NavigationDefaults
  - Update artifact_scan_tree response to carry the new data
acceptance:
  - DocNode includes frontmatter as Option<HashMap<String, serde_json::Value>>
  - NavType includes filterable_fields, sortable_fields, navigation_config
  - schema.json enum properties are correctly extracted as FilterableField with ordered values
  - _navigation.json is parsed when present, None when absent
  - make clippy and make test-rust pass
---

## What

The frontend needs three pieces of data from the backend to drive schema-driven filtering and sorting:

1. **DocNode frontmatter** — all scalar YAML fields so the frontend can filter/sort by any field
2. **Schema metadata** — which fields are filterable (enums) and which are sortable (dates, strings)
3. **Navigation config** — per-type defaults from `_navigation.json`

All three flow through the existing `artifact_scan_tree` response. No new Tauri commands needed.

## How

1. In `artifact_reader.rs`, when parsing frontmatter for DocNode leaf nodes, include all scalar values in a `frontmatter: Option<HashMap<String, serde_json::Value>>` field
2. When building NavType for a directory, read `schema.json` from that directory and walk its `properties` object:
   - Properties with `enum` arrays → `FilterableField { name, values }`
   - Properties with `type: "string"` and `format: "date"` → `SortableField { name, field_type: "date" }`
   - The `title` property → `SortableField { name: "title", field_type: "string" }`
3. Check for `_navigation.json` in the same directory — parse as `NavigationConfig` if present
4. Add all new types to `domain/mod.rs` with Serialize/Deserialize derives

## Verification

- [ ] `make clippy` passes with zero warnings
- [ ] `make test-rust` passes
- [ ] NavType response includes filterable_fields derived from schema.json
- [ ] DocNode frontmatter bag contains status, priority, created, updated values
- [ ] _navigation.json is loaded when present
