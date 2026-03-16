---
id: TASK-041
title: Memory leak investigation and fix
description: Identifies and resolves memory growth during extended artifact browsing sessions by cleaning up orphaned DOM event listeners and adding an eviction policy to the viewer cache.
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-002
acceptance:
  - MarkdownRenderer cleans up event listeners on content change and unmount
  - Artifact viewer cache has an eviction policy (e.g.
  - LRU with max size)
  - No observable memory growth after navigating 50+ artifacts in sequence
  - No crash or freeze during extended artifact browsing sessions
relationships:
  - target: EPIC-043
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-005
    type: grounded-by
  - target: SKILL-008
    type: grounded-by
  - target: TASK-333
    type: depended-on-by
---
## Findings Addressed

- **F3**: Crash/freeze after navigating many artifacts (possible memory leak)

## Investigation Notes

Two candidates identified:

1. **MarkdownRenderer.svelte (HIGH risk)**: `processArtifactLinks()` adds click listeners via `btn.addEventListener("click", ...)` every time content changes (called via `$effect`). No cleanup mechanism — old listeners remain attached to DOM nodes that may be replaced. Rapid artifact navigation accumulates orphaned listeners.

2. **artifactStore.viewerCache (MEDIUM risk)**: `Map<string, string>` cache with no eviction policy. Grows unbounded as more artifacts are viewed. Each entry contains full markdown content.

3. **navigationStore.pendingArtifactId (LOW risk)**: Could remain set if navigation fails, but unlikely to cause memory issues.

## Priority

This is a stability bug — should be fixed before UX improvements. Memory leaks in a dogfood tool used for extended sessions are blocking.

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
