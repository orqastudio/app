---
id: TASK-80b08c75
title: Update session-start hook with uncommitted changes check
description: Update session-start hook with uncommitted changes check
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-1dab5ebe
acceptance:
  - Hook runs git status --short and counts uncommitted files
  - Warning displayed if count exceeds threshold (suggest 20)
  - Summary groups changes by directory (.orqa/
  - backend/src-tauri/
  - ui/
  - sidecars/claude-agentsdk-sidecar/)
  - Warning includes instruction to commit before starting new work
relationships:
  - target: EPIC-39860e8b
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-648a5a90
    type: depended-on-by
  - target: TASK-5e116826
    type: depended-on-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
