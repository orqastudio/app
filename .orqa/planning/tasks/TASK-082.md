---
id: TASK-082
title: "Migrate viewer components to SDK: frontmatter from graph, link handling"
description: Replace parseFrontmatter() calls in ArtifactViewer, AgentViewer, and SkillViewer with artifactGraph metadata lookups. Update internal link handling.
status: done
created: "2026-03-10"
updated: "2026-03-10"
epic: EPIC-048
depends-on:
  - TASK-076
assignee: AGENT-002
skills:
  - SKILL-030
  - SKILL-016
scope:
  - ui/lib/components/artifact/ArtifactViewer.svelte
  - ui/lib/components/artifact/AgentViewer.svelte
  - ui/lib/components/artifact/SkillViewer.svelte
acceptance:
  - ArtifactViewer reads metadata from artifactGraph.resolve() or resolveByPath()
  - AgentViewer and SkillViewer read metadata from graph instead of parsing frontmatter
  - Internal link click handler uses SDK-based navigation
  - parseFrontmatter() kept as fallback for files not yet in graph
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
