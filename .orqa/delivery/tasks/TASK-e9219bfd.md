---
id: TASK-e9219bfd
title: "Enrich graph nodes with status, title, priority as first-class fields"
description: "Update ArtifactNode to expose status, title, description, priority as direct fields instead of requiring frontmatter JSON parsing."
status: completed
priority: P1
scoring:
  impact: 5
  urgency: 4
  complexity: 3
  dependencies: 5
created: 2026-03-14
updated: 2026-03-14
assignee: null
acceptance:
  - "ArtifactNode Rust struct has status, title, description, priority as direct fields"
  - Rust artifact_graph.rs node builder promotes these fields from frontmatter into first-class struct fields
  - "TypeScript ArtifactNode type updated to match Rust struct (status, title, description, priority as direct properties)"
  - SDK and frontend can read these fields without parsing frontmatter JSON
  - Existing consumers updated to use direct fields instead of frontmatter parsing
relationships:
  - target: EPIC-58ba6d53
    type: delivers
    rationale: Enriched graph nodes are the foundation for all artifact viewer improvements
  - target: TASK-41df15c4
    type: depended-on-by
  - target: TASK-c744d86a
    type: depended-on-by
  - target: TASK-e0165bca
    type: depended-on-by
  - target: TASK-a00f73cf
    type: depended-on-by
  - target: TASK-c4de13e1
    type: depended-on-by
  - target: TASK-7fcdc743
    type: depended-on-by
---

## Scope

Update Rust artifact_graph.rs node builder and ArtifactNode struct to promote status, title, description, and priority from frontmatter into first-class fields. Update TypeScript types in the SDK/frontend to match. Update any SDK consumers that read these fields.
