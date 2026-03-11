---
id: EPIC-005
title: Artifact Browser — Sort, Filter, Search, Edit
description: Complete the core artifact browsing experience with sorting/grouping/filtering in the browser panel, AI-driven cross-artifact search, and in-app artifact editing. Absorbs EPIC-004 (editing UI).
status: draft
priority: P1
created: "2026-03-07"
updated: "2026-03-11"
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - RES-029
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

The core app's job is to let users **navigate, search, and edit** artifacts ([AD-033](AD-033)). Navigation and cross-linking are built. What's missing is the ability to sort/group/filter the artifact list, search across all artifacts semantically, and edit artifacts without leaving the app. Without these three, users must fall back to the terminal for basic governance work.

## What's Already Done

Previous work (EPIC-043, EPIC-044, and prior phases of this epic) delivered:

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

The core app's job is to let users navigate, search, and edit artifacts. Navigation and cross-linking are built. This epic completes the remaining core capabilities: sorting/filtering the artifact list, AI-driven semantic search, in-app editing, and a references panel. It absorbs EPIC-004 (editing UI) to consolidate all remaining artifact interaction work. The architecture decision [AD-033](AD-033) establishes that all system-level visualizations (roadmaps, dashboards, kanban) are plugin territory — the core app stays focused on these three capabilities.

## Design Principles

> The core app UI provides three capabilities: navigate, search, and edit. All system-level visualizations (roadmaps, dashboards, dependency graphs) are plugins. — [AD-033](AD-033)

> Cross-artifact search is AI-driven, not keyword-based. The AI infers search intent and presents results in a structured way, giving infinite flexibility. — User direction, 2026-03-11

## Remaining Scope

### 1. Sort, Group, and Filter in the Browser Panel

The ArtifactNav currently supports text filtering only. Users need to:

- **Sort** artifact lists by: title (alpha), created date, updated date, priority, status
- **Group** artifacts by: status, priority, milestone, epic (for tasks)
- **Filter** by: status (active/done/draft), priority (P1/P2/P3), layer (core/project)
- **Persist** sort/group/filter preferences per artifact type across sessions

These controls live in the ArtifactNav panel alongside the existing search input. The UI should be compact — dropdown/chip selectors, not a full filter panel.

### 2. AI-Driven Cross-Artifact Search

A search experience that uses the AI provider to understand intent and return structured results:

- **Search input** — prominent, always accessible (command palette style or dedicated panel)
- **AI query routing** — search query sent to the AI with artifact graph context
- **Structured results** — AI returns artifact IDs with relevance explanations, rendered as a navigable result list
- **Examples of semantic queries**: "what's blocking the next milestone", "show me all rules about error handling", "which tasks depend on EPIC-005"

The AI search builds on the existing artifact graph SDK — the AI has access to the full graph for context when answering queries.

### 3. In-App Artifact Editing (absorbed from EPIC-004)

Edit artifacts without leaving the app:

- **CodeMirror 6 editor** — markdown + YAML frontmatter editing with syntax highlighting
- **Edit mode toggle** — view ↔ edit on artifact viewers
- **Create from template** — new artifact with pre-filled frontmatter from schema
- **Delete with confirmation** — ConfirmDeleteDialog integration
- **Schema-aware validation** — frontmatter validated against the artifact type's schema.json on save
- **Wire to backend** — connect to existing artifact_create, artifact_update, artifact_delete commands

### 4. References Panel

Surface the graph's cross-reference data in the viewer:

- **Incoming references** — "Referenced by: EPIC-048, TASK-163, RULE-004"
- **Outgoing references** — "References: PILLAR-001, MS-001, RES-029"
- Rendered as ArtifactLink chips below the frontmatter header
- Uses existing `referencesFrom()` and `referencesTo()` from the graph SDK

## Implementation Design

### Phase 1: Sort, Group, Filter

Add controls to ArtifactNav:
- Sort dropdown (title, date, priority, status)
- Group dropdown (none, status, priority, milestone)
- Filter chips (status, priority, layer) — toggle on/off
- Store sort/group/filter state per artifact type key in the navigation store
- Backend: may need a `artifact_list_filtered` command or do client-side sorting on the NavTree data

### Phase 2: References Panel

- New `ReferencesPanel.svelte` component
- Placed below FrontmatterHeader in ArtifactViewer
- Calls `artifactGraphSDK.referencesFrom(id)` and `referencesTo(id)`
- Renders two collapsible sections with ArtifactLink chips

### Phase 3: Artifact Editing

- Add `codemirror` package + markdown/yaml language support
- `ArtifactEditor.svelte` component wrapping CodeMirror
- Edit button on ArtifactViewer toolbar — toggles view/edit mode
- On save: validate frontmatter against schema, call `artifact_update`
- Create flow: select type → pre-fill from schema → open editor
- Delete flow: button → ConfirmDeleteDialog → `artifact_delete`

### Phase 4: AI Search

- `ArtifactSearch.svelte` — search dialog (command palette style, Cmd+K)
- Search query sent to AI provider with system prompt including artifact graph summary
- AI responds with structured result (artifact IDs + explanations)
- Results rendered as navigable list with ArtifactLink + explanation text
- Needs: search-specific Tauri command or reuse of conversation streaming for search queries

## Tasks

- TASK-164: Audit artifact group README files for accuracy

Full task breakdown to be created during planning.
