---
id: IDEA-9334b770
title: Artifact Traceability Graph
description: "Build a navigable knowledge graph from artifact cross-references that auto-updates when content changes. Enables plugin-driven views (dependency trees, impact analysis, knowledge provenance) without manual reverse links."
status: surpassed
created: 2026-03-07
updated: 2026-03-12
research-needed:
  - "Graph data model for artifact relationships (references, promotions, supersessions)"
  - Incremental update strategy when a single artifact changes
  - "Plugin API for consuming the graph (query interface, event hooks)"
  - "Visualization options (force-directed, hierarchical, timeline)"
relationships:
  - target: EPIC-0a8a5e72
    type: realises
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-a4b15450
    type: benefits
---
## Motivation

Artifacts reference each other via structured fields but these are one-directional. A navigable knowledge graph would enable reverse-link queries, impact analysis, and dependency visualization.

## Archived

The artifact traceability graph was largely implemented by [EPIC-0a8a5e72](EPIC-0a8a5e72) (Artifact Graph SDK). The SDK provides graph traversal, cross-reference resolution, and reverse-link queries over artifact frontmatter. Remaining visualization work (force-directed graphs, impact analysis views) can be pursued as separate features if needed.

## Concept

Artifacts already reference each other via structured fields (`research-refs`, `milestone`, `epic`, `depends-on`, `evolves-into`, `evolves-into`, etc.). Today these are one-directional — the consumer points at the source, but the source doesn't know who references it.

Instead of maintaining reverse links manually (which drift), build an automatic traceability graph that:

1. **Parses all artifact frontmatter** on scan and extracts cross-references
2. **Builds a directed graph** of relationships (edges typed by field name)
3. **Updates incrementally** when any artifact changes
4. **Exposes a query API** so plugins and views can ask questions like:
   - "What depends on this research doc?"
   - "Show me the full chain from idea → research → epic → tasks → decisions"
   - "What would be affected if this decision is superseded?"

## Why This Matters

This eliminates the need for reverse-link fields (like `produces_decisions`, `informs_epics`) on artifacts. Traceability is derived, not declared. The graph becomes the single source of truth for "where did this knowledge come from?" and "what does this affect?"

## Plugin Potential

Third-party plugins could consume the graph to create:
- Impact analysis views ("what breaks if I change X?")
- Knowledge provenance trails ("how did we arrive at this decision?")
- Dependency visualizations (force-directed graphs, Sankey diagrams)
- Stale artifact detection ("nothing references this anymore")
