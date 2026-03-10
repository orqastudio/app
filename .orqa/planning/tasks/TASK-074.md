---
id: TASK-074
title: Add artifact graph Tauri commands
description: "Expose the artifact graph through Tauri commands: resolve_artifact, resolve_path, get_references_from, get_references_to, get_artifacts_by_type, read_artifact_content, get_graph_stats."
status: todo
created: 2026-03-10
updated: 2026-03-10
epic: EPIC-048
depends-on:
  - TASK-073
assignee: backend-engineer
skills:
  - rust-async-patterns
  - tauri-v2
  - orqa-ipc-patterns
scope:
  - src-tauri/src/commands/
acceptance:
  - "resolve_artifact command returns ArtifactNode for a given ID"
  - "resolve_path command returns ArtifactNode for a given file path"
  - "get_references_from and get_references_to return Vec of ArtifactRef"
  - "get_artifacts_by_type returns filtered node list"
  - "read_artifact_content returns raw markdown body from disk (no caching)"
  - "get_graph_stats returns node count, edge count, orphan count, broken ref count"
  - "All commands registered in Tauri app builder"
---
