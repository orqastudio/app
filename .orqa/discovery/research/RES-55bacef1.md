---
id: RES-55bacef1
title: "Agent Team Design v2: Plugin-Composed Workflows, Token-Efficient Prompts, and Deterministic State Machines"
type: research
status: active
category: architecture
description: >
  Comprehensive synthesis of seven parallel research streams investigating how OrqaStudio
  should architect its multi-agent team system. Covers workflow composition from plugins,
  agent type specialization, knowledge plugin architecture, programmatic prompt generation,
  deterministic state machines with human gates, and token efficiency as an architectural
  constraint. Supersedes patch-level solutions from prior research with a unified
  plugin-composed architecture.
created: 2026-03-25
updated: 2026-03-25
relationships:
  - target: RES-97e3aa4b
    type: evolves
    rationale: "Supersedes the persistent dev team model with workflow-first architecture"
  - target: RES-2f602d54
    type: related
    rationale: "Incorporates token efficiency findings into architectural design"
  - target: TASK-8870f959
    type: related
    rationale: "Research produced for this task"
  - target: EPIC-4304bdcc
    type: related
    rationale: "Part of the stabilisation epic"
  - target: AD-e9f71dc1
    type: informs
    rationale: "Open questions in section 11 resolved as architecture decisions"
  - target: AD-8727f99a
    type: informs
    rationale: "tmp→.state rename decision arose from comment on section 8"
tags:
  - multi-agent
  - plugin-architecture
  - token-efficiency
  - workflow-composition
  - state-machines
  - knowledge-injection
  - prompt-generation
---

# Agent Team Design v2

## 1. Executive Summary

OrqaStudio's current multi-agent system works but does not scale. The orchestrator's context window carries 48K+ tokens of rules per prompt. Agent teams spawn with 63K tokens of shared context. The overhead ratio sits at 13.4x the theoretical minimum. These are not tuning problems; they are architectural ones.

This research synthesizes seven parallel investigations into a unified architecture that addresses the root causes. The central insight is that **workflow structure, agent specialization, knowledge injection, prompt generation, state machines, and token efficiency are not separate concerns** -- they are facets of a single design: plugin-composed workflows that generate precisely-scoped agent contexts at delegation time.

### Key Findings

**Workflow composition** should follow a contribution-point model: the core framework defines a workflow skeleton with named slots, and stage plugins fill those slots via declarative manifests. The resolved workflow is a YAML file on disk -- deterministic, diffable, and inspectable. This replaces the current monolithic CLAUDE.md approach.

**Agent specialization** follows a three-layer taxonomy: `Universal Role + Stage Context + Domain Knowledge = Effective Agent Type`. Universal roles (Implementer, Reviewer, etc.) come from the core framework. Stage context comes from the active workflow stage. Domain knowledge is composed from knowledge plugins at delegation time. This eliminates the need for fixed specialist agent definitions. (*COMMENT* Note that these universal roles should come with tool constraints - research needs web-search but should only be creating/editing research artifacts etc.)

**Knowledge architecture** uses a hybrid manifest + capability matching system. Knowledge plugins declare their artifacts in structured manifests with injection tiers (always, stage-triggered, on-demand). Conflict resolution follows a strict priority: project rules > project knowledge > plugin knowledge > core knowledge. Individual knowledge artifacts are atomic and self-contained at 500-2000 tokens each.

**Programmatic prompt generation** replaces the current "load everything" approach with a five-stage pipeline: Plugin Registry, Schema Assembly, Section Resolution, Token Budgeting, and Prompt Output. Expected per-agent prompt size drops from 9,500-16,500 tokens to 1,500-4,000 tokens -- a 60-75% reduction.

**State machines** are plugin-owned, deterministic, and YAML-defined. The core framework provides a state machine engine with guard primitives, action primitives, and state categories. Plugins define complete state machines for their artifact types. Human gates are first-class sub-workflows with a gather-review-decide-learn pipeline. Ad-hoc work uses workflow variants with selection rules.

**Token efficiency** is an architectural constraint, not an optimization pass. The architecture achieves 60-75% token reduction through compiled prompts, role-specific context bundles, on-demand knowledge retrieval, model tiering, and KV-cache-aware prompt structure. Session costs drop from ~300K+ tokens to ~80-120K tokens.

### What Changes from Prior Research

RES-97e3aa4b's composability model (skills, knowledge, rules as composable units) carries forward as the foundation. RES-2f602d54's token baselines and measurement methodology carry forward as the efficiency targets. What is discarded: the persistent agent model, the fixed team of 5, SendMessage-based handoffs, session-long agent lifecycles, and patch-level solutions like lazy rule loading. These are replaced by an architectural approach where efficiency is built into the system design rather than bolted on afterward.

---

## 2. Design Principles

Seven principles emerged consistently across all research streams. These are not aspirational -- they are load-bearing constraints that the architecture must satisfy.

### P1: Plugin-Composed Everything

No governance pattern is hardcoded in the core framework. Workflow stages, artifact types, state machines, knowledge artifacts, and agent specializations all come from plugins. The core framework provides engines, not definitions. This makes OrqaStudio adaptable to any domain without forking.

### P2: One Context Window Per Task

Each agent spawns fresh for a single task with a precisely-scoped prompt. No persistent agents, no session-long lifecycles, no accumulated context. This eliminates stale agents, context drift, shutdown discipline problems, and the "context rot" documented in recent research (longer context degrades model reasoning quality, not just cost).

### P3: Generated, Not Loaded

System prompts are generated programmatically from plugin registries and workflow state -- not loaded wholesale from disk. The prompt pipeline assembles only what the agent needs for its current task. Full rule text is available on-demand via semantic search; compressed summaries are the default.

### P4: Declarative Over Imperative

State machines, guards, actions, and workflow definitions are YAML declarations validated by JSON Schema -- not code. Plugin authors write configuration, not functions. This keeps state machines portable, diffable, and inspectable. Guard logic uses declarative primitives (field checks, relationship checks, graph queries), not arbitrary functions.

### P5: Token Efficiency as Architecture

Token efficiency is not an optimization pass applied after the architecture is designed. It is a first-class architectural constraint that shapes every decision: prompt structure, knowledge loading strategy, agent lifecycle, model selection, and cache behavior. The target is a 2-4x overhead ratio, down from the current 13.4x.

### P6: Hub-Spoke Orchestration

A persistent orchestrator coordinates ephemeral task-scoped workers. This is validated by Google's ADK research and Microsoft's AutoGen patterns. The orchestrator maintains high-level plan memory; workers maintain granular sub-task memory. The orchestrator never sees worker-level implementation details -- it reads structured summaries from findings files.

### P7: The Resolved Workflow Is a File on Disk

After plugin contributions are merged, the resolved workflow must be a YAML file on disk -- deterministic, diffable, and inspectable. The runtime reads this resolved file; it does not re-merge contributions on every evaluation. This makes debugging straightforward: read the file, see the workflow.

---

## 3. Workflow Composition Architecture

### The Composition Model

The core framework defines a workflow skeleton with named contribution points (slots). Stage plugins fill those slots via declarative manifests. The composition happens at install time, not runtime.

```
Core Workflow Skeleton (YAML)
  |
  +-- contribution-point: "planning"
  |     filled by: software-discovery plugin
  |
  +-- contribution-point: "implementation"
  |     filled by: software-kanban plugin
  |
  +-- contribution-point: "review"
  |     filled by: software-kanban plugin (review sub-workflow)
  |
  +-- contribution-point: "learning"
        filled by: governance plugin (lesson pipeline)
```

### Composition Format: Hybrid YAML + Code Hooks

The workflow definition is YAML (structure, stages, transitions, guards). Gate logic that requires computation runs as code hooks registered by plugins. This matches OrqaStudio's existing artifact model where YAML frontmatter defines structure and markdown body defines content.

**YAML handles:** Stage ordering, transition definitions, guard declarations (field checks, relationship checks, graph queries), action declarations (set field, append log, create artifact), human gate definitions, UI metadata (categories, icons, colors).

**Code hooks handle:** Complex guard evaluation (multi-artifact queries with business logic), side effects beyond simple field updates, external integrations (notifications, CI triggers), custom validation that cannot be expressed declaratively.

### Contribution Merging

At `orqa install` time, the resolver: (*COMMENT* should be part of the `orqa plugin install` sub-process that `orqa install` triggers. It's the plugins that drive this, and need to be triggered when plugins change)

1. Reads the core workflow skeleton
2. Reads each plugin's contribution manifest
3. Merges contributions into slots, respecting priority order
4. Validates the merged result against the workflow schema
5. Writes the resolved workflow to `.orqa/workflows/<name>.resolved.yaml`

The runtime reads only the resolved file. Re-resolution happens only on plugin install/update.

### Workflow Templates with Overrides

Plugins can provide workflow templates that project owners override for their specific needs. The override mechanism uses the same YAML merge pattern: the project-level workflow file overrides specific sections of the plugin-provided template while inheriting everything else.

### Stage Granularity

Research recommends medium-grained stages (5-8 per workflow) as the optimal balance between flexibility and complexity. Too few stages (3) force plugins to overload each stage. Too many stages (15+) create navigation overhead without adding value.

Recommended stages for a software development workflow: `discover`, `plan`, `document`, `implement`, `review`, `learn`. Each stage has a clear owner (plugin) and clear boundaries (what enters, what exits). (*Comment* we need our plugins to match this, but we also need these workflow stages to be configurable per domain - software development is just one domain. We should have plugins that define the workflow and then plugins that depend on that plugin that define each stage - composability!).

---

## 4. Agent Architecture

### The Three-Layer Taxonomy

```
Universal Role + Stage Context + Domain Knowledge = Effective Agent Type
```

**Layer 1 -- Universal Roles (core framework):** Orchestrator, Implementer, Reviewer, Researcher, Planner, Writer, Designer, Governance Steward. These define behavioral boundaries (what the agent may and may not do) and capability sets (which tools it can use). They are stable across all projects and all domains.

**Layer 2 -- Stage Context (stage plugins):** When a universal role operates within a specific workflow stage, the stage plugin provides stage-specific instructions. An Implementer in the "implement" stage gets coding standards, architecture decisions, and error handling patterns. An Implementer in the "document" stage gets documentation standards and pillar alignment requirements. Same role, different context.

**Layer 3 -- Domain Knowledge (knowledge plugins):** Domain expertise is composed into agents at delegation time based on what the task requires. A backend Implementer gets Rust patterns, Tauri IPC knowledge, and error composition knowledge. A frontend Implementer gets Svelte 5 patterns, store orchestration knowledge, and component extraction knowledge. The orchestrator determines which knowledge to inject based on the task's file paths and subject matter.

### Why Not Fixed Specialist Agents?

Research across CrewAI, AutoGen, MetaGPT, ChatDev, LangGraph, and CAMEL-AI consistently shows that fixed specialist agents create coordination overhead that outweighs their specialization benefit. Google's research found that multi-agent systems degrade 39-70% on sequential tasks compared to single-agent systems. The benefit of specialists appears only on parallelizable tasks where agents work independently.

OrqaStudio's current pattern (hub-spoke with ephemeral workers) is already validated by this research. The improvement is composing agent context from plugins rather than hardcoding it in agent definitions.

### Generalist + Knowledge Injection Model

The recommended model: spawn a universal role (e.g., Implementer), inject stage context from the active workflow stage, inject domain knowledge based on the task scope, and let the agent work. The agent is a generalist specialized at spawn time, not a permanent specialist.

This eliminates: maintaining separate agent definitions per domain, knowledge drift when agent definitions fall out of sync with knowledge artifacts, the "wrong specialist" problem where the orchestrator must choose between overlapping specialists.

### Model Tiering

Not all agents need the most capable model. Research consistently shows 60-80% cost reduction through intelligent model routing.

| Role | Recommended Model | Rationale |
|------|------------------|-----------|
| Orchestrator | Opus | Strongest reasoning for delegation decisions |
| Planner | Opus | Architecture planning requires deep reasoning |
| Implementer (complex) | Opus | Multi-file refactoring needs full capability |
| Implementer (simple) | Sonnet | Straightforward single-file changes |
| Reviewer | Sonnet | Pattern matching and checklist verification |
| Researcher | Sonnet | Accuracy in reading, not creativity |
| Writer | Sonnet | Documentation follows templates |
| Designer | Sonnet | Component creation follows patterns |

The orchestrator determines complexity at delegation time and selects the model tier. Task artifacts can override the default. (*COMMENT* This needs to be defined by the connector/integration in coordination with the workflow created. These are good mappings for our software 'base', but the architecture needs to allow for different mappings for different project domains)

---

## 5. Knowledge Architecture

### Registration Model: Hybrid Manifest + Capability Matching

Knowledge plugins declare their artifacts in a structured manifest within their `plugin.json`:

```yaml
knowledge:
  artifacts:
    - id: rust-error-composition
      tier: always          # Always injected for matching roles
      roles: [implementer]
      paths: ["backend/**/*.rs"]

    - id: svelte5-patterns
      tier: stage-triggered  # Injected when workflow stage matches
      stages: [implement, review]
      paths: ["ui/**/*.svelte", "ui/**/*.ts"]

    - id: tauri-ipc-patterns
      tier: on-demand        # Available via semantic search
      tags: [ipc, tauri, commands]
```

### Three Injection Tiers

| Tier | When Loaded | Token Budget | Example |
|------|-------------|-------------|---------|
| **Always** | At agent spawn for matching roles/paths | 200-500 tokens (compressed summary) | Safety rules, error handling |
| **Stage-triggered** | When workflow enters a matching stage | 500-1,000 tokens | Coding standards during implement, review criteria during review |
| **On-demand** | When agent queries semantic search | 1,000-2,000 tokens (full artifact) | Specific domain patterns, historical decisions |

### Conflict Resolution

When multiple plugins provide knowledge for the same domain, priority follows:

1. **Project rules** (`.orqa/process/rules/`) -- highest priority, always wins
2. **Project knowledge** (`.orqa/process/knowledge/`) -- project-specific overrides
3. **Plugin knowledge** (from installed plugins) -- domain defaults
4. **Core knowledge** (from core framework) -- universal fallbacks

Within the same priority level, the most recently installed plugin wins, with explicit conflict resolution available in `project.json`.

### Knowledge Artifact Structure

Individual knowledge artifacts are atomic and self-contained:

- **Size target:** 500-2,000 tokens each (the "knowledge unit")
- **Structure:** YAML frontmatter (id, title, tier, roles, paths, tags) + markdown body
- **Self-contained:** Each artifact must be understandable without reading other artifacts
- **Hierarchical organization:** Directories group related knowledge, but each leaf is independently injectable
- **Compressed summaries:** Each artifact has a 100-150 token structured summary for the "always" tier; full text is loaded only at the "on-demand" tier

---

## 6. Programmatic Prompt Generation

### The Five-Stage Pipeline

```
Plugin Registry → Schema Assembly → Section Resolution → Token Budgeting → Prompt Output
```

**Stage 1 -- Plugin Registry:** All installed plugins register their prompt contributions: role definitions, stage instructions, knowledge artifacts, rules, and constraints. The registry is built at install time and cached.

**Stage 2 -- Schema Assembly:** For a given (role, workflow-stage, task) tuple, the assembler collects all applicable prompt sections from the registry. Sections are tagged with priority (P0 = safety/never-cut, P1 = role-critical, P2 = task-relevant, P3 = nice-to-have).

**Stage 3 -- Section Resolution:** Sections that reference other artifacts (e.g., "include coding-standards knowledge") are resolved to their compressed summaries. Cross-references are followed to a depth of 1. Circular references are detected and broken.

**Stage 4 -- Token Budgeting:** The assembled sections are measured against the token budget for this agent type. If over budget, P3 sections are trimmed first, then P2, then P1. P0 sections are never trimmed. The budget enforcer ensures no agent prompt exceeds its allocation.

**Stage 5 -- Prompt Output:** The final prompt is emitted with static content at the top (for KV-cache reuse) and dynamic content (task description, file paths) at the bottom. The prompt includes a preamble telling the agent how to access full rule/knowledge text on demand via semantic search.

### Expected Token Budgets

| Agent Type | Static Core | Workflow Stage | On-Demand | Total Budget |
|-----------|-------------|---------------|-----------|-------------|
| Orchestrator | 1,500 | 500 | 500 | 2,500 |
| Implementer | 800 | 500 | 1,500 | 2,800 |
| Reviewer | 600 | 300 | 1,000 | 1,900 |
| Researcher | 400 | 200 | 1,500 | 2,100 |
| Writer | 500 | 300 | 1,000 | 1,800 |
| Designer | 500 | 300 | 1,000 | 1,800 |

Compare to current: 9,500-16,500 tokens per orchestrator turn, 6,400 tokens per agent spawn.

### Token Efficiency Techniques

Research identified several high-impact techniques, each with measured results:

| Technique | Token Impact | Source |
|-----------|-------------|--------|
| Modular prompting (sections over monolith) | 42% reduction | Factory.ai research |
| Mermaid statecharts over prose | 4x more efficient | Measured in RES-2f602d54 |
| Structured rule summaries | ~100-150 tokens vs ~800 per rule | Calculated from current rules |
| Priority-based trimming (P0-P3) | Prevents blowouts while preserving safety | LLMLingua methodology |
| Claude XML tags for structure | Better model parsing, less ambiguity | Anthropic guidance |
| Prompt prefix stability for KV-cache | 10x cost difference (cached vs uncached) | Manus production data |
| Role-specific context bundles | 1-2K vs 4-8K per agent | Calculated from current injection |
| On-demand knowledge via semantic search | Eliminates duplicate loading across agents | CrewAI Mem0 pattern |
| Observation masking (tool output compression) | Reduces 100:1 input-to-output ratio | Manus production data |

### KV-Cache-Aware Prompt Structure

Manus reports that KV-cache hit rate is the single most important production metric, with a 10x cost difference between cached ($0.30/MTok) and uncached ($3/MTok) tokens. Prompt structure must maximize cache hits:

1. **Static core at the TOP** (role definition, safety rules, workflow stage) -- cached across turns
2. **Semi-static middle** (task context, acceptance criteria) -- cached within a task
3. **Dynamic content at the BOTTOM** (tool results, conversation history) -- changes every turn
4. **Never reorder sections between turns** -- breaks the cache prefix

---

## 7. State Machine Design

### Format: Hybrid YAML with Declarative Guards

After comparing XState (JavaScript statecharts), Temporal (durable execution), AWS Step Functions (ASL), BPMN 2.0 (Camunda/Flowable), Symfony Workflow (PHP/YAML), and CFlow (TypeScript/YAML), the recommended format is **Hybrid YAML with statechart semantics**.

This format combines YAML readability with the expressiveness needed for real workflow governance:

- **States** with categories (planning, active, review, completed, terminal)
- **Transitions** triggered by named events
- **Guards** as declarative expressions (field checks, relationship checks, graph queries)
- **Actions** as declarative operations (set field, append log, create artifact)
- **Gates** as first-class sub-workflow definitions
- **JSON Schema validation** of the entire structure

### Ownership: Plugin Owns the State Machine

The plugin that defines an artifact type owns its complete state machine. The core framework provides:

- The state machine evaluation engine
- State category vocabulary (planning, active, review, completed, terminal)
- Guard primitives (field_check, relationship_check, query, role_check)
- Action primitives (set_field, append_log, create_artifact, notify)
- Human gate infrastructure (gather, present, collect, execute, learn)

Plugins provide the actual state definitions, transitions, guards, and gates for their artifact types. There is no inheritance model -- each plugin's state machine is self-contained.

### Category-Based Composition

The core framework defines state categories; plugins map their states to categories. Categories drive cross-cutting concerns:

| Category | Purpose | UI Treatment |
|----------|---------|-------------|
| `planning` | Work being designed/scoped | Blue indicators |
| `active` | Work in progress | Green indicators |
| `review` | Work being reviewed | Amber indicators |
| `completed` | Work finished | Purple indicators |
| `terminal` | Final state, no further transitions | Gray indicators |

This pattern (validated by Azure DevOps) means the UI can render any artifact type generically. Dashboard aggregation works across artifact types ("show me everything in review") without knowing the specific states of each plugin.

### Human Gates as Sub-Workflows

Human gates are not boolean approve/reject flags. They are structured sub-workflows with a five-phase pipeline:

1. **GATHER** -- Collect data from artifact fields, run automated pre-checks, generate summary
2. **PRESENT** -- Show inputs in structured format, display verdict options, provide context
3. **COLLECT** -- Reviewer provides verdict + feedback, record timestamp and rationale
4. **EXECUTE** -- Apply the transition, run post-transition actions, log to audit trail
5. **LEARN** -- On FAIL: create/update lesson, check promotion threshold, update recurrence

Five gate patterns cover the full range of review needs:

| Pattern | Use Case | Mechanism |
|---------|----------|-----------|
| Simple Approval | Task completion, small changes | Single reviewer, approve/reject |
| Structured Review (Maker-Checker) | Epic completion, milestone gates | AI review first, then human review |
| Multi-Reviewer (Four-Eyes) | Major releases, compliance-sensitive | Multiple independent reviewers, all must pass |
| Escalation | Time-sensitive approvals | Timeout triggers escalation to another reviewer |
| Scope Decision | Mid-epic scope changes, idea promotion | Multiple outcome paths (proceed, descope, expand, cancel) |

### Ad-Hoc Workflow Patterns

Not all work follows the full pipeline. The architecture supports workflow variants:

| Scenario | Variant | Key Difference |
|----------|---------|----------------|
| Bug fix (small) | `task-quickfix` | Skip planning, automated review only |
| UX tweak | `task-quickfix` | Skip planning, automated review only |
| Security fix | `task-security` | Skip planning, mandatory human review |
| Documentation fix | `task-docs-only` | Skip review entirely |
| Hotfix (production) | `task-hotfix` | Skip planning, expedited review |

Workflow selection rules in the plugin manifest automatically assign the appropriate variant based on artifact properties (priority, labels, scope).

### State Migration

Migration follows a forward-compatible-first approach:

- **Adding states/transitions:** Non-breaking, no migration needed. Existing artifacts remain valid.
- **Renaming/removing states:** Status mapping migration via `orqa migrate`. The plugin includes a `migration` section in its workflow definition that maps old status values to new ones.
- **Complex restructuring:** Dual-write period where both old and new values are accepted on read, with new values written on all updates.

---

## 8. Token Efficiency Architecture

### Core Principle

**Generated system prompts contain only what the agent needs for its current task.** No more, no less.

### Architecture Overview

```
[User Request]
     |
     v
[Orchestrator] --- minimal static core (~1,500 tokens)
     |                + workflow stage instructions (~500 tokens)
     |                + on-demand rule retrieval (~500 tokens when needed)
     |
     v
[Task Classification] --- determines complexity, model tier, required context
     |
     v
[Agent Spawn] --- role-specific context bundle (~1,000-1,500 tokens)
                  + task description + acceptance criteria (~500 tokens)
                  + on-demand knowledge via semantic search (0-2,000 tokens)
                  = TOTAL: 1,500-4,000 tokens per agent
                  (vs. current 9,500-16,500)
```

### Six Components

**1. Compiled System Prompts:** Replace "CLAUDE.md + 58 rule files" with generated, role-specific prompts assembled from the plugin registry.

**2. Model Tiering:** Default model selection by role, with orchestrator override based on task complexity. Expected 45-65% cost savings on mixed workloads.

**3. Token Budget Enforcement:** Each agent spawn has a token budget. The orchestrator monitors cumulative session cost and adjusts strategy (switch to Sonnet when approaching budget).

**4. KV-Cache Optimization:** Static core at top, dynamic content at bottom. Never reorder sections between turns.

**5. Findings-to-Disk Enforcement:** Orchestrator reads only structured summary headers from findings files (~200 tokens), never full findings. Detailed review is delegated to Reviewer agents.

**6. Token Tracking:** Hook-based tracking captures tokens per API call, attributed to session/team/agent/task. Metrics written to `tmp/token-metrics.jsonl`. Future dashboard in OrqaStudio UI. (*COMMENT* I'm concerned about tmp being used as the semantics for this (and session state) in the directory naming, could we use something that is less 'temporary' and therefore could be confused as being disposable - both this and session state are state data, not disposable information).

### Expected Impact

| Metric | Current | After Architecture | Improvement |
|--------|---------|-------------------|-------------|
| Per-prompt overhead (orchestrator) | 9,500-16,500 tokens | 2,000-3,500 tokens | 65-80% |
| Per-agent spawn cost | 6,400 tokens | 1,500-4,000 tokens | 40-75% |
| 8-agent team spawn | 63K tokens | 16-36K tokens | 43-75% |
| Session total (20 prompts, 2 teams) | ~300K+ tokens | ~80-120K tokens | 60-73% |
| Overhead ratio | 13.4x | 2-4x | 70% improvement |

### Token Tracking and Observability

Four levels of metrics, inspired by Langfuse, AgentOps, and Manus production systems:

- **Level 1 (Per-Request):** Input/output tokens, cache hit rate, reasoning tokens, tool call tokens
- **Level 2 (Per-Agent):** Total tokens, context utilization ratio, knowledge injection tokens, agent lifetime
- **Level 3 (Per-Task/Session):** Tokens per deliverable, overhead ratio, rule loading tokens, team spawn cost
- **Level 4 (Trends):** 7-day cost per task, cache hit rate trend, model tier distribution, waste reduction

Dashboard design: Session Overview (total/budget/cost), Agent Breakdown (per-agent cost attribution), Efficiency Analysis (overhead ratio, rule injection efficiency), Trends (7-day/30-day).

---

## 9. Alignment with Existing Research

### What Carries Forward

From **RES-97e3aa4b** (Composability Research):

| Element | Status | How It Carries Forward |
|---------|--------|----------------------|
| Composability model (skills, knowledge, rules as units) | Carry forward | Foundation of the knowledge plugin architecture |
| Knowledge patterns (atomic, self-contained, injectable) | Carry forward | Knowledge artifact structure at 500-2000 tokens |
| Sequential workflow (understand-plan-document-implement-review-learn) | Carry forward | Becomes the workflow skeleton that plugins compose into |
| Parallel safety rules | Carry forward | Resource safety constraints on agent spawning |
| Resource safety (no concurrent Rust compilation) | Carry forward | Enforced by the workflow engine |
| Stack adaptation principle | Carry forward | Plugin system makes this concrete |

From **RES-2f602d54** (Token Efficiency Baselines):

| Element | Status | How It Carries Forward |
|---------|--------|----------------------|
| Token baselines (9.5K-16.5K per prompt, 63K per team) | Carry forward | Benchmarks for measuring improvement |
| Orchestrator discipline recommendations | Carry forward | Hub-spoke with findings-to-disk |
| Model tiering recommendations | Carry forward | Integrated into agent spawn pipeline |
| Measurement methodology | Carry forward | Token tracking architecture |

### What Is Discarded

| Element | Why Discarded | Replaced By |
|---------|--------------|-------------|
| Persistent agents (session-long lifecycle) | Context rot, stale agents, shutdown discipline | Ephemeral task-scoped agents |
| Fixed team of 5 agents | Rigid, cannot adapt to task needs | Dynamic team composition from workflow stage |
| SendMessage-based handoffs | Unreliable during concurrent turns | Findings-to-disk + task completion events |
| Session-long agent lifecycle | Accumulated context degrades quality | One context window per task |
| Lazy rule loading (patch) | Addresses symptom, not cause | Compiled system prompts from plugin registry |
| Skill registry patch | Partial solution | Full prompt generation pipeline |
| Rule compression to tables (patch) | Partial solution | Structured summaries + on-demand full text |

### Gaps Filled by This Research

| Gap | Prior State | Now Addressed |
|-----|------------|---------------|
| Workflow composition from plugins | Not investigated | Contribution-point model with YAML manifests |
| Programmatic prompt generation | Not investigated | Five-stage pipeline with token budgeting |
| Deterministic state machines | Informal status tracking | YAML state machines with declarative guards |
| Discovery plugin architecture | Not investigated | Three-layer taxonomy with knowledge injection |
| Token observability | No tracking | Four-level metrics with dashboard design |
| Ad-hoc workflow patterns | Not investigated | Workflow variants with selection rules |
| Review sub-workflows | Not investigated | Five-pattern gate system with learning integration |

---

## 10. Recommended Path Forward

### Proposed Epic Structure

The architecture should be implemented across four epics, ordered by dependency:

**Epic 1: Core Workflow Engine + State Machines**
Build the state machine evaluation engine, YAML workflow format, guard/action primitives, and category-based composition. This is the foundation everything else depends on.

- Implement YAML workflow parser with JSON Schema validation
- Build state machine evaluation engine (transition resolution, guard evaluation, action execution)
- Define state category vocabulary and UI metadata
- Build workflow resolver (merge plugin contributions into resolved files)
- Implement migration framework (forward-compatible addition + status mapping)
- Define workflow file structure within plugins

**Epic 2: Prompt Generation Pipeline + Knowledge Architecture**
Build the five-stage prompt generation pipeline and the knowledge plugin registration/injection system.

- Build plugin registry for prompt contributions
- Implement schema assembly (collect applicable sections for role + stage + task)
- Implement section resolution (resolve references, produce compressed summaries)
- Implement token budgeting (P0-P3 priority trimming)
- Implement prompt output with KV-cache-aware structure
- Build knowledge plugin manifest format and injection tiers
- Implement conflict resolution (project > plugin > core priority)
- Build on-demand knowledge retrieval via semantic search integration

**Epic 3: Agent Lifecycle + Model Tiering**
Implement the ephemeral agent spawn model, model tier selection, and token tracking.

- Implement agent spawn from generated prompts (replace CLAUDE.md loading)
- Build model tier selection logic (role-based defaults + complexity override)
- Implement token tracking hooks (per-request, per-agent, per-task attribution)
- Build findings-to-disk summary format and orchestrator summary reader
- Implement agent token budget enforcement
- Build session metrics reporter

**Epic 4: Human Gates + Review Sub-Workflows**
Implement the human gate infrastructure and the five gate patterns.

- Build gate infrastructure (gather, present, collect, execute, learn pipeline)
- Implement simple approval gate pattern
- Implement structured review gate (AI-then-human)
- Implement multi-reviewer gate
- Implement scope decision gate
- Build learning integration (lesson creation/update on FAIL, recurrence tracking)
- Implement workflow variant selection rules
- Build ad-hoc workflow variants (quickfix, security, docs-only)

### Task Breakdown Approach

Each epic should be broken into tasks following the established pattern:

- **Documentation tasks first** (target-state docs before implementation)
- **Schema/format tasks** (define the YAML formats, JSON Schemas)
- **Engine tasks** (build the evaluation/resolution engines)
- **Integration tasks** (wire into existing plugin system, artifact graph)
- **Migration tasks** (convert existing hardcoded workflows to plugin-defined)
- **Verification tasks** (independent review of each phase)

### Code and Content Migration

The transition from current architecture to plugin-composed architecture requires migrating existing content:

| Current Location | Migration Target | Notes |
|-----------------|-----------------|-------|
| CLAUDE.md (monolithic orchestrator prompt) | Plugin-composed generated prompt | Decompose into role definition + stage instructions + safety rules |
| 58 rule files in .orqa/process/rules/ | Knowledge plugin artifacts + compressed summaries | Each rule produces a 100-150 token summary for injection |
| Agent definitions in .orqa/process/agents/ | Universal role templates + knowledge composition | Agent definitions become role + capability + knowledge declarations |
| Hardcoded status values in schema.json | Plugin-owned workflow.yaml files | Status values derived from state machine definitions |
| Knowledge artifacts in .orqa/process/knowledge/ | Knowledge plugin manifests | Add injection tier, roles, paths metadata |

### Implementation Order

The four epics have a dependency chain:

```
Epic 1 (Workflow Engine)
  |
  +-- Epic 2 (Prompt Generation + Knowledge)
  |     |
  |     +-- Epic 3 (Agent Lifecycle + Tiering)
  |
  +-- Epic 4 (Human Gates + Review)
```

Epic 1 must complete before Epic 2 (prompt generation needs workflow stage context). Epic 2 must complete before Epic 3 (agent spawning needs generated prompts). Epic 4 depends on Epic 1 (gates are part of the state machine) but can run in parallel with Epics 2-3.

**Recommended sequence:** Epic 1 -> Epic 2 -> (Epic 3 + Epic 4 in parallel).

---

## 11. Open Questions

### Architecture Questions

1. **Workflow inheritance vs. composition:** Should plugins be able to extend another plugin's workflow (add states to an existing state machine), or should composition be limited to contribution points in the skeleton? The research recommends starting with no inheritance (plugin owns its complete state machine) and adding extension points only if a clear need emerges. (*COMMENT* AGREED)

2. **Guard expression language:** The declarative guard system (field_check, relationship_check, query) covers most cases, but some guards may need more complex logic. How far should the expression language go before it becomes a programming language? Recommendation: keep guards declarative and use code hooks for anything that cannot be expressed as a field check or graph query. (*COMMENT* AGREED)

3. **Cross-plugin workflow coordination:** When two plugins define artifact types that have inter-dependencies (e.g., an epic's tasks must all be done before the epic can transition), how is this expressed? The current recommendation is graph queries in guards (`query: { type: task, epic: $self, status: { not: done } }`), but this creates implicit coupling between plugins. (*COMMENT* I think implicit coupling with a project domain is ok - as long as this is documented so that alternative plugins that provide stage data use the same terminology)

4. **Workflow versioning strategy:** Should the workflow engine support running multiple versions simultaneously (Temporal-style patching), or is forward-compatible addition sufficient? For file-based artifacts that are not "running processes," forward-compatible addition with status mapping migrations seems sufficient. Revisit if OrqaStudio adds long-running automated workflows. (*COMMENT* AGREED - Also to note, that I DO NOT want backwards-compatibility to be a factor in the tasks that come from this, we are in pre-release, breaking changes are expected and necessary, data should be migrated)

### Token Efficiency Questions

1. **Compressed summary generation:** Who generates the 100-150 token structured summaries of rules and knowledge? Options: (a) the rule/knowledge author writes them, (b) an LLM generates them at install time, (c) a hybrid where the author writes a summary template and the LLM fills it. Recommendation: author writes them as part of the knowledge artifact (a `summary` field in frontmatter), with an `orqa summarize` CLI command that generates drafts for review. (*COMMENT* AGREED - although note that authors can be agents)

2. **On-demand retrieval latency:** Semantic search adds 1-2 seconds per query. For agents that need multiple knowledge lookups, this could add 5-10 seconds to task start. Is this acceptable? Recommendation: yes, if the alternative is loading 10x more tokens. The latency cost is paid once at task start; the token cost compounds throughout the agent's lifetime. (*COMMENT* AGREED)

3. **Token budget enforcement granularity:** Should token budgets be enforced per-agent, per-team, or per-session? Recommendation: per-agent budgets for prompt size, per-session budgets for total cost. Team-level budgets add complexity without clear benefit. (*COMMENT* AGREED)

### Process Questions

1. **MCP server as application boundary:** The current OrqaStudio architecture has the MCP server, LSP server, and ONNX search engine as libraries compiled into the app. The service extraction initiative (planned) would make them standalone services. Should the prompt generation pipeline run in the MCP server (accessible from both CLI and app) or in the core framework (requiring reimplementation for each connector)? (*COMMENT* This wasn't the intended architecture as to my understanding - the daemon is where the business logic should live, the MCP is an access method)

2. **Migration timeline:** Converting 58 rules to compressed summaries, 20+ knowledge artifacts to plugin manifests, and 8 agent definitions to universal role templates is significant content migration work. Should this be done incrementally (one plugin at a time) or as a big-bang migration? Recommendation: incrementally, starting with the software-kanban plugin as the proving ground. (*COMMENT* Incrementally, but in one epic with tasks done sequentially with validation in between)

3. **Backwards compatibility during transition:** During the migration from current architecture to plugin-composed architecture, both systems will need to coexist. How long should the transition period be? Recommendation: the prompt generation pipeline should fall back to current CLAUDE.md loading when no plugin-composed prompt is available, allowing gradual migration. (*COMMENT* AGREED - but this should only be for a short period whilst the LLM is performing the migration)

---

## 12. Detailed Research References

### Primary Research Documents (This Investigation)

| Task | Topic | Key Findings |
|------|-------|-------------|
| task-1 | Workflow Composition from Plugins | Contribution-point model, YAML + code hooks, install-time merging, resolved file on disk |
| task-2 | Agent Type Specialization | Three-layer taxonomy, generalist + injection model, Google/Microsoft validation of hub-spoke |
| task-3 | Knowledge Plugin Architecture | Hybrid manifest + capability matching, three injection tiers, atomic 500-2000 token artifacts |
| task-4 | Programmatic Prompt Generation | Five-stage pipeline, 60-75% token reduction, Mermaid 4x efficient, priority-based trimming |
| task-5 | State Machine Design | Hybrid YAML format, plugin-owned machines, category-based composition, 5 human gate patterns, ad-hoc variants |
| task-6 | Token Efficiency Architecture | Context engineering as discipline, compiled prompts, model tiering, KV-cache optimization, four-level metrics |
| task-7 | Existing Research Alignment | Carry-forward list, discard list, 7 gap areas now addressed |

### Prior Research (Evolved From)

| Document | Relationship |
|----------|-------------|
| RES-97e3aa4b (Composability) | Composability model carries forward; persistent agent model discarded |
| RES-2f602d54 (Token Efficiency) | Baselines and measurement carry forward; patch-level solutions superseded |

### External Sources (Key References)

- **Anthropic:** Context engineering guidance, multi-agent research system architecture, agent skills methodology
- **Google ADK:** Context as compiled views, context-aware multi-agent framework
- **Manus:** Five-dimensional context engineering, KV-cache as primary metric, 100:1 input-to-output ratio
- **Microsoft AutoGen:** Message filtering, teachable agents, GroupChat coordination
- **CrewAI + Mem0:** 90% token reduction via intelligent memory compression
- **SC-MAS (2026):** Unified controller for role selection + model allocation, 11-16% token reduction
- **Factory.ai:** Structured summarization methodology, modular prompting (42% reduction)
- **LLMLingua (Microsoft):** Coarse-to-fine compression, up to 20x with 1.5-point performance drop
- **Azure DevOps:** Category-based state composition pattern (Proposed, In Progress, Resolved, Completed)
- **XState v5:** Statechart formalism with parallel/hierarchical states, guard/action model
- **Temporal:** Durable execution, workflow versioning with patching, signal-based human gates
- **BPMN 2.0:** Industry-standard process modeling, User Tasks, maker-checker pattern
