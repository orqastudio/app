---
id: TASK-b1688502
title: Redesign dashboard widgets for actionable artifact insight
description: Merge artifact graph and governance widgets into per-type cards with count and status breakdown. Exclude documentation from orphan detection. Auto-scan integrity on navigation. Replace collapsible categories with sortable/filterable data table.
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 2
created: 2026-03-13
updated: 2026-03-13
assignee: null
acceptance:
  - Documentation artifacts excluded from orphan count in artifact graph
  - Single combined widget shows per-artifact-type cards with total count and status breakdown
  - Integrity scan runs automatically on dashboard navigation — scan button is for manual refresh only
  - Error and warning counts exclude documentation artifacts
  - "Integrity findings displayed as a sortable data table with columns for category, artifact, severity, message"
  - Table supports filtering by category and sorting by any column
relationships:
  - target: EPIC-fd22ca6c
    type: delivers
    rationale: Theme B — dashboard widget redesign from UAT
---

## Scope

### Exclude Documentation Orphans (Findings #1, #4)
- **Root cause**: `artifact_graph.rs` (lines 324-328) counts all nodes without edges as orphans, including `type: "doc"`
- **Fix**: Filter out `"doc"` type nodes from orphan count. Also filter from integrity checks that don't apply to documentation.

### Merge Widgets (Finding #2)
- **Current**: Separate "Artifact Graph" stats card and "Governance Artifacts" button grid
- **Target**: Single widget showing one card per artifact type with count + status breakdown (e.g., "Rules: 45 total — 42 active, 3 inactive")

### Auto-Scan (Finding #3)
- **Root cause**: `IntegrityWidget.svelte` has no `$effect` for auto-scan on mount
- **Fix**: Add `$effect` like `HealthTrendWidget.svelte` already does (lines 13-17)

### Data Table (Findings #5, #6)
- **Current**: Collapsible category sections with lazy-expand
- **Target**: Sortable data table with columns: severity, category, artifact ID, message
- **Filter**: By category, severity. Sort: by any column.
