---
id: IDEA-c40febb6
title: "Artifact graph as agent memory — .orqa/ replaces separate memory systems"
description: >
  AI agents built with the Agent SDK could use the .orqa/ artifact graph
  as their memory system instead of a separate memory layer. The governance
  artifacts (decisions, rules, lessons, skills) already capture structured
  knowledge in a way that's visible, queryable, and versioned. Agent memory
  becomes thinking made visible — not a hidden side-channel.
status: captured
created: 2026-03-17
horizon: someday
relationships:
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

## Motivation

The Claude Agent SDK provides a Memory Tool where you implement your own storage backend. Claude Code has its own auto-memory system with MEMORY.md indexes. Both are separate from the actual project knowledge.

OrqaStudio's `.orqa/` artifact graph already IS a structured memory system:
- **Decisions** = what the agent decided and why
- **Rules** = learned constraints on behaviour
- **Lessons** = patterns extracted from experience
- **Skills** = how-to knowledge for specific tasks
- **Relationships** = the connections between all of these

Instead of agents maintaining a parallel memory layer, they could read and write governance artifacts directly. An agent's "memory" of a past decision becomes an actual decision artifact. A learned pattern becomes a lesson. A repeated procedure becomes a skill.

## Research Needed

- How would an agent distinguish between "my working memory" and "project governance"?
- Should agents create draft artifacts that humans promote to governance status?
- How does agent memory interact with the integrity system — do agent-created artifacts need validation?
- What's the interface between the Agent SDK Memory Tool and the .orqa/ filesystem?
- How would multiple agents share knowledge through the artifact graph?
