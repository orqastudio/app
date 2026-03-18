---
id: TASK-189
title: Integrate enforcement with agent tool approval pipeline
description: Wire the Rust enforcement engine into the agent execution pipeline for pre-tool-use checks.
status: surpassed
created: 2026-03-11
updated: 2026-03-11
assignee: AGENT-002
docs:
  - DOC-021
acceptance:
  - Tool approval pipeline calls enforcement engine before each tool execution
  - Blocked tools return enforcement message to the agent
  - Warned tools include enforcement context in the response
  - Violations are logged to SQLite for audit trail
relationships:
  - target: EPIC-050
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-188
    type: depends-on
  - target: TASK-340
    type: depended-on-by
---

## What

Connect the enforcement engine to the app's tool approval pipeline so rules are
evaluated before every tool execution during agent conversations.

## How

1. In the tool approval flow, call the enforcement engine with tool name and input
2. If blocked: return denial with enforcement message
3. If warned: pass through with additionalContext containing the warning
4. If allowed: proceed normally
5. Log all enforcement decisions to SQLite (violation audit trail)

## Verification

- Blocked tool attempts are denied with the rule's enforcement message
- Warnings appear in the conversation context
- Violations are persisted to SQLite
