---
id: "EPIC-6787bb93"
title: "Artifact Browser — Sort, Filter, Search"
description: "Complete the core artifact browsing experience with sorting/grouping/filtering in the browser panel, AI-driven cross-artifact search, and a references panel."
status: "active"
priority: "P1"
created: "2026-03-07"
updated: "2026-03-11"
horizon: "active"
scoring:
  impact: 5
  urgency: 4
  complexity: 3
  dependencies: 4
relationships:
  - target: "RES-6ca176bb"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-6ca176bb"
  - target: "RES-14aacfa0"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-14aacfa0"
  - target: "MS-654badde"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
  - target: "TASK-db9be55f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-e6d10f6f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-d05c56ea"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-cc68b8df"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-5e1e41de"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2e1df475"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-21b461ea"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-3b119193"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "DOC-01ddd8aa"
    type: "documented-by"
  - target: "IDEA-325a4ddc"
    type: "realised-by"
---
## Why P1

The core app's job is to let users **navigate and search** artifacts ([AD-0291fa65](AD-0291fa65)). Navigation and cross-linking are built. What's missing is the ability to sort/group/filter the artifact list, search across all artifacts semantically, and surface cross-references. Editing is tracked separately in [EPIC-fe75b52c](EPIC-fe75b52c).

## What's Already Done

Previous work ([EPIC-a2fa3068](EPIC-a2fa3068), [EPIC-31c9baca](EPIC-31c9baca), and prior phases of this epic) delivered:

- **Config-driven sidebar navigation** — ActivityBar, ArtifactNav, GroupSubPanel
- **Navigation store** with `navigateToArtifact(id)` and `navigateToPath(path)`
- **Generic artifact viewer** with FrontmatterHeader + MarkdownRenderer
- **4 type-specific viewers** — RuleViewer, AgentViewer, SkillViewer, HookViewer
- **Cross-linking** — ArtifactLink (frontmatter) + MarkdownLink (body) + bidirectional graph
- **Artifact graph SDK** — resolve, referencesFrom/To, byType, byStatus, brokenRefs, orphans
- **Backend CRUD** — artifact_create, artifact_update, artifact_delete, artifact_list, artifact_get
- **File watcher** — automatic NavTree and graph cache invalidation
- **Platform portability** — .orqa/ as source of truth, .claude/ symlinks

## Context

The core app's job is to let users navigate, search, and edit artifacts. Navigation and cross-linking are built. This epic completes the browsing and search capabilities: sorting/filtering the artifact list, AI-driven semantic search, and a references panel. Editing is a separate concern tracked in [EPIC-fe75b52c](EPIC-fe75b52c) — artifact mutations are AI-mediated proposals, not raw CRUD. The architecture decision [AD-0291fa65](AD-0291fa65) establishes that all system-level visualizations (roadmaps, dashboards, kanban) are plugin territory.

## Design Principles

> The core app UI provides three capabilities: navigate, search, and edit. All system-level visualizations (roadmaps, dashboards, dependency graphs) are plugins. — [AD-0291fa65](AD-0291fa65)

> Cross-artifact search is AI-driven, not keyword-based. The AI infers search intent and presents results in a structured way, giving infinite flexibility. — [AD-0f6286cd](AD-0f6286cd)

> Sorting and filtering are schema-driven — generated dynamically from each artifact type's `schema.json` enum fields. No hardcoded field lists. — [AD-89391ab6](AD-89391ab6)

> Default views are config-driven via `_navigation.json` per artifact type directory. Supports sort/group/filter defaults, group ordering overrides, and custom curated layouts. — [AD-2f1991c2](AD-2f1991c2)

## Remaining Scope

### 1. Sort, Group, and Filter Toolbar

Replace the current text-only `SearchInput` in ArtifactNav with an icon-based toolbar:

- **Fixed height** — matches `h-10` of NavSubPanel headers and breadcrumb bars for visual consistency
- **Two icon buttons** — Filter (funnel icon) and Sort (arrow-up-down icon), each opening a rich dropdown
- **Generic, not artifact-specific** — the toolbar operates on frontmatter fields common across all artifact types (status, created, updated, title). It does NOT contain type-specific logic.
- **Active indicator** — when filters or non-default sort are active, the icon shows a subtle visual indicator (dot badge or colour change)

#### Filter Dropdown (rich, sectioned)

The filter dropdown is NOT a simple list. It is a structured panel with:

- **Section headers** — grouped by field (Status, Priority, Layer) with muted uppercase labels
- **Checkbox items** — each value shown as a checkbox with status dot/badge matching the existing `StatusIndicator` colour system
- **Count badges** — each filter option shows how many items match (e.g. "Draft (12)")
- **Clear section** — each section has a "clear" action; bottom of dropdown has "Clear all filters"
- **Only relevant sections shown** — if the current artifact type has no `priority` field, that section is hidden

#### Sort Dropdown (compact, single-select)

- **Radio-style selection** — one active sort at a time
- **Sort options**: Title (A-Z), Title (Z-A), Created (newest), Created (oldest), Updated (newest), Updated (oldest), Status, Priority
- **Current sort indicated** with a check icon
- **Default sort** comes from `_navigation.json` — if no user override, the configured default applies

### 2. `_navigation.json` — Per-Group View Configuration

Each artifact group directory (e.g. `.orqa/delivery/`, `.orqa/process/`) can contain a `_navigation.json` file that configures the default browsing experience for that group's artifact types.

```jsonc
{
  "defaults": {
    "sort": { "field": "created", "direction": "desc" },
    "group": "status",
    "filters": { "status": ["draft", "in-progress", "ready", "review"] }
  },
  "layout": null  // null = use standard sort/group/filter; object = custom layout
}
```

**Default view** — pre-set grouping, sorting, and filter state that loads when the user first navigates to that artifact type. User can override interactively; overrides persist in the navigation store (not in the file).

**Custom layout** — an alternative to sort/group/filter that arranges items in a curated order. When `layout` is non-null, the toolbar shows a "layout" indicator instead of sort/filter state. The layout is JSON-configured (UI builder is a future idea).

```jsonc
{
  "defaults": null,
  "layout": {
    "sections": [
      { "label": "Getting Started", "items": ["README", "vision", "governance"] },
      { "label": "Architecture", "items": ["architecture/*"] },
      { "label": "Development", "items": ["development/*"] }
    ]
  }
}
```

This addresses the Documentation use case specifically — docs should read like a book's table of contents, not a date-sorted list.

**Scanner integration** — the Rust artifact scanner reads `_navigation.json` alongside `README.md` when scanning a directory. The config is included in the `NavType` response so the frontend can apply defaults without an extra round-trip.

### 3. AI-Driven Global Search (Spotlight-Style)

Search is NOT in the artifact browser panel. It is a **Spotlight-style overlay** that appears over the current view without disrupting context:

- **Overlay modal** — centred, floating search bar (similar to macOS Spotlight or VS Code command palette) with results displayed underneath
- **Non-destructive** — the current panel content remains visible behind the overlay; dismissing search returns to exactly where the user was
- **AI query routing** — search query sent to the AI with artifact graph context as system prompt
- **Structured results** — AI returns artifact IDs with relevance explanations, rendered as a navigable list with ArtifactLink chips
- **Examples**: "what's blocking the next milestone", "show me all rules about error handling", "which tasks depend on [EPIC-6787bb93](EPIC-6787bb93)"
- **Keyboard shortcut** — Ctrl+Space opens search from anywhere (Cmd+Space on macOS if available, otherwise Ctrl+Space as primary)
- **ActivityBar icon** — Search icon in the bottom section of the ActivityBar, above Settings (alternative trigger)

The AI search builds on the existing artifact graph SDK — the AI has access to the full graph for context when answering queries.

### 4. References Panel

Surface the graph's cross-reference data in the viewer:

- **Incoming references** — "Referenced by: [EPIC-0a8a5e72](EPIC-0a8a5e72), [TASK-61934d01](TASK-61934d01), [RULE-7b770593](RULE-7b770593)"
- **Outgoing references** — "References: [PILLAR-569581e0](PILLAR-569581e0), [MS-654badde](MS-654badde), [RES-6ca176bb](RES-6ca176bb)"
- Rendered as ArtifactLink chips below the frontmatter header
- Uses existing `referencesFrom()` and `referencesTo()` from the graph SDK

## Implementation Design

### Phase 1: Schema-Driven Sort/Group/Filter Toolbar + `_navigation.json`

**Backend:**
- Extend DocNode to carry all scalar frontmatter fields
- Read each type's `schema.json` — extract enum fields as `filterable_fields` and date/string fields as `sortable_fields` on NavType
- Read `_navigation.json` from each type directory — include as `navigation_config` on NavType
- No new Tauri commands — data flows through the existing `artifact_scan_tree` response

**Frontend — Toolbar:**
- Replace `SearchInput` in `ArtifactNav` with `ArtifactToolbar.svelte` — fixed `h-10` bar
- Two ghost icon buttons: `ArrowUpDownIcon` (sort) and `FilterIcon` (filter)
- Active state indicators on icons when non-default sort/filter is applied

**Frontend — Filter panel (Popover):**
- Sections generated dynamically from NavType's `filterable_fields` (schema enum properties)
- Each section: heading + checkbox rows for each enum value
- Visual decorators by field name: status dots for `status`, priority colours for `priority`

**Frontend — Sort dropdown (DropdownMenu):**
- Sort options from NavType's `sortable_fields` plus universal fields (title)
- Group-by options from `filterable_fields` (enum fields)
- Collapsible group headers replace tree rendering
- Group header ordering: `_navigation.json` `group_order` → schema enum order → alphabetical

**Frontend — State:**
- `ArtifactViewState`: `{ sort: SortConfig; filters: FilterConfig; group: string | null }`
- Defaults from `_navigation.json` → user overrides in navigation store per type key
- Client-side sorting/filtering/grouping on the DocNode array

**Frontend — Tree mode removal + custom layout:**
- Remove tree rendering — all types render as flat lists with optional collapsible groups
- When `_navigation.json` has `layout`, render curated sections instead of sort/filter

### Phase 2: References Panel

- New `ReferencesPanel.svelte` component
- Placed below FrontmatterHeader in ArtifactViewer
- Calls `artifactGraphSDK.referencesFrom(id)` and `referencesTo(id)`
- Renders two collapsible sections with ArtifactLink chips

### Phase 3: Global AI Search (Spotlight-Style)

- `ArtifactSearchOverlay.svelte` — Spotlight-style floating modal with search input + results area
- Centred overlay that doesn't replace panel content — current context preserved
- Search input auto-focused on open — user can start typing immediately
- Keyboard shortcut: Ctrl+Space (Cmd+Space on macOS if available)
- Add Search icon to ActivityBar bottom section (above Settings) as alternative trigger
- Search query sent to AI provider with system prompt including artifact graph summary
- AI responds with structured result (artifact IDs + explanations)
- Results rendered as navigable list with ArtifactLink + explanation text
- Selecting a result navigates to the artifact and closes the overlay
- Escape key or clicking outside dismisses the overlay

## Tasks

**Phase 1 — Sort/Group/Filter Toolbar:**
- [TASK-db9be55f](TASK-db9be55f): Audit artifact group README files for accuracy
- [TASK-e6d10f6f](TASK-e6d10f6f): Backend — extend DocNode with frontmatter, NavType with schema metadata
- [TASK-d05c56ea](TASK-d05c56ea): Frontend — TypeScript types for schema metadata and navigation config
- [TASK-cc68b8df](TASK-cc68b8df): Frontend — ArtifactToolbar with sort dropdown and filter popover
- [TASK-5e1e41de](TASK-5e1e41de): Frontend — Client-side sorting, filtering, and grouping logic

**Phase 2 — References Panel:**
- [TASK-2e1df475](TASK-2e1df475): Frontend — References panel in artifact viewer

**Phase 3 — AI Search:**
- [TASK-21b461ea](TASK-21b461ea): Frontend — Spotlight-style AI search overlay
