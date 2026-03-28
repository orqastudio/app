---
id: TASK-e485da72
type: task
title: "Remove dead Cytoscape graph health analysis code"
description: "The Rust backend (compute_graph_health) is the live graph health data source — the epic is surpassed. Remove the dead Cytoscape SDK computeGraphHealth code path and any unused Cytoscape analysis utilities that are no longer called."
status: archived
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 2
  dependencies: 1
created: 2026-03-24
updated: 2026-03-24
horizon: active
acceptance:
  - "Dead Cytoscape SDK computeGraphHealth code path removed"
  - "Any unused Cytoscape analysis utilities (that were only called by the dead path) removed"
  - "Frontend confirmed to use only the Rust compute_graph_health backend"
  - "No regressions — graph health widget still displays correctly"
relationships:
  - target: EPIC-ff7db83e
    type: delivers
---

## What

The Rust backend `compute_graph_health` command is the live graph health data source. The Cytoscape SDK `computeGraphHealth` function is dead code — the epic has been marked surpassed because the Rust backend replaced the Cytoscape analysis approach. This task removes the dead Cytoscape analysis code path and any associated unused utilities.

## Scope

1. Remove `computeGraphHealth` from the Cytoscape-based artifact graph SDK
2. Remove any Cytoscape analysis utility functions that were only used by the dead path
3. Verify the frontend graph health widget still works correctly against the Rust backend
4. Confirm no other consumers depend on the removed code
