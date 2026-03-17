---
id: TASK-433
title: Relationships graph visualization view
description: "Add a toggle between list and graph view in the relationships panel, with a radial graph showing the focused artifact at centre."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 3
  complexity: 4
  dependencies: 1
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - Toggle between list and graph view in relationships panel
  - Graph shows focused artifact at centre with nodes radiating out grouped by edge type
relationships:
  - target: EPIC-067
    type: delivers
    rationale: Graph visualization provides spatial understanding of artifact relationships
  - target: TASK-429
    type: depends-on
---

## Scope

Create new RelationshipGraphView.svelte component that renders a radial graph of relationships. Add a view toggle in ReferencesPanel to switch between list view and graph view. Nodes radiate from the focused artifact, grouped by edge type.
