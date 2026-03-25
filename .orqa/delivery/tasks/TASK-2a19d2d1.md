---
id: TASK-2a19d2d1
type: task
title: "Migrate nav and linking to SDK: ArtifactLink, ArtifactNav, FrontmatterHeader, AppLayout"
description: "Replace ArtifactLink prefix routing, ArtifactNav pendingArtifactId workaround, and AppLayout watch init with SDK-based patterns."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-e5dd38e4
acceptance:
  - ArtifactLink uses artifactGraph.resolve(id) for navigation
  - ArtifactNav removes isTree guard — auto-select works for flat AND tree types
  - FrontmatterHeader uses SDK resolve to determine if a value is a valid artifact link
  - AppLayout watcher integration replaced with SDK auto-refresh
  - ARTIFACT_ID_RE regex in FrontmatterHeader removed — SDK determines linkability
relationships:
  - target: EPIC-d45b4dfd
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-7f13e4a6
    type: depends-on
  - target: TASK-25368885
    type: depended-on-by
  - target: TASK-021790b8
    type: depended-on-by
  - target: TASK-16ff9010
    type: depended-on-by
  - target: TASK-51f5c500
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.