---
id: KNOW-d4c68f2e
type: knowledge
status: active
title: "Prompt Generation Pipeline (5 Stages)"
description: "Five-stage pipeline for generating agent system prompts from plugin registries and workflow state — use when building or debugging the prompt pipeline"
tier: on-demand
created: 2026-03-29
roles: [implementer, reviewer, planner]
paths: [engine/prompt/, engine/agent/]
tags: [architecture, agents, prompts, pipeline, token-budget]
relationships:
  - type: synchronised-with
    target: DOC-b951327c
---

# Prompt Generation Pipeline (5 Stages)

## Overview

System prompts are **generated programmatically** from plugin registries and workflow state — NOT loaded wholesale from disk. This is P3 (Generated, Not Loaded). The pipeline assembles only what the agent needs for its current task.

```text
Plugin Registry → Schema Assembly → Section Resolution → Token Budgeting → Prompt Output
```

## Stage Details

### Stage 1: Plugin Registry

All installed plugins register prompt contributions at install time. The registry maps: (role, workflow stage, content type) → prompt sections.

### Stage 2: Schema Assembly

For the (base role, workflow, task) tuple:

1. Identify the base role (e.g., Implementer)
2. Identify the active workflow stage (e.g., Implementation)
3. Identify the task scope (artifact types, file paths, subject matter)
4. Collect all applicable prompt sections from the registry

### Stage 3: Section Resolution

For each collected section:

1. Resolve references to compressed summaries
2. Follow cross-references (depth 1 only — no recursive resolution)
3. Compress verbose sections to summaries where token budget demands

On-demand retrieval via MCP search is the fallback for detail that exceeds the budget. Compressed summaries are the default; full text available on-demand.

### Stage 4: Token Budgeting

Measure the assembled sections against the role's token budget. If over budget, trim in priority order:

| Priority | What | Trimming |
| --------- | ------ | --------- |
| P0 | Safety constraints, critical boundaries | NEVER trim |
| P1 | Role definition, core behavioral rules | Last to trim |
| P2 | Workflow context, stage-specific instructions | Before P1 |
| P3 | Domain knowledge, examples, enrichment | First to trim |

### Stage 5: Prompt Output

Structure: static core at **TOP** (cached between turns), dynamic content at **BOTTOM** (changes per turn).

- Static core: role definition, P0/P1 content, foundational constraints
- Dynamic bottom: task-specific context, current workflow state, retrieved knowledge

This ordering maximizes prompt caching efficiency.

## Token Budgets by Role

| Role | Total Budget |
| ------ | ------------ |
| Orchestrator | 2,500 tokens |
| Implementer | 2,800 tokens |
| Reviewer | 1,900 tokens |
| Researcher | 2,100 tokens |
| Writer | 1,800 tokens |
| Planner | 2,500 tokens |
| Designer | 1,800 tokens |
| Governance Steward | 1,800 tokens |

These budgets constrain the OUTPUT of the pipeline, not the base role definitions themselves.

## Task-Specific Agent Generation

Agents are generated on a bespoke basis for each task:

```text
Base Role + Workflow Context + Domain Knowledge = Task-Specific Agent
```

- **Base Role** (from methodology plugin): permissions, boundaries, tool access
- **Workflow Context** (from active workflow): stage-specific instructions. An Implementer in Implementation gets different context than one in Documentation.
- **Domain Knowledge** (from knowledge plugins): selected at delegation time based on task scope, file paths, and subject matter

## Key Design Principle: Accuracy Over Speed

The pipeline errs on the side of providing agents what they need to be correct, even if it means more MCP lookups at runtime. On-demand retrieval latency is acceptable. Accuracy is not.

**Embedded vs on-demand decision:**

- Embed: safety constraints, role definition, active workflow rules, critical domain knowledge
- On-demand: detailed standards, large knowledge bases, historical context, rarely-needed reference
