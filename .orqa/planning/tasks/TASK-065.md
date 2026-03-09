---
id: TASK-065
title: "Update pre-commit-reminder hook with commit prompt"
status: done
epic: EPIC-047
created: 2026-03-09
updated: 2026-03-09
depends-on: []
assignee: orchestrator
skills: [orqa-governance]
scope:
  - .orqa/governance/hooks/pre-commit-reminder.sh
acceptance:
  - Stop hook checks for uncommitted changes
  - If uncommitted changes exist, prompts to commit before ending session
  - Suggests logical commit groupings based on changed file paths
tags: [git, hooks, enforcement]
---
