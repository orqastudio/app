---
id: KNOW-586bfa9a
type: knowledge
status: active
title: "Knowledge Auto-Injection Mechanisms"
description: "Two complementary injection mechanisms: declared knowledge via employs relationships and semantic search for task-relevant knowledge — how agents get domain knowledge at spawn time"
tier: always
created: 2026-03-29
roles: [orchestrator, planner, governance-steward]
paths: [engine/prompt/, engine/agent/, .orqa/documentation/]
tags: [architecture, knowledge, injection, agents, semantic-search, employs]
relationships:
  - type: synchronised-with
    target: DOC-586bfa9a
---

# Knowledge Auto-Injection Mechanisms

## Overview

Agents need domain knowledge to work correctly. OrqaStudio automatically injects knowledge through two complementary mechanisms — rather than relying on agents to self-load knowledge (which they forget) or the orchestrator to enumerate every relevant artifact in delegation prompts.

## Mechanism 1: Declared Knowledge (employs Relationships)

Every agent definition can declare `employs` relationships to knowledge artifacts. When the platform spawns an agent:

1. Read the agent definition
2. Follow all `employs` relationships
3. Resolve each target knowledge artifact
4. Load content into the agent's system context

**This gives agents their base knowledge** — foundational information every instance of that agent type needs, regardless of task.

```yaml
# In an agent definition
relationships:
  - target: KNOW-e3432947
    type: employs
    rationale: "Plugin-canonical architecture — where artifacts belong"
  - target: KNOW-57365826
    type: employs
    rationale: "Schema lookup before write"
```

**Examples:**

- Orchestrator always gets: search methodology, planning methodology, plugin-canonical architecture
- Governance Steward always gets: plugin-canonical architecture, schema lookup patterns

## Mechanism 2: Semantic Search (Task-Relevant Knowledge)

When a task is assigned, the platform runs a semantic search against all knowledge artifacts:

1. Embed the task description using ONNX embedding model
2. Search the knowledge corpus using DuckDB vector similarity
3. Rank by relevance score, take top-N results
4. Deduplicate against knowledge already loaded via Mechanism 1
5. Inject additional knowledge into agent context

**This gives agents situational knowledge** — information specific to the task at hand.

**Example:** An Implementer assigned "add a new Tauri command for artifact validation" receives:

- Via Mechanism 1: composability, search methodology, error handling patterns (always loaded)
- Via Mechanism 2: Tauri IPC patterns, validation engine architecture, command structure (task-relevant)

## How They Work Together

```text
Agent spawn request
├── Agent ID → Read agent definition
│   └── Follow employs → Load declared knowledge
│
├── Task description → ONNX semantic search
│   └── Top-N relevant → Deduplicate → Load task-relevant
│
└── Combined knowledge injected into agent context
```

Deduplication is per-session and ephemeral — does not persist across restarts.

## Adding Knowledge to Agents

**For base knowledge (always needed):** Add an `employs` relationship to the agent's YAML frontmatter.

**For situational knowledge (sometimes needed):** Ensure the knowledge artifact has a descriptive `title` and `description` — the semantic search uses these for matching. Well-described knowledge surfaces automatically for relevant tasks.

**For very specific, rare tasks:** Manual orchestrator injection in the delegation prompt.

## Choosing Between Declared and Semantic

| Agent needs this knowledge... | Use |
| ------------------------------- | ----- |
| Every time, regardless of task | Declared (`employs` relationship) |
| Only for certain task types | Semantic (good title + description) |
| Only for rare, specific tasks | Manual orchestrator injection |
