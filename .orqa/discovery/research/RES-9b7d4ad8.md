---
id: RES-9b7d4ad8
title: "Token Efficiency as Architectural Constraint"
type: research
status: active
category: architecture
description: "Token-efficient architecture by design — context engineering, model tiering, observability"
created: 2026-03-25
updated: 2026-03-25
tags:
  - agent-teams
  - plugin-architecture
---

# Token Efficiency Architecture Research

## Task

Research best practices for designing token-efficient multi-agent AI systems, covering framework comparisons, context loading strategies, prompt compression, model tiering, token tracking, orchestrator context management, duplicate context elimination, and recommended architecture for OrqaStudio.

## Status: Complete

---

## 1. Framework Comparison: How Multi-Agent Frameworks Handle Token Efficiency

### 1.1 LangGraph

**Context management approach:** Graph-based state machines with explicit state persistence. LangGraph manages state using reducer logic to merge concurrent updates, giving precise control over execution order, branching, and error recovery.

**Token efficiency strategy:**
- State is persisted externally (checkpoints), not carried in-context
- Each node in the graph receives only the state slice it needs
- Reducer functions control what gets added to state vs. discarded
- Supports streaming and incremental state updates

**Strengths:** Fine-grained state control, explicit context boundaries per node, good for long-running workflows where different steps need different context.

**Weaknesses:** Requires manual state management design. Developers must explicitly decide what to persist vs. discard.

### 1.2 CrewAI

**Context management approach:** Role-based agents with hierarchical delegation. Each agent has a defined role, goal, and backstory.

**Token efficiency strategy:**
- Agents communicate through structured task outputs, not full conversation history
- Supports Mem0 integration for persistent memory, reducing token costs by up to 90% (from 26K tokens to ~1.8K per conversation)
- Memory architecture supports short-term (conversation), long-term (cross-session), and entity memory

**Strengths:** Role-based scoping naturally limits context per agent. Mem0 integration provides dramatic token savings.

**Weaknesses:** Native memory is static and doesn't evolve across sessions without Mem0. Communication between agents can be verbose.

### 1.3 AutoGen (Microsoft)

**Context management approach:** Conversational collaboration between agents with explicit message protocols.

**Token efficiency strategy:**
- GroupChat manager controls which agent speaks next, preventing unnecessary context accumulation
- Supports "teachable agents" that learn and compress knowledge to external storage
- Message filtering controls which messages each agent sees

**Strengths:** Message-level filtering gives fine control over what enters each agent's context. Teachable agents compress repeated patterns.

**Weaknesses:** Conversational overhead — agents exchange full messages rather than structured data. Context grows linearly with conversation length without manual intervention.

### 1.4 Google ADK (Agent Development Kit)

**Context management approach:** Context as a "compiled view over a richer stateful system." ADK treats context engineering as a first-class architectural concern.

**Token efficiency strategy:**
- Context is not raw history — it's a compiled, filtered view assembled at each step
- Explicit semantics for handing off "the right slice of context" between agents
- Built-in context compaction via Google's own compression APIs
- Addresses cost and latency spirals by design, not as an afterthought

**Strengths:** Most architecturally intentional about context efficiency. Context compilation is built into the framework, not bolted on. Multi-agent hierarchies with explicit context boundaries.

**Weaknesses:** Newer framework (2025), ecosystem still maturing. Tightly coupled with Google's model ecosystem.

### 1.5 Manus

**Context management approach:** Five-dimensional context engineering: offloading, reduction, retrieval, isolation, and caching.

**Token efficiency strategy:**
- KV-cache hit rate treated as the single most important production metric
- Context offloading to file systems and sandbox environments
- Context reduction through compaction and summarization
- Context retrieval via file-based search tools (load on demand)
- Context isolation via multi-agent architectures
- Context-aware state machine manages tool availability by masking token logits
- Average input-to-output ratio of 100:1 — making cache efficiency critical

**Strengths:** Most mature production system. Five refactors since launch. Practical, battle-tested strategies.

**Weaknesses:** Closed system — many techniques are internal. Requires infrastructure for sandbox environments and file-based retrieval.

### Summary Comparison

| Framework | Primary Strategy | Token Savings | Context Isolation | Production Maturity |
|-----------|-----------------|---------------|-------------------|-------------------|
| LangGraph | State graph with reducers | Moderate | Per-node state slices | High |
| CrewAI | Role-based + Mem0 | High (90% with Mem0) | Per-agent role scope | High |
| AutoGen | Message filtering + teachable agents | Moderate | Per-agent message filter | High |
| Google ADK | Compiled context views | High | Framework-native | Medium (newer) |
| Manus | 5-dimensional context engineering | Very High | Multi-agent + sandbox | Very High |

---

## 2. Context Loading Strategies: Eager vs. Lazy vs. Hybrid

### 2.1 Eager Loading (Current OrqaStudio Approach)

**Pattern:** Load all potentially relevant rules, knowledge, and context at session start or agent spawn.

| Pros | Cons |
|------|------|
| Simple implementation | 48K+ tokens of rules loaded per prompt |
| No retrieval latency at decision time | Most loaded context is irrelevant to current task |
| Guaranteed availability | Context window fills quickly |
| Predictable behavior | Duplicate context across agents |

**OrqaStudio current cost:** ~9,500-16,500 tokens per orchestrator turn. 63K tokens per 8-agent team spawn.

### 2.2 Lazy Loading (On-Demand / RAG-Based)

**Pattern:** Load only the specific rules, knowledge, and context relevant to the current task, using semantic search or classification.

| Pros | Cons |
|------|------|
| Dramatic token reduction (60-80%) | Retrieval latency (1-2 seconds per query) |
| Context stays focused and relevant | Risk of missing critical rules |
| Scales to any number of rules/knowledge | Requires quality semantic search |
| Each agent gets only what it needs | Cold-start problem for new topics |

**Implementation patterns:**
- **Tool RAG:** Retrieve only relevant tools from a large registry. Studies show 3x accuracy improvement while halving prompt length.
- **Semantic rule retrieval:** Instead of loading 5 critical rules always, query "what rules apply to [this specific task]?" and load top-3.
- **Progressive disclosure:** Start with minimal context, load more only when the agent signals it needs specific information.

### 2.3 Hybrid Loading (Recommended)

**Pattern:** Small static core + on-demand retrieval for everything else.

| Component | Loading Strategy | Size Target |
|-----------|-----------------|-------------|
| Agent role definition | Static (always loaded) | 200-500 tokens |
| Current workflow stage instructions | Static (workflow-specific) | 500-1,000 tokens |
| Critical safety rules | Static (always loaded) | 500 tokens (compressed) |
| Domain knowledge | On-demand via semantic search | 1,000-2,000 tokens when needed |
| Task-specific rules | On-demand via classification | 500-1,500 tokens when triggered |
| Historical context | On-demand via memory system | Variable |

**Expected total per agent:** 1,200-2,000 tokens static + 1,000-3,500 tokens on-demand = **2,200-5,500 tokens** (vs. current 9,500-16,500).

**Key insight from Manus:** "The guiding principle remains: find the smallest set of high-signal tokens that maximize the likelihood of your desired outcome. Every token you add to the context window competes for the model's attention."

---

## 3. Prompt Compression Techniques

### 3.1 LLMLingua (Microsoft)

**Approach:** Coarse-to-fine compression using a small language model's perplexity to identify which tokens can be removed.

**Components:**
- Budget Controller: Dynamically allocates compression ratios by section type
- Iterative Token-level Compression: Removes low-information tokens
- Alignment: Ensures compressed output preserves semantic meaning

**Results:**
- Up to 20x compression with only 1.5-point performance drop
- 1.7-5.7x inference acceleration
- LLMLingua-2 adds 3-6x speed improvement via BERT-level encoder

**Compression ratio recommendations:**
| Content Type | Recommended Compression | Token Cost Impact |
|-------------|------------------------|-------------------|
| Instructions/system prompts | 10-20% (light) | Preserves clarity |
| Examples/few-shot | 60-80% (heavy) | High redundancy in examples |
| Questions/tasks | 0-10% (minimal) | Must preserve intent |
| Rules/constraints | 20-40% (moderate) | Structure matters |

**Applicability to OrqaStudio:** High. The 58 rule files averaging 92 lines each are excellent candidates for 20-40% compression — removing verbose explanations, examples, and "Related Rules" sections while preserving the core constraint. A 30% compression on 48K tokens of rules saves ~14K tokens.

### 3.2 Structured Summarization

**Approach:** Replace raw rule text with structured summaries that force preservation of key information categories.

**Pattern (from Factory.ai research):**
```
For each rule, generate:
- Core constraint (1-2 sentences)
- Trigger condition (when does this apply?)
- Forbidden patterns (what violates this?)
- Verification (how to check compliance?)
```

**Results:** Retains more useful information than freeform summarization. Forces completeness by requiring each section to be populated or explicitly marked empty.

**Applicability to OrqaStudio:** Very high. Each of the 58 rules could be pre-compressed to a structured summary (~100-150 tokens each vs. current ~800 tokens). Load the summary by default, load the full rule only when the agent is actively working in that rule's domain.

### 3.3 ACON (Agent Context Optimization)

**Approach:** Failure-driven compression optimization. Identifies what information compression lost that caused failures, then revises the compression prompt.

**Results:** 26-54% reduction in peak token usage.

**Applicability to OrqaStudio:** Medium. Useful as a continuous improvement mechanism — track which rule compressions cause agent failures and iteratively refine the compressed versions.

### 3.4 Context Compaction (Anthropic/Google)

**Approach:** Built-in conversation summarization that runs when approaching context limits.

**Anthropic's approach:** Context Compaction (beta) summarizes older context as token limits are reached.

**Google ADK approach:** Built-in compaction APIs that compress conversation history while preserving key information.

**Applicability to OrqaStudio:** Already available in Claude Code. The key is ensuring OrqaStudio's structured content (rules, knowledge) survives compaction well — which argues for the structured summarization approach above.

### 3.5 Observation Masking

**Approach:** In agent tool call loops, mask the observation (tool output) from previous turns while preserving action and reasoning history.

**Rationale:** Tool outputs often dominate token count (file contents, search results). Preserving the action history ("I read file X") without the full content drastically reduces context.

**Applicability to OrqaStudio:** High for implementation agents that make many tool calls. Manus reports a 100:1 input-to-output ratio, suggesting most tokens are tool observations.

---

## 4. Model Tiering Recommendations

### 4.1 The Cost Problem

Using the most capable model for every agent is the most common cause of cost blowouts in multi-agent systems. Research consistently shows 60-80% cost reduction through intelligent model routing.

### 4.2 Recommended Tiering for OrqaStudio

| Role | Task Complexity | Recommended Tier | Rationale |
|------|----------------|-----------------|-----------|
| **Orchestrator** | High (coordination, planning) | Opus | Needs strongest reasoning for delegation decisions |
| **Planner** | High (architecture, dependencies) | Opus | Architecture planning requires deep reasoning |
| **Implementer** (complex) | High (multi-file refactoring) | Opus | Complex code changes need full capability |
| **Implementer** (simple) | Medium (single-file changes) | Sonnet | Straightforward implementations don't need Opus |
| **Researcher** | Medium (information gathering) | Sonnet | Research is about accuracy in reading, not creativity |
| **Reviewer** | Medium (pattern matching) | Sonnet | Code review is mostly pattern matching and checklist verification |
| **Writer** | Medium (documentation) | Sonnet | Documentation writing follows templates |
| **Designer** | Medium (UI components) | Sonnet | Component creation follows established patterns |
| **Governance Steward** | Low-Medium (artifact updates) | Sonnet | YAML frontmatter and structured content |

### 4.3 Cost-Quality Analysis

| Scenario | All Opus | Tiered | Savings |
|----------|----------|--------|---------|
| 8-agent team, mixed tasks | $X | ~0.35X | 65% |
| Orchestrator + 3 implementers | $X | ~0.55X | 45% |
| Research + review pipeline | $X | ~0.30X | 70% |

**Implementation approach:**
1. Add `model` field to task artifacts (optional, defaults based on role)
2. Orchestrator determines complexity at delegation time
3. Start conservative (Opus for anything uncertain) and tier down as confidence grows
4. Track quality metrics per model tier to validate

### 4.4 Dynamic Routing (Advanced)

The SC-MAS framework (2026) proposes a unified controller that progressively selects agent roles, assigns collaboration strategies, and allocates LLM backbones to individual agents based on query complexity. Results: 11-16% token reduction with 1.5-3.3% accuracy improvement.

For OrqaStudio, this could mean the orchestrator assesses task complexity before spawning and routes to the appropriate model tier dynamically.

---

## 5. Token Tracking and Reporting Design

### 5.1 What to Measure

Based on analysis of Langfuse, AgentOps, Braintrust, and production systems:

**Level 1: Per-Request Metrics**
| Metric | What It Measures | Why It Matters |
|--------|-----------------|---------------|
| Input tokens | Tokens sent to the model | Cost driver (cached vs. uncached) |
| Output tokens | Tokens generated by the model | Quality signal + cost |
| Cache hit rate | % of input from KV-cache | 10x cost difference (Manus insight) |
| Reasoning tokens | Tokens used for chain-of-thought | Extended thinking cost |
| Tool call tokens | Tokens in tool inputs/outputs | Often the largest component |

**Level 2: Per-Agent Metrics**
| Metric | What It Measures | Why It Matters |
|--------|-----------------|---------------|
| Total tokens per agent | Lifetime token consumption | Cost attribution |
| Context utilization ratio | Used tokens / available window | Efficiency signal |
| Knowledge injection tokens | Tokens from injected rules/knowledge | Measures injection efficiency |
| Wasted tokens | Tokens from irrelevant context | Optimization target |
| Agent lifetime | Time from spawn to completion | Stale agent detection |

**Level 3: Per-Task/Session Metrics**
| Metric | What It Measures | Why It Matters |
|--------|-----------------|---------------|
| Tokens per deliverable | Total tokens to produce a task output | Efficiency benchmark |
| Overhead ratio | Actual tokens / theoretical minimum | The 13x gap metric |
| Rule loading tokens | Tokens spent on governance rules | Injection optimization target |
| Team spawn cost | Total tokens to create a team | Baseline cost signal |
| Session total cost | Total tokens for the session | Budget management |

**Level 4: Trend Metrics**
| Metric | What It Measures | Why It Matters |
|--------|-----------------|---------------|
| Cost per task (7-day trend) | Average token cost to complete a task | Efficiency trajectory |
| Cache hit rate trend | Improving or degrading cache performance | Infra health |
| Model tier distribution | % of tokens on each model tier | Cost optimization progress |
| Waste reduction trend | Improvement in overhead ratio | ROI of optimization work |

### 5.2 How to Display

**Dashboard Design (inspired by Langfuse, Braintrust, and production patterns):**

**View 1: Session Overview**
- Total tokens (input/output/cached breakdown)
- Total estimated cost
- Agent count and model distribution
- Session timeline with token usage per turn

**View 2: Agent Breakdown**
- Table: Agent name | Role | Model | Input tokens | Output tokens | Cache hits | Cost
- Sortable by any column
- Drill-down to individual agent traces

**View 3: Efficiency Analysis**
- Overhead ratio gauge (current vs. target)
- Rule injection analysis (which rules were loaded, which were used)
- Knowledge injection analysis (loaded vs. referenced)
- Waste identification (largest unnecessary context items)

**View 4: Trends**
- 7-day/30-day cost trend
- Token efficiency trend (overhead ratio over time)
- Model tier distribution changes
- Per-task-type cost benchmarks

### 5.3 Implementation Approach

**Phase 1: Basic tracking (hook-based)**
- PostToolUse hook on Agent tool captures token usage from Claude API responses
- Write to `tmp/token-metrics.jsonl` (append-only log)
- Session summary in `tmp/session-state.md`

**Phase 2: Structured attribution**
- Tag each API call with: session_id, team_name, agent_role, task_id, model
- Store in SQLite (conversation persistence channel per AD-2aa4d6db)
- Generate per-session and per-task cost reports

**Phase 3: Dashboard**
- Token usage view in OrqaStudio UI
- Real-time session cost counter
- Historical trends and benchmarks
- Budget alerts (configurable per-session or per-day limits)

---

## 6. Orchestrator Context Management Patterns

### 6.1 The Problem

The orchestrator is the longest-lived agent in any session. It coordinates everything, reads findings, tracks state, and makes delegation decisions. Its context fills up with implementation details it will never need again.

Current OrqaStudio impact: After reading 3-4 agent findings, the orchestrator has ~7K tokens of stale implementation details. This leaves insufficient room for planning and coordination.

### 6.2 Proven Patterns

**Pattern 1: Blackboard Architecture**
The orchestrator maintains a shared workspace (blackboard) where agents write results. The orchestrator reads only summaries from the blackboard, never full results.

- **How it works:** Agents write findings to disk (`tmp/team/*/task-*.md`). Orchestrator reads only a structured summary (status, key decisions, follow-ups) — not the full findings.
- **Token savings:** 5-10K tokens per session by avoiding full findings in orchestrator context.

**Pattern 2: Tiered Memory**
The orchestrator maintains separate working memory (current task) and long-term memory (session state file).

- **Working memory:** Current task, active team, immediate coordination needs (~2-3K tokens)
- **Session state:** Written to disk (`tmp/session-state.md`), referenced when needed
- **Long-term memory:** Artifact graph (queried, not loaded)
- **Token savings:** Prevents context accumulation across tasks

**Pattern 3: Selective Joint Attention (from PC-Agent research)**
The orchestrator maintains high-level plan memory while workers maintain granular sub-task memory. Collaboration happens at the intersection.

- **How it works:** Orchestrator tracks: task statuses, key decisions, blockers. Workers track: file paths, code patterns, test results. Orchestrator never sees worker-level details.
- **Token savings:** Orchestrator context stays at 3-5K tokens regardless of how many agents are running.

**Pattern 4: Context Reset Between Phases**
When transitioning between work phases (e.g., planning to implementation to review), the orchestrator explicitly compacts its context.

- **How it works:** Before starting a new phase, write a structured summary of the completed phase to session state. Start the new phase with fresh context + summary reference.
- **Token savings:** Prevents context from growing linearly across phases.

### 6.3 Recommended Approach for OrqaStudio

Combine patterns 1 + 2 + 4:
1. **Blackboard for findings:** Orchestrator never reads full findings files. Agents write structured summaries to a standard format. Orchestrator reads only the summary section.
2. **Session state as external memory:** All intermediate state goes to `tmp/session-state.md`, not held in context.
3. **Phase boundaries as compaction points:** When switching teams or work phases, summarize and clear.

---

## 7. Duplicate Context Elimination Strategies

### 7.1 The Problem

When 5-8 agents in a team need similar context (governance rules, architecture decisions, coding standards), loading it into each agent's context independently wastes 5-8x the tokens.

Current OrqaStudio cost: If each agent loads ~4K tokens of shared knowledge, an 8-agent team wastes ~28K tokens on duplicate context.

### 7.2 Strategies

**Strategy 1: Shared Workspace (Blackboard)**
A centralized knowledge space that agents query instead of loading into context.

- **Implementation:** Knowledge artifacts stay on disk. Agents query via semantic search when they need specific information. No pre-loading.
- **Token savings:** Up to 80% reduction in knowledge injection tokens.
- **Tradeoff:** Agents need search capability and may pay latency cost for retrieval.

**Strategy 2: Context Inheritance**
Parent agent (orchestrator) compiles shared context once. Child agents inherit a reference, not a copy.

- **Implementation:** Orchestrator generates a "context package" for the team — a compiled, compressed summary of relevant rules and knowledge. Each agent receives a pointer to this package rather than the raw content.
- **Token savings:** Shared context loaded once (~4K) instead of 8 times (~32K) = 28K saved.
- **Tradeoff:** Requires infrastructure for shared context packages. Currently not supported by Claude Code agent teams.

**Strategy 3: Role-Specific Context Scoping**
Each agent role gets only the context relevant to its specific function.

- **Implementation:** Instead of loading all governance rules, each role gets a pre-compiled context bundle:
  - Implementer: coding standards + architecture decisions + error handling
  - Reviewer: quality checks + acceptance criteria + review methodology
  - Writer: documentation standards + pillar alignment + template
- **Token savings:** Each agent loads 1-2K tokens of role-specific context instead of 4-8K of generic context.
- **Tradeoff:** Requires maintaining role-specific context bundles. Risk of missing cross-cutting concerns.

**Strategy 4: De-duplication at Injection Time**
Track which knowledge has been injected and skip duplicates.

- **Implementation:** Already partially exists in OrqaStudio's knowledge-injector (dedup cache). Extend to cover rules, not just knowledge artifacts.
- **Token savings:** Moderate (prevents loading the same artifact twice per agent).
- **Tradeoff:** Only prevents within-agent duplication, not across-agent.

### 7.3 Recommended Approach for OrqaStudio

**Layer 1: Role-specific context bundles** (highest impact)
Pre-compile a small, focused context package for each universal role. Load it at agent spawn. This replaces the current "inject all critical rules + category rules" approach.

**Layer 2: On-demand knowledge retrieval** (reduces shared context)
Don't pre-load knowledge artifacts. Let agents query semantic search when they encounter a domain-specific question. This eliminates the duplicate knowledge injection across agents.

**Layer 3: De-duplication within agents** (existing, improve)
Raise the MIN_SCORE threshold from 0.25 to 0.40 for semantic search results. Deduplicate between declared knowledge and search results.

---

## 8. Recommended Token Efficiency Architecture for OrqaStudio

### 8.1 Core Principle

**Generated system prompts contain only what the agent needs for its current task.** No more, no less.

### 8.2 Architecture Overview

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

### 8.3 Implementation Components

**Component 1: Compiled System Prompts**
Replace the current "CLAUDE.md + 58 rule files" with generated, role-specific system prompts.

| Agent Type | Static Core | Workflow Stage | On-Demand | Total Budget |
|-----------|-------------|---------------|-----------|-------------|
| Orchestrator | 1,500 | 500 | 500 | 2,500 |
| Implementer | 800 | 500 | 1,500 | 2,800 |
| Reviewer | 600 | 300 | 1,000 | 1,900 |
| Researcher | 400 | 200 | 1,500 | 2,100 |
| Writer | 500 | 300 | 1,000 | 1,800 |
| Designer | 500 | 300 | 1,000 | 1,800 |

**How to generate:**
1. Each workflow stage (plan, implement, review, learn) has a compiled prompt template
2. Template includes only the rules and knowledge relevant to that stage
3. Rules are pre-compressed to structured summaries (~100-150 tokens each)
4. Full rules are available on-demand via semantic search

**Component 2: Model Tiering**
Default model selection by role (see Section 4). Orchestrator selects model at delegation time. Task artifacts can override.

**Component 3: Token Budget Enforcement**
Each agent spawn has a token budget. The orchestrator monitors cumulative session cost and adjusts strategy:
- Under budget: Continue normally
- Approaching budget: Switch remaining agents to Sonnet
- Over budget: Warn user, suggest session scope reduction

**Component 4: KV-Cache Optimization**
Structure system prompts for maximum KV-cache reuse:
- Static core at the TOP of every prompt (cached across turns)
- Dynamic content (task description, tool results) at the BOTTOM
- Avoid re-ordering sections between turns (breaks cache)

Following Manus's insight: with cached tokens at $0.30/MTok vs. uncached at $3/MTok, a 10x cost difference, prompt structure that maximizes cache hits is critical.

**Component 5: Findings-to-Disk Enforcement**
Orchestrator never reads full findings files into its own context. Pattern:
1. Agent writes findings to `tmp/team/<team>/task-<id>.md` with a standardized summary header
2. Orchestrator reads ONLY the summary header (~200 tokens)
3. For detailed review, orchestrator delegates to a Reviewer agent

**Component 6: Token Tracking**
Hook-based tracking integrated into the existing PostToolUse pipeline:
- Capture tokens per API call, attributed to session/team/agent/task
- Write metrics to `tmp/token-metrics.jsonl`
- Summary in session state
- Future: dashboard in OrqaStudio UI

### 8.4 Expected Impact

| Metric | Current | After Optimization | Improvement |
|--------|---------|-------------------|-------------|
| Per-prompt overhead (orchestrator) | 9,500-16,500 tokens | 2,000-3,500 tokens | 65-80% reduction |
| Per-agent spawn cost | 6,400 tokens | 1,500-4,000 tokens | 40-75% reduction |
| 8-agent team spawn | 63K tokens | 16-36K tokens | 43-75% reduction |
| Session total (20 prompts, 2 teams) | ~300K+ tokens | ~80-120K tokens | 60-73% reduction |
| Overhead ratio (theoretical min) | 13.4x | 2-4x | 70% improvement |

### 8.5 Implementation Priority

| Priority | Component | Effort | Token Savings | Risk |
|----------|-----------|--------|---------------|------|
| P1 | Compiled system prompts (workflow-embedded) | Medium | 35-40% | Low |
| P1 | Findings-to-disk enforcement | Low | 5-10% | Very Low |
| P1 | Role-specific context bundles | Medium | 15-25% | Low |
| P2 | Model tiering | Medium | 20-30% cost | Medium |
| P2 | On-demand knowledge retrieval | Medium | 10-15% | Low |
| P2 | Token tracking hooks | Low | Enables optimization | Very Low |
| P3 | KV-cache optimization | Low | 5-15% cost | Very Low |
| P3 | Token budget enforcement | Medium | Prevents blowouts | Low |
| P3 | Dashboard UI | High | Visibility | Very Low |

---

## 9. Specific Metrics and Dashboards for Token Usage Tracking

### 9.1 Proposed Metrics Taxonomy

**Efficiency Metrics (core - always tracked):**

| Metric ID | Name | Calculation | Target |
|-----------|------|-------------|--------|
| TE-01 | Overhead Ratio | actual_tokens / theoretical_min_tokens | < 4x |
| TE-02 | Context Utilization | relevant_tokens / total_input_tokens | > 60% |
| TE-03 | Cache Hit Rate | cached_tokens / total_input_tokens | > 70% |
| TE-04 | Tokens Per Deliverable | total_tokens / completed_tasks | Decreasing trend |
| TE-05 | Rule Injection Efficiency | used_rules / loaded_rules | > 50% |

**Cost Metrics (business - tracked per session):**

| Metric ID | Name | Calculation | Target |
|-----------|------|-------------|--------|
| TC-01 | Session Cost | sum(tokens * price_per_token) | Budget-dependent |
| TC-02 | Cost Per Task | session_cost / completed_tasks | Decreasing trend |
| TC-03 | Model Mix Ratio | opus_tokens / total_tokens | < 40% |
| TC-04 | Waste Cost | (irrelevant_context_tokens * price) | Decreasing trend |

**Health Metrics (operational - alerts):**

| Metric ID | Name | Calculation | Alert Threshold |
|-----------|------|-------------|----------------|
| TH-01 | Stale Agent Count | agents_alive_past_timeout | > 0 |
| TH-02 | Context Window Pressure | current_tokens / max_tokens | > 80% |
| TH-03 | Search Availability | search_server_responding | false |
| TH-04 | Agent Spawn Failure Rate | failed_spawns / total_spawns | > 10% |

### 9.2 Dashboard Wireframes

**Dashboard 1: Session Token Overview**

```
+--------------------------------------------------+
| Session: 2026-03-25 14:30                        |
| Duration: 45 min | Tasks: 6 | Agents: 12        |
+--------------------------------------------------+
| [============================] 78K / 200K budget  |
+--------------------------------------------------+
| Input: 62K | Output: 14K | Cached: 41K (66%)     |
| Cost: $0.34 | Overhead: 3.2x | Efficiency: 68%   |
+--------------------------------------------------+
|                                                  |
| Token Usage Timeline                             |
| [line chart: tokens per turn over session]       |
|                                                  |
+--------------------------------------------------+
```

**Dashboard 2: Agent Cost Breakdown**

```
+--------------------------------------------------+
| Agent           | Role    | Model  | Tokens | $  |
|-----------------|---------|--------|--------|----|
| orchestrator    | Coord   | Opus   | 22K    |.12 |
| impl-backend    | Impl    | Sonnet | 18K    |.05 |
| impl-frontend   | Impl    | Sonnet | 15K    |.04 |
| reviewer        | Review  | Sonnet | 12K    |.03 |
| researcher      | Research| Sonnet | 8K     |.02 |
| writer          | Writer  | Sonnet | 3K     |.01 |
+--------------------------------------------------+
```

**Dashboard 3: Efficiency Analysis**

```
+--------------------------------------------------+
| Overhead Ratio: 3.2x (target: < 4x)   [green]  |
+--------------------------------------------------+
| Context Breakdown (last turn):                   |
| [===] Static core: 1,500 (42%)                   |
| [==] Workflow stage: 500 (14%)                    |
| [=] Task-specific: 800 (22%)                     |
| [=] Knowledge (on-demand): 600 (17%)             |
| [] Waste: 200 (5%)                               |
+--------------------------------------------------+
| Rules Loaded: 4 | Rules Referenced: 3 (75%)      |
| Knowledge Loaded: 2 | Knowledge Used: 2 (100%)   |
+--------------------------------------------------+
```

**Dashboard 4: Trends (7-day)**

```
+--------------------------------------------------+
| Cost Per Task Trend                              |
| [line chart: decreasing from $0.12 to $0.05]     |
|                                                  |
| Overhead Ratio Trend                             |
| [line chart: decreasing from 8x to 3x]          |
|                                                  |
| Model Mix                                        |
| [stacked bar: Opus decreasing, Sonnet increasing]|
+--------------------------------------------------+
```

### 9.3 Reporting Outputs

**Per-Session Report** (written to `tmp/session-metrics.md`):
```markdown
## Token Usage Report
- Total: 78,432 tokens (62,100 input, 14,332 output, 2,000 reasoning)
- Cached: 41,200 (66% hit rate)
- Cost: $0.34
- Overhead ratio: 3.2x (target: < 4x)
- Model mix: 28% Opus, 72% Sonnet

## Agent Breakdown
| Agent | Tokens | Cost | Efficiency |
|-------|--------|------|------------|
...

## Optimization Opportunities
- Rule "RULE-7b770593" loaded 8 times, referenced 2 times (75% waste)
- Agent "governance-steward" could use Sonnet (current: Opus)
- 3 knowledge artifacts loaded but never referenced
```

---

## 10. Key Insights and Cross-Cutting Themes

### 10.1 Context Engineering is the New Discipline

The field has moved from "prompt engineering" (writing better prompts) to "context engineering" (designing the full information environment). This is not just about reducing tokens — it's about ensuring the right information reaches the right agent at the right time.

Anthropic's own guidance: "Every token you add to the context window competes for the model's attention. Stuff a hundred thousand tokens of history into the window and the model's ability to reason about what actually matters degrades."

### 10.2 Context Rot is Real

Research from 2025 shows that "longer context doesn't automatically translate to better performance." Context rot — the unpredictable degradation of performance as input context expands — means that reducing context isn't just about cost. It directly improves agent quality.

This validates OrqaStudio's current experience: the 13x overhead ratio isn't just expensive — it's actively harmful to agent reasoning quality.

### 10.3 The 90/10 Rule

Mem0 research shows that intelligent memory compression can achieve 90% token reduction (from 26K to 1.8K per conversation) while maintaining or improving quality. This suggests the vast majority of context in current systems is redundant.

### 10.4 Cache Architecture Matters More Than Compression

Manus's finding that KV-cache hit rate is the single most important production metric suggests that prompt structure (keeping static content at the top, dynamic at the bottom) may be more impactful than content compression. A 10x cost difference between cached and uncached tokens dwarfs most compression gains.

### 10.5 Tool RAG is the Future for Large Tool Libraries

Claude Code's own Tool Search Tool achieved 85% token reduction while maintaining accuracy. This validates the on-demand loading approach for OrqaStudio's 58 rules — treat them as a "tool library" that's searched, not loaded.

---

## Sources

- [The Great AI Agent Showdown of 2026](https://topuzas.medium.com/the-great-ai-agent-showdown-of-2026-openai-autogen-crewai-or-langgraph-7b27a176b2a1)
- [LangGraph vs AutoGen vs CrewAI Comparison](https://latenode.com/blog/platform-comparisons-alternatives/automation-platform-comparisons/langgraph-vs-autogen-vs-crewai-complete-ai-agent-framework-comparison-architecture-analysis-2025)
- [CrewAI vs LangGraph vs AutoGen - DataCamp](https://www.datacamp.com/tutorial/crewai-vs-langgraph-vs-autogen)
- [LLMLingua - Microsoft Research](https://www.microsoft.com/en-us/research/blog/llmlingua-innovating-llm-efficiency-with-prompt-compression/)
- [LLMLingua-2 Paper](https://arxiv.org/abs/2403.12968)
- [Prompt Compression Techniques - Medium](https://medium.com/@kuldeep.paul08/prompt-compression-techniques-reducing-context-window-costs-while-improving-llm-performance-afec1e8f1003)
- [Tool RAG - Red Hat](https://next.redhat.com/2025/11/26/tool-rag-the-next-breakthrough-in-scalable-ai-agents/)
- [Agentic RAG Survey](https://arxiv.org/abs/2501.09136)
- [Traditional RAG vs Agentic RAG - NVIDIA](https://developer.nvidia.com/blog/traditional-rag-vs-agentic-rag-why-ai-agents-need-dynamic-knowledge-to-get-smarter/)
- [From RAG to Context - 2025 Review](https://ragflow.io/blog/rag-review-2025-from-rag-to-context)
- [Token Usage Tracking - Statsig](https://www.statsig.com/perspectives/tokenusagetrackingcontrollingaicosts)
- [15 AI Agent Observability Tools 2026](https://aimultiple.com/agentic-monitoring)
- [AI Cost Tracking - Coralogix](https://coralogix.com/platform/ai-observability/cost-tracking/)
- [AI Cost Observability - TrueFoundry](https://www.truefoundry.com/blog/ai-cost-observability)
- [LLM Cost Optimization and Multi-Model Routing](https://atlosz.hu/en/blog/llm-koltsegoptimalizalas-routing-strategia/)
- [AI Agent Cost Optimization Guide 2026](https://moltbook-ai.com/posts/ai-agent-cost-optimization-2026)
- [The Multi-Model Routing Pattern](https://dev.to/askpatrick/the-multi-model-routing-pattern-how-to-cut-ai-agent-costs-by-78-1631)
- [SC-MAS: Cost-Efficient Multi-Agent Systems](https://arxiv.org/abs/2601.09434)
- [Amazon Bedrock AgentCore Memory](https://aws.amazon.com/blogs/machine-learning/amazon-bedrock-agentcore-memory-building-context-aware-agents/)
- [Memory in LLM-based Multi-agent Systems](https://www.techrxiv.org/users/1007269/articles/1367390/master/file/data/LLM_MAS_Memory_Survey_preprint_/LLM_MAS_Memory_Survey_preprint_.pdf)
- [Orchestrator-Worker Agents Comparison - Arize AI](https://arize.com/blog/orchestrator-worker-agents-a-practical-comparison-of-common-agent-frameworks/)
- [Memory for AI Agents: Context Engineering - New Stack](https://thenewstack.io/memory-for-ai-agents-a-new-paradigm-of-context-engineering/)
- [Agent Memory - Letta](https://www.letta.com/blog/agent-memory)
- [Multi-Agent Context Sharing Patterns - Fast.io](https://fast.io/resources/multi-agent-context-sharing-patterns/)
- [Why Multi-Agent Systems Need Memory Engineering - O'Reilly](https://www.oreilly.com/radar/why-multi-agent-systems-need-memory-engineering/)
- [Context Engineering for AI Agents: Lessons from Building Manus](https://manus.im/blog/Context-Engineering-for-AI-Agents-Lessons-from-Building-Manus)
- [Effective Context Engineering - Anthropic](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents)
- [How we built our multi-agent research system - Anthropic](https://www.anthropic.com/engineering/multi-agent-research-system)
- [Equipping agents with Agent Skills - Anthropic](https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills)
- [Architecting efficient context-aware multi-agent framework - Google](https://developers.googleblog.com/architecting-efficient-context-aware-multi-agent-framework-for-production/)
- [Google ADK Documentation](https://google.github.io/adk-docs/)
- [Mem0 - Memory Layer for AI Apps](https://mem0.ai/)
- [Mem0 Research Paper](https://arxiv.org/abs/2504.19413)
- [Claude Code Agent Teams](https://code.claude.com/docs/en/agent-teams)
- [Claude Opus 4.6 - VentureBeat](https://venturebeat.com/technology/anthropics-claude-opus-4-6-brings-1m-token-context-and-agent-teams-to-take)
- [Evaluating Context Compression - Factory.ai](https://factory.ai/news/evaluating-compression)
- [ACON: Optimizing Context Compression](https://arxiv.org/html/2510.00615v1)
- [Context Compression - Google ADK](https://google.github.io/adk-docs/context/compaction/)
- [Efficient Context Management - JetBrains Research](https://blog.jetbrains.com/research/2025/12/efficient-context-management/)
- [Context Engineering - Martin Fowler](https://martinfowler.com/articles/exploring-gen-ai/context-engineering-coding-agents.html)
- [Langfuse Token & Cost Tracking](https://langfuse.com/docs/observability/features/token-and-cost-tracking)
- [Best AI Observability Platforms 2025](https://softcery.com/lab/top-8-observability-platforms-for-ai-agents-in-2025)
- [AI Agent Monitoring Best Practices 2026](https://uptimerobot.com/knowledge-hub/monitoring/ai-agent-monitoring-best-practices-tools-and-metrics/)
