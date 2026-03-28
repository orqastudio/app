---
id: "TASK-3c586ee4"
type: "task"
title: "Enrich graph nodes with status, title, priority as first-class fields"
description: "Update ArtifactNode to expose status, title, description, priority as direct fields instead of requiring frontmatter JSON parsing."
status: archived
priority: "P1"
scoring:
  impact: 5
  urgency: 4
  complexity: 3
  dependencies: 5
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
assignee: null
acceptance:
  - "ArtifactNode Rust struct has status, title, description, priority as direct fields"
  - "Rust artifact_graph.rs node builder promotes these fields from frontmatter into first-class struct fields"
  - "TypeScript ArtifactNode type updated to match Rust struct (status, title, description, priority as direct properties)"
  - "SDK and frontend can read these fields without parsing frontmatter JSON"
  - "Existing consumers updated to use direct fields instead of frontmatter parsing"
relationships:
  - target: "EPIC-469add1c"
    type: "delivers"
    rationale: "Enriched graph nodes are the foundation for all artifact viewer improvements"
---

## Scope

Update Rust artifact_graph.rs node builder and ArtifactNode struct to promote status, title, description, and priority from frontmatter into first-class fields. Update TypeScript types in the SDK/frontend to match. Update any SDK consumers that read these fields.
