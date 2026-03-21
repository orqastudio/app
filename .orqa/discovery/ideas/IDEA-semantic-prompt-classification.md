---
id: IDEA-a3f2c17e
type: idea
title: "Inference-time decision tree — self-navigating knowledge discovery"
description: "Instead of pre-injecting knowledge into agents, inject a thinking framework (decision tree) that the agent works through as part of its inference process. The agent asks itself structured questions after each prompt, self-navigates to the right knowledge articles, and only loads what it determines it needs. Lighter context, higher precision, self-documenting reasoning."
status: exploring
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: PILLAR-94b281db
    type: grounded
    rationale: "Purpose Through Continuity — ensuring the right process is applied to the right kind of work"
  - target: PILLAR-569581e0
    type: grounded
    rationale: "Clarity Through Structure — making the orchestrator's decision-making visible and structured"
  - target: PERSONA-cda6edd6
    type: benefits
    rationale: "Alex (Lead) — the orchestrator serves Alex's coordination needs"
  - target: AD-e7c4a1f3
    type: crystallises
    rationale: "This idea crystallised into the inference-time decision tree architecture decision"
  - target: EPIC-b2d9f506
    type: crystallises
    rationale: "This idea crystallised into the implementation epic"
---

## The Shift: From Pre-Injection to Self-Navigation

Current model: the `UserPromptSubmit` hook pre-injects knowledge into the agent's context based on keyword matching or semantic search. The agent receives pre-selected content whether it needs it or not.

New model: instead of injecting knowledge AT the agent, inject a **thinking framework** — a decision tree the agent works through as part of its own inference process. The agent asks itself structured questions, self-navigates to the right knowledge, and only reads what it determines it needs.

**The injected content is not knowledge — it's a reasoning protocol:**

```
After receiving this prompt, before acting, work through:

1. What kind of work is this?
   → Research: read knowledge/research-methodology
   → Implementation: read knowledge/delegation-patterns
   → Feedback: read knowledge/governance-maintenance
   → Planning: read knowledge/planning

2. What domain does it touch? (check file paths, artifact types, prompt words)
   → Frontend (.svelte, ui/): read knowledge/svelte5-best-practices
   → Backend (.rs, commands/): read knowledge/orqa-ipc-patterns
   → Governance (.orqa/): read knowledge/orqa-governance
   → Store (.svelte.ts, stores/): read knowledge/orqa-store-patterns

3. What do I already know vs what do I need to load?
   → Check session dedup — don't reload what's already in context

4. Have I checked the graph for related work?
   → graph_query for related artifacts before starting

5. What's my role boundary?
   → Orchestrator: delegate, don't implement
   → Implementer: build, don't self-certify
   → Reviewer: check, don't fix
```

The agent **self-navigates** through this tree as part of its reasoning. Knowledge files aren't pre-loaded — they're referenced as paths the agent chooses to read based on its own assessment.

## Decision Tree as a Governance Artifact

The decision tree itself is a structured artifact — editable, versionable, inspectable in the app:

```
Level 1: Intent (what kind of work?)
  ├── Research → ask: "what domain?"
  │   ├── Frontend → READ: svelte5-best-practices, orqa-frontend
  │   ├── Backend → READ: rust-async-patterns, orqa-backend
  │   └── Governance → READ: orqa-governance, artifact-relationships
  ├── Implementation → ask: "what layer?"
  │   ├── UI component → READ: component-extraction, tailwind-design-system
  │   ├── Rust command → READ: orqa-ipc-patterns, orqa-domain-services
  │   ├── Store → READ: orqa-store-patterns, orqa-store-orchestration
  │   └── Artifact → READ: orqa-governance, schema-validation
  ├── Feedback → ask: "what type?"
  │   ├── Bug → READ: diagnostic-methodology
  │   ├── Process → READ: governance-maintenance, systems-thinking
  │   └── Quality → READ: code-quality-review, test-engineering
  └── Planning → ask: "what scope?"
      ├── Epic → READ: epic-requirement-inference, planning
      ├── Task → READ: software-delivery, delegation-patterns
      └── Architecture → READ: architectural-evaluation, composability
```

Each leaf points to knowledge articles. The agent reads them **on demand** — only when its own reasoning determines they're relevant.

## Embeddings Enhancement

The decision tree questions can optionally be enhanced with semantic similarity:
- Level 1 classification can use embeddings to match the prompt against category descriptions
- Level 2 domain detection can use file context + embeddings
- But the tree works WITHOUT embeddings too — keyword heuristics or the agent's own judgment suffice

This means the system degrades gracefully — with the search server, it's precise; without it, the agent still has the structured questions to guide its reasoning.

## Plugin Extension

Plugins add branches to the tree:
- Software plugin adds implementation sub-branches (UI, Rust, Store)
- A data-science plugin could add analysis/ML branches
- Each plugin's `orqa-plugin.json` declares its decision tree branches
- Trees merge at install time like all other plugin schemas

## Why This is Better Than Pre-Injection

| Aspect | Pre-Injection | Self-Navigation |
|--------|--------------|-----------------|
| Context usage | Inject everything potentially relevant | Agent loads only what it needs |
| Precision | Hook guesses what's relevant | Agent's own reasoning determines relevance |
| Transparency | User can't see why knowledge was injected | Decision tree reasoning is visible in thinking |
| Adaptability | Static rules, need updating | Agent adapts to novel prompts |
| Token efficiency | Wastes tokens on irrelevant knowledge | Minimal — only reads what it decides to |
| Debuggability | "Why did it inject X?" | "The agent asked Q1→Q2→loaded X" |

## Value

- Lighter context window (no wasted tokens on pre-injected irrelevant knowledge)
- Higher precision (agent's own reasoning picks the right path)
- Self-documenting (decision tree visible in agent thinking/reasoning)
- Graceful degradation (works with or without embeddings server)
- Plugin-extensible (plugins add branches, not injection rules)
- Foundation for agent-specific trees (orchestrator, implementer, reviewer get different questions)

## Cross-Project Learning (Future)

The classification mode descriptions improve over time as the learning loop captures corrections. Three dimensions:

**1. Opt-in telemetry** — projects that choose to share their classification corrections contribute to improving the global starting prompt. Strictly opt-in, anonymised, user-controlled. This is about democratising AI tooling, not data extraction. Users must be able to opt out completely with zero impact on their local experience.

**2. Global prompt evolution** — across consenting projects, classification corrections aggregate into improved starting descriptions. "Users who expected learning-loop mode but got implementation when saying X" refines the learning-loop description for everyone. Same governance pipeline (observation → lesson → rule) applied cross-project.

**3. AI-driven semantic merge for upgrades** — when a new version ships with improved classification prompts, it cannot overwrite local project refinements. An AI merge process must:
- Take the new global prompt (improved from cross-project learning)
- Take the local project's refinements (lessons that corrected classifications locally)
- Merge so local learnings are preserved but global improvements are incorporated
- Flag conflicts for human review rather than silently overwriting

This is a semantic merge — understanding the intent of both versions, not just diffing text. The separate-artifacts-per-mode architecture makes this tractable: each mode description is independently mergeable.

**Privacy principle:** The system must be as useful for projects that share nothing as for those that share everything. Opt-in telemetry is an enhancement, not a requirement. Local learning always works.
