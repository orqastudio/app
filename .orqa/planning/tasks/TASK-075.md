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
acceptance:
  - "ArtifactGraphSDK class with reactive graph state"
  - "Synchronous resolve, resolveByPath, referencesFrom, referencesTo, children, byType, byStatus methods"
  - "Async readContent method that reads from disk via Tauri command"
  - "brokenRefs and orphans methods for graph health"
  - "subscribe(id, callback) and subscribeType(type, callback) with unsubscribe return"
  - "Auto-refresh on artifact-graph-updated Tauri event"
  - "TypeScript types for ArtifactNode, ArtifactRef, GraphStats exported"
---
