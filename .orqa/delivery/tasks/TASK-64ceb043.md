---
id: TASK-64ceb043
title: "Migrate nav and linking to SDK: ArtifactLink, ArtifactNav, FrontmatterHeader, AppLayout"
description: "Replace ArtifactLink prefix routing, ArtifactNav pendingArtifactId workaround, and AppLayout watch init with SDK-based patterns."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-cc255bc8
acceptance:
  - ArtifactLink uses artifactGraph.resolve(id) for navigation
  - ArtifactNav removes isTree guard — auto-select works for flat AND tree types
  - FrontmatterHeader uses SDK resolve to determine if a value is a valid artifact link
  - AppLayout watcher integration replaced with SDK auto-refresh
  - ARTIFACT_ID_RE regex in FrontmatterHeader removed — SDK determines linkability
relationships:
  - target: EPIC-0a8a5e72
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-451dd8b1
    type: depends-on
  - target: TASK-ff295517
    type: depended-on-by
  - target: TASK-2b47b899
    type: depended-on-by
  - target: TASK-db618792
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
