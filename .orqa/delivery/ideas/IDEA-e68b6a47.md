---
id: IDEA-e68b6a47
type: discovery-idea
title: "Daemon filesystem watching — live graph refresh on artifact changes"
description: "The daemon loads the artifact graph once at startup and never refreshes. This is a regression from the in-process graph which watched for filesystem changes. The daemon should watch .orqa/ and rebuild the graph when artifacts change."
status: captured
priority: P1
created: 2026-03-24
updated: 2026-03-24
horizon: active
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "A stale graph is invisible structure — the daemon must reflect the live state"
  - target: PERSONA-477971bf
    type: benefits
    rationale: "Practitioners need real-time validation feedback as they edit artifacts"
---

## What

The daemon (libs/validation/) serves the artifact graph from memory. It builds the graph once at startup and never updates it. When files change on disk:
- MCP serves stale graph queries
- Validation reports errors for already-fixed issues
- LSP diagnostics are wrong
- Agents waste tokens working against stale data

## Regression

The in-process graph (before daemon extraction) watched the filesystem and rebuilt on changes. The isolated daemon process lost this capability.

## Fix

Add filesystem watching to the daemon:
1. Watch `.orqa/` recursively for `.md` file changes
2. Debounce (500ms-1s)
3. Rebuild the affected portion of the graph (or full rebuild if simpler)
4. Optionally expose a `POST /refresh` endpoint for manual refresh
5. Log graph refresh events for debugging
