---
id: TASK-076
title: "Migrate stores to SDK: replace artifact/navigation store ad-hoc patterns"
description: "Replace invoke('read_artifact') + viewerCache and ARTIFACT_PREFIX_MAP + pendingArtifactId in the artifact and navigation stores with Artifact Graph SDK calls."
status: todo
created: 2026-03-10
updated: 2026-03-10
epic: EPIC-048
depends-on:
  - TASK-075
assignee: frontend-engineer
skills:
  - svelte5-best-practices
  - orqa-store-patterns
  - orqa-store-orchestration
scope:
  - ui/lib/stores/artifact.svelte.ts
  - ui/lib/stores/navigation.svelte.ts
acceptance:
  - "artifact.svelte.ts uses artifactGraph.readContent() instead of invoke('read_artifact')"
  - "viewerCache removed — SDK reads from disk, no frontend caching"
  - "ARTIFACT_PREFIX_MAP removed from navigation.svelte.ts"
  - "pendingArtifactId replaced with artifactGraph.resolve(id).path + navigateToPath()"
  - "navigateToPath() walks full NavTree including tree children"
---
