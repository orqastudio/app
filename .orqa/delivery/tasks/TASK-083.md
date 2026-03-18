---
id: TASK-083
title: "Migrate nav and linking to SDK: ArtifactLink, ArtifactNav, FrontmatterHeader, AppLayout"
description: "Replace ArtifactLink prefix routing, ArtifactNav pendingArtifactId workaround, and AppLayout watch init with SDK-based patterns."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-002
acceptance:
  - ArtifactLink uses artifactGraph.resolve(id) for navigation
  - ArtifactNav removes isTree guard — auto-select works for flat AND tree types
  - FrontmatterHeader uses SDK resolve to determine if a value is a valid artifact link
  - AppLayout watcher integration replaced with SDK auto-refresh
  - ARTIFACT_ID_RE regex in FrontmatterHeader removed — SDK determines linkability
relationships:
  - target: EPIC-048
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-076
    type: depends-on
  - target: TASK-077
    type: depended-on-by
  - target: TASK-078
    type: depended-on-by
  - target: TASK-080
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
