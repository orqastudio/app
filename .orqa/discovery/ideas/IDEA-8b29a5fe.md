---
id: IDEA-8b29a5fe
type: idea
title: Artifact Relationships Tab Panel
description: "Add a relationships tab to the artifact detail view that visualises how the current artifact connects to other artifacts — showing inbound and outbound references as an interactive graph or structured list, powered by the Artifact Graph SDK."
status: captured
created: 2026-03-11
updated: 2026-03-13
horizon: later
research-needed:
  - "What visualisation works best for a single-artifact ego graph? (force-directed, radial tree, structured list with icons)"
  - "How deep should the graph go? (direct refs only, or 2-hop neighbours?)"
  - "Should the panel show both directions (references-out and references-in) or separate them?"
  - "How to handle artifacts with many connections without overwhelming the view?"
  - "What interaction patterns make sense? (click to navigate, hover to preview, drag to explore)"
  - "How does this relate to the full graph dashboard (IDEA-451b29b6) vs being a per-artifact contextual view?"
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---
## Motivation

Every artifact in `.orqa/` exists in a web of relationships — epics reference milestones, tasks depend on other tasks, ideas promote to epics, decisions supersede each other. The Artifact Graph already tracks all of these connections, but the only way to explore them today is through raw frontmatter fields.

A relationships tab on the artifact detail view would make these connections visible and navigable. When looking at an epic, you'd immediately see its tasks, the milestone it belongs to, the research that informed it, and the decisions it produced. When looking at a task, you'd see what blocks it and what it depends on.

This is the per-artifact counterpart to the system-wide dashboard insights (IDEA-451b29b6) and funnel visualisation (IDEA-dc45db92). Those show the forest; this shows the tree and its immediate neighbours.

## Sketch

- New tab in the artifact detail panel alongside the existing content/metadata views
- Ego graph centred on the current artifact, showing direct references out and in
- Nodes coloured by artifact type (epic=blue, task=green, decision=purple, etc.)
- Edge labels showing the relationship field (epic, depends-on, research-refs, etc.)
- Click a connected node to navigate to that artifact
- Compact list fallback for artifacts with many connections (sorted by type, then by field)
- Data sourced from `get_references_from` and `get_references_to` graph commands — no new backend work needed