---
id: IMPL-062
title: Stale cache after disk changes — scan operations must refresh the graph first
description: "The integrity scan operates on the in-memory graph cache, not disk. If artifacts changed on disk (via CLI commits, auto-fix file writes), the scan reports stale data until the graph is manually refreshed."
status: active
created: 2026-03-14
updated: 2026-03-14
recurrence: 1
maturity: understanding
relationships:
  - target: EPIC-068
    type: cautions
    rationale: "TASK-439 (refresh before rescan) is the direct fix"
---

## Pattern

User clicks auto-fix → files modified on disk → user clicks rescan → scan runs on the old in-memory graph → stale results. The graph cache and the disk state diverged, and the scan didn't detect the divergence.

This is a broader pattern: any operation that reads the graph should ensure the graph reflects current disk state. The file watcher catches some changes but auto-fix writes bypass it because they happen inside the same process.

## Fix

Rescan must call `artifactGraphSDK.refresh()` before `runIntegrityScan()`. More broadly, any write operation that modifies `.orqa/` files should trigger a graph refresh, not rely on the file watcher catching it.
