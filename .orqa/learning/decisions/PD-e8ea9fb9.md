---
id: "PD-e8ea9fb9"
type: principle-decision
title: "Config-Driven Navigation Defaults"
description: "Each artifact type directory can contain a _navigation.json file that configures default sort, group, filter, group ordering, and custom layout for the browser panel."
status: completed
created: "2026-03-11"
updated: "2026-03-11"
relationships: []
---

## Decision

Each artifact type directory (e.g. `.orqa/implementation/epics/`, `.orqa/documentation/`) can contain a `_navigation.json` file with two mutually exclusive modes:

### Standard defaults mode

Pre-configures sort, group-by, group ordering, and active filters:

```jsonc
{
  "defaults": {
    "sort": { "field": "updated", "direction": "desc" },
    "group": "status",
    "group_order": {
      "status": ["draft", "ready", "in-progress", "review", "done"]
    },
    "filters": {
      "status": ["draft", "ready", "in-progress", "review"]
    }
  },
  "layout": null
}
```

### Custom layout mode

Arranges artifacts into curated, ordered sections — ideal for documentation that should read like a book's table of contents rather than a date-sorted list:

```jsonc
{
  "defaults": null,
  "layout": {
    "sections": [
      { "label": "Product", "items": ["product/*"] },
      { "label": "Architecture", "items": ["architecture/*"] }
    ],
    "uncategorized": "append"
  }
}
```

Users can override defaults interactively in the browser. Overrides persist in the navigation store (in memory), not in the file.

### Group ordering

When artifacts are grouped by a field, group headers are ordered using a three-tier priority:

1. `_navigation.json` `group_order` — explicit override per field
2. Schema enum array order — the intentional lifecycle ordering in `schema.json` (e.g. `draft → ready → in-progress → review → done`)
3. Alphabetical — fallback when no configured or schema order exists

## Rationale

Different artifact types have fundamentally different browsing needs:

- **Epics** benefit from grouping by status with lifecycle ordering (draft → in-progress → done), filtered to exclude completed work by default
- **Documentation** benefits from a curated book-like structure where pages are arranged by topic, not by date
- **Tasks** benefit from sorting by updated date to surface recent activity
- **Rules** benefit from alphabetical title sorting for quick lookup

A one-size-fits-all sort/filter doesn't serve any of these well. Config-driven defaults let each type directory declare its ideal browsing experience without requiring code changes.

The file lives in the artifact type directory (not in app config) because:

- It's version-controlled alongside the artifacts it configures
- Different projects can have different defaults for the same artifact type
- It follows the `.orqa/` as source of truth principle

## Consequences

- The Rust artifact scanner reads `_navigation.json` alongside `README.md` when scanning directories
- `NavigationConfig` is included in the `NavType` response — no extra round-trip needed
- Frontend applies defaults on first render; user overrides take precedence
- When `layout` is set, the toolbar shows a layout indicator instead of sort/filter state
- Adding new group ordering is a JSON config edit, not a code change

## Related Decisions

- [PD-a47f313a](PD-a47f313a) — Schema-driven filtering (the schema provides the options; this decision provides the defaults)
- [PD-80f39962](PD-80f39962) — Core UI boundary (the browser is the primary navigation tool)
