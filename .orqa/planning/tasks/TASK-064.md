---
id: TASK-064
title: Update session-start hook with uncommitted changes check
description: Update session-start hook with uncommitted changes check
status: done
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-047
depends-on: []
assignee: AGENT-003
skills:
  - SKILL-011
acceptance:
  - Hook runs git status --short and counts uncommitted files
  - Warning displayed if count exceeds threshold (suggest 20)
  - Summary groups changes by directory (.orqa/
  - src-tauri/
  - ui/
  - sidecar/)
  - Warning includes instruction to commit before starting new work
---

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
