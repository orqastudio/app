---
id: "EPIC-0a8a5e72"
type: epic
title: "Artifact Graph SDK and Structural Integrity"
description: "Build a bidirectional artifact node graph with a typed frontend SDK, body template enforcement, markdown cross-linking, file watcher for live refresh, and plugin-ready subscription API — establishing the foundation for the plugin architecture."
status: "completed"
priority: "P1"
created: "2026-03-10"
updated: "2026-03-10"
horizon: null
scoring:
  impact: 5
  urgency: 4
  complexity: 5
  dependencies: 5
relationships:
  - target: "RES-06ba5474"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-06ba5474"
  - target: "RES-a15d38de"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-a15d38de"
  - target: "RES-c387773d"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-c387773d"
  - target: "TASK-f950424e"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-6a4eea2f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-8c0e5f1d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-137ec554"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-126265d4"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-18eee9b0"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-451dd8b1"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ff295517"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2b47b899"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-832a3128"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-db618792"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-30307a19"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-aba97fb4"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-64ceb043"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-12eec0f3"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "IDEA-a99c270e"
    type: "realised-by"
  - target: "IDEA-9334b770"
    type: "realised-by"
  - target: "DOC-e6f39c35"
    type: "documented-by"
  - target: "DOC-3c65a1e3"
    type: "documented-by"
  - target: "DOC-e0042602"
    type: "documented-by"
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
## Context

Three systemic gaps identified during dogfooding prevent the artifact system from being self-consistent and extensible:

1. **Body structure is freeform** — Artifact frontmatter is now schema-enforced (JSON Schema + pre-commit validation), but everything below the `---` is whatever the author invents. Some types have naturally converged (pillars, milestones, decisions, lessons) while others vary widely (epics, tasks) or are nearly empty (ideas).

2. **Cross-linking is fragile** — ArtifactLink navigation uses a hardcoded `ARTIFACT_PREFIX_MAP` in the frontend and `label.startsWith(pendingId)` string matching. This breaks for tree-structured directories, misses artifact types without prefix entries (RES, PILLAR, RULE), and silently fails when titles don't match filenames. There is no backend resolution.

3. **No unified artifact API** — The sidebar, viewer, linking system, and future plugins all access artifact data through different ad-hoc patterns. There is no single source of truth for artifact metadata and relationships, no way to query backreferences ("what links to [EPIC-e045ab6d](EPIC-e045ab6d)?"), and no foundation for plugin development.

This epic addresses all three by building a bidirectional artifact node graph in the backend, exposing it through a typed frontend SDK, and migrating all artifact access to use it. The SDK becomes the foundation for the plugin architecture (see [IDEA-75424fb8](IDEA-75424fb8) for future expansion to full-codebase graph).

## Implementation Design

### Part 1: Body Templates [RES-a15d38de](RES-a15d38de)

Document and enforce minimum body structure for each artifact type:

| Type | Required Sections | Status |
|------|-------------------|--------|
| Pillar | What This Pillar Means, Examples, Anti-Patterns, Conflict Resolution | Already consistent |
| Milestone | Context, Epics, Completion Criteria | Already consistent |
| Decision | Decision, Rationale, Consequences | Already consistent |
| Lesson | Pattern, Fix | Already consistent |
| Epic | Context, Implementation Design, Tasks, Out of Scope (optional) | Needs enforcement |
| Task | What, How, Verification | Needs enforcement |
| Idea | Motivation, Sketch (optional) | Needs enforcement |
| Rule | Opening paragraph, domain sections, FORBIDDEN, Related Rules | Semi-structured |
| Research | Intentionally freeform | No template |

**Enforcement:** Two levels:

1. **Documentation** — templates documented in artifact-framework.md
2. **Linting** — pre-commit hook checks for required `## Heading` patterns, driven by template definitions in each type's schema.json

Body templates are defined in schema.json alongside frontmatter schemas — one source of truth for all structural expectations per artifact type.

### Part 2: Backend Artifact Node Graph ([RES-c387773d](RES-c387773d), [RES-06ba5474](RES-06ba5474))

Replace the flat ID→path index with a full bidirectional graph:

```rust
ArtifactGraph {
    nodes: HashMap<String, ArtifactNode>,    // keyed by artifact ID
    path_index: HashMap<String, String>,     // path → ID reverse lookup
}

ArtifactNode {
    id, path, artifact_type, title, description, status,
    frontmatter: serde_json::Value,          // full parsed frontmatter
    references_out: Vec<ArtifactRef>,        // forward links
    references_in: Vec<ArtifactRef>,         // backlinks (computed)
}

ArtifactRef { target_id, field, source_id }
```

**Graph construction** during `artifact_scan_tree`:
1. First pass: scan all `.md` files, extract frontmatter, create nodes with `references_out`
2. Second pass: invert all `references_out` to populate `references_in`
3. Store in `AppState`

**Tauri commands:**
- `resolve_artifact(id)` / `resolve_path(path)` — core resolution
- `get_references_from(id)` / `get_references_to(id)` — relationship queries
- `get_artifacts_by_type(type)` / `get_artifact_children(id)` — bulk queries
- `read_artifact_content(path)` — raw markdown body (always from disk, no caching)
- `get_graph_stats()` — node count, edge count, orphans, broken refs

### Part 3: Frontend Artifact Graph SDK [RES-06ba5474](RES-06ba5474)

Typed Svelte 5 rune store at `ui/src/lib/sdk/artifact-graph.svelte.ts`:

```typescript
class ArtifactGraphSDK {
    // Reactive state
    graph, loading, lastRefresh

    // Resolution (synchronous — in-memory lookups)
    resolve(id), resolveByPath(path)

    // Relationships (synchronous)
    referencesFrom(id), referencesTo(id), children(id, type?)

    // Bulk queries (synchronous)
    byType(type), byStatus(status)

    // Content (async — reads from disk)
    readContent(path)

    // Graph health (synchronous)
    brokenRefs(), orphans()

    // Subscriptions (plugin API)
    subscribe(id, callback): unsubscribe
    subscribeType(type, callback): unsubscribe

    // Lifecycle
    refresh()
    // Auto-refreshes via file watcher events
}
```

**Design principles:**
- Eagerly loaded — full graph on app start, in-memory thereafter
- Synchronous reads for metadata/relationships (no IPC round-trip)
- Async only for `readContent()` (disk I/O)
- Subscriptions for plugins — callbacks fire on graph refresh when watched node/type changes

### Part 4: Dogfood Migration — Built-in Views as SDK Consumers

The built-in artifact views are the first consumer of the SDK. Every ad-hoc artifact access pattern in the existing UI is replaced by SDK calls, proving the API works before plugins use it.

**Migration map (10 files, 3 phases):**

Phase 4a — Stores (data layer):

| File | Current Pattern | Replaced By |
|------|----------------|-------------|
| `artifact.svelte.ts` | `invoke("artifact_scan_tree")` for NavTree | Keep for sidebar shape; SDK handles metadata/relationships |
| `artifact.svelte.ts` | `invoke("read_artifact")` + `viewerCache` | `artifactGraph.readContent(path)` (always from disk, no frontend cache) |
| `navigation.svelte.ts` | `ARTIFACT_PREFIX_MAP` hardcoded prefix→group map | `artifactGraph.resolve(id)` for ID→path, then `navigateToPath()` |
| `navigation.svelte.ts` | `pendingArtifactId` + `label.startsWith()` workaround | `artifactGraph.resolve(id).path` — exact match, no label guessing |

Phase 4b — Viewer components:

| File | Current Pattern | Replaced By |
|------|----------------|-------------|
| `ArtifactViewer.svelte` | `parseFrontmatter(content)` to extract metadata | `artifactGraph.resolve(id).frontmatter` (already parsed in graph) |
| `ArtifactViewer.svelte` | Internal link click handler parsing hrefs | SDK-based navigation via `navigateToPath()` |
| `AgentViewer.svelte` | `parseFrontmatter()` for agent metadata | `artifactGraph.resolveByPath(path).frontmatter` |
| `SkillViewer.svelte` | `parseFrontmatter()` for skill metadata | `artifactGraph.resolveByPath(path).frontmatter` |

Phase 4c — Navigation and linking components:

| File | Current Pattern | Replaced By |
|------|----------------|-------------|
| `ArtifactLink.svelte` | `navigateToArtifact(id)` via prefix map | `artifactGraph.resolve(id)` — broken if undefined |
| `ArtifactNav.svelte` | `pendingArtifactId` auto-select with `if (isTree) return` guard | SDK resolution — works for flat AND tree types |
| `FrontmatterHeader.svelte` | `ARTIFACT_ID_RE` regex for link detection | SDK `resolve()` — if resolvable, render as link |
| `GroupSubPanel.svelte` | Icon lookup from config + NavTree | Unchanged (NavTree stays for sidebar shape) |
| `AppLayout.svelte` | `invoke("artifact_watch_start")` | SDK watcher integration (auto-refresh) |

**What stays:** `frontmatter.ts` parser stays as a lightweight fallback for files not yet in the graph (e.g., newly created files before watcher fires). NavTree remains for sidebar shape. Icon mapping and file-type routing unchanged.

**NavTree vs SDK:** NavTree defines sidebar structure (groups, ordering, icons). SDK provides artifact data (metadata, relationships, content). `DocNode.id` connects the two.

### Part 5: Link Rendering and Broken Link Detection

- `ArtifactLink` uses `artifactGraph.resolve(id)` — if undefined, render as broken link
- **Broken links** styled with broken-link icon + app warning colour token
- `docs-required`/`docs-produced` paths validated against disk during scan, flagged in UI
- `FrontmatterHeader` distinguishes ID links from path links

### Part 6: Markdown Cross-Linking

- Regex pass in `MarkdownRenderer` matching all artifact ID patterns (EPIC-NNN, TASK-NNN, AD-NNN, MS-NNN, IDEA-NNN, IMPL-NNN, RES-NNN, PILLAR-NNN, RULE-NNN)
- Wrap matches in clickable elements that call `navigateToArtifact`
- Always-on for all known patterns

### Part 7: File Watcher

- Watch `.orqa/` for file system changes (create, modify, delete, rename)
- Rebuild artifact graph on change (debounced)
- Emit full graph snapshot as Tauri event to frontend
- SDK receives event, replaces local graph, fires subscription callbacks
- Note: full snapshot approach will need to become incremental when graph expands to full codebase (see [IDEA-75424fb8](IDEA-75424fb8))

### Built-in vs Plugin Boundary

The app ships with **default views only** — these are the artifact browser, viewers, and core navigation. They use the SDK exactly as plugins will. Everything beyond the defaults is a plugin.

| Layer | Examples | Distribution |
|-------|----------|-------------|
| **Built-in (default)** | Artifact browser, viewers (artifact, agent, skill, rule, hook), navigation, conversation | Shipped with app binary |
| **Official plugins** | Governance dashboard, dependency graph, sprint planning | Official plugins repo [IDEA-b77e2955](IDEA-b77e2955) |
| **Community plugins** | Third-party extensions shared publicly | Their own repos, installable via URL |
| **User plugins** | Custom extensions built locally with the AI | Local file path, git-versioned locally |

### Plugin Development Skill [TASK-30307a19](TASK-30307a19)

The skill must guide the AI to:

1. **Always create plugins in a new standalone project** — never inside the user's production codebase
2. **Generate seed data** for testing — a mock `.orqa/` directory with representative artifacts that exercise the plugin's features
3. **Develop and test against seed data** using the Artifact Graph SDK
4. **Only import into production** once the plugin is working and tested

This protects production governance data during development. The skill references [IDEA-b77e2955](IDEA-b77e2955) for the distribution architecture that will be built in the next phase.

## Out of Scope

- Graph visualization UI (node/edge rendering) — separate epic
- App-assisted template pre-population (artifact editor) — deferred to [EPIC-fe75b52c](EPIC-fe75b52c)
- Full-codebase graph expansion — captured as [IDEA-75424fb8](IDEA-75424fb8)
- Plugin runtime, loading, and distribution mechanism — captured as [IDEA-b77e2955](IDEA-b77e2955)
- Artifact write operations via SDK — [EPIC-fe75b52c](EPIC-fe75b52c) scope

## Tasks

| Task | Title | Scope |
|------|-------|-------|
| [TASK-f950424e](TASK-f950424e) | Document body templates in artifact-framework.md and schema.json | .orqa/documentation/, .orqa/**/schema.json |
| [TASK-6a4eea2f](TASK-6a4eea2f) | Add body template linting to pre-commit hook | .githooks/validate-schema.mjs |
| [TASK-8c0e5f1d](TASK-8c0e5f1d) | Backfill existing artifacts to match body templates | .orqa/delivery/, .orqa/process/ |
| [TASK-137ec554](TASK-137ec554) | Build backend artifact node graph with bidirectional references | backend/src-tauri/src/domain/ |
| [TASK-126265d4](TASK-126265d4) | Add artifact graph Tauri commands | backend/src-tauri/src/commands/ |
| [TASK-18eee9b0](TASK-18eee9b0) | Build frontend Artifact Graph SDK with subscription API | ui/src/lib/sdk/ |
| [TASK-451dd8b1](TASK-451dd8b1) | Migrate stores to SDK: replace artifact/navigation store ad-hoc patterns | ui/src/lib/stores/ |
| [TASK-aba97fb4](TASK-aba97fb4) | Migrate viewer components to SDK: frontmatter from graph, link handling | ui/src/lib/components/artifact/*Viewer.svelte |
| [TASK-64ceb043](TASK-64ceb043) | Migrate nav and linking to SDK: ArtifactLink, ArtifactNav, FrontmatterHeader, AppLayout | ui/src/lib/components/ |
| [TASK-ff295517](TASK-ff295517) | Broken link styling and path validation | ui/src/lib/components/artifact/ |
| [TASK-2b47b899](TASK-2b47b899) | Markdown cross-linking in MarkdownRenderer | ui/src/lib/components/shared/ |
| [TASK-832a3128](TASK-832a3128) | File watcher for .orqa/ with graph rebuild and event emission | backend/src-tauri/src/ |
| [TASK-db618792](TASK-db618792) | Write Artifact Graph SDK documentation | .orqa/documentation/development/ |
| [TASK-30307a19](TASK-30307a19) | Create orqa-plugin-development skill (new project + seed data approach) | .orqa/process/skills/ |

## Dependency Chain

```
Track A — Body Templates (governance-only, no code changes):
TASK-f950424e (templates + schema) ──> TASK-6a4eea2f (linting) ──> TASK-8c0e5f1d (backfill)

Track B — Graph + SDK + Dogfood Migration:
TASK-137ec554 (backend graph) ──> TASK-126265d4 (Tauri commands) ──> TASK-18eee9b0 (frontend SDK)
  ──> TASK-451dd8b1 (migrate stores)
    ──> TASK-aba97fb4 (migrate viewers)
    ──> TASK-64ceb043 (migrate nav/linking)
      ──> TASK-ff295517 (broken links)
      ──> TASK-2b47b899 (markdown cross-links)

TASK-137ec554 (backend graph) ──> TASK-832a3128 (file watcher)

Track C — Documentation (after migration proves the SDK):
TASK-64ceb043 (migration complete) ──> TASK-db618792 (SDK docs) ──> TASK-30307a19 (plugin skill)
```

Tracks A and B are independent and can be parallelized. Track C depends on the migration being complete — the SDK docs describe the proven API, not a speculative one. Within Track B, [TASK-aba97fb4](TASK-aba97fb4) and [TASK-64ceb043](TASK-64ceb043) can be parallelized after [TASK-451dd8b1](TASK-451dd8b1). [TASK-ff295517](TASK-ff295517) and [TASK-2b47b899](TASK-2b47b899) can be parallelized after [TASK-64ceb043](TASK-64ceb043).