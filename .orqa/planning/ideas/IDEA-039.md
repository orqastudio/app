---
id: IDEA-039
title: Local Model Support and Democratised AI Development
description: "Explore running OrqaStudio with cheaper local models once the structured thinking foundation is mature enough, reducing dependency on expensive cloud subscriptions and democratising AI-assisted development."
status: captured
created: 2026-03-10
updated: 2026-03-10
pillars:
  - PILLAR-001
  - PILLAR-002
research-needed:
  - "Minimum model capability threshold — what reasoning quality is needed for structured artifact creation, planning, and code generation?"
  - "Context window requirements — the file-based architecture means loading the right files at the right time, not everything at once. What's the minimum context window?"
  - "Memory footprint — local models (llama.cpp, Ollama, LM Studio) vs cloud API. What hardware is required?"
  - "Provider abstraction — AD-025 already defines provider-agnostic integration. How much work to add local model backends (Ollama, llama.cpp, vLLM)?"
  - "Quality degradation mapping — which OrqaStudio capabilities degrade gracefully with smaller models vs which require frontier-class reasoning?"
  - "Hybrid approach — could a local model handle routine tasks (artifact creation, simple edits) while cloud models handle complex planning and architecture?"
  - "Skill-driven context injection — since skills load domain knowledge at the right time, does this compensate for smaller context windows?"
promoted-to: null
---
## Motivation

OrqaStudio currently requires a Claude Max Pro subscription for development — an expensive dependency that limits who can use the tool. The structured thinking foundation (artifact graph, enforced schemas, skill-driven context injection, file-based governance) creates an interesting opportunity:

**If the structure does enough of the thinking, does the model need to be as smart?**

The file-based architecture (AD-032) means governance data is flat files with enforced frontmatter. The artifact graph (EPIC-048) provides the relational layer. Skills inject the right domain knowledge at the right time. Rules enforce constraints mechanically. The model's job shifts from "figure everything out from scratch" to "follow structured patterns with the right context loaded."

This is fundamentally different from asking a local model to reason about an unstructured codebase. The structure IS the intelligence — the model is the executor.

## Sketch

### Why This Might Work

1. **File-based architecture keeps memory low** — no database queries, no embedding indexes loaded permanently. Load what you need, process it, move on.
2. **Enforced schemas reduce model burden** — the model doesn't need to invent artifact structure, it follows templates.
3. **Skill injection provides context** — instead of relying on the model's training data, we inject domain knowledge on demand.
4. **Rules provide guardrails** — mechanical enforcement catches model mistakes, reducing the need for high-quality reasoning.
5. **The artifact graph provides relationships** — the model doesn't need to discover connections, the graph tells it.

### Potential Architecture

```
Tier 1: Local model (Ollama/llama.cpp) — routine tasks, artifact CRUD, simple code generation
Tier 2: Cloud model (Claude/GPT) — complex planning, architecture decisions, multi-step reasoning
Tier 3: Embedded model (ONNX) — search, classification, embeddings (already implemented via AD-024)
```

The app could auto-route based on task complexity, or the user could choose.

### Key Question

What's the minimum viable model for each capability?

| Capability | Complexity | Minimum Model Class |
|-----------|-----------|-------------------|
| Create artifact from template | Low | 7B quantised? |
| Follow a skill's instructions | Medium | 13B? |
| Plan a multi-task epic | High | Frontier? |
| Debug cross-boundary issues | High | Frontier? |
| Apply a rule mechanically | Low | 7B quantised? |
| Write code following patterns | Medium | 13B-70B? |

This needs real testing with real governance data to validate.
