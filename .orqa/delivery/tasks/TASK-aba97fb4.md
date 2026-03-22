---
id: TASK-aba97fb4
title: "Migrate viewer components to SDK: frontmatter from graph, link handling"
description: "Replace parseFrontmatter() calls in ArtifactViewer, AgentViewer, and SkillViewer with artifactGraph metadata lookups. Update internal link handling."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-cc255bc8
acceptance:
  - ArtifactViewer reads metadata from artifactGraph.resolve() or resolveByPath()
  - AgentViewer and SkillViewer read metadata from graph instead of parsing frontmatter
  - Internal link click handler uses SDK-based navigation
  - parseFrontmatter() kept as fallback for files not yet in graph
relationships:
  - target: EPIC-0a8a5e72
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-451dd8b1
    type: depends-on
  - target: TASK-12eec0f3
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
