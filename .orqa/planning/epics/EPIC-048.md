---
id: EPIC-048
title: Artifact Graph SDK and Structural Integrity
description: "Build a bidirectional artifact node graph with a typed frontend SDK, body template enforcement, markdown cross-linking, file watcher for live refresh, and plugin-ready subscription API — establishing the foundation for the plugin architecture."
status: draft
priority: P1
created: 2026-03-10
updated: 2026-03-10
milestone: MS-001
pillars:
  - PILLAR-001
  - PILLAR-002
research-refs:
  - RES-032
  - RES-033
  - RES-034
docs-required:
  - .orqa/planning/research/RES-032.md
  - .orqa/planning/research/RES-033.md
  - .orqa/planning/research/RES-034.md
  - .orqa/documentation/product/artifact-framework.md
docs-produced:
  - .orqa/documentation/product/artifact-framework.md
  - .orqa/documentation/development/artifact-graph-sdk.md
  - .orqa/team/skills/plugin-development/SKILL.md
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

3. **No unified artifact API** — The sidebar, viewer, linking system, and future plugins all access artifact data through different ad-hoc patterns. There is no single source of truth for artifact metadata and relationships, no way to query backreferences ("what links to EPIC-001?"), and no foundation for plugin development.

This epic addresses all three by building a bidirectional artifact node graph in the backend, exposing it through a typed frontend SDK, and migrating all artifact access to use it. The SDK becomes the foundation for the plugin architecture (see IDEA-036 for future expansion to full-codebase graph).

## Implementation Design

### Part 1: Body Templates (RES-032)

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

### Part 2: Backend Artifact Node Graph (RES-033, RES-034)

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

### Part 3: Frontend Artifact Graph SDK (RES-034)

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

### Part 4: Migration to SDK

| Current Pattern | Replaced By |
|----------------|-------------|
| `ARTIFACT_PREFIX_MAP` in NavigationStore | `artifactGraph.resolve(id)` |
| `pendingArtifactId` + `label.startsWith()` | `artifactGraph.resolve(id).path` → `navigateToPath()` |
| `ArtifactLink` hardcoded prefix routing | `artifactGraph.resolve(id)` |
| `read_artifact` for viewer | `artifactGraph.readContent(path)` + `artifactGraph.resolve(id)` |
| Ad-hoc frontmatter parsing in components | `node.frontmatter` from graph |

**NavTree remains** for sidebar shape (groups, ordering, icons). `DocNode.id` connects sidebar items to graph nodes. The graph does NOT replace NavTree — they serve different purposes.

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
- Note: full snapshot approach will need to become incremental when graph expands to full codebase (see IDEA-036)

## Out of Scope

- Graph visualization UI (node/edge rendering) — separate epic
- App-assisted template pre-population (artifact editor) — deferred to EPIC-004
- Full-codebase graph expansion — captured as IDEA-036
- Plugin runtime and loading mechanism — this epic builds the SDK foundation, not the plugin system itself
- Artifact write operations via SDK — EPIC-004 scope

## Tasks

| Task | Title | Scope |
|------|-------|-------|
| TASK-070 | Document body templates in artifact-framework.md and schema.json | .orqa/documentation/, .orqa/**/schema.json |
| TASK-071 | Add body template linting to pre-commit hook | .githooks/validate-schema.mjs |
| TASK-072 | Backfill existing artifacts to match body templates | .orqa/planning/, .orqa/governance/ |
| TASK-073 | Build backend artifact node graph with bidirectional references | src-tauri/src/domain/ |
| TASK-074 | Add artifact graph Tauri commands | src-tauri/src/commands/ |
| TASK-075 | Build frontend Artifact Graph SDK with subscription API | ui/lib/sdk/ |
| TASK-076 | Migrate navigation, viewer, and links to use SDK | ui/lib/stores/, ui/lib/components/ |
| TASK-077 | Broken link styling and path validation | ui/lib/components/artifact/ |
| TASK-078 | Markdown cross-linking in MarkdownRenderer | ui/lib/components/shared/ |
| TASK-079 | File watcher for .orqa/ with graph rebuild and event emission | src-tauri/src/ |
| TASK-080 | Write Artifact Graph SDK documentation | .orqa/documentation/development/ |
| TASK-081 | Create plugin-development skill | .orqa/team/skills/ |

## Dependency Chain

```
Track A — Body Templates (governance-only, no code changes):
TASK-070 (templates + schema) ──> TASK-071 (linting) ──> TASK-072 (backfill)

Track B — Graph + SDK + Migration:
TASK-073 (backend graph) ──> TASK-074 (Tauri commands) ──> TASK-075 (frontend SDK)
  ──> TASK-076 (migrate nav/viewer/links)
  ──> TASK-077 (broken links)
  ──> TASK-078 (markdown cross-links)

TASK-073 (backend graph) ──> TASK-079 (file watcher)

Track C — Documentation (after SDK is built):
TASK-075 (SDK) ──> TASK-080 (SDK docs) ──> TASK-081 (plugin skill)
```

Tracks A and B are independent and can be parallelized. Track C depends on the SDK being built. Within Track B, TASK-076/077/078 can be parallelized after TASK-075 is complete.
