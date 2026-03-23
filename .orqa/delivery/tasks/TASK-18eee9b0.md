---
id: TASK-18eee9b0
type: task
title: Build frontend Artifact Graph SDK with subscription API
description: Create a typed Svelte 5 rune store at ui/src/lib/sdk/artifact-graph.svelte.ts wrapping the Tauri commands with synchronous in-memory lookups and a plugin subscription API.
status: completed
created: 2026-03-10
updated: 2026-03-10
assignee: AGENT-cc255bc8
acceptance:
  - ArtifactGraphSDK class with reactive graph state
  - "Synchronous resolve, resolveByPath, referencesFrom, referencesTo, children, byType, byStatus methods"
  - Async readContent method that reads from disk via Tauri command
  - brokenRefs and orphans methods for graph health
  - "subscribe(id, callback) and subscribeType(type, callback) with unsubscribe return"
  - Auto-refresh on artifact-graph-updated Tauri event
  - "TypeScript types for ArtifactNode, ArtifactRef, GraphStats exported"
  - "Status bar indexing button: triggers manual refresh, disabled with spinner while indexing"
  - "Dashboard Graph Insights card showing: node count, edge count, orphan count, broken ref count, artifacts-by-type breakdown, and status distribution"
relationships:
  - target: EPIC-0a8a5e72
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-126265d4
    type: depends-on
  - target: TASK-451dd8b1
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