---
id: DOC-78ed20f0
title: "Project Configuration (`.orqa/project.json`)"
category: reference
description: Schema and usage of the .orqa/project.json configuration file that defines project-level settings.
created: 2026-03-03
updated: 2026-03-18
sort: 3
relationships:
  - target: AD-4f5277f0
    type: documents
---

**Date:** 2026-03-03 | **Status:** Active | **Decision:** [AD-4f5277f0](AD-4f5277f0) — File-based project settings

---

## Overview

OrqaStudio uses a file-based configuration model for project settings. Each managed project stores its configuration in `.orqa/project.json` at the project root. This file is the **source of truth** for project-specific settings — navigation structure, status vocabulary, delivery hierarchy, relationships, AI model preferences, and artifact display.

The SQLite `projects` table remains as the app-wide registry of known projects (recent list, IDs, timestamps). It does NOT own project configuration — `.orqa/project.json` does.

---

## `.orqa/` Directory Convention

OrqaStudio creates a `.orqa/` directory in each managed project for OrqaStudio-specific configuration. `.orqa/` is the source of truth for all governance artifacts.

| Path | Purpose |
|------|---------|
| `.orqa/project.json` | Project configuration file (this document) |

The `.orqa/` directory is created automatically when the user saves project settings for the first time.

---

## Configuration Merging Model

Project configuration is not a standalone blob — it is the final layer in a three-layer merge:

1. **Platform defaults (`core.json`)** — Canonical artifact types, relationships, semantic categories, and navigation shipped with OrqaStudio. Every project inherits these. They cannot be overridden by project config.
2. **Plugin provides (`orqa-plugin.json`)** — Installed plugins contribute additional schemas (artifact types with frontmatter and status transitions), relationships, navigation groups, delivery types, views, widgets, skills, and semantic categories. These extend the platform layer.
3. **Project overrides (`project.json`)** — The project file adds project-specific settings: name, description, model preferences, the `artifacts` navigation tree, status definitions (with custom transitions and auto-rules), delivery hierarchy, project-level relationships, artifact link display, and excluded paths.

At runtime, the merged configuration is a union: platform types are always present, plugin-provided types are added on top, and project-level overrides apply last. Relationships from all three layers form a single vocabulary — the integrity checker validates against the full merged set.

---

## Complete Field Reference

```json
{
  "name": "OrqaStudio Dev",
  "description": "Organisation-mode development environment for the OrqaStudio ecosystem",
  "organisation": true,
  "projects": [
    { "name": "app", "path": "app" },
    { "name": "types", "path": "libs/types" }
  ],
  "default_model": "auto",
  "show_thinking": true,
  "custom_system_prompt": "You are running inside OrqaStudio...",
  "icon": "icon.svg",
  "excluded_paths": ["node_modules", ".git", "target", "dist", "build"],
  "artifacts": [ ... ],
  "statuses": [ ... ],
  "delivery": { ... },
  "relationships": [ ... ],
  "artifactLinks": { ... },
  "stack": { ... },
  "governance": { ... }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | Yes | Display name for the project |
| `description` | `string \| null` | No | Brief project description |
| `organisation` | `boolean` | No | When `true`, this project aggregates child projects into a single graph. See [Organisation Mode](#organisation-mode). |
| `projects` | `ChildProjectConfig[]` | No | Child project paths. Only meaningful when `organisation` is `true`. |
| `default_model` | `string` | Yes | Default AI model identifier. `"auto"` defers to the provider default; provider-specific model IDs (e.g. `"claude-sonnet-4-6"`) select a specific model. |
| `show_thinking` | `boolean` | No | Whether to display AI thinking/reasoning blocks in the chat UI. Defaults to `false`. |
| `custom_system_prompt` | `string \| null` | No | Additional system prompt text prepended to every AI request for this project. |
| `icon` | `string \| null` | No | Path to the project icon file (relative to project root), displayed in the project switcher and title bar. |
| `excluded_paths` | `string[]` | Yes | Directory names to skip during file scanning. |
| `artifacts` | `ArtifactEntry[]` | No | Navigation tree defining how artifact types appear in the sidebar. See [Artifacts (Navigation Tree)](#artifacts-navigation-tree). |
| `statuses` | `StatusDefinition[]` | No | The 12-status vocabulary with allowed transitions and auto-rules. See [Statuses](#statuses). |
| `delivery` | `DeliveryConfig` | No | Delivery hierarchy configuration. See [Delivery](#delivery). |
| `relationships` | `ProjectRelationshipConfig[]` | No | Project-level relationship types that extend the canonical vocabulary. See [Relationships](#relationships). |
| `artifactLinks` | `ArtifactLinksConfig` | No | Display modes and colours for artifact link chips in the UI. See [Artifact Links](#artifact-links). |
| `stack` | `DetectedStack \| null` | No | Detected technology stack (populated by the project scanner). |
| `governance` | `GovernanceCounts \| null` | No | Governance artifact counts (populated by the project scanner). |

---

## Organisation Mode

When `organisation` is `true`, the project acts as a mono-repo aggregator. The `projects` array lists child projects that are scanned into a single unified graph.

```json
{
  "organisation": true,
  "projects": [
    { "name": "app", "path": "app" },
    { "name": "types", "path": "libs/types" },
    { "name": "sdk", "path": "libs/sdk" },
    { "name": "cli", "path": "libs/cli" },
    { "name": "connector-claude-code", "path": "connectors/claude-code" },
    { "name": "components", "path": "libs/svelte-components" },
    { "name": "visualiser", "path": "libs/graph-visualiser" }
  ]
}
```

### `ChildProjectConfig`

| Field | Type | Description |
|-------|------|-------------|
| `name` | `string` | Human-readable name for the child project |
| `path` | `string` | Path to the child project root (relative to parent project root, or absolute) |

Each child project can have its own `.orqa/project.json`. The organisation-mode scanner aggregates all artifacts from all child projects into a single graph, meaning relationships can cross project boundaries.

---

## AI Model Settings

Three fields control AI behaviour:

```json
{
  "default_model": "auto",
  "show_thinking": true,
  "custom_system_prompt": "You are running inside OrqaStudio — the same app you are helping develop..."
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `default_model` | `string` | `"auto"` | `"auto"` delegates model selection to the provider. Named model IDs lock to a specific model. |
| `show_thinking` | `boolean` | `false` | Displays extended thinking/reasoning blocks in chat responses. |
| `custom_system_prompt` | `string \| null` | `null` | Injected into every AI request for this project. Use this for project-specific conventions, constraints, or architecture context. |

---

## Artifacts (Navigation Tree)

The `artifacts` array defines the sidebar navigation structure. It is a two-level tree: top-level **groups** contain **leaf entries** that map to filesystem directories.

```json
{
  "artifacts": [
    {
      "key": "principles",
      "label": "Principles",
      "icon": "landmark",
      "children": [
        { "key": "pillar", "label": "Pillars", "icon": "columns-3", "path": ".orqa/principles/pillars" },
        { "key": "vision", "label": "Vision", "icon": "eye", "path": ".orqa/principles/vision" },
        { "key": "persona", "label": "Personas", "icon": "users", "path": ".orqa/principles/personas" },
        { "key": "grounding", "label": "Grounding", "icon": "anchor", "path": ".orqa/principles/grounding" }
      ]
    },
    {
      "key": "discovery",
      "label": "Discovery",
      "icon": "compass",
      "children": [
        { "key": "idea", "label": "Ideas", "icon": "lightbulb", "path": ".orqa/discovery/ideas" },
        { "key": "research", "label": "Research", "icon": "book-open", "path": ".orqa/discovery/research" },
        { "key": "wireframe", "label": "Wireframes", "icon": "layout", "path": ".orqa/discovery/wireframes" }
      ]
    },
    {
      "key": "delivery",
      "label": "Delivery",
      "icon": "package",
      "children": [
        { "key": "milestone", "label": "Milestones", "icon": "flag", "path": ".orqa/delivery/milestones" },
        { "key": "epic", "label": "Epics", "icon": "layers", "path": ".orqa/delivery/epics" },
        { "key": "task", "label": "Tasks", "icon": "check-square", "path": ".orqa/delivery/tasks" }
      ]
    }
  ]
}
```

### `ArtifactGroupConfig` (group entry)

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | `string` | Yes | Unique key for the group (e.g. `"principles"`, `"delivery"`) |
| `label` | `string` | No | Display label in the sidebar. Falls back to humanized key name. |
| `icon` | `string` | No | Lucide icon name for the group. |
| `children` | `ArtifactTypeConfig[]` | Yes | Ordered list of leaf artifact types within this group. |

### `ArtifactTypeConfig` (leaf entry)

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | `string` | Yes | Unique key matching a canonical artifact type (e.g. `"idea"`, `"epic"`) |
| `label` | `string` | No | Display label. Falls back to directory README frontmatter, then humanized key. |
| `icon` | `string` | No | Lucide icon name for this type. |
| `path` | `string` | Yes | Relative path to the directory containing artifacts of this type. |

Navigation sections are views into the graph, not filesystem groupings. The same artifact type can appear in multiple groups if needed. The `key` field is what links a navigation entry to the artifact type defined in `core.json` or a plugin.

---

## Statuses

The `statuses` array defines the project's status vocabulary — the 12 canonical statuses, their allowed transitions, visual presentation, and automatic transition rules.

```json
{
  "statuses": [
    {
      "key": "captured",
      "label": "Captured",
      "icon": "circle-dot",
      "transitions": ["exploring", "ready", "archived"]
    },
    {
      "key": "active",
      "label": "Active",
      "icon": "loader",
      "spin": true,
      "transitions": ["review", "hold", "blocked"],
      "auto_rules": []
    },
    {
      "key": "blocked",
      "label": "Blocked",
      "icon": "x-circle",
      "transitions": ["active", "archived"],
      "auto_rules": [
        { "condition": "all-dependencies-completed", "target": "active" }
      ]
    }
  ]
}
```

### `StatusDefinition`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | `string` | Yes | Machine key (e.g. `"captured"`, `"active"`, `"blocked"`) |
| `label` | `string` | Yes | Human-readable display label |
| `icon` | `string` | Yes | Lucide icon name |
| `spin` | `boolean` | No | Whether the icon animates (spins). Defaults to `false`. Used for `active` status to indicate in-progress work. |
| `transitions` | `string[]` | No | List of status keys this status can transition to. An empty array means terminal (no outbound transitions). |
| `auto_rules` | `StatusAutoRule[]` | No | Automatic transition rules evaluated by the graph engine. |

### `StatusAutoRule`

| Field | Type | Description |
|-------|------|-------------|
| `condition` | `string` | Graph condition to evaluate (e.g. `"all-dependencies-completed"`, `"dependency-blocked"`, `"all-children-completed"`, `"recurrence-threshold"`, `"dependencies-met"`) |
| `target` | `string` | Status key to propose transitioning to when the condition is met |

### The 12 Canonical Statuses

| Status | Transitions To | Auto Rules |
|--------|---------------|------------|
| `captured` | `exploring`, `ready`, `archived` | — |
| `exploring` | `ready`, `captured`, `archived` | — |
| `ready` | `prioritised`, `exploring`, `archived` | — |
| `prioritised` | `active`, `ready`, `archived` | — |
| `active` | `review`, `hold`, `blocked` | `all-children-completed` -> `review` |
| `hold` | `active`, `archived` | — |
| `blocked` | `active`, `archived` | `all-dependencies-completed` -> `active` |
| `review` | `completed`, `active` | — |
| `completed` | `archived` | — |
| `surpassed` | `archived` | — |
| `archived` | *(terminal)* | — |
| `recurring` | `archived` | — |

Auto-rules are graph queries. For example, a `blocked` artifact automatically proposes transitioning to `active` when all its dependency targets reach `completed` status. The UI surfaces these as proposals — the user authorises the transition.

---

## Delivery

The `delivery` section defines the delivery hierarchy — a chain of artifact types connected by parent relationships. This drives the roadmap view, pipeline widget, and automatic rollup behaviour.

```json
{
  "delivery": {
    "types": [
      {
        "key": "milestone",
        "label": "Milestone",
        "path": ".orqa/delivery/milestones",
        "gate_field": "gate"
      },
      {
        "key": "epic",
        "label": "Epic",
        "path": ".orqa/delivery/epics",
        "parent": { "type": "milestone", "relationship": "delivers" }
      },
      {
        "key": "task",
        "label": "Task",
        "path": ".orqa/delivery/tasks",
        "parent": { "type": "epic", "relationship": "delivers" }
      }
    ]
  }
}
```

### `DeliveryTypeConfig`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `key` | `string` | Yes | Artifact type key (must match a key from `artifacts` or a plugin schema) |
| `label` | `string` | Yes | Human-readable label |
| `path` | `string` | Yes | Directory path for this delivery type's artifacts |
| `gate_field` | `string` | No | Frontmatter field used as a gate condition for this type. Only milestones use this (`"gate"`). |
| `parent` | `DeliveryParentConfig \| null` | No | Defines the parent type and relationship for hierarchy rollup. `null` or absent for the top-level type (milestone). |

### `DeliveryParentConfig`

| Field | Type | Description |
|-------|------|-------------|
| `type` | `string` | The parent artifact type key (e.g. `"milestone"`, `"epic"`) |
| `relationship` | `string` | The relationship type connecting child to parent (e.g. `"delivers"`) |

The hierarchy forms a chain: **task** --`delivers`--> **epic** --`delivers`--> **milestone**. The graph engine uses this to compute rollup status — when all tasks delivering to an epic complete, the epic is proposed for review.

---

## Relationships

The `relationships` array adds project-level relationship types that extend the canonical vocabulary defined in `core.json` and any plugin-provided relationships.

```json
{
  "relationships": [
    {
      "key": "depends-on",
      "inverse": "depended-on-by",
      "label": "Depends On",
      "inverse_label": "Depended On By"
    }
  ]
}
```

### `ProjectRelationshipConfig`

| Field | Type | Description |
|-------|------|-------------|
| `key` | `string` | Forward relationship key (e.g. `"depends-on"`) |
| `inverse` | `string` | Inverse relationship key (e.g. `"depended-on-by"`) |
| `label` | `string` | Human-readable label for the forward direction |
| `inverse_label` | `string` | Human-readable label for the inverse direction |

Project-level relationships are simpler than platform relationships — they do not include `from`/`to` type constraints or semantic categories. They exist for project-specific connection types that do not belong in the canonical vocabulary.

All relationships are bidirectional. When an artifact declares a `depends-on` relationship, the target artifact must have the corresponding `depended-on-by` inverse. The integrity checker validates this.

---

## Artifact Links

The `artifactLinks` section controls how artifact reference chips render in the UI — whether they show the artifact's ID or its resolved title, and what colour the chip uses per artifact type.

```json
{
  "artifactLinks": {
    "displayModes": {
      "EPIC": "title",
      "TASK": "title",
      "RULE": "title",
      "AD": "title",
      "IDEA": "title",
      "IMPL": "title",
      "SKILL": "title",
      "PILLAR": "title",
      "RES": "title",
      "MS": "title",
      "DOC": "title",
      "AGENT": "title"
    },
    "colors": {
      "EPIC": "#3b82f6",
      "TASK": "#06b6d4",
      "RULE": "#a78bfa",
      "AD": "#8b5cf6",
      "IDEA": "#c084fc",
      "IMPL": "#67e8f9",
      "SKILL": "#2dd4bf",
      "PILLAR": "#818cf8",
      "RES": "#6366f1",
      "MS": "#38bdf8",
      "DOC": "#94a3b8",
      "AGENT": "#f472b6"
    }
  }
}
```

### `ArtifactLinksConfig`

| Field | Type | Description |
|-------|------|-------------|
| `displayModes` | `Record<string, "id" \| "title">` | Per-type display mode. Keys are ID prefixes (e.g. `"EPIC"`, `"TASK"`). `"id"` shows the artifact ID; `"title"` resolves and shows the artifact title. Absent prefixes default to `"id"`. |
| `colors` | `Record<string, string>` | Per-type hex colour for the chip background. Keys are ID prefixes. |

The keys in both maps are **ID prefixes** (uppercase), not artifact type keys. For example, decisions use `"AD"` (not `"decision"`), lessons use `"IMPL"` (not `"lesson"`), and milestones use `"MS"` (not `"milestone"`). These prefixes come from the `idPrefix` field in `core.json` artifact type definitions.

---

## Scanner-Populated Fields

Two fields are populated automatically by the project scanner — they are not manually authored.

### `DetectedStack`

| Field | Type | Description |
|-------|------|-------------|
| `languages` | `string[]` | Detected programming languages (lowercase) |
| `frameworks` | `string[]` | Detected frameworks and tools |
| `package_manager` | `string \| null` | Primary package manager (`"npm"`, `"cargo"`, `"yarn"`, `"pnpm"`, `"bun"`) |
| `has_claude_config` | `boolean` | Whether `.orqa/project.json` or `.claude/` config exists |
| `has_design_tokens` | `boolean` | Whether design token files are present |

### `GovernanceCounts`

| Field | Type | Description |
|-------|------|-------------|
| `docs` | `number` | Count of documentation artifacts |
| `agents` | `number` | Count of agent artifacts |
| `rules` | `number` | Count of rule artifacts |
| `skills` | `number` | Count of skill artifacts |
| `hooks` | `number` | Count of hook files |
| `has_claude_config` | `boolean` | Whether `.orqa/project.json` or `.claude/` config exists |

---

## Discovery Rules

1. When a project is opened (`project_open`), OrqaStudio checks for `.orqa/project.json` at the project root
2. **File exists** — load it as the source of truth for project settings
3. **File missing** — not an error; the UI shows a setup wizard that scans the project and creates the file
4. The `project_open` command syncs the file-based name to SQLite so the recent projects list stays current

---

## Relationship to SQLite

| Concern | Owner | Why |
|---------|-------|-----|
| Project registry (ID, path, timestamps) | SQLite `projects` table | App needs a cross-project list for recent projects, session associations |
| Project configuration (name, model, artifacts, statuses, delivery, relationships) | `.orqa/project.json` | User-visible, version-controllable, portable |

When `project_settings_write` is called, the `name` field is synced back to the SQLite `projects` table to keep the recent projects list display current.

---

## Schema Versioning

Reserved for future use. When a `version` field is needed for migrations, it will be added to the JSON root. For now, all fields are additive — missing fields use sensible defaults during deserialization (`#[serde(default)]`).

---

## Error Handling

| Scenario | Error | Behavior |
|----------|-------|----------|
| Malformed JSON in `.orqa/project.json` | `OrqaError::Serialization` | UI shows error, offers to re-scan and overwrite |
| Permission denied reading/writing | `OrqaError::FileSystem` (from `io::Error`) | UI shows error message |
| `.orqa/project.json` does not exist | Not an error | `project_settings_read` returns `None`, UI shows setup wizard |
| `.orqa/` directory does not exist | Not an error | Created automatically on first write |

---

## IPC Commands

Three commands manage project settings:

- `project_settings_read(path)` — reads `.orqa/project.json`, returns `Option<ProjectSettings>`
- `project_settings_write(path, settings)` — writes `.orqa/project.json`, creates `.orqa/` dir if needed
- `project_scan(path, excluded_paths?)` — scans filesystem for stack detection and governance counts

See [IPC Command Catalog](./ipc-commands.md) for full parameter tables.

---

## Related Documents

- [IPC Command Catalog](./ipc-commands.md) — command specifications
- [Settings & Onboarding Wireframe](../wireframes/settings-onboarding.md) — UI design
- [SQLite Schema](./sqlite-schema.md) — projects table definition
