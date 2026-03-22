---
id: DOC-99a1b71a
type: doc
title: Plugin Manifest Schema Reference
description: Complete reference for orqa-plugin.json — the manifest file every plugin must provide to register types, relationships, views, and knowledge artifacts.
category: reference
created: 2026-03-18
updated: 2026-03-18
relationships:
  - target: KNOW-b453410f
    type: synchronised-with
  - target: KNOW-63cc1a00
    type: synchronised-with
  - target: KNOW-e1333874
    type: synchronised-with
---

# Plugin Manifest Schema Reference

Every plugin must provide an `orqa-plugin.json` file at its root. This file declares what the plugin provides to the platform.

## Top-Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Package name (e.g., `@orqastudio/plugin-software-project`) |
| `version` | string | Yes | Semver version |
| `displayName` | string | Yes | Human-readable name |
| `description` | string | Yes | What this plugin does |
| `provides` | object | Yes | What the plugin registers |

## `provides` Object

### `provides.schemas`

Array of artifact type definitions. Each entry:

```json
{
  "key": "epic",
  "label": "Epic",
  "plural": "Epics",
  "icon": "layers",
  "defaultPath": ".orqa/delivery/epics",
  "idPrefix": "EPIC",
  "frontmatter": {
    "required": ["id", "type", "status"],
    "optional": ["name", "description", "priority", "relationships"]
  },
  "statusTransitions": {
    "captured": ["exploring", "ready", "prioritised"],
    "active": ["hold", "blocked", "review"],
    "review": ["completed", "active"],
    "completed": ["surpassed"]
  }
}
```

### `provides.relationships`

Array of relationship type definitions. Each entry:

```json
{
  "key": "delivers",
  "inverse": "delivered-by",
  "label": "Delivers",
  "inverseLabel": "Delivered By",
  "from": ["task"],
  "to": ["epic"],
  "description": "Task delivers work to an epic",
  "semantic": "hierarchy",
  "constraints": {
    "required": true,
    "minCount": 1,
    "statusRules": [
      {
        "evaluate": "target",
        "condition": "all-targets-in",
        "statuses": ["completed"],
        "proposedStatus": "review",
        "description": "Epic moves to review when all delivering tasks are completed"
      }
    ]
  }
}
```

### `provides.knowledge`

Array of knowledge artifact registrations:

```json
{
  "key": "software-delivery",
  "id": "KNOW-1d47d8d8",
  "label": "Software Delivery Lifecycle"
}
```

### `provides.views`

Array of custom views:

```json
{ "key": "roadmap", "label": "Roadmap", "icon": "kanban" }
```

### `provides.widgets`

Array of dashboard widgets:

```json
{
  "key": "pipeline",
  "label": "Delivery Pipeline",
  "icon": "git-branch",
  "defaultPosition": { "row": 0, "col": 0 },
  "defaultSpan": { "rows": 1, "cols": 2 }
}
```

## Optional Sections

### `defaultNavigation`

Declares how plugin artifacts appear in the sidebar navigation. Uses `group` and `plugin` node types.

### `delivery`

Declares the delivery hierarchy for artifact types that participate in delivery tracking (parent-child relationships, gate fields).

### `semantics`

Groups relationship keys by semantic category for the graph visualiser and enforcement engine.

### `artifactLinks`

Display configuration for artifact links (display modes and colors by ID prefix).

## Conventions

- Plugin-namespaced IDs use prefixes like `KNOW-1d47d8d8`, `DOC-2c9bfdda`
- Relationship keys should be lowercase kebab-case
- Semantic categories should match or extend: `hierarchy`, `dependency`, `lineage`, `corrective`, `knowledge-flow`, `foundation`, `governance`, `observation`, `synchronisation`
