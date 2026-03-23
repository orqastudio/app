---
id: IMPL-2ba0fc33
type: lesson
title: Graph nodes must carry display metadata — not just IDs
description: "Relationship chips, dynamic tables, action indicators, and kanban cards all need status, title, and priority from referenced artifacts. Currently this requires full node lookup or frontmatter parsing. Display metadata should be first-class on graph nodes."
status: active
created: 2026-03-14
updated: 2026-03-14
recurrence: 1
maturity: understanding
relationships:
  - target: EPIC-58ba6d53
    type: cautions
    rationale: "TASK-e9219bfd (graph node enrichment) is the direct fix"
---

## Pattern

Multiple UAT findings (F13 relationship chips, F19 dynamic tables, F18 action indicators, F29 roadmap kanban) all hit the same wall: displaying meaningful information about a referenced artifact requires looking up the full node and parsing its frontmatter. This is expensive for lists and impossible for components that only receive an ID.

The ArtifactNode struct has status, title, and description but the frontend relationship display code doesn't use them because the SDK returns ref IDs, not enriched refs.

## Fix

Ensure ArtifactNode carries status, title, description, and priority as first-class accessible fields. The SDK's relationship resolution methods should return enriched data (not just IDs) so display components can render meaningful chips without additional lookups.