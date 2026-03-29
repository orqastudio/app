---
id: KNOW-8a5f3c1b
type: knowledge
status: active
title: "Knowledge Injection Tiers"
description: "Three injection tiers (always, stage-triggered, on-demand) — how and when knowledge artifacts are delivered to agents"
tier: always
created: 2026-03-29
roles: [orchestrator, implementer, governance-steward, writer]
paths: [engine/prompt/, .orqa/documentation/]
tags: [architecture, knowledge, injection, tiers, agents]
relationships:
  - type: synchronised-with
    target: DOC-b951327c
---

# Knowledge Injection Tiers

## The Three Tiers

Every knowledge artifact (`KNOW-*`) must declare a `tier` in its frontmatter. The tier controls when and how the knowledge is delivered to agents.

| Tier | When Delivered | Use For |
| ------ | --------------- | -------- |
| `always` | Compressed summary at every agent spawn | Foundational constraints, critical invariants, vocabulary, system boundaries |
| `stage-triggered` | When the active workflow stage matches the knowledge domain | Stage-specific process rules, review checklists, stage-level quality criteria |
| `on-demand` | Via MCP semantic search when the agent's task matches | Detailed domain knowledge, reference material, rarely-needed specifics |

## Choosing the Right Tier

### `always`

Use when an agent cannot operate correctly without this knowledge, regardless of task.

**Examples:**

- Design principles (P1-P7)
- Language boundary (Rust vs TypeScript)
- Access layer taxonomy (daemon, app, CLI, connector)
- Glossary of critical terms
- Artifact directory structure

**Rule:** If injecting this at every spawn would double the token budget, it should not be `always`. Compress the content or downgrade to `stage-triggered`.

### `stage-triggered`

Use when the knowledge is important during a specific stage of work but not universally.

**Examples:**

- Audit criteria — relevant during review stages
- State machine design — relevant during workflow implementation
- Plugin manifest format — relevant during plugin development

**Mechanism:** The prompt pipeline checks the active workflow stage and includes `stage-triggered` KNOWs that match the stage context.

### `on-demand`

Use when the knowledge is detailed, large, or only relevant for specific task types.

**Examples:**

- Migration plan (detailed phase-by-phase)
- Target state specifications
- Detailed enforcement config formats
- Historical decisions and rationale

**Mechanism:** Agents retrieve via MCP semantic search when their task description matches. The knowledge's `title` and `description` are used for matching — write them to be semantically findable.

## Required Frontmatter Fields for Knowledge Artifacts

Every `KNOW-*` artifact needs these injection metadata fields:

```yaml
tier: always | stage-triggered | on-demand
roles: [role1, role2]      # which agent roles need this
paths: [path1/, path2/]    # codebase paths this knowledge applies to
tags: [tag1, tag2]         # semantic matching tags for on-demand retrieval
```

**`roles`** — valid values: `orchestrator`, `implementer`, `reviewer`, `researcher`, `writer`, `planner`, `designer`, `governance-steward`

**`paths`** — relative paths from repo root (e.g., `engine/`, `app/src/`, `.orqa/`)

**`tags`** — semantic keywords that help the search engine match this knowledge to relevant tasks

## Token Budget Impact

The `always` tier directly impacts every agent's token budget. Content injected at `always` tier is compressed into a summary — full text available via MCP on-demand. The compression ratio should preserve the key actionable constraints while reducing size.

Target overhead ratio: 2-4x (token budget / base role definition size). The prompt pipeline enforces this by trimming P3 content first when over budget.

## Anti-Patterns

- Do NOT default all KNOWs to `always` — this inflates every agent's prompt
- Do NOT set `on-demand` for safety-critical constraints — agents may miss them
- Do NOT create KNOWs without injection metadata (`roles`, `paths`, `tags`) — they cannot be correctly targeted
- A KNOW without a tier, roles, paths, and tags is an incomplete artifact
