---
id: IDEA-084
title: Frontend artifact graph in state — display flexibility via graph-synchronised store
description: "The frontend should hold the full artifact graph in a Svelte store, synchronised with the backend graph. This gives maximum display flexibility — components can traverse edges, resolve relationships, and render artifacts by ID without per-view backend calls. IDs remain the connector in frontmatter, but the graph store provides the human-readable context (titles, descriptions, status) that IDs alone don't convey."
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: next
research-needed:
  - "What shape should the graph store take? Adjacency list? Node map + edge list? How does it handle 500+ artifacts performantly?"
  - "How should graph sync work? Full reload on app start + incremental updates via file watcher events? Or lazy loading with caching?"
  - "Which existing views would benefit most from graph-backed rendering? (sidebar nav, relationship panels, epic task tables, milestone breakdowns)"
  - "How does this interact with IDEA-083 (auto-rendered task tables)? The graph store would be the data source for those auto-rendered views."
  - "Should the graph store replace existing per-type stores (artifactStore, etc.) or layer on top of them?"
  - "How do edge traversals work in practice? e.g., 'show me all rules that enforce AD-029' = traverse enforced-by edges from AD-029"
relationships:
  - target: MS-001
    type: delivers
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-002
    type: grounded-by
---

## Motivation

Currently, artifact display in the frontend requires per-view backend calls and type-specific stores. Each component that needs to show a related artifact (e.g., an epic's tasks, a rule's enforced ADs) must make its own `invoke()` call. This makes cross-artifact views expensive and limits the flexibility of the display layer.

The artifact graph already exists in the backend — the scanner builds it from `.orqa/` frontmatter. Synchronising this graph into a frontend store means any component can traverse edges and resolve artifact metadata without additional backend calls. This is the foundation for:

- Auto-rendered task tables in epics (IDEA-083)
- Relationship panels that show incoming/outgoing edges with human-readable titles
- Pipeline visualisation (observation → understanding → principle → practice → enforcement)
- Dependency graphs, milestone progress views, and other plugin-territory visualisations
- Search results enriched with graph context

## Sketch

**Graph store shape**: A `Map<string, ArtifactNode>` where each node has `id`, `title`, `status`, `type`, `relationships[]`, and the node's frontmatter. Edges are stored on the nodes (adjacency list style) rather than in a separate edge list — this matches how frontmatter stores relationships.

**Sync strategy**: Full graph load on app start (the scanner already does this). Incremental updates via Tauri file watcher events — when a `.orqa/` file changes, re-scan that single file and update the graph store.

**Display pattern**: Components receive an artifact ID and resolve it via the graph store to get title, status, type, and outgoing edges. No `invoke()` needed for basic metadata lookups.

```typescript
// Conceptual usage
const node = graphStore.get("AD-029");
const enforcedBy = graphStore.traverse("AD-029", "enforced-by");
// enforcedBy = [{ id: "RULE-006", title: "Coding Standards", ... }, ...]
```
