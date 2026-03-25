---
id: TASK-2478dac2
type: task
title: "Rename tmp/ to .state/ across project"
status: captured
priority: P2
created: 2026-03-25
updated: 2026-03-25
acceptance:
  - "tmp/ directory renamed to .state/"
  - ".gitignore updated"
  - "All references in rules, hooks, scripts updated"
  - "Session state path is .state/session-state.md"
  - "Token metrics path is .state/token-metrics.jsonl"
relationships:
  - target: EPIC-c828007a
    type: delivers
---

## Scope

Rename `tmp/` to `.state/` per AD-8727f99a. Update all references across rules, hooks, scripts, and code.
