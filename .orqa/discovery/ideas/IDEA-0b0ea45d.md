---
id: IDEA-0b0ea45d
type: discovery-idea
title: "Hash-based routing for the explorer panel"
description: "Replace the if/else view chain in AppLayout with hash-based URL routing. Enables back/forward navigation, hot reload preserves the current view, deep linking to artifacts, and data-driven view resolution instead of hardcoded conditionals."
status: archived
created: 2026-03-19
updated: 2026-03-19
research-needed:
  - SvelteKit hash-mode routing vs custom router — which fits a Tauri desktop app better?
  - How does the NavigationStore interact with the URL? Does it drive the hash, or react to it?
  - How do plugin views register their routes? From project.json navigation config?
  - Can we lazy-load view components based on route (code splitting)?
  - How does artifact deep linking work? #/artifacts/EPIC-797972a7 → load artifact, show in viewer
  - How do we handle query params for view state (e.g. #/plugin/software/roadmap?filter=active)?
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: URL-based routing makes the current view state visible and addressable — you can share a link to a specific artifact or view
  - target: PERSONA-477971bf
    type: benefits
  - target: EPIC-904ae175
    type: realises
---

# Hash-Based Routing for the Explorer Panel

## The Problem

AppLayout currently uses an if/else chain to decide what to render in the explorer panel:

```svelte
{#if activePluginView}
  <PluginViewContainer ... />
{:else if activity === "project"}
  <ProjectDashboard />
{:else if activity === "artifact-graph"}
  <FullGraphView />
{:else if activity === "settings"}
  <SettingsView />
{:else}
  <ArtifactViewer />
{/if}
```

Problems:

- Hot reload resets to dashboard (view state is in memory, not URL)
- No back/forward navigation
- No deep linking to specific artifacts
- Adding new views requires editing AppLayout
- Plugin views need special handling

## The Design

### Layout stays the same

```text
[Activity Bar] [Nav Panel] [Explorer ← routes here] [Chat Panel ← always visible]
```

The chat panel sits outside the routed area. It's always present regardless of what the explorer shows. Only the explorer content changes based on the route.

### Route structure

```text
#/project                              → ProjectDashboard
#/artifacts/:id                        → ArtifactViewer for that artifact
#/artifacts                            → ArtifactViewer (list/browse mode)
#/plugin/:pluginName/:viewKey          → PluginViewContainer
#/settings                             → SettingsView
#/graph                                → FullGraphView
#/setup                                → SetupWizard
```

### How it works

1. The hash fragment encodes the explorer view state
2. NavigationStore reads the hash and derives the current view
3. Clicking nav items updates the hash (not internal state)
4. Back/forward buttons work (browser history)
5. Hot reload preserves the hash (URL survives reload)
6. AppLayout renders a single `<ExplorerRouter />` component instead of the if/else chain
7. ExplorerRouter maps the hash to the correct component

### Plugin view routes

Plugin views get routes from their manifest's `views` array + `project.json` navigation:

```json
// project.json navigation item
{ "key": "roadmap", "type": "plugin", "pluginSource": "@orqastudio/plugin-software-kanban" }
```

This maps to: `#/plugin/@orqastudio/plugin-software-kanban/roadmap`

### What this enables

- **Back/forward** — navigate through view history with browser buttons
- **Hot reload** — view state preserved across reloads (critical for development)
- **Deep linking** — share `#/artifacts/EPIC-797972a7` with a colleague
- **Data-driven views** — no hardcoded if/else, just route → component mapping
- **Plugin views** — naturally routed without special handling
- **Lazy loading** — views can be code-split and loaded on demand per route
