---
id: TASK-429
title: "Enrich graph nodes with status, title, priority as first-class fields"
description: "Update ArtifactNode to expose status, title, description, priority as direct fields instead of requiring frontmatter JSON parsing."
status: completed
priority: P1
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
  - target: EPIC-067
    type: delivers
    rationale: Enriched graph nodes are the foundation for all artifact viewer improvements
  - target: TASK-432
    type: depended-on-by
  - target: TASK-433
    type: depended-on-by
  - target: TASK-437
    type: depended-on-by
  - target: TASK-438
    type: depended-on-by
  - target: TASK-448
    type: depended-on-by
  - target: TASK-456
    type: depended-on-by
---

## Scope

Update Rust artifact_graph.rs node builder and ArtifactNode struct to promote status, title, description, and priority from frontmatter into first-class fields. Update TypeScript types in the SDK/frontend to match. Update any SDK consumers that read these fields.
