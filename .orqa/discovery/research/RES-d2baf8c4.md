---
id: RES-d2baf8c4
type: research
title: Graph analysis — using Cytoscape.js algorithms to power governance insights
description: "Research how Cytoscape.js built-in graph analysis algorithms can feed dashboard widgets, health metrics, dependency tracing, impact analysis, and knowledge gap detection in OrqaStudio."
status: completed
created: 2026-03-15
updated: 2026-03-15
relationships:
  - target: EPIC-ff7db83e
    type: guides
---

## Available Algorithms

Cytoscape.js provides these algorithms out of the box (no additional packages):

| Algorithm | Method | Returns |
|-----------|--------|---------|
| Connected components | `cy.elements().components()` | Array of element collections — each is a disconnected subgraph |
| Betweenness centrality | `eles.betweennessCentrality()` | Per-node score — how often a node sits on shortest paths between others |
| PageRank | `eles.pageRank()` | Per-node importance score based on incoming edge count and quality |
| Degree centrality | `eles.degreeCentrality()` | Per-node count of connections (in-degree, out-degree, total) |
| Closeness centrality | `eles.closenessCentrality()` | Per-node score — how close a node is to all other nodes |
| BFS / DFS | `eles.bfs()` / `eles.dfs()` | Traversal results — ordered visit of nodes from a root |
| Dijkstra | `eles.dijkstra()` | Shortest path between two nodes |
| A* | `eles.aStar()` | Optimal path with heuristic |
| Neighborhood | `node.neighborhood()` | All nodes and edges 1 hop away |

## Feature Mapping

### 1. Graph Health Scoring (Dashboard — GraphHealthWidget)

**Current**: Health score computed from error/warning counts from file-based integrity scan.

**With graph analysis**:
- `components()` → count disconnected clusters. A healthy graph has 1 large component. Multiple components = orphaned clusters.
- `degreeCentrality()` → find nodes with 0 in-degree (true orphans — nothing points to them)
- Health score formula: `(largest_component_size / total_nodes) * 100` — percentage of graph that's connected

### 2. Most Important Artifacts (New dashboard widget or graph overlay)

**`pageRank()`** ranks every artifact by structural importance:
- High PageRank = many other artifacts reference this one (backbone artifacts)
- Low PageRank = peripheral artifacts (may be orphans or very specific)
- Surface top 10 as "backbone artifacts" — if these break, the graph degrades

**`betweennessCentrality()`** finds bridge artifacts:
- High betweenness = artifact connects otherwise-separate parts of the graph
- If removed, the graph splits into components
- Surface as "critical path artifacts" — extra attention during changes

### 3. Dependency Chain Tracing (Artifact viewer enhancement)

**`bfs()` from any artifact** shows the full chain:
- Task → Epic → Milestone → Pillar (upward chain: "why does this exist?")
- Rule → Lessons that informed it → Decisions that produced it (provenance chain)
- Pillar → Rules grounded by it → Skills that practice them (downward chain: "how is this enforced?")

Display as a linear path in the artifact viewer: "This task traces to Pillar 1 through: TASK-2665cbf6 → EPIC-c96c9f12 → MS-b1ac0a20 → PILLAR-c9e0a695"

### 4. Impact Analysis (Before making changes)

**`node.neighborhood()` recursively** (or BFS with depth limit):
- "If I change RULE-9814ec3c, what is affected?" → all artifacts with `enforced-by`, `grounded-by`, `informed-by` relationships to RULE-9814ec3c
- Surface as a warning before editing: "This change affects 12 artifacts"
- Could power a pre-edit impact preview panel

### 5. Knowledge Gap Detection (Governance audit)

**`components()`** on the full graph:
- Each disconnected component is a knowledge silo
- The governance system should be one connected graph
- Disconnected components indicate missing relationships
- Surface as: "3 disconnected clusters detected — 8 artifacts are not connected to the main graph"

**Degree analysis per artifact type**:
- Rules with 0 `grounded-by` → not traced to a pillar
- Skills with 0 `grounded` → no agent uses this skill
- Decisions with 0 `enforces` → decision exists but nothing implements it
- Documentation with 0 relationships → orphaned knowledge

### 6. Trend Analysis (Dashboard — ImprovementTrendsWidget)

Store graph metrics in health snapshots:
- Connected component count (fewer = better)
- Orphan percentage (lower = better)
- Average degree centrality (higher = better connected)
- PageRank concentration (Gini coefficient — lower = more evenly distributed importance)

Track these over time alongside existing error/warning counts.

## Implementation Approach

The graph analysis should run on the Cytoscape instance in the FullGraphView (it already has the full graph loaded). Results can be exposed through a shared service or store that dashboard widgets read from.

Alternatively, build a lightweight analysis pass in the artifact graph SDK that constructs a cytoscape instance headless (no DOM container) purely for analysis:

```typescript
const cy = cytoscape({ elements, headless: true });
const components = cy.elements().components();
const pageRank = cy.elements().pageRank();
// ... extract metrics
cy.destroy();
```

This keeps analysis separate from visualization and can run on every graph refresh.

## Recommended Phases

**Phase 1**: Graph health scoring using `components()` + `degreeCentrality()`. Replace file-based orphan detection in GraphHealthWidget. Store metrics in health snapshots.

**Phase 2**: Dependency chain tracing in artifact viewer. BFS from current artifact to show upward/downward chains.

**Phase 3**: Impact analysis panel. Neighborhood traversal before edits. "This change affects N artifacts."

**Phase 4**: Most important artifacts widget. PageRank + betweenness for backbone identification.

**Phase 5**: Knowledge gap detection in governance audit. Per-type degree analysis for unlinked artifacts.