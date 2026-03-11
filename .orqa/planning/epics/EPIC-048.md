---
id: EPIC-048
title: Artifact Graph SDK and Structural Integrity
description: Build a bidirectional artifact node graph with a typed frontend SDK, body template enforcement, markdown cross-linking, file watcher for live refresh, and plugin-ready subscription API — establishing the foundation for the plugin architecture.
status: done
priority: P1
created: "2026-03-10"
updated: "2026-03-10"
milestone: MS-001
pillars:
  - PILLAR-001
  - PILLAR-002
research-refs:
  - RES-032
  - RES-033
  - RES-034
docs-required:
  - RES-032
  - RES-033
  - RES-034
  - DOC-036
docs-produced:
  - DOC-036
  - DOC-020
  - SKILL-020
scoring:
  user-value: 5
  pillar-alignment: 5
  dependency-weight: 5
  effort: 5
  risk: 3
  score: 17
---
## Context

Three systemic gaps identified during dogfooding prevent the artifact system from being self-consistent and extensible:

1. **Body structure is freeform** — Artifact frontmatter is now schema-enforced (JSON Schema + pre-commit validation), but everything below the `---` is whatever the author invents. Some types have naturally converged (pillars, milestones, decisions, lessons) while others vary widely (epics, tasks) or are nearly empty (ideas).

2. **Cross-linking is fragile** — ArtifactLink navigation uses a hardcoded `ARTIFACT_PREFIX_MAP` in the frontend and `label.startsWith(pendingId)` string matching. This breaks for tree-structured directories, misses artifact types without prefix entries (RES, PILLAR, RULE), and silently fails when titles don't match filenames. There is no backend resolution.

3. **No unified artifact API** — The sidebar, viewer, linking system, and future plugins all access artifact data through different ad-hoc patterns. There is no single source of truth for artifact metadata and relationships, no way to query backreferences ("what links to [EPIC-001](EPIC-001)?"), and no foundation for plugin development.

This epic addresses all three by building a bidirectional artifact node graph in the backend, exposing it through a typed frontend SDK, and migrating all artifact access to use it. The SDK becomes the foundation for the plugin architecture (see [IDEA-036](IDEA-036) for future expansion to full-codebase graph).

## Implementation Design

### Part 1: Body Templates ([RES-032](RES-032))

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

### Part 2: Backend Artifact Node Graph ([RES-033](RES-033), [RES-034](RES-034))

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

### Part 3: Frontend Artifact Graph SDK ([RES-034](RES-034))

Typed Svelte 5 rune store at `ui/lib/sdk/artifact-graph.svelte.ts`:

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
- Note: full snapshot approach will need to become incremental when graph expands to full codebase (see [IDEA-036](IDEA-036))

### Built-in vs Plugin Boundary

The app ships with **default views only** — these are the artifact browser, viewers, and core navigation. They use the SDK exactly as plugins will. Everything beyond the defaults is a plugin.

| Layer | Examples | Distribution |
|-------|----------|-------------|
| **Built-in (default)** | Artifact browser, viewers (artifact, agent, skill, rule, hook), navigation, conversation | Shipped with app binary |
| **Official plugins** | Governance dashboard, dependency graph, sprint planning | Official plugins repo ([IDEA-038](IDEA-038)) |
| **Community plugins** | Third-party extensions shared publicly | Their own repos, installable via URL |
| **User plugins** | Custom extensions built locally with the AI | Local file path, git-versioned locally |

### Plugin Development Skill ([TASK-081](TASK-081))

The skill must guide the AI to:

1. **Always create plugins in a new standalone project** — never inside the user's production codebase
2. **Generate seed data** for testing — a mock `.orqa/` directory with representative artifacts that exercise the plugin's features
3. **Develop and test against seed data** using the Artifact Graph SDK
4. **Only import into production** once the plugin is working and tested

This protects production governance data during development. The skill references [IDEA-038](IDEA-038) for the distribution architecture that will be built in the next phase.

## Out of Scope

- Graph visualization UI (node/edge rendering) — separate epic
- App-assisted template pre-population (artifact editor) — deferred to [EPIC-004](EPIC-004)
- Full-codebase graph expansion — captured as [IDEA-036](IDEA-036)
- Plugin runtime, loading, and distribution mechanism — captured as [IDEA-038](IDEA-038)
- Artifact write operations via SDK — [EPIC-004](EPIC-004) scope

## Tasks

| Task | Title | Scope |
|------|-------|-------|
| [TASK-070](TASK-070) | Document body templates in artifact-framework.md and schema.json | .orqa/documentation/, .orqa/**/schema.json |
| [TASK-071](TASK-071) | Add body template linting to pre-commit hook | .githooks/validate-schema.mjs |
| [TASK-072](TASK-072) | Backfill existing artifacts to match body templates | .orqa/planning/, .orqa/governance/ |
| [TASK-073](TASK-073) | Build backend artifact node graph with bidirectional references | src-tauri/src/domain/ |
| [TASK-074](TASK-074) | Add artifact graph Tauri commands | src-tauri/src/commands/ |
| [TASK-075](TASK-075) | Build frontend Artifact Graph SDK with subscription API | ui/lib/sdk/ |
| [TASK-076](TASK-076) | Migrate stores to SDK: replace artifact/navigation store ad-hoc patterns | ui/lib/stores/ |
| [TASK-082](TASK-082) | Migrate viewer components to SDK: frontmatter from graph, link handling | ui/lib/components/artifact/*Viewer.svelte |
| [TASK-083](TASK-083) | Migrate nav and linking to SDK: ArtifactLink, ArtifactNav, FrontmatterHeader, AppLayout | ui/lib/components/ |
| [TASK-077](TASK-077) | Broken link styling and path validation | ui/lib/components/artifact/ |
| [TASK-078](TASK-078) | Markdown cross-linking in MarkdownRenderer | ui/lib/components/shared/ |
| [TASK-079](TASK-079) | File watcher for .orqa/ with graph rebuild and event emission | src-tauri/src/ |
| [TASK-080](TASK-080) | Write Artifact Graph SDK documentation | .orqa/documentation/development/ |
| [TASK-081](TASK-081) | Create plugin-development skill (new project + seed data approach) | .orqa/team/skills/ |

## Dependency Chain

```
Track A — Body Templates (governance-only, no code changes):
TASK-070 (templates + schema) ──> TASK-071 (linting) ──> TASK-072 (backfill)

Track B — Graph + SDK + Dogfood Migration:
TASK-073 (backend graph) ──> TASK-074 (Tauri commands) ──> TASK-075 (frontend SDK)
  ──> TASK-076 (migrate stores)
    ──> TASK-082 (migrate viewers)
    ──> TASK-083 (migrate nav/linking)
      ──> TASK-077 (broken links)
      ──> TASK-078 (markdown cross-links)

TASK-073 (backend graph) ──> TASK-079 (file watcher)

Track C — Documentation (after migration proves the SDK):
TASK-083 (migration complete) ──> TASK-080 (SDK docs) ──> TASK-081 (plugin skill)
```

Tracks A and B are independent and can be parallelized. Track C depends on the migration being complete — the SDK docs describe the proven API, not a speculative one. Within Track B, [TASK-082](TASK-082) and [TASK-083](TASK-083) can be parallelized after [TASK-076](TASK-076). [TASK-077](TASK-077) and [TASK-078](TASK-078) can be parallelized after [TASK-083](TASK-083).
