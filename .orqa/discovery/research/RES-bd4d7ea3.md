---
id: RES-bd4d7ea3
title: "Knowledge Plugin Architecture"
type: discovery-research
status: active
category: architecture
description: "Knowledge registration, injection tiers, conflict resolution — framework comparison and recommended architecture"
created: 2026-03-25
updated: 2026-03-25
tags:
  - agent-teams
  - plugin-architecture
---

# Research: Knowledge Plugin Architecture for AI Agent Systems

## Status: Complete
## Researcher: knowledge-architecture-researcher
## Date: 2026-03-25

---

## 1. Framework Comparison: How AI Frameworks Handle Knowledge Management

### 1.1 LangChain / LangGraph

**Architecture**: LangChain treats knowledge as retrievable content accessed through tools. Knowledge is not "registered" with agents — instead, agents discover and invoke retrieval tools at runtime via function calling.

**Knowledge loading**: Dynamic. Agents use VectorStoreRetriever tools to query knowledge bases on demand. There is no static preloading of knowledge into agent context.

**Registration**: Tools (including knowledge retrievers) are registered with a Kernel object. The LLM sees tool schemas and decides which to invoke. This is pull-based — the agent decides what knowledge it needs.

**Plugin model**: LangChain Integrations provide vector store connectors (Chroma, Pinecone, Weaviate), document loaders, and embedding models as swappable components. No formal "knowledge plugin" manifest.

**Conflict handling**: No built-in conflict resolution. If two retrieval tools return contradictory information, the LLM reconciles based on prompt instructions.

**Key insight**: LangChain evolved from prompt-based planners to function-calling-based planning (LangGraph). The industry trend is away from predefined plans toward dynamic tool selection by the LLM itself.

### 1.2 Semantic Kernel (Microsoft)

**Architecture**: Three-pillar design — Plugins (capabilities), Planners (orchestration), Memory (context). Plugins encapsulate AI capabilities that can be composed together.

**Knowledge loading**: Hybrid. Semantic Memory provides long-term vector-based knowledge retrieval. Plugins can also be "semantic functions" — templated prompts that encode domain knowledge.

**Registration**: Plugins register with the Kernel via `AddFromType<T>()` or `add_plugin()`. Each plugin declares functions with descriptions. The kernel's function calling system auto-discovers available capabilities.

```csharp
builder.Plugins.AddFromType<LightsPlugin>("Lights");
```

**Plugin model**: Strongly typed plugin classes with decorated functions. Each function has a description that the LLM uses for selection. This is the most formal registration pattern among the frameworks.

**Conflict handling**: Plugin functions are namespaced (`PluginName.FunctionName`). The LLM selects which plugin to invoke based on descriptions. No explicit priority system — the LLM resolves ambiguity.

**Key insight**: Semantic Kernel's deprecated Stepwise/Handlebars planners prove that pre-computed plans scale poorly. Function calling (dynamic selection) won. But the strongly-typed plugin registration pattern with descriptions is the gold standard for discoverability.

### 1.3 CrewAI

**Architecture**: Most relevant to OrqaStudio. Knowledge attaches at two levels — agent-specific and crew-wide (shared). Uses RAG internally with vector storage.

**Knowledge loading**: Static at kickoff. Knowledge sources are embedded every time `crew.kickoff()` runs. This is a known inefficiency for large datasets.

**Registration**: Two-level attachment:
```python
# Agent-level (private knowledge)
agent = Agent(role="Specialist", knowledge_sources=[specialist_knowledge])

# Crew-level (shared knowledge)
crew = Crew(agents=[agent], tasks=[task], knowledge_sources=[crew_knowledge])
```

**Knowledge source types**: StringKnowledgeSource, TextFileKnowledgeSource, PDFKnowledgeSource, CSVKnowledgeSource, ExcelKnowledgeSource, JSONKnowledgeSource, CrewDoclingSource (web). Custom sources extend `BaseKnowledgeSource`.

**Configuration**: `KnowledgeConfig` with `results_limit` (default 3) and `score_threshold` (default 0.35).

**Storage**: ChromaDB (default) or Qdrant. Provider-neutral RAG client abstraction.

**Conflict handling**: No explicit conflict resolution. Crew-level knowledge is available to all agents. Agent-level knowledge is scoped. If both levels provide contradictory information, the agent receives both and must reconcile.

**Key insight**: CrewAI's two-level knowledge model (agent vs crew) directly maps to OrqaStudio's bidirectional knowledge model (knowledge ↔ agent). The BaseKnowledgeSource extension point is the plugin pattern.

### 1.4 Google ADK (Agent Development Kit)

**Architecture**: Composition-based. BaseAgent is the foundation. Three categories: LLM Agents (reasoning), Workflow Agents (deterministic flow), Custom Agents (specialized logic).

**Knowledge loading**: Tool-mediated. Knowledge is accessed through Tools, MCP integrations, and "Skills" (pre-packaged behaviors).

**Registration**: Tools and integrations register with agents at construction time. MCP toolsets provide dynamic capability discovery. Custom Service Registration for extensibility.

**Plugin model**: McpToolset primitive allows adding third-party integrations with minimal code. Graph-based workflows (ADK 2.0) enable complex agent composition.

**Conflict handling**: No built-in knowledge conflict resolution. Agent hierarchy determines precedence.

**Key insight**: ADK's separation of concerns — LLM Agents for reasoning, Workflow Agents for flow — is directly applicable. The McpToolset pattern shows how plugin knowledge can be loaded via standard protocols.

### 1.5 AG2 (formerly AutoGen)

**Architecture**: Event-driven core, async-first, pluggable orchestration. Layered and extensible design with clearly divided responsibilities.

**Knowledge loading**: Dynamic. Tools are registered with agents and invoked at runtime. Memory system for persistent knowledge across conversations.

**Registration**: `register_reply_method()` for custom behaviors. Tools registered per-agent. MCP integration via McpWorkbench.

**Plugin model**: Pluggable components — custom agents, tools, memory, and models. Community extensions ecosystem.

**Conflict handling**: GroupChat patterns allow agents to negotiate. No formal knowledge conflict resolution.

**Key insight**: AG2's event-driven architecture and pluggable component model are closest to what OrqaStudio needs for a dynamic plugin system. The layered design allows different abstraction levels.

### 1.6 PydanticAI

**Architecture**: Dependency injection-first. Knowledge and services are injected into agents via typed `RunContext`.

**Knowledge loading**: Dynamic injection at runtime. Dependencies are declared as types and provided as instances during execution.

**Registration**:
```python
@dataclass
class MyDeps:
    api_key: str
    http_client: httpx.AsyncClient

agent = Agent('openai:gpt-5.2', deps_type=MyDeps)
result = await agent.run('prompt', deps=MyDeps('key', client))
```

**Key insight**: PydanticAI's dependency injection pattern is the cleanest for knowledge injection. Dependencies are type-safe, testable, and can be overridden. This is the most adaptable pattern for OrqaStudio's context.

---

## 2. Options Analysis: Static vs Dynamic vs Hybrid Knowledge Loading

### 2.1 Static Loading (All Knowledge at Spawn)

**How it works**: When an agent is spawned, all declared knowledge is loaded into its system prompt or context before any task begins.

**Pros**:
- Predictable context — agent always has full knowledge
- No runtime overhead for retrieval
- Simpler implementation — load once, use throughout
- Knowledge is guaranteed available (no retrieval failures)

**Cons**:
- Context window waste — loads knowledge that may not be needed for the current task
- Scales poorly — as knowledge grows, context window fills with irrelevant material
- LLMs struggle to distinguish valuable information when "flooded with unfiltered information" (RAGFlow 2025 review)
- Every agent spawn pays the full loading cost even for simple tasks
- Static snapshots go stale if knowledge updates mid-session

**Best for**: Small knowledge sets (<5 documents), critical safety constraints that must always be present, agent role definitions.

### 2.2 Dynamic Loading (On-Demand Retrieval)

**How it works**: Agent has no pre-loaded knowledge. It queries knowledge bases as needed during task execution via RAG tools or semantic search.

**Pros**:
- Minimal context usage — only loads what's needed for the current query
- Scales to large knowledge bases
- Always retrieves latest version of knowledge
- Agent can refine queries based on initial results (agentic RAG pattern)

**Cons**:
- Retrieval latency adds to every knowledge access
- Agent may not know to search for knowledge it doesn't know exists
- Retrieval failures cause task failures
- Quality depends on embedding/search quality
- No guarantee that critical knowledge will be loaded

**Best for**: Large knowledge bases, frequently updated content, research tasks, fact-checking.

### 2.3 Hybrid Loading (Baseline + On-Demand) — RECOMMENDED

**How it works**: A baseline of critical knowledge is loaded statically at agent spawn. Task-specific knowledge is loaded dynamically based on context signals (file paths, task type, domain tags).

**Pros**:
- Critical knowledge is always present (safety, architectural constraints)
- Task-relevant knowledge is loaded just-in-time without wasting context
- Scales — baseline stays small, dynamic loading handles the long tail
- Context signals enable intelligent pre-fetching without agent initiative
- Matches the "context engineering" trend: "providing the right information at the right time"

**Cons**:
- More complex implementation — need baseline selection + dynamic injection
- Must define what counts as "baseline" vs "on-demand"
- Risk of loading duplicate knowledge (baseline + dynamic both hit same artifact)

**Best for**: OrqaStudio's use case. The three-tier model already describes this: Tier 1 (baseline/always), Tier 2 (orchestrator-injected based on task), Tier 3 (context-resolved).

### Recommendation for OrqaStudio

The existing three-tier model in RULE-dd5b69e6 already implements hybrid loading:

| Tier | Loading Strategy | What |
|------|-----------------|------|
| Tier 1 | Static (always loaded) | Universal skills + role knowledge |
| Tier 2 | Semi-dynamic (orchestrator-injected) | Task-specific knowledge |
| Tier 3 | Dynamic (context-resolved) | File-path-triggered injection |

The plugin architecture should extend this model: plugins declare which tier their knowledge operates at.

---

## 3. Knowledge Registration Patterns

### 3.1 Pattern A: Manifest-Based Registration (Plugin Declares Offerings)

The plugin provides a manifest file describing what knowledge it offers and which agent types should receive it.

```yaml
# rust-plugin/orqa-plugin.json
{
  "name": "rust",
  "knowledge": [
    {
      "id": "KNOW-rust-error-handling",
      "title": "Rust Error Handling Patterns",
      "target_agents": ["implementer", "reviewer"],
      "target_capabilities": ["shell_execute"],
      "injection_tier": 2,
      "triggers": {
        "file_patterns": ["**/*.rs", "**/Cargo.toml"],
        "task_tags": ["rust", "backend", "error-handling"]
      }
    },
    {
      "id": "KNOW-rust-async",
      "title": "Rust Async Patterns",
      "target_agents": ["implementer"],
      "injection_tier": 2,
      "triggers": {
        "file_patterns": ["**/*.rs"],
        "task_tags": ["async", "tokio", "futures"]
      }
    }
  ]
}
```

**Pros**: Explicit, declarative, easy to validate, supports tooling (UI can show which plugins provide which knowledge).

**Cons**: Requires manual maintenance when agent types change. Plugin author must know the target system's agent types.

**Used by**: Semantic Kernel (function descriptions in plugin decorators), Microsoft 365 Copilot (plugin manifests).

### 3.2 Pattern B: Capability-Based Matching (Semantic Discovery)

Knowledge declares its domain tags. Agents declare their domain needs. A matching engine connects them.

```yaml
# Knowledge artifact
---
id: KNOW-rust-error-handling
domain_tags: ["rust", "error-handling", "result-types", "thiserror"]
applicable_to:
  roles: ["implementer", "reviewer"]
  when_editing: ["**/*.rs"]
---

# Agent definition
---
id: AGENT-implementer
domain_needs: ["${project.stack.languages}", "error-handling"]
---
```

**Pros**: Loose coupling — plugins don't need to know exact agent IDs. Automatic discovery via tag matching. Scales to many plugins.

**Cons**: Semantic matching can be imprecise. "rust" might match "rust removal" in a cleaning domain. Requires a robust tagging vocabulary.

**Used by**: MCP Registry (capability-based discovery), Agent Discovery protocols (semantic profiling).

### 3.3 Pattern C: Dependency Injection (Agent Declares Needs)

Agents declare typed dependencies. The system resolves them from available plugins at spawn time.

```yaml
# Agent definition
---
id: AGENT-implementer
knowledge_deps:
  - type: "language-patterns"
    for: "${task.stack.primary_language}"
  - type: "framework-patterns"
    for: "${task.stack.framework}"
  - type: "error-handling"
    for: "${task.stack.primary_language}"
---

# Plugin registration
---
plugin: rust
provides:
  - type: "language-patterns"
    for: "rust"
    artifact: KNOW-rust-patterns
  - type: "error-handling"
    for: "rust"
    artifact: KNOW-rust-error-handling
---
```

**Pros**: Type-safe. Agent defines what it needs, not where to get it. Easy to test (swap in mock knowledge). Clean separation of concerns.

**Cons**: More complex resolution logic. Must handle "no provider found" gracefully.

**Used by**: PydanticAI (RunContext deps), Semantic Kernel (kernel service resolution), traditional DI frameworks.

### 3.4 Recommended Approach for OrqaStudio: Hybrid Manifest + Capability Matching

Combine Patterns A and B:

1. **Plugins declare offerings via manifest** (Pattern A) — what knowledge they provide, what file patterns trigger it
2. **Agent definitions declare needs via capability tags** (Pattern B) — what domains they need knowledge for
3. **The graph engine matches** offerings to needs using both explicit `target_agents` and semantic `domain_tags`
4. **Fallback to semantic search** when explicit matching fails — use the existing ONNX search engine

This preserves OrqaStudio's graph-first architecture while enabling automatic knowledge discovery.

---

## 4. Knowledge Conflict Resolution Strategies

### 4.1 The Problem

When two plugins provide knowledge for the same domain (e.g., Plugin A says "use unwrap() for prototyping" and Plugin B says "never use unwrap()"), the system must resolve the conflict.

### 4.2 Strategy Comparison

| Strategy | How It Works | Best For | Risk |
|----------|-------------|----------|------|
| **Priority layers** | Knowledge has a priority: core > project > plugin > community | Clear hierarchy, deterministic | Rigid — can't override core knowledge |
| **Specificity wins** | More specific knowledge overrides general knowledge | Natural resolution — "rust error handling" beats "general error handling" | Hard to measure "specificity" |
| **Last-write wins** | Most recently installed/updated knowledge takes precedence | Simple to implement | Silently overrides good knowledge |
| **Explicit override** | Plugin declares `overrides: KNOW-xxx` to explicitly replace | Intentional, auditable | Requires plugin author to know what to override |
| **Merge with flags** | Conflicting knowledge is merged, conflicts flagged to user | No silent data loss | Noisy — many false positives |
| **Scope isolation** | Knowledge scoped to specific contexts, never globally merged | Clean boundaries | May miss cross-scope insights |

### 4.3 Recommended Strategy for OrqaStudio: Layered Priority with Explicit Override

**Layer priority** (higher wins):
1. **Project rules** (`.orqa/process/rules/`) — always wins. These are the user's explicit constraints.
2. **Project knowledge** (`.orqa/process/knowledge/` with `layer: project`) — project-specific patterns
3. **Plugin knowledge** (from installed plugins) — domain expertise
4. **Core knowledge** (shipped with OrqaStudio, `layer: core`) — universal methodology

**Explicit override mechanism**:
```yaml
# Plugin knowledge that intentionally overrides core
---
id: KNOW-rust-error-handling
overrides: KNOW-general-error-handling
override_scope: "when editing **/*.rs files"
priority: plugin
---
```

**Conflict detection**: When knowledge is installed, the graph engine checks for:
- Same `domain_tags` + same `target_agents` = potential conflict
- Same `triggers.file_patterns` = potential conflict
- Explicit `overrides` field = intentional replacement (no conflict)

**Conflict resolution at injection time**:
1. Check if any knowledge has `overrides` pointing to the other → use the overriding one
2. Check priority layers → higher layer wins
3. If same layer, check specificity (more specific file pattern wins)
4. If still tied, inject both and let the LLM reconcile (with a system note about the conflict)

---

## 5. Knowledge Structure for Maximum Reusability

### 5.1 Approaches Compared

| Approach | Description | Pros | Cons |
|----------|-------------|------|------|
| **Monolithic documents** | One large knowledge document per domain | Easy to write, complete context | Wastes context window, hard to compose |
| **Atomic fragments** | Small, single-concept knowledge pieces | Precise injection, composable | Loses cross-concept context, many files |
| **Hierarchical trees** | Tree of knowledge with parent-child relationships | Natural organization, can load at right depth | Complex graph relationships |
| **Layered composites** | Base + override layers, each self-contained | Extensible, plugin-friendly | Resolution complexity |

### 5.2 Recommended: Hierarchical with Atomic Leaves

OrqaStudio already uses flat knowledge files (KNOW-xxx.md). The plugin architecture should add:

**Knowledge tree structure per plugin**:
```
rust-plugin/
  knowledge/
    KNOW-rust-overview.md          # Summary — loaded as Tier 1 baseline
    KNOW-rust-error-handling.md    # Detailed — loaded Tier 2 when editing .rs
    KNOW-rust-async-patterns.md    # Detailed — loaded Tier 2 when async code detected
    KNOW-rust-unsafe.md            # Critical safety — loaded Tier 1 always
```

**Each knowledge artifact is self-contained** — it can be injected alone without requiring other knowledge from the same plugin. But it can reference related knowledge via `relationships` for agents that want to explore deeper.

**Size guideline**: Each knowledge artifact should be **500-2000 tokens** when rendered. Smaller is better for injection. Larger documents should be split into focused pieces.

### 5.3 Schema Example for Plugin Knowledge

```yaml
---
id: KNOW-rust-error-handling
type: knowledge
title: Rust Error Handling Patterns
description: How to compose errors using thiserror, when to use anyhow vs thiserror, Result type patterns
status: active
category: domain
version: 1.0.0
layer: plugin
plugin: rust
user-invocable: false

# Plugin-specific fields
injection:
  tier: 2
  triggers:
    file_patterns: ["**/*.rs"]
    task_tags: ["error-handling", "rust"]
  target_agents: ["implementer", "reviewer"]

# Conflict resolution
overrides: null
priority: plugin
domain_tags: ["rust", "error-handling", "result-types"]

relationships:
  - target: KNOW-rust-overview
    type: child-of
  - target: AGENT-implementer
    type: enriches
  - target: AGENT-reviewer
    type: enriches
---
```

---

## 6. Knowledge Versioning and Compatibility

### 6.1 The Problem

When a plugin updates its knowledge, running agents may have stale knowledge in their context. This is especially important in long-running sessions.

### 6.2 Patterns from the Industry

**MCP approach**: String-based version identifiers (YYYY-MM-DD format). Protocol version only incremented for backwards-incompatible changes. Incremental improvements preserved without breaking clients.

**Semantic Versioning**: `MAJOR.MINOR.PATCH` where MAJOR = breaking changes to knowledge structure, MINOR = new knowledge additions, PATCH = content corrections.

**Contract testing**: Verify that updated knowledge still satisfies the expectations of agents that depend on it.

### 6.3 Recommended Approach for OrqaStudio

**Version field on knowledge artifacts** (already exists: `version: 0.2.0`).

**Compatibility rules**:
- PATCH updates (0.2.0 → 0.2.1): Content corrections. No structural change. Safe to hot-reload.
- MINOR updates (0.2.0 → 0.3.0): New sections added, no sections removed. Safe to hot-reload.
- MAJOR updates (0.2.0 → 1.0.0): Structural changes, sections removed or renamed. Requires re-evaluation of dependent agents.

**Hot-reload strategy**: Knowledge artifacts are markdown files on disk. The graph engine already watches for file changes. When a knowledge file changes:
1. If PATCH/MINOR: Update the graph node, invalidate any cached embeddings
2. If MAJOR: Flag to the user that dependent agents may need review
3. Currently-running agents keep their loaded knowledge until session end (no mid-session disruption)

**Version pinning**: Plugins can pin knowledge versions in their manifest:
```json
{
  "knowledge_version_policy": "minor",  // accept patch and minor updates automatically
  "pinned_versions": {
    "KNOW-rust-error-handling": ">=1.0.0 <2.0.0"
  }
}
```

---

## 7. Recommended Architecture for OrqaStudio's Knowledge Plugin System

### 7.1 Design Principles

1. **Graph-first**: Knowledge registration goes through the artifact graph, not side channels
2. **Bidirectional**: Knowledge → Agent edges (enriches) and Agent → Knowledge edges (employs)
3. **Plugin-provided**: Plugins declare knowledge offerings via manifest; the install process creates graph nodes
4. **Context-triggered**: Injection happens based on file patterns, task tags, and agent roles — not manual wiring
5. **Layered priority**: Project rules > Project knowledge > Plugin knowledge > Core knowledge
6. **Self-contained artifacts**: Each knowledge piece is independently injectable

### 7.2 The Flow

```
Plugin Installed
  ↓
orqa install reads plugin manifest
  ↓
For each knowledge entry in manifest:
  → Create KNOW-xxx.md in .orqa/process/knowledge/ (managed copy per RULE-plugin-consumption)
  → Create graph edges: KNOW → target agents (enriches)
  → Create graph edges: target agents → KNOW (employs)
  → Register file pattern triggers for Tier 2/3 injection
  ↓
Agent Spawned for Task
  ↓
Tier 1: Load agent's explicit knowledge list (from agent YAML frontmatter)
  ↓
Tier 2: Orchestrator queries graph for task-relevant knowledge
  → Match task tags against knowledge domain_tags
  → Match task files against knowledge triggers.file_patterns
  → Inject matched knowledge into delegation prompt
  ↓
Tier 3: File-path-triggered injection (PostToolUse hook on Write/Edit)
  → When agent writes to a file matching a trigger pattern
  → Inject the corresponding knowledge as systemMessage
  → Deduplicate (don't re-inject if already loaded this session)
```

### 7.3 Plugin Manifest Schema (Knowledge Section)

```jsonc
{
  "name": "rust",
  "version": "1.0.0",
  "knowledge": {
    "artifacts": [
      {
        "id": "KNOW-rust-error-handling",
        "title": "Rust Error Handling Patterns",
        "description": "thiserror composition, Result patterns, error propagation",
        "tier": 2,
        "target_roles": ["implementer", "reviewer"],
        "domain_tags": ["rust", "error-handling", "result-types", "thiserror"],
        "triggers": {
          "file_patterns": ["**/*.rs"],
          "task_tags": ["error-handling"]
        },
        "overrides": null,
        "source": "knowledge/rust-error-handling.md"
      },
      {
        "id": "KNOW-rust-unsafe",
        "title": "Rust Unsafe Code Safety",
        "description": "When unsafe is acceptable, safety invariants, review requirements",
        "tier": 1,
        "target_roles": ["implementer", "reviewer"],
        "domain_tags": ["rust", "unsafe", "safety"],
        "triggers": null,
        "overrides": null,
        "source": "knowledge/rust-unsafe.md"
      }
    ],
    "version_policy": "minor",
    "conflict_resolution": "layer_priority"
  }
}
```

### 7.4 Agent Definition Extension

Agents gain a `knowledge_needs` field for dependency-injection-style matching:

```yaml
---
id: AGENT-implementer
knowledge:
  # Explicit knowledge (always loaded)
  - implementer-tree

knowledge_needs:
  # Dynamic needs resolved from installed plugins
  - type: "language-patterns"
    for: "${project.stack.languages}"
  - type: "framework-patterns"
    for: "${project.stack.frameworks}"
  - type: "error-handling"
    for: "${project.stack.languages}"
---
```

The graph engine resolves `knowledge_needs` against installed plugin manifests at agent spawn time.

### 7.5 Conflict Detection and Resolution

At install time:
```
For each new KNOW artifact:
  1. Check existing knowledge with same domain_tags + target_roles
  2. If found:
     a. If new knowledge has overrides pointing to existing → intentional replacement
     b. If same layer → warn user of potential conflict
     c. If different layers → layer priority applies automatically
  3. Record conflict edges in graph for visibility
```

At injection time:
```
For each knowledge artifact matching current context:
  1. Group by domain_tags
  2. Within each group, sort by layer priority
  3. Inject highest-priority knowledge
  4. If multiple at same priority, inject all with a conflict note
```

---

## 8. MCP Integration Considerations

The Model Context Protocol is increasingly the standard for tool/knowledge discovery. OrqaStudio should consider:

1. **Knowledge as MCP Resources**: Plugin knowledge artifacts could be exposed as MCP resources, queryable by any MCP-compatible client
2. **MCP Registry pattern**: OrqaStudio's plugin registry could follow the MCP Registry pattern — machine-readable catalog, capability-based filtering
3. **OAuth 2.1 scoping**: MCP's November 2025 spec added incremental scope negotiation — knowledge injection could use similar patterns (grant access to specific knowledge domains as needed)

---

## 9. Key Takeaways

1. **The industry has converged on hybrid loading** — static baseline + dynamic retrieval. OrqaStudio's three-tier model already captures this.

2. **Manifest-based registration is the standard** for plugin ecosystems. Semantic Kernel and MCP both use descriptive manifests. OrqaStudio should extend `orqa-plugin.json` with a `knowledge` section.

3. **Bidirectional edges are unique to OrqaStudio**. No framework we studied has true bidirectional knowledge ↔ agent registration. This is a competitive advantage — the graph engine makes it possible.

4. **Conflict resolution must be layered**. The priority stack (project rules > project knowledge > plugin knowledge > core) is the most robust pattern. Explicit `overrides` fields handle intentional replacements.

5. **Knowledge artifacts should be atomic and self-contained** (500-2000 tokens each). This enables precise injection and composability across plugins.

6. **Versioning should use semver** with hot-reload for non-breaking changes and user notification for breaking changes.

7. **Context engineering is the new RAG** — the trend is toward intelligent context assembly rather than brute-force retrieval. OrqaStudio's path-based injection triggers and orchestrator-mediated loading already implement this pattern.

---

## Follow-up Items

- None — research is complete. Architecture decisions should be captured in the synthesis document.
