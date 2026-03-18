---
id: SKILL-020
title: Plugin Development
description: |
  OrqaStudio plugin development: creating plugins in standalone projects with seed data,
  using the Artifact Graph SDK, and the four-layer plugin model (built-in, official,
  community, user). Use when: Building new plugins, helping users create custom views,
  or extending OrqaStudio with additional artifact visualisations.
status: active
created: 2026-03-01
updated: 2026-03-10
category: domain
version: 1.0.0
user-invocable: true
relationships:
  - target: DOC-086
    type: synchronised-with
---

Guide the AI in building OrqaStudio plugins. Plugins are standalone projects that extend
OrqaStudio with custom views, dashboards, and visualisations built on the Artifact Graph SDK.

## Core Principle

A plugin IS a project. It has its own directory, its own `.orqa/` structure, and its own
governance. It is developed and tested independently, then installed into a production
project when ready.

## Always Create a New Standalone Project

Plugins MUST be developed in a **separate project directory**, never inside the user's
production project. This protects production governance data during development.

```bash
# Create plugin project
mkdir ~/code/my-plugin
cd ~/code/my-plugin
git init

# Scaffold plugin structure
mkdir -p src test-data/.orqa
```

### Plugin Project Structure

```
my-plugin/
  plugin.json              # Manifest: name, version, entry point, SDK version required
  src/                     # Plugin source (Svelte components, stores, types)
    index.svelte           # Entry point component
    ...
  test-data/               # Seed data for development and testing
    .orqa/                 # Mock governance directory
      project.json
      planning/
        epics/
        tasks/
        milestones/
      governance/
        rules/
        lessons/
        decisions/
      documentation/
  README.md
```

## Seed Data Generation

Every plugin project MUST include seed data that exercises the plugin's features.
Generate representative `.orqa/` artifacts so the plugin can be developed and tested
without a real production project.

### Seed Data Guidelines

1. **Representative variety** -- include multiple artifact types the plugin consumes
2. **Realistic relationships** -- artifacts should reference each other (epic -> tasks, task -> depends-on)
3. **Edge cases** -- include broken refs, orphaned artifacts, empty frontmatter fields
4. **Sufficient volume** -- at least 10-20 artifacts to test bulk queries and aggregations
5. **Valid frontmatter** -- seed data MUST pass schema validation against `schema.json` files

### Example Seed Artifact

```markdown
---
id: EPIC-001
title: "Example epic for plugin testing"
status: in-progress
milestone: MS-001
priority: P1
pillars:
  - PILLAR-001
scoring:
  user-value: 8
  structural-integrity: 5
  learning-potential: 3
score: 6.0
docs-required: []
docs-produced: []
research-refs: []
---

## Implementation Design

This is a seed epic for testing the plugin's epic visualisation.
```

## The Artifact Graph SDK

Plugins consume the same SDK that built-in views use. The SDK provides:

- **Resolution**: `resolve(id)` and `resolveByPath(path)` for artifact metadata
- **Relationships**: `referencesFrom(id)` and `referencesTo(id)` for traversal
- **Bulk queries**: `byType(type)` and `byStatus(status)` for filtering
- **Content**: `readContent(path)` for full markdown body
- **Health**: `brokenRefs()` and `orphans()` for integrity checks
- **Subscriptions**: `subscribe(id, cb)` and `subscribeType(type, cb)` for reactive updates

### SDK Import

```typescript
import { artifactGraphSDK } from '$lib/sdk/artifact-graph.svelte';
```

The SDK is a singleton. It eagerly loads the full graph on initialisation and provides
synchronous lookups. Content reads are async (always from disk).

### Reactive Updates

The SDK listens for `"artifact-graph-updated"` Tauri events from the file watcher.
When `.orqa/` files change, the graph rebuilds automatically and all subscribers are
notified. Plugins that use `subscribe()` or `subscribeType()` receive callbacks on
graph changes without polling.

## Four-Layer Plugin Model

OrqaStudio ships with default views only. Everything else is a plugin:

| Layer | What | Distribution | Versioning |
|-------|------|-------------|------------|
| **Built-in** | Artifact browser, viewers, navigation, conversation | Shipped with app binary | App version |
| **Official** | Curated plugins by the OrqaStudio team | Official plugins repo (git submodules) | Semver per plugin |
| **Community** | Third-party plugins shared publicly | Own GitHub repos, installed via URL | Author-managed |
| **User** | Local plugins created by the user | Local file path | User-managed |

### User Plugins Are the Priority

User plugins represent the core promise: anyone can use the app to extend the app.
The workflow is:

1. User asks the AI to build a plugin
2. AI creates a new standalone project with seed data (this skill)
3. User develops collaboratively with the AI, testing against seed data
4. When ready, user installs into their production project via file path
5. Plugin uses the Artifact Graph SDK -- same API as built-in views

### Installing a Local Plugin

Local plugins are installed by file path. The app reads the plugin's `plugin.json`
manifest and loads the entry point component.

```json
// In the production project's .orqa/project.json
{
  "plugins": [
    { "path": "~/code/my-plugin", "enabled": true }
  ]
}
```

The plugin is loaded at app startup and has access to the full Artifact Graph SDK.

## Plugin Manifest (`plugin.json`)

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "description": "A custom dashboard for milestone progress",
  "entry": "src/index.svelte",
  "sdk_version": ">=1.0.0",
  "author": "Your Name",
  "license": "MIT"
}
```

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Unique plugin identifier (kebab-case) |
| `version` | Yes | Semver version string |
| `description` | Yes | Short description for the plugin catalogue |
| `entry` | Yes | Path to the entry Svelte component |
| `sdk_version` | Yes | Minimum Artifact Graph SDK version required |
| `author` | No | Plugin author |
| `license` | No | License identifier |

## Development Workflow

### Step 1: Scaffold

Create a new directory, initialise git, set up the plugin structure with `plugin.json`
and seed data.

### Step 2: Develop

Write Svelte components that import and use the Artifact Graph SDK. Test against
seed data. Use `$derived` for reactive computations over graph data.

### Step 3: Test

Run the plugin against the seed data directory. Verify all artifact types, edge cases,
and error states render correctly.

### Step 4: Install

Add the plugin path to the production project's `project.json` plugins array.
Restart the app. The plugin appears in the navigation.

### Step 5: Iterate

Changes to the plugin source are picked up by Vite HMR during development.
Changes to seed data trigger graph rebuild via the file watcher.

## What This Skill Does NOT Cover

- **Plugin distribution architecture** -- distribution and catalogue design is future work
- **Plugin marketplace UI** -- future feature, not yet designed
- **Plugin API beyond the Artifact Graph SDK** -- plugins currently only access the graph SDK
- **Plugin security/sandboxing** -- research needed

## Anti-Patterns

| Anti-Pattern | Correct Approach |
|-------------|-----------------|
| Developing a plugin inside the production `.orqa/` | Always create a standalone project |
| Testing against production data | Generate seed data with representative artifacts |
| Hardcoding artifact paths | Use SDK resolution (`resolve`, `resolveByPath`) |
| Polling for changes | Use SDK subscriptions (`subscribe`, `subscribeType`) |
| Importing internal app modules | Only import the public SDK singleton |
| Skipping edge cases in seed data | Include broken refs, orphans, empty fields |
