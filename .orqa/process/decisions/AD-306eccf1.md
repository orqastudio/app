---
id: "AD-306eccf1"
type: "decision"
title: "AI-Driven Cross-Artifact Search"
description: "Cross-artifact search uses AI semantic understanding rather than keyword matching. Queries are routed through the AI provider with artifact graph context, returning structured results with relevance explanations."
status: completed
created: "2026-03-11"
updated: "2026-03-11"
relationships: []
---
## Decision

Cross-artifact search in OrqaStudio is AI-driven, not keyword-based. When a user searches:

1. The query is sent to the AI provider as a structured prompt
2. The AI receives the artifact graph summary as context (types, statuses, references, dependencies)
3. The AI interprets the search intent semantically and returns structured results: artifact IDs with relevance explanations
4. Results are rendered as a navigable list with ArtifactLink chips

The search UI is a **Spotlight-style floating overlay** (Ctrl+Space) that preserves the user's current browsing context.

**Examples of queries the AI can handle:**
- "what's blocking the next milestone"
- "show me all rules about error handling"
- "which tasks depend on [EPIC-9ddef7f9](EPIC-9ddef7f9)"
- "find research that informed the streaming architecture"

## Rationale

Keyword search (Elasticsearch-style) matches text but doesn't understand intent. In a governance system where artifacts are richly connected, the most valuable searches are structural: "what depends on X", "what's blocking Y", "show me everything related to Z".

AI-driven search provides:

- **Semantic understanding** — the AI interprets what the user means, not just what they typed
- **Structural awareness** — the AI understands artifact relationships (dependencies, references, status transitions) and can answer questions about the graph
- **Flexible result presentation** — the AI structures results with explanations, not just a ranked list of matches
- **Natural language queries** — no filter syntax to learn; users describe what they want in plain language
- **Improving over time** — as AI models improve, search quality improves without code changes

This aligns with Pillar 1 (Clarity Through Structure) — search doesn't just find text matches, it helps users understand the structure of what they're looking at.

## Consequences

- Search quality depends on the AI provider's capabilities and the quality of the artifact graph context prompt
- Search has latency (AI round-trip) — the overlay should show a loading state
- The artifact graph SDK must be able to produce a concise summary suitable for an AI system prompt
- No local search index is needed for this feature (the AI is the search engine)
- Offline search is not supported (requires AI provider connectivity)

## Related Decisions

- [AD-80f39962](AD-80f39962) — Core UI boundary (search is one of the three core capabilities)
- [AD-a47f313a](AD-a47f313a) — Schema-driven filtering (complements search with structured browsing)
- [AD-e8ea9fb9](AD-e8ea9fb9) — Config-driven navigation defaults (search and browsing serve different discovery needs)