---
id: TASK-126265d4
title: Add artifact graph Tauri commands
description: "Expose the artifact graph through Tauri commands: resolve_artifact, resolve_path, get_references_from, get_references_to, get_artifacts_by_type, read_artifact_content, get_graph_stats."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-cc255bc8
acceptance:
  - resolve_artifact command returns ArtifactNode for a given ID
  - resolve_path command returns ArtifactNode for a given file path
  - get_references_from and get_references_to return Vec of ArtifactRef
  - get_artifacts_by_type returns filtered node list
  - read_artifact_content returns raw markdown body from disk (no caching)
  - "get_graph_stats returns node count, edge count, orphan count, broken ref count"
  - All commands registered in Tauri app builder
relationships:
  - target: EPIC-0a8a5e72
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-137ec554
    type: depends-on
  - target: TASK-18eee9b0
    type: depended-on-by
  - target: TASK-12eec0f3
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
