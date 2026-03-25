---
id: KNOW-c89f28b3
type: knowledge
title: "Thinking Mode: Governance"
thinking-mode: governance
priority-rules: [process, planning]
description: "The user wants rules created, enforced, audited, or maintained — governance structure and pipeline integrity, not feature work."
status: active
created: 2026-03-23
updated: 2026-03-23
relationships: []
---

# Thinking Mode: Governance

The user wants governance artifacts created, updated, enforced, or audited. This includes rules, knowledge, lessons, enforcement pipelines, and process integrity. The agent acts as a Governance Steward — writing to `.orqa/process/` rather than source code.

## Example Signals

"create a rule for X", "enforce Y", "audit the governance pipeline", "promote this lesson", "update the enforcement chain", "add a knowledge artifact", "check rule compliance", "fix the enforcement gap"

## What the Agent Needs

- Full artifact graph context — rules, lessons, knowledge, enforcement entries
- Understanding of enforcement mechanisms: validators, hooks, prompt injection, knowledge injection
- Lesson promotion pipeline: observation → lesson → rule → enforcement
- Graph integrity tools: `graph_validate`, `graph_relationships`

## Distinguishing from Similar Modes

- Not **Implementation**: governance writes to `.orqa/`, not to source code
- Not **Review**: governance creates or updates constraints, not verdicts on existing work
- Not **Learning Loop**: learning loop captures one observation; governance builds the full enforcement pipeline
