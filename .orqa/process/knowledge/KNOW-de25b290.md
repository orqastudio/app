---
id: KNOW-de25b290
type: knowledge
title: "Thinking Mode: Planning"
thinking-mode: planning
description: "The user wants work scoped, broken down, prioritised, or designed — structure and approach before any execution begins."
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: DOC-646d0876
    type: synchronised-with
---

# Thinking Mode: Planning

The user wants work scoped, broken down, prioritised, or designed. This is about structure and approach before execution. The agent produces plans and task artifacts, not code.

## Example Signals

"plan the next milestone", "break this epic into tasks", "what's the priority order", "design an approach for X", "scope out the work needed", "what should we tackle first", "map the dependencies", "how should we structure this"

## What the Agent Needs

- Artifact graph context: `graph_query`, `graph_resolve`, `graph_relationships`
- Current state of delivery items — what's in-progress, what's blocked
- Dependency mapping before task ordering
- Pillar gate questions — every task must serve at least one pillar

## Distinguishing from Similar Modes

- Not **Implementation**: no code written — plans and task artifacts only
- Not **Research**: the domain is understood; the question is how to execute
- Not **Review**: agent produces a plan, not a quality verdict
