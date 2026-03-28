---
id: IDEA-f6fed08e
type: discovery-idea
title: "Decision Pros/Cons Slash Command"
description: "A Claude Code connector slash command (e.g. /pending-decisions) that queries the artifact graph for decisions in exploring or review status, then presents each with a structured pros/cons analysis to help the user make informed choices."
status: captured
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

# IDEA-144: Decision Pros/Cons Slash Command

A connector slash command that surfaces pending decisions and structures the trade-off analysis the user needs to make a choice.

## Behaviour

1. Run `graph_query({ type: "decision", status: "exploring" })` and `graph_query({ type: "decision", status: "review" })` to find open decisions
2. For each decision, present:
   - **Context** — what the decision is about
   - **Options** — the alternatives under consideration
   - **Pros** — benefits of each option
   - **Cons** — costs or risks of each option
   - **Recommendation** (if any evidence points one way)
3. Ask the user to choose or defer

## Candidate Items

- `/pending-decisions` command in the connector plugin
- Query pattern: `graph_query({ type: "decision" })` filtered to open statuses
- Output template for pros/cons per option
- Follow-up: update decision status to `accepted` or `superseded` based on user choice
