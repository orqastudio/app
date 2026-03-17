---
id: TASK-064
title: Update session-start hook with uncommitted changes check
description: Update session-start hook with uncommitted changes check
status: completed
created: 2026-03-09
updated: 2026-03-09
assignee: AGENT-003
acceptance:
  - Hook runs git status --short and counts uncommitted files
  - Warning displayed if count exceeds threshold (suggest 20)
  - Summary groups changes by directory (.orqa/
  - backend/src-tauri/
  - ui/
  - sidecars/claude-agentsdk-sidecar/)
  - Warning includes instruction to commit before starting new work
relationships:
  - target: EPIC-047
    type: delivers
    rationale: Task belongs to this epic
  - target: SKILL-011
    type: informed-by
  - target: TASK-068
    type: depended-on-by
  - target: TASK-337
    type: depended-on-by
  - target: SKILL-011
    type: informed-by
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
