---
id: TASK-c72d3ebd
type: task
title: "Add preamble field to 5 existing agent artifacts surfaced by frontmatter validation audit"
description: "5 agent artifacts are missing the preamble frontmatter field. Add a meaningful preamble to each that describes the agent's role, constraints, and behavioral expectations."
status: captured
created: 2026-03-21
updated: 2026-03-21
acceptance:
  - All 5 identified agent artifacts have a non-empty preamble field in their frontmatter
  - Each preamble accurately describes the agent's role, delegation boundary, and key constraints
  - Preambles are concise (under 200 words each) and actionable — not generic boilerplate
  - orqa enforce passes on all 5 updated agent artifacts
  - Preamble content is consistent with the agent's existing description and acceptance criteria
relationships:
  - target: EPIC-6967c7dc
    type: delivers
---

## What

The frontmatter validation audit surfaced 5 agent artifacts missing the `preamble` field. The preamble is injected into the agent's system context at the start of each delegation — it's the primary mechanism for communicating role constraints to the agent. Without it, agents have no role-specific behavioral guidance.

## Why

Agent preambles are the behavioral contract between the orchestrator and the delegated agent. A missing preamble means the agent operates without domain-specific constraints, increasing the risk of role boundary violations (e.g., an implementer self-certifying quality, a reviewer attempting fixes).

## How

1. Run `orqa enforce` or `graph_query({ type: "agent" })` to identify the 5 agents missing preambles
2. For each agent, read its existing artifact (description, responsibilities, rules) to understand its role
3. Write a preamble that captures:
   - The agent's role in one sentence
   - What the agent may and may not do (delegation boundary)
   - Key rules that apply (reference by RULE-ID)
   - Output format expectations (what the agent returns)
4. Add the preamble field to the frontmatter
5. Run `orqa enforce` to confirm compliance

## Verification

1. `orqa enforce` reports zero missing-preamble violations for agent artifacts
2. All 5 updated agents have preambles under 200 words
3. Each preamble references at least one RULE-ID relevant to the agent's domain
4. Preamble content matches the agent's documented responsibilities