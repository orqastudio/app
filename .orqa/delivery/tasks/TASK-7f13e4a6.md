---
id: "TASK-7f13e4a6"
type: "task"
title: "Migrate stores to SDK: replace artifact/navigation store ad-hoc patterns"
description: "Replace invoke('read_artifact') + viewerCache and ARTIFACT_PREFIX_MAP + pendingArtifactId in the artifact and navigation stores with Artifact Graph SDK calls."
status: archived
created: 2026-03-10T00:00:00.000Z
updated: 2026-03-10T00:00:00.000Z
acceptance:
  - "artifact.svelte.ts uses artifactGraph.readContent() instead of invoke('read_artifact')"
  - "viewerCache removed — SDK reads from disk, no frontend caching"
  - "ARTIFACT_PREFIX_MAP removed from navigation.svelte.ts"
  - "pendingArtifactId replaced with artifactGraph.resolve(id).path + navigateToPath()"
  - "navigateToPath() walks full NavTree including tree children"
relationships:
  - target: "EPIC-d45b4dfd"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-27d91d91"
    type: "depends-on"
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
