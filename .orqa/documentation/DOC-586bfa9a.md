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

Agents need domain knowledge to work correctly. OrqaStudio uses a **three-tier knowledge injection system** that loads the right knowledge at the right time, controlled by plugin manifests and the five-stage prompt generation pipeline. This replaces the earlier two-mechanism approach (declared `employs` relationships + semantic search) with a more structured, token-efficient model.

## The Three Injection Tiers

### Tier 1: Always (Base Knowledge)

Knowledge declared with `tier: "always"` in plugin manifests is injected at agent spawn time for matching roles and/or file paths. This provides the foundational information that specific agent roles always need.

**How it works:** The prompt pipeline's Schema Assembly stage queries the cached prompt registry for entries matching the agent's role and the task's file paths. Matching entries are included as compressed summaries (100-150 tokens each).

**Examples:**
- Implementer agents always get error handling patterns for their language domain
- All agents always get safety rules (P0 priority, never trimmed)

### Tier 2: Stage-Triggered (Workflow Context)

Knowledge declared with `tier: "stage-triggered"` is injected when the current workflow stage matches. This provides context-appropriate knowledge that changes as work progresses through stages.

**How it works:** The Schema Assembly stage checks the current workflow stage against each knowledge entry's `stages` array. Only entries whose stage matches the active workflow stage are included.

**Examples:**
- Coding standards injected during the `implement` stage
- Review checklists injected during the `review` stage
- Documentation standards injected during the `document` stage

### Tier 3: On-Demand (Deep Knowledge)

Knowledge declared with `tier: "on-demand"` is NOT pre-loaded. Instead, agents retrieve it at runtime via semantic search when they need specific details.

**How it works:** The Prompt Output stage appends a preamble instructing the agent to use `mcp__orqastudio__search_semantic` for on-demand retrieval. A disk-based fallback (`knowledge-retrieval.ts`) provides tag-based and text-based retrieval within a configurable token budget.

**Examples:**
- Specific domain patterns (Tauri IPC, store orchestration)
- Historical architecture decisions
- Detailed reference documentation

## How They Work Together

```
Agent spawn request (role, workflow-stage, task, file-paths)
|
+-- Stage 1: Plugin Registry loaded from .orqa/prompt-registry.json
|
+-- Stage 2: Schema Assembly
|   +-- always-tier: matched by role + file paths --> compressed summaries
|   +-- stage-triggered: matched by workflow stage --> section content
|   +-- on-demand: counted (not loaded) --> preamble appended
|
+-- Stage 3: Section Resolution (load content from disk, resolve cross-refs)
|
+-- Stage 4: Token Budgeting (trim P3 first, never trim P0)
|
+-- Stage 5: Prompt Output (static top, dynamic bottom, on-demand preamble)
```

### Token Budgets Per Role

| Role | Static Core | Workflow Stage | On-Demand | Total Budget |
|------|-------------|---------------|-----------|-------------|
| Orchestrator | 1,500 | 500 | 500 | 2,500 |
| Implementer | 800 | 500 | 1,500 | 2,800 |
| Reviewer | 600 | 300 | 1,000 | 1,900 |
| Researcher | 400 | 200 | 1,500 | 2,100 |
| Writer | 500 | 300 | 1,000 | 1,800 |

Compare to the pre-v2 architecture: 9,500-16,500 tokens per orchestrator turn, 6,400 tokens per agent spawn.

## Conflict Resolution

When multiple plugins provide knowledge for the same domain (same artifact ID), priority follows:

1. **Project rules** (`.orqa/process/rules/`) -- highest priority
2. **Project knowledge** (`.orqa/process/knowledge/`) -- project-specific overrides
3. **Plugin knowledge** (from installed plugins) -- domain defaults
4. **Core knowledge** (from core framework) -- universal fallbacks

## Declaring Knowledge in Plugin Manifests

### Always Tier (Role-Matched)

```json
{
  "id": "rust-error-composition",
  "tier": "always",
  "roles": ["implementer"],
  "paths": ["backend/**/*.rs"],
  "priority": "P1",
  "summary": "Use thiserror for error types, anyhow for propagation..."
}
```

### Stage-Triggered Tier

```json
{
  "id": "coding-standards",
  "tier": "stage-triggered",
  "stages": ["implement", "review"],
  "priority": "P2",
  "content_file": "knowledge/coding-standards.md"
}
```

### On-Demand Tier

```json
{
  "id": "tauri-ipc-patterns",
  "tier": "on-demand",
  "tags": ["ipc", "tauri", "commands"],
  "content_file": "knowledge/tauri-ipc.md"
}
```

## Legacy Mechanism: Declared Knowledge (employs Relationships)

The earlier `employs` relationship mechanism remains functional for the Tauri app path. Agent definitions in YAML frontmatter can still declare `employs` relationships to knowledge artifacts:

```yaml
relationships:
  - target: KNOW-e3432947
    type: employs
    rationale: "Plugin-canonical architecture"
```

The app's `SkillInjector` follows these relationships and loads content at spawn time. This path will be subsumed by the plugin-composed pipeline as the daemon integrates the TypeScript prompt generation.

## Choosing the Right Tier

| If the agent needs this knowledge... | Tier | Approach |
|--------------------------------------|------|----------|
| Every time for a specific role, regardless of task | **always** | Plugin manifest with `roles` and/or `paths` |
| Only during certain workflow stages | **stage-triggered** | Plugin manifest with `stages` |
| Only when specifically needed for a task | **on-demand** | Plugin manifest with `tags`, good title/description for search |

## Related Documents

- [KNOW-586bfa9a](KNOW-586bfa9a) -- Agent-facing knowledge pair for this documentation page
- [DOC-3d8ed14e](DOC-3d8ed14e) -- Core Application Architecture (System 4: Prompt Generation Pipeline)
- [DOC-e6fb92b0](DOC-e6fb92b0) -- Plugin Architecture (Plugin Manifest Contributions)
- [RES-d6e8ab11](RES-d6e8ab11) -- Agent Team Design v2 research (sections 5-6)
