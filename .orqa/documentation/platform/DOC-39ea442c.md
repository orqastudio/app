---
id: DOC-39ea442c
type: doc
title: Prompt Pipeline Architecture
category: architecture
description: "Five-stage prompt generation pipeline: Plugin Registry, Schema Assembly, Section Resolution, Token Budgeting, and Prompt Output. Replaces monolithic CLAUDE.md loading with generated, role-specific, token-budgeted, KV-cache-aware prompts."
sort: 13
created: 2026-03-25
updated: 2026-03-25
relationships:
  - target: AD-1ef9f57c
    type: documents
    rationale: "Documents the resolved decisions on prompt generation, summary generation, budget granularity, retrieval latency"
  - target: DOC-3d8ed14e
    type: synchronised-with
    rationale: "System 4 in the Core Architecture doc covers the same pipeline from a system-level view"
  - target: DOC-586bfa9a
    type: synchronised-with
    rationale: "Knowledge injection tiers are a key input to the pipeline"
  - target: DOC-e6fb92b0
    type: synchronised-with
    rationale: "Plugin manifest contributions feed the pipeline registry"
---

# Prompt Pipeline Architecture

## Purpose

The prompt pipeline replaces OrqaStudio's monolithic "load everything" approach to system prompts with a five-stage pipeline that generates role-specific, token-budgeted, KV-cache-aware prompts from plugin registries. The goal is a 60-75% reduction in per-agent prompt size (from 9,500-16,500 tokens down to 1,500-4,000 tokens).

## Pipeline Overview

```
Plugin Registry --> Schema Assembly --> Section Resolution --> Token Budgeting --> Prompt Output
     (1)                (2)                  (3)                   (4)               (5)
```

Each stage has a single responsibility and a clear input/output contract. The pipeline is synchronous and deterministic -- given the same registry, role, stage, and task, it always produces the same prompt.

---

## Stage 1: Plugin Registry

**Module:** `libs/cli/src/lib/prompt-registry.ts`

**Purpose:** Scan installed plugins for knowledge declarations and prompt sections, build a cached lookup registry.

**When it runs:** At `orqa plugin install` or `orqa plugin refresh` time. The runtime never reads raw plugin manifests.

**Input:** Plugin directories (`plugins/`, `connectors/`, `integrations/`) containing `orqa-plugin.json` manifests.

**Output:** Cached registry at `.orqa/prompt-registry.json` containing:
- All `RegistryKnowledgeEntry` records (knowledge declarations with source metadata)
- All `RegistryPromptSection` records (prompt sections with source metadata)
- List of contributing plugins
- Any scanning errors

**Key operations:**
- `scanPluginPromptContributions(projectRoot)` -- walks plugin directories, parses manifests
- `buildPromptRegistry(projectRoot)` -- builds and writes the cached registry
- `readPromptRegistry(projectRoot)` -- reads the cached registry (used by Stage 2)

**Source classification:** Each entry is classified by its conflict resolution priority:
1. `project-rules` -- reserved for `.orqa/process/rules/` (highest priority)
2. `project-knowledge` -- reserved for `.orqa/process/knowledge/`
3. `plugin` -- most installed plugins
4. `core` -- the core framework plugin (lowest priority)

### Registry Entry Structure

```typescript
interface RegistryKnowledgeEntry {
  id: string;            // e.g. "rust-error-composition"
  plugin: string;        // Contributing plugin name
  source: KnowledgeSource;  // Conflict resolution category
  tier: "always" | "stage-triggered" | "on-demand";
  roles: string[];       // Agent roles that receive this
  stages: string[];      // Workflow stages that trigger injection
  paths: string[];       // File path globs that trigger injection
  tags: string[];        // Semantic tags for on-demand retrieval
  priority: "P0" | "P1" | "P2" | "P3";  // Token budget trim order
  summary: string | null;    // Compressed summary (100-150 tokens)
  content_file: string | null;  // Path to full content
}
```

---

## Stage 2: Schema Assembly

**Module:** `libs/cli/src/lib/prompt-pipeline.ts` (`assembleSchema` function)

**Purpose:** For a given (role, workflow-stage, task) tuple, collect all applicable prompt sections from the registry, applying conflict resolution.

**Input:** Cached `PromptRegistry` + `PromptPipelineOptions` (role, stage, task context with description/files/criteria).

**Output:** Array of `AssembledSection` records ready for content resolution.

**Knowledge entry filtering by tier:**

| Tier | Filter Logic |
|------|-------------|
| `always` | Match by role (entry's roles includes query role) OR by path (entry's path globs match task files) |
| `stage-triggered` | Must match stage (entry's stages includes query stage), optionally also match role |
| `on-demand` | Never returned by query -- counted for the preamble in Stage 5 |

**Conflict resolution:** When two entries share the same ID but come from different sources, the higher-priority source wins (project-rules > project-knowledge > plugin > core).

**Task context injection:** If task context is provided (description, files, acceptance criteria), it is formatted as a dynamic section with P1 priority using Claude XML tags:

```xml
<task-description>
Fix the validation bug in artifact_graph.rs
</task-description>

<relevant-files>
backend/src-tauri/src/domain/artifact_graph.rs
</relevant-files>

<acceptance-criteria>
1. MissingInverse validation removed
2. All tests pass
</acceptance-criteria>
```

---

## Stage 3: Section Resolution

**Module:** `libs/cli/src/lib/prompt-pipeline.ts` (`resolveSections` function)

**Purpose:** Resolve assembled sections by loading actual content from disk or using inline summaries.

**Input:** Array of `AssembledSection` records from Stage 2.

**Output:** Array of `ResolvedSection` records with content text and estimated token counts.

**Resolution priority:**
1. Content file on disk (full text) -- preferred when available
2. Inline content (compressed summary) -- fallback for always-tier entries
3. Skip with error -- when neither is available

**Cross-reference resolution:** Content text can reference other artifacts using `{{ref:ARTIFACT-ID}}` syntax. These are resolved at depth 1 (no recursive following) and replaced with placeholders. Circular references are detected and broken.

**Token estimation:** Uses the `chars / 4` approximation for token counting.

---

## Stage 4: Token Budgeting

**Module:** `libs/cli/src/lib/prompt-pipeline.ts` (`applyTokenBudget` function)

**Purpose:** Enforce the token budget by trimming low-priority sections until the prompt fits.

**Input:** Array of `ResolvedSection` records + token budget (from options or role default).

**Output:** Two arrays: `included` sections (within budget) and `trimmed` sections (cut).

**Default budgets per role:**

| Role | Budget (tokens) |
|------|----------------|
| Orchestrator | 2,500 |
| Implementer | 2,800 |
| Reviewer | 1,900 |
| Researcher | 2,100 |
| Writer | 1,800 |
| Designer | 1,800 |

**Trim algorithm:**
1. Separate P0 sections (never trimmed) from trimmable sections
2. Sort trimmable sections: P3 first, then P2, then P1; within same priority, largest first
3. Work from the highest-priority end, keeping sections that fit the remaining budget
4. Everything that does not fit is trimmed

**Priority semantics:**

| Priority | Meaning | Trim Behavior |
|----------|---------|---------------|
| P0 | Safety-critical, never cut | Immune to trimming |
| P1 | Role-critical | Trimmed only under extreme pressure |
| P2 | Task-relevant | Trimmed after P3 |
| P3 | Nice-to-have | Trimmed first |

---

## Stage 5: Prompt Output

**Module:** `libs/cli/src/lib/prompt-pipeline.ts` (`assemblePrompt` function)

**Purpose:** Assemble the final prompt with KV-cache-aware section ordering and Claude XML tags.

**Input:** Array of `ResolvedSection` records (post-budget) + role + on-demand entry count.

**Output:** Final prompt string.

**KV-cache-aware zone ordering:** Sections are sorted into zones so that static content (stable across turns) appears at the top and dynamic content (changes per task) appears at the bottom. This maximizes KV-cache prefix hits, which Manus reports as a 10x cost difference.

| Zone | Section Type | Stability |
|------|-------------|-----------|
| 0 | `role-definition` | Static (same across all tasks for a role) |
| 1 | `safety-rule` | Static |
| 2 | `constraint` | Static |
| 3 | `stage-instruction` | Semi-static (changes per workflow stage) |
| 4 | `knowledge` | Semi-static (changes per task scope) |
| 5 | `task-template` | Dynamic (changes per task) |
| 6 | `task-context` | Dynamic (unique per task) |

**Output structure:**

```xml
<role>implementer</role>

<safety-rule id="no-destructive-ops" priority="P0">
Never run destructive git operations without explicit user approval.
</safety-rule>

<constraint id="forward-only-refs" priority="P0">
Store only forward relationships. Never add inverse references.
</constraint>

<stage-instruction id="implement-standards" priority="P1">
Follow the project's coding standards for Rust and TypeScript...
</stage-instruction>

<knowledge id="rust-error-composition" priority="P1">
Use thiserror for error types, anyhow for propagation...
</knowledge>

<task-context id="__task-context__" priority="P1">
<task-description>
Fix the MissingInverse validation in artifact_graph.rs
</task-description>
</task-context>

<on-demand-knowledge>
There are 12 additional knowledge artifacts available on-demand.
To retrieve full content for a specific topic, use the semantic search tool:
  mcp__orqastudio__search_semantic with your query
</on-demand-knowledge>
```

---

## Supporting Modules

### Knowledge Retrieval (`knowledge-retrieval.ts`)

Handles on-demand knowledge access for agents at runtime.

**Key functions:**
- `generateOnDemandPreamble(count)` -- generates the instruction text appended to prompts
- `queryOnDemandEntries(registry, options)` -- filters registry for on-demand entries by tags/role
- `retrieveKnowledge(projectPath, options)` -- disk-based fallback that reads knowledge artifacts from `.orqa/process/knowledge/`, filtered by tags/role/text within a token budget
- `countOnDemandEntries(registry)` -- counts on-demand entries for the preamble

### Agent Spawner (`agent-spawner.ts`)

Creates complete agent configurations by combining the prompt pipeline with model tier selection and tool constraints.

**Key operations:**
- `createAgentConfig(params)` -- main entry point, produces an `AgentSpawnConfig` with prompt, model tier, tool constraints, and findings path
- `selectModelTier(role, complexity, overrides)` -- selects model tier (Opus for orchestrator/planner, Sonnet for others, with complexity-based upgrade for implementer)
- `serializeFindings(doc)` / `parseFindingsHeader(content)` -- structured findings format for orchestrator consumption (~200 tokens)

**Universal roles:** orchestrator, implementer, reviewer, researcher, planner, writer, designer, governance_steward

**Tool constraints per role:** Each role has declarative constraints specifying allowed tools and artifact scopes (e.g., implementer can Edit source code, researcher can Write research artifacts only).

### Token Tracker (`token-tracker.ts`)

Four-level metrics capture for token usage observability.

**Metrics storage:** `.state/token-metrics.jsonl` (newline-delimited JSON events)

**Levels:**
- Level 1 (Per-Request): input/output tokens, cache hit rate, reasoning tokens, model, latency
- Level 2 (Per-Agent): total tokens, context utilization, request count, lifetime
- Level 3 (Per-Session): total tokens, cost, agent spawns, overhead ratio
- Level 4 (Trends): computed aggregates over 7/30-day windows from historical data

**Key class:** `TokenTracker` -- stateful session tracker. Call `trackRequest()` for each API response, `trackAgentComplete()` when an agent finishes, `finalize()` when the session ends.

### Budget Enforcer (`budget-enforcer.ts`)

Stateful budget enforcement for agent spawning and session cost control.

**Budget limits (defaults):**

| Budget | Default | Enforcement |
|--------|---------|-------------|
| Per-agent prompt tokens | 4,000 | Hard block |
| Per-agent total tokens | 100,000 | Hard block at 100%, warnings at 75%/90% |
| Per-session total tokens | 500,000 | Hard block at 100%, warnings at 75%/90% |
| Per-session cost (USD) | $5.00 | Hard block at 100%, warnings at 75%/90% |

**Model tier pricing (per million tokens):**

| Tier | Input | Output | Cached |
|------|-------|--------|--------|
| Opus | $15.00 | $75.00 | $1.50 |
| Sonnet | $3.00 | $15.00 | $0.30 |
| Haiku | $0.25 | $1.25 | $0.025 |

**Key class:** `BudgetEnforcer` -- call `checkAgentSpawn()` before spawning, `recordUsage()` after each API response. Suggests model tier downgrades when budget pressure increases.

---

## Data Flow Summary

```
orqa plugin install
  |
  v
prompt-registry.ts: scanPluginPromptContributions()
  |
  v
.orqa/prompt-registry.json (cached registry)



Agent spawn request (role, stage, task)
  |
  v
agent-spawner.ts: createAgentConfig()
  |
  v
prompt-pipeline.ts: generatePrompt()
  |-- Stage 1: readPromptRegistry()
  |-- Stage 2: assembleSchema()  --> conflict resolution
  |-- Stage 3: resolveSections() --> load content from disk
  |-- Stage 4: applyTokenBudget() --> trim by priority
  |-- Stage 5: assemblePrompt()  --> KV-cache-aware output
  |
  v
AgentSpawnConfig { prompt, modelTier, toolConstraints, tokenBudget }
  |
  v
Connector spawns agent with generated prompt
  |
  v
token-tracker.ts: trackRequest() / trackAgentComplete()
budget-enforcer.ts: recordUsage() / checkAgentContinue()
```

---

## Design Decisions

These decisions are recorded in [AD-1ef9f57c](AD-1ef9f57c):

| Decision | Resolution |
|----------|-----------|
| Summary generation | Author writes summaries. `summary` field in frontmatter. `orqa summarize` CLI generates drafts. |
| On-demand retrieval latency | Acceptable. 1-2s per query paid once at task start beats 10x token cost compounding. |
| Budget enforcement granularity | Per-agent for prompt size, per-session for total cost. No team-level budgets. |
| Business logic boundary | Daemon, not MCP. Prompt generation belongs in the daemon. MCP/LSP are access protocols. |

---

## Relationship to Prior Architecture

| Aspect | Before (Monolithic) | After (Plugin-Composed Pipeline) |
|--------|--------------------|---------------------------------|
| Prompt source | `CLAUDE.md` + 58 rule files + `AGENTS.md` | Plugin registry with sections and knowledge |
| Loading strategy | Load everything at message-send time | Generate role-specific prompt at agent spawn |
| Token overhead | 9,500-16,500 per orchestrator turn | 2,000-3,500 per orchestrator turn |
| Knowledge delivery | Full rule text in system prompt | Compressed summaries + on-demand search |
| Agent specialization | Fixed agent definitions with `employs` | Universal roles + stage context + domain knowledge |
| Cache behavior | Random ordering, no cache benefit | Static-top/dynamic-bottom, KV-cache optimized |
