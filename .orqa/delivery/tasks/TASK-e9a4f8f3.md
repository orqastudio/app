---
id: TASK-e9a4f8f3
title: "Implement session management in the connector — state persistence, scope tracking, clean handoff"
status: completed
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-6967c7dc
    type: delivers
  - target: TASK-cd24193f
    type: depends-on
---

# TASK-e9a4f8f3: Connector Session Management

## Acceptance Criteria

1. **SessionStart hook** reads `tmp/session-state.md` and injects previous session context
2. **Stop hook** updated to prompt writing structured session state (scope, done, in progress, next, blockers)
3. **PreCompact hook** (already exists) verified to save governance context correctly
4. **Orchestrator agent** prompt includes session management protocol (read state → set scope → work → write state)
5. **Session scope** — session state includes which epic/task is in focus
6. **Clean handoff** — next session can pick up exactly where the previous one left off from session state alone
7. Must be implemented before switching to the connector plugin
