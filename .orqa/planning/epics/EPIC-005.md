---
id: EPIC-005
title: Artifact Browser — Sort, Filter, Search
description: Complete the core artifact browsing experience with sorting/grouping/filtering in the browser panel, AI-driven cross-artifact search, and a references panel.
status: draft
priority: P1
created: "2026-03-07"
updated: "2026-03-11"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-029
  - RES-038
docs-required: []
docs-produced: []
scoring:
  pillar: 5
  impact: 5
  dependency: 3
  effort: 4
  score: 8.8
---

## Why P1

The core app's job is to let users **navigate and search** artifacts ([AD-033](AD-033)). Navigation and cross-linking are built. What's missing is the ability to sort/group/filter the artifact list, search across all artifacts semantically, and surface cross-references. Editing is tracked separately in [EPIC-004](EPIC-004).

## What's Already Done

Previous work ([EPIC-043](EPIC-043), [EPIC-044](EPIC-044), and prior phases of this epic) delivered:

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

The core app's job is to let users navigate, search, and edit artifacts. Navigation and cross-linking are built. This epic completes the browsing and search capabilities: sorting/filtering the artifact list, AI-driven semantic search, and a references panel. Editing is a separate concern tracked in [EPIC-004](EPIC-004) — artifact mutations are AI-mediated proposals, not raw CRUD. The architecture decision [AD-033](AD-033) establishes that all system-level visualizations (roadmaps, dashboards, kanban) are plugin territory.

## Design Principles

> The core app UI provides three capabilities: navigate, search, and edit. All system-level visualizations (roadmaps, dashboards, dependency graphs) are plugins. — [AD-033](AD-033)

> Cross-artifact search is AI-driven, not keyword-based. The AI infers search intent and presents results in a structured way, giving infinite flexibility. — [AD-037](AD-037)

> Sorting and filtering are schema-driven — generated dynamically from each artifact type's `schema.json` enum fields. No hardcoded field lists. — [AD-034](AD-034)

> Default views are config-driven via `_navigation.json` per artifact type directory. Supports sort/group/filter defaults, group ordering overrides, and custom curated layouts. — [AD-035](AD-035)

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

Each artifact group directory (e.g. `.orqa/planning/`, `.orqa/governance/`) can contain a `_navigation.json` file that configures the default browsing experience for that group's artifact types.

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
- **Examples**: "what's blocking the next milestone", "show me all rules about error handling", "which tasks depend on [EPIC-005](EPIC-005)"
- **Keyboard shortcut** — Ctrl+Space opens search from anywhere (Cmd+Space on macOS if available, otherwise Ctrl+Space as primary)
- **ActivityBar icon** — Search icon in the bottom section of the ActivityBar, above Settings (alternative trigger)

The AI search builds on the existing artifact graph SDK — the AI has access to the full graph for context when answering queries.

### 4. References Panel

Surface the graph's cross-reference data in the viewer:

- **Incoming references** — "Referenced by: [EPIC-048](EPIC-048), [TASK-163](TASK-163), [RULE-004](RULE-004)"
- **Outgoing references** — "References: [PILLAR-001](PILLAR-001), [MS-001](MS-001), [RES-029](RES-029)"
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
- [TASK-164](TASK-164): Audit artifact group README files for accuracy
- [TASK-165](TASK-165): Backend — extend DocNode with frontmatter, NavType with schema metadata
- [TASK-166](TASK-166): Frontend — TypeScript types for schema metadata and navigation config
- [TASK-167](TASK-167): Frontend — ArtifactToolbar with sort dropdown and filter popover
- [TASK-168](TASK-168): Frontend — Client-side sorting, filtering, and grouping logic

**Phase 2 — References Panel:**
- [TASK-169](TASK-169): Frontend — References panel in artifact viewer

**Phase 3 — AI Search:**
- [TASK-170](TASK-170): Frontend — Spotlight-style AI search overlay
