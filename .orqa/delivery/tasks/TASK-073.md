---
id: TASK-073
title: Build backend artifact node graph with bidirectional references
description: "Build an ArtifactGraph struct with HashMap nodes and bidirectional ArtifactRef edges, constructed during artifact_scan_tree."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-002
acceptance:
  - ArtifactGraph struct with nodes HashMap and path_index HashMap exists
  - "ArtifactNode contains id, path, artifact_type, title, status, frontmatter as JSON, references_out and references_in"
  - Graph is built during artifact_scan_tree with two-pass construction
  - "All known link fields extracted as references (milestone, epic, depends-on, pillars, research-refs, supersedes, etc)"
  - Graph stored in AppState behind a Mutex or RwLock
relationships:
  - target: EPIC-048
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-027
    type: grounded-by
  - target: SKILL-032
    type: grounded-by
  - target: SKILL-009
    type: grounded-by
  - target: TASK-074
    type: depended-on-by
  - target: TASK-079
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
