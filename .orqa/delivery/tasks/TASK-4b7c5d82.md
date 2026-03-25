---
id: TASK-4b7c5d82
type: task
title: "Migrate viewer components to SDK: frontmatter from graph, link handling"
description: "Replace parseFrontmatter() calls in ArtifactViewer, AgentViewer, and SkillViewer with artifactGraph metadata lookups. Update internal link handling."
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-e5dd38e4
acceptance:
  - ArtifactViewer reads metadata from artifactGraph.resolve() or resolveByPath()
  - AgentViewer and SkillViewer read metadata from graph instead of parsing frontmatter
  - Internal link click handler uses SDK-based navigation
  - parseFrontmatter() kept as fallback for files not yet in graph
relationships:
  - target: EPIC-d45b4dfd
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-7f13e4a6
    type: depends-on
  - target: TASK-51f5c500
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.