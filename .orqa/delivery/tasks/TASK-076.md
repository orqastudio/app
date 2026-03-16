---
id: TASK-076
title: "Migrate stores to SDK: replace artifact/navigation store ad-hoc patterns"
description: "Replace invoke('read_artifact') + viewerCache and ARTIFACT_PREFIX_MAP + pendingArtifactId in the artifact and navigation stores with Artifact Graph SDK calls."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-002
acceptance:
  - "artifact.svelte.ts uses artifactGraph.readContent() instead of invoke('read_artifact')"
  - "viewerCache removed — SDK reads from disk, no frontend caching"
  - ARTIFACT_PREFIX_MAP removed from navigation.svelte.ts
  - pendingArtifactId replaced with artifactGraph.resolve(id).path + navigateToPath()
  - navigateToPath() walks full NavTree including tree children
relationships:
  - target: EPIC-048
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-075
    type: depends-on
  - target: SKILL-030
    type: grounded-by
  - target: SKILL-016
    type: grounded-by
  - target: SKILL-015
    type: grounded-by
  - target: TASK-082
    type: depended-on-by
  - target: TASK-083
    type: depended-on-by
  - target: TASK-338
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
