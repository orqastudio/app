---
id: "RES-e757fc6e"
type: research
title: "Graph SDK as primary data source for artifact UI"
description: "Investigate extending artifactGraphSDK to replace per-type invoke() calls with graph-backed rendering. The SDK already holds the full graph in reactive state — this research identifies what's missing for typed edge traversal, relationship-aware rendering, and component migration."
status: "completed"
created: "2026-03-13"
updated: "2026-03-13"
sources: []
relationships:
  - target: "EPIC-e37794bf"
    type: "guides"
    rationale: "Research findings informed the design of Pipeline health dashboard"
---
## Question

Can the existing `artifactGraphSDK` serve as the primary data source for artifact display, replacing per-component `invoke()` calls? What extensions are needed?

## Context

[IDEA-0f690b5f](IDEA-0f690b5f) proposed building a frontend graph store. Investigation reveals the store **already exists** — `artifactGraphSDK` in `ui/src/lib/sdk/artifact-graph.svelte.ts` is a Svelte 5 rune-based singleton holding the full artifact graph in a reactive `SvelteMap<string, ArtifactNode>`, auto-refreshing via Tauri events.

### What Already Exists

| Capability | Implementation | Status |
|-----------|---------------|--------|
| Full graph in reactive state | `SvelteMap<string, ArtifactNode>` with `$state` | Working |
| Auto-refresh on file changes | Tauri `artifact-graph-updated` event listener | Working |
| Resolve by ID | `resolve(id)` → `ArtifactNode` | Working |
| Resolve by path | `resolveByPath(path)` → `ArtifactNode` | Working |
| Outgoing edges | `referencesFrom(id)` → `ArtifactRef[]` | Working |
| Incoming edges | `referencesTo(id)` → `ArtifactRef[]` | Working |
| Filter by type | `byType("epic")` → `ArtifactNode[]` | Working |
| Filter by status | `byStatus("done")` → `ArtifactNode[]` | Working |
| Graph health | `brokenRefs()`, `orphans()` | Working |
| Subscription API | `subscribe(id, cb)`, `subscribeType(type, cb)` | Working |
| Graph stats | `stats` → `GraphStats` (nodes, edges, orphans, broken refs) | Working |
| Full frontmatter | `node.frontmatter` → `Record<string, unknown>` | Working |

### What's Missing

**1. Typed relationship traversal**

`ArtifactRef` has `target_id`, `field`, and `source_id` — but no `type` field for relationship semantics (`enforced-by`, `grounded`, `grounded-by`, etc.). The `relationships` array from frontmatter is available via `node.frontmatter.relationships` but is untyped (`unknown`).

To traverse the pipeline (Observation → Understanding → Principle → Practice → Enforcement → Verification), the SDK needs:

```typescript
// Proposed API
sdk.traverse(id, "enforced-by")     // → ArtifactNode[] with rationale
sdk.traverse(id, "grounded")        // → ArtifactNode[] (upstream pipeline)
sdk.pipelineChain(id)               // → full upstream/downstream chain
```

This requires either:
- **(a)** Extending `ArtifactRef` with a `relationship_type` field populated by the Rust backend during graph construction
- **(b)** A frontend-only helper that reads `node.frontmatter.relationships` and resolves targets

Option (a) is better — it keeps the graph model clean and avoids frontend JSON parsing.

**2. Relationship-aware graph stats**

`GraphStats` currently tracks `node_count`, `edge_count`, `orphan_count`, `broken_ref_count`. For the pipeline health dashboard, we need:

- Relationship type distribution (how many `enforced-by`, `grounded`, etc.)
- Missing inverse detection count
- Pipeline stage coverage (how many ADs have enforcement chains)

These could be added to `GraphStats` or returned by a separate `get_integrity_stats` command.

**3. Component migration path**

17 files already reference the SDK, but many artifact views still use `invoke()` directly. The migration is:

| Current Pattern | Graph-backed Pattern |
|----------------|---------------------|
| `invoke("get_artifacts_by_type", { artifactType: "task" })` | `artifactGraphSDK.byType("task")` |
| `invoke("read_artifact_content", { path })` | `artifactGraphSDK.readContent(path)` (already in SDK) |
| Per-component loading states | SDK-level `loading`/`error` states (already in SDK) |
| Relationship display via frontmatter parsing | `sdk.traverse(id, type)` (needs extension) |

## Findings

1. **The graph store already exists and is production-quality.** IDEA-0f690b5f's core proposal is already implemented. The SDK is well-designed — reactive `SvelteMap`, auto-refresh, synchronous lookups, subscription API.

2. **The main gap is typed relationship traversal.** Extending `ArtifactRef` with `relationship_type: string | null` in the Rust backend and populating it during graph construction is the cleanest approach. The `relationships` array in frontmatter already has `type` and `target` — the backend just needs to propagate the type into `ArtifactRef`.

3. **Component migration is low-risk.** Most components already import the SDK. Switching from `invoke()` to `sdk.byType()` or `sdk.resolve()` is a straightforward refactor that removes boilerplate.

4. **Integrity checks should be native graph operations.** The SDK already has `brokenRefs()` and `orphans()`. Adding `missingInverses()`, `pipelineGaps()`, and per-category integrity counts makes the dashboard a pure view over the SDK — no separate backend commands needed for basic health display.

## Recommendation

Add typed relationship traversal to the graph SDK as Phase 0 of [EPIC-e37794bf](EPIC-e37794bf). This is a small backend change (extend `ArtifactRef` with `relationship_type`) plus SDK helper methods. Every subsequent phase benefits:

- **Phase 1** (integrity engine): `sdk.missingInverses()`, `sdk.pipelineGaps()` — graph operations, not separate commands
- **Phase 2** (dashboard widget): renders SDK stats and integrity results — pure view layer
- **Phase 3** (pipeline visualization): `sdk.pipelineChain(id)` traverses the graph — no backend round-trips
- **Phase 4** (temporal analytics): snapshot `sdk.stats` periodically — already available