---
id: TASK-064
title: "Update session-start hook with uncommitted changes check"
status: done
epic: EPIC-047
created: 2026-03-09
updated: 2026-03-09
depends-on: []
assignee: orchestrator
skills: [orqa-governance]
scope:
  - .orqa/governance/hooks/session-start-hook.sh
acceptance:
  - Hook runs git status --short and counts uncommitted files
  - Warning displayed if count exceeds threshold (suggest 20)
  - Summary groups changes by directory (.orqa/, src-tauri/, ui/, sidecar/)
  - Warning includes instruction to commit before starting new work
tags: [git, hooks, enforcement]
---
