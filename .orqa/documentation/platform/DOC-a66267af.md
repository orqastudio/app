---
id: DOC-a66267af
title: "Thinking Mode: Research"
description: "The user wants something investigated, explored, or understood — information gathering with no code changes, only findings."
category: platform
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: KNOW-1a8eb147
    type: synchronised-with
---

## What This Mode Is

Research Mode is active when the user wants something investigated, explored, compared, or understood. This is pure information gathering — the agent produces a findings report, not code or changes. Research ends when the question is answered, not when a fix is applied.

The distinction matters for quality: a research agent that starts making changes has conflated understanding with execution. These are separate cognitive steps and should remain separate in the workflow.

---

## When It Activates

The orchestrator routes here when the user's request is about understanding the current state of something, not changing it.

Typical signals:
- "investigate why X happens"
- "explore options for Y"
- "compare these two approaches"
- "what does this module do"
- "audit the current state of the plugin system"
- "understand the architecture before we change it"
- "what are the tradeoffs between A and B"
- "find all the places in the codebase that do X"

---

## What the Agent Needs

Research agents rely primarily on search tooling:

- `search_semantic` — find code or artifacts by concept, not exact name
- `search_research` — end-to-end understanding of a feature area (docs + code together)
- `search_regex` — find exact function names, command names, identifiers
- `graph_query` / `graph_resolve` — navigate the artifact graph for governance context

Cross-referencing is mandatory. A finding based on a single source is stated as uncertain. The `research-methodology` knowledge artifact defines the confidence tiers (T1–T4) and structured documentation format.

---

## How It Connects to the Thinking Framework

Research Mode is often a prerequisite for other modes:

- Research findings feed **Planning Mode** — you understand the system before scoping changes
- Research findings feed **Implementation Mode** — the implementer knows where to put the code
- Research findings feed **Debugging Mode** — investigation is targeted by prior understanding

When the research reveals a pattern or anti-pattern, the output may route to **Learning Loop Mode** to capture the observation as a lesson.

---

## Governance

- RULE-005 (search over grep) applies — semantic search before file-level grep
- Research artifacts live in `.orqa/process/research/` with `type: research`
- Findings documents reference their sources using the structured sources format
