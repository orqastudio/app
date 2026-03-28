---
id: KNOW-36befd20
type: knowledge
title: "Thinking Mode: Research"
thinking-mode: research
description: "The user wants something investigated, explored, or understood — information gathering with no code changes, only findings."
summary: "The user wants something investigated, explored, or understood — information gathering with no code changes, only findings."
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: DOC-36befd20
    type: synchronised-with
---

# Thinking Mode: Research

The user wants something investigated, explored, compared, or understood. This is information gathering only — the agent produces findings, not changes. No code is written.

## Example Signals

"investigate why X happens", "explore options for Y", "compare approaches", "what does this module do", "audit the current state", "understand the architecture", "what are the tradeoffs", "find all places that do X"

## What the Agent Needs

- Search tools: `search_semantic`, `search_research`, `search_regex`
- Investigation methodology (`research-methodology` knowledge)
- Artifact graph context via `graph_query` and `graph_resolve`
- Cross-reference findings before reporting — one source is not a conclusion

## Distinguishing from Similar Modes

- Not **Debugging**: no broken behaviour — the question is informational
- Not **Implementation**: agent produces a findings report, not code
- Not **Planning**: no work is being scoped — understanding comes first
