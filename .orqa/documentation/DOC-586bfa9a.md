---
id: DOC-586bfa9a
type: doc
title: Knowledge Auto-Injection
description: "How OrqaStudio automatically injects relevant knowledge into agents: declared injection from agent employs relationships, semantic search for task-relevant knowledge, deduplication, and hook-based enforcement."
category: architecture
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: KNOW-586bfa9a
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
---

# Knowledge Auto-Injection

## Overview

Agents need domain knowledge to work correctly. Rather than relying on agents to manually load knowledge (which they forget) or the orchestrator to enumerate every relevant artifact in delegation prompts (which is error-prone), OrqaStudio **automatically injects knowledge** into agents at spawn time through two complementary mechanisms.

## The Two Mechanisms

### Mechanism 1: Declared Knowledge (employs Relationships)

Every agent definition in YAML frontmatter can declare `employs` relationships to knowledge artifacts:

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

When the platform spawns an agent, it reads the agent definition, follows all `employs` relationships, resolves each target knowledge artifact, and loads the content into the agent's system context.

**This gives agents their base knowledge** — the foundational information every instance of that agent type needs, regardless of the specific task.

**Examples of declared knowledge:**
- Orchestrator always gets: search methodology, planning methodology, plugin-canonical architecture
- Governance Steward always gets: plugin-canonical architecture, schema lookup patterns
- Svelte Specialist always gets: Svelte 5 patterns, component extraction methodology

### Mechanism 2: Semantic Search (Task-Relevant Knowledge)

When an agent is spawned with a task description, the platform runs a semantic search to find knowledge artifacts relevant to the specific task:

1. **Embed the task description** using the ONNX embedding model
2. **Search the knowledge corpus** using DuckDB vector similarity
3. **Rank by relevance score** and take the top-N results
4. **Deduplicate** against knowledge already loaded via Mechanism 1
5. **Inject** the additional knowledge into the agent's context

**This gives agents situational knowledge** — information specific to the task at hand that the agent's base knowledge doesn't cover.

**Example:** A Rust Specialist assigned "add a new Tauri command for artifact validation" would receive:
- **Via Mechanism 1:** Composability, search methodology, error handling patterns (always loaded)
- **Via Mechanism 2:** Tauri IPC patterns, validation engine architecture, command structure (task-relevant)

## How They Work Together

```
Agent spawn request
├── Agent ID → Read agent definition
│   └── Follow employs relationships → Load declared knowledge
│
├── Task description → Semantic search
│   └── Top-N relevant knowledge → Deduplicate → Load task-relevant knowledge
│
└── Combined knowledge injected into agent context
```

### Deduplication

If a knowledge artifact is found by both mechanisms (declared AND semantically relevant), it is only loaded once. The dedup cache is per-session and ephemeral — it does not persist across restarts.

## Enforcement

Knowledge injection is enforced through hooks in the connector plugin:

| Hook | Event | Purpose |
|------|-------|---------|
| Agent spawn hook | Agent created | Reads agent definition, follows `employs`, loads declared knowledge |
| Task delegation hook | Task assigned to agent | Runs semantic search on task description, injects relevant knowledge |
| Dedup cache | Per session | Prevents duplicate injection across mechanisms |

Without these hooks, agents would need to self-load knowledge (unreliable) or the orchestrator would need to manually include all relevant knowledge in every delegation prompt (error-prone and clutters the orchestrator's context).

## Adding Knowledge to Agents

### For Base Knowledge (Always Needed)

Add an `employs` relationship to the agent's YAML frontmatter:

```yaml
relationships:
  - target: KNOW-<id>
    type: employs
    rationale: "Why this agent always needs this knowledge"
```

### For Situational Knowledge (Sometimes Needed)

Ensure the knowledge artifact has a descriptive `title` and `description` in its frontmatter. The semantic search uses these fields for matching. Well-described knowledge surfaces automatically for relevant tasks — no `employs` relationship needed.

### Choosing Between Declared and Semantic

| If the agent needs this knowledge... | Use |
|--------------------------------------|-----|
| Every time, regardless of task | Declared (`employs` relationship) |
| Only for certain types of tasks | Semantic (good title + description) |
| Only for very specific, rare tasks | Manual orchestrator injection in delegation prompt |

## Related Documents

- [KNOW-586bfa9a](KNOW-586bfa9a) — Agent-facing knowledge pair for this documentation page
- [DOC-8d2e5eef](DOC-8d2e5eef) — Agent Team Structure (which agents have which declared knowledge)
- [DOC-7068f40a](DOC-7068f40a) — Documentation Placement Guide (where to write knowledge artifacts)
