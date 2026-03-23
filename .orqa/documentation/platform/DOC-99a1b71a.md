---
id: DOC-99a1b71a
title: Plugin Manifest Schema Reference
description: Complete reference for orqa-plugin.json — the manifest file every plugin must provide to register types, relationships, views, and knowledge artifacts.
category: reference
created: 2026-03-18
updated: 2026-03-23
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
| `displayName` | string | No | Human-readable name shown in the plugin manager |
| `description` | string | No | What this plugin does |
| `category` | string | No | `thinking`, `delivery`, `governance`, `connector`, `tooling`, or `coding-standards` |
| `provides` | object | Yes | What the plugin registers at runtime |
| `content` | object | No | Source → target directory mappings for `.orqa/` content files |
| `dependencies` | object | No | npm packages and system binaries required by this plugin |
| `build` | string | No | Shell command run after deps install (e.g., `npm run build`) |
| `lifecycle` | object | No | Commands for `install` and `uninstall` lifecycle hooks |
| `defaultNavigation` | array | No | Recommended sidebar navigation additions |
| `requires` | array | No | Other plugin names that must be loaded before this one |

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

### `provides.enforcement_mechanisms`

Array of enforcement mechanism registrations. Each mechanism defines a key, description, and strength level (1-10):

```json
{
  "key": "pre-commit",
  "description": "Git pre-commit hook enforcement",
  "strength": 8
}
```

Rules reference mechanism keys in their `enforcement` entries. The validator checks that every referenced mechanism is registered by an installed plugin.

### `provides.knowledge`

Array of knowledge artifact references bundled with this plugin:

```json
{ "key": "my-domain", "id": "KNOW-b453410f", "label": "My Domain Knowledge" }
```

## Content & Lifecycle Fields

### `content`

Maps plugin-local source directories to `.orqa/` target paths. Files are copied at install time and tracked in `.orqa/manifest.json`. Only `.md` files are copied.

```json
{
  "content": {
    "rules": { "source": "rules", "target": ".orqa/process/rules" },
    "knowledge": { "source": "knowledge", "target": ".orqa/process/knowledge" }
  }
}
```

All content fields are optional. If absent, the plugin installs no content files.

### `dependencies`

Declares runtime requirements for the plugin:

```json
{
  "dependencies": {
    "npm": ["@orqastudio/types"],
    "system": [{ "binary": "node", "minVersion": "20.0.0" }]
  }
}
```

- `npm` — packages installed via `npm install` in the plugin directory (skipped if `node_modules` already exists)
- `system` — binaries checked on the system PATH; installation fails if any are missing

### `build`

Shell command run after deps install. The working directory is the plugin root.

```json
{ "build": "npm run build" }
```

### `lifecycle`

Custom commands run during install and uninstall. The working directory is the plugin root.

```json
{
  "lifecycle": {
    "install": "node scripts/post-install.mjs",
    "uninstall": "node scripts/pre-uninstall.mjs"
  }
}
```

- `install` — runs after content is copied to `.orqa/`
- `uninstall` — runs before content is removed from `.orqa/`

Both fields are optional. If absent, no lifecycle command is run.

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
- At least one entry in `provides` or a `content` mapping is required — a plugin must contribute something
