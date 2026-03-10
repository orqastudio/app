---
id: TASK-075
title: Build frontend Artifact Graph SDK with subscription API
description: "Create a typed Svelte 5 rune store at ui/lib/sdk/artifact-graph.svelte.ts wrapping the Tauri commands with synchronous in-memory lookups and a plugin subscription API."
status: todo
created: 2026-03-10
updated: 2026-03-10
epic: EPIC-048
depends-on:
  - TASK-074
assignee: frontend-engineer
skills:
  - svelte5-best-practices
  - typescript-advanced-types
  - orqa-store-patterns
scope:
  - ui/lib/sdk/
  - ui/lib/components/shared/StatusBar.svelte
  - ui/lib/components/dashboard/ProjectDashboard.svelte
acceptance:
  - "ArtifactGraphSDK class with reactive graph state"
  - "Synchronous resolve, resolveByPath, referencesFrom, referencesTo, children, byType, byStatus methods"
  - "Async readContent method that reads from disk via Tauri command"
  - "brokenRefs and orphans methods for graph health"
  - "subscribe(id, callback) and subscribeType(type, callback) with unsubscribe return"
  - "Auto-refresh on artifact-graph-updated Tauri event"
  - "TypeScript types for ArtifactNode, ArtifactRef, GraphStats exported"
  - "Status bar indexing button: triggers manual refresh, disabled with spinner while indexing"
  - "Dashboard Graph Insights card showing: node count, edge count, orphan count, broken ref count, artifacts-by-type breakdown, and status distribution"
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
