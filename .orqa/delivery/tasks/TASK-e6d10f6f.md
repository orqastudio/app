---
id: TASK-e6d10f6f
title: "Backend: Extend DocNode with frontmatter and NavType with schema metadata"
description: "Extend the Rust DocNode to carry all scalar frontmatter fields, extract filterable/sortable fields from schema.json, and read _navigation.json per type directory."
status: completed
created: 2026-03-11
updated: 2026-03-11
acceptance:
  - "DocNode includes frontmatter as Option<HashMap<String, serde_json::Value>>"
  - "NavType includes filterable_fields, sortable_fields, navigation_config"
  - schema.json enum properties are correctly extracted as FilterableField with ordered values
  - "_navigation.json is parsed when present, None when absent"
  - make lint-backend and make test-rust pass
relationships:
  - target: EPIC-6787bb93
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-d05c56ea
    type: depended-on-by
  - target: TASK-3b119193
    type: depended-on-by
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

- [ ] `make lint-backend` passes with zero warnings
- [ ] `make test-rust` passes
- [ ] NavType response includes filterable_fields derived from schema.json
- [ ] DocNode frontmatter bag contains status, priority, created, updated values
- [ ] _navigation.json is loaded when present
