# OrqaStudio Architecture Reference

This document describes the **intended architecture** of OrqaStudio. It serves as the benchmark for all development work and as the target state that the codebase must converge toward. Anything that contradicts this document is either legacy pollution or a deliberate, documented exception.

Sources: RES-d6e8ab11 (Agent Team Design v2), AD-1ef9f57c (Resolved Questions), and architectural clarifications from the 2026-03-26 codebase audit.

---

## 1. What OrqaStudio Is

A **plugin-composed governance platform** for AI-assisted development. The engine provides core capabilities as Rust library crates (graph, workflow, state machine, prompt pipeline, search, enforcement). Plugins provide definitions (methodology, workflows, artifact types, state machines, knowledge). Nothing is hardcoded in the engine — the engine provides capabilities, plugins provide definitions.

The app itself is composable. It provides the UI and composition infrastructure but is an empty shell without plugins. Without a methodology plugin there is no workflow to follow. Without workflow plugins there are no stages to execute. Without a sidecar there is no LLM inference. The app is the connective tissue that composes together different ways of working.

### Product Pillars

| Pillar | Meaning |
|--------|---------|
| **Clarity Through Structure** | Making thinking, standards, and decisions visible and structured |
| **Purpose Through Continuity** | Preventing drift between intention and action |
| **Learning Through Reflection** | Improving over time through structured retrospection |

The product's USP is that continuous learning and reflection is built in — but even this is defined as a plugin, reinforcing that everything in the ecosystem is composable.

### Core Product Principles

- **Accuracy over speed.** When making trade-offs, prefer correctness over latency.
- **Mechanical enforcement enables autonomy.** The more you can mechanically verify, the less human review is needed, and the more trust can be placed in agents to work independently.
- **The learning loop hardens the system.** Every failure is a candidate for a new mechanical guard. The system grows smarter over time by converting lessons into prevention.
- **Zero tech debt.** No legacy code, no legacy artifacts, no "we'll clean this up later." If it doesn't serve the target architecture, it is removed — not deferred, not commented out, not marked as TODO. Nothing survives that could influence future development in the wrong direction. When removing code, leave no comments documenting what was removed — that's code smell, not documentation. Comments describe active code. Every file should have a comment describing its purpose. Every function should have a comment describing what it does and why. Good inline documentation for active code; zero documentation of dead code. **File migration is never a blind copy.** Every file being moved to a new location must be reviewed against the target architecture: keep if it aligns, adapt if the concept is valid but implementation is stale, drop if it's obsolete. Moving old content into new directories just relocates tech debt — it doesn't eliminate it.

---

## 2. Design Principles (Load-Bearing Constraints)

These are not aspirational. They are architectural constraints that every file, component, and design decision must satisfy.

### P1: Plugin-Composed Everything

No governance pattern is hardcoded in the engine. Methodology, workflows, artifact types, state machines, knowledge artifacts, and agent specializations all come from plugins. The engine provides **capabilities**, not **definitions**.

### P2: One Context Window Per Task

Each agent spawns fresh for a single task with a precisely-scoped prompt. No persistent agents, no session-long lifecycles, no accumulated context. This eliminates context drift and "context rot."

### P3: Generated, Not Loaded

System prompts are generated programmatically from plugin registries and workflow state -- not loaded wholesale from disk. The prompt pipeline assembles only what the agent needs for its current task. Full text available on-demand via semantic search; compressed summaries are the default.

### P4: Declarative Over Imperative

State machines, guards, actions, and workflow definitions are YAML declarations validated by JSON Schema -- not code. Plugin authors write configuration, not functions.

### P5: Token Efficiency as Architecture

Token efficiency is a first-class architectural constraint that shapes every decision. Target: 2-4x overhead ratio, down from 13.4x. Per-agent prompts: 1,500-4,000 tokens (down from 9,500-16,500).

### P6: Hub-Spoke Orchestration with Mandatory Review

A persistent orchestrator coordinates ephemeral task-scoped workers. The orchestrator never sees worker-level implementation details — it reads structured summaries from findings files.

Every task completed by an agent MUST be independently verified by a Reviewer agent before it is accepted. The orchestrator does NOT review — it delegates review. This applies to all work, not just migration:

1. Agent (Implementer/Writer/Governance Steward) completes task, writes findings
2. Orchestrator spawns a Reviewer agent to independently verify against acceptance criteria
3. Reviewer produces PASS/FAIL verdict with evidence
4. If FAIL: orchestrator re-assigns to the original agent with reviewer feedback
5. If PASS: orchestrator marks the task complete

The orchestrator reads the Reviewer's verdict, not the implementer's self-assessment. No task is complete without a PASS verdict from an independent Reviewer.

After receiving a Reviewer PASS, the orchestrator performs its own gate check before marking the task complete — verifying the verdict is consistent with the acceptance criteria, the architecture, and the zero tech debt standard. This double-layer check (Reviewer + Orchestrator gate) prevents drift from accumulating across tasks.

**No autonomous decisions.** If something isn't clear in the task description, the agent refers to the architecture documentation. If still not clear, the agent raises it to the orchestrator — it does NOT make its own interpretation and proceed. The orchestrator compiles all ambiguities and discoveries into a list for human review before the task moves forward. If that blocks work, so be it. No room for mistakes.

**Discovery during execution:** Both implementing agents AND reviewers must report anything they encounter that was not covered by the task list — legacy code, stale artifacts, inconsistencies, undocumented dependencies, or anything that would violate the zero tech debt standard if left unaddressed. Agents do NOT act on these discoveries themselves — they report them back to the orchestrator. The orchestrator compiles all discoveries into a list for user review. The user decides what to action. Nothing discovered during implementation is silently ignored or autonomously resolved.

### P7: The Resolved Workflow Is a File on Disk

After plugin contributions are merged, the resolved workflow is a YAML file on disk -- deterministic, diffable, and inspectable. The runtime reads this resolved file; it does not re-merge at runtime.

---

## 3. System Architecture

### 3.1 Language Boundary

**Rust** is the base language for all libraries, the CLI, and the daemon. **TypeScript** is the frontend language only (SvelteKit UI). This is a clean boundary: Rust below, TypeScript only at the UI surface.

### 3.2 Engine Libraries

The engine is a collection of **Rust library crates**, each covering a functional domain. These are not a single monolith — they are independently consumable by any access layer.

| Crate | Domain |
|-------|--------|
| `graph` | Artifact relationships, traceability, inverse computation |
| `workflow` | State machine evaluation, transition resolution, guard/action execution |
| `prompt` | Five-stage prompt generation from plugin registries |
| `search` | Semantic search over governance artifacts via ONNX embeddings |
| `enforcement` | Rule evaluation, artifact validation, coding standards, linting config generation |
| `plugin` | Installation, composition, schema generation, content management |
| `agent` | Base role definitions, task-specific agent generation from role + workflow + knowledge |

These crates are consumed by multiple frontends. No access layer IS the engine — they all consume it.

MCP and LSP are **access protocols** that expose engine capabilities to consumers. They are NOT application boundaries. Business logic belongs in the engine crates, not in the protocols.

### 3.3 Access Layers

The engine libraries are consumed through four access layers:

```
                    +-----------------------+
                    |   ENGINE LIBRARIES    |
                    |   (Rust crates)       |
                    |                       |
                    | graph, workflow,      |
                    | prompt, search,       |
                    | enforcement, plugin,  |
                    | agent                 |
                    +-----------+-----------+
                                |
         +----------+-----------+-----------+----------+
         |          |                       |          |
    +----+---+ +----+----+          +-------+--+ +----+----+
    | Daemon | |   App   |          |   CLI    | |Connector|
    |        | |         |          |          | |         |
    | File   | | Tauri + |          | Thin     | | Generates|
    | watchers| | Svelte  |          | wrapper  | | tool-   |
    | MCP/LSP| | UI      |          | around   | | native  |
    | System | |         |          | engine   | | plugins |
    | tray   | | Requires|          | crates   | |         |
    |        | | sidecar |          |          | | Watches |
    |        | | plugins |          |          | | for     |
    |        | | for LLM |          |          | | changes |
    +--------+ +---------+          +----------+ +---------+
```

**Daemon:** A persistent Rust process that runs file watchers, serves MCP/LSP, manages the system tray icon, and consumes the engine libraries. The system tray represents the daemon running and provides a context menu for process status, launching the app, etc. The daemon handles persistent runtime concerns — it outlives the app.

**App + Sidecar:** The desktop application (SvelteKit frontend + Tauri backend) hooks into the engine libraries for all business logic. It is the custom UI for interacting with the engine. Because the app has no built-in LLM inference, it requires **sidecar plugins** that integrate with LLM providers. The tool executor in the app provides a consistent set of tools via MCP to any sidecar instance. Connectors can map these to the third-party tool's native tools for better performance.

**CLI:** A thin Rust wrapper around the engine libraries. An access method, not business logic. Commands like `orqa install`, `orqa check`, `orqa graph` delegate entirely to the engine crates.

**Connector:** Uses the engine to **generate** a tool-native plugin (e.g., a Claude Code Plugin). Once generated, the third-party tool interacts with the engine directly via CLI/MCP. The connector also **watches** for changes to plugins, rules, and composition and regenerates in real time. The generated output goes where the third-party tool expects it (e.g., `.claude/` for Claude Code).

**App and connector are peers** — both are interfaces to the same underlying engine. A project is ready for dogfooding when the same methodology and principles are applied identically whether working via the app or via a connector-generated plugin.

### 3.4 Sidecars and Connectors Are Parallel Concepts

Both bridge the gap between the engine and LLM capability, from different directions:

| | Sidecar | Connector |
|---|---------|-----------|
| **Direction** | Provides LLM inference TO the app | Generates config FOR a tool that already has inference |
| **Why needed** | The app has no native inference | The third-party tool doesn't know about OrqaStudio |
| **Composable** | Yes — multiple sidecars for different providers | Yes — multiple connectors for different tools |
| **Required** | At least one (app is useless without inference) | Optional (only needed if using third-party tools) |
| **Runtime role** | Active — routes inference requests | Generation-time — generates then watches for changes |

### 3.5 OrqaStudio Plugins vs Tool-Native Plugins

This distinction is critical:

- **OrqaStudio Plugin** — a package within the OrqaStudio ecosystem that provides definitions to the engine, extends the app, or both. Managed by `orqa install`.
- **Tool-Native Plugin** (e.g., Claude Code Plugin) — the **output artifact** generated by a connector. It is what the target agent framework consumes. It is NOT an OrqaStudio plugin.

A connector is an OrqaStudio plugin that generates a tool-native plugin.

### 3.6 Storage

Data lives with the process that manages it. The engine libraries define storage traits, and each consumer provides its own implementation:

- **Daemon** — manages persistent operational data (health snapshots, metrics, search index)
- **App** — manages UI-specific data (window state, preferences)
- **CLI** — reads and writes directly to disk, no persistent state

### 3.7 Git Integration

Git operations (via Forgejo as local version control, users, organisations) are an engine concern. The engine library handles git integration directly rather than through external infrastructure services.

---

## 4. Plugin Architecture

### 4.1 Plugin Purposes

A plugin serves one or more of these purposes:

| Purpose | What It Provides | Effect at Install Time |
|---------|-----------------|----------------------|
| **Methodology definition** | The overarching workflow skeleton and stage slots | Triggers full schema/workflow recomposition |
| **Workflow definition** | A complete sub-workflow for one methodology stage | Triggers full schema/workflow recomposition |
| **Knowledge/rules** | Domain expertise, behavioral constraints, documentation | Assets installed. No schema recomposition, but rule changes trigger enforcement regeneration. |
| **App extension** | Custom views, UI components | Assets installed, no recomposition |
| **Sidecar** | LLM provider integration for the app | Provides inference capability |
| **Connector** | Generation pipeline for a third-party tool | Generates + watches for regeneration |
| **Infrastructure** | Tooling integrations (linting config generation) | Generates enforcement configs from engine rules |

A single plugin may serve multiple purposes. The `orqa-plugin.json` manifest is the single source of truth for what each plugin provides and what installation actions are needed. Manifests must declare: plugin purpose/type, stage slot (if workflow), content types provided, and installation targets.

### 4.2 Methodology and Workflows

This distinction is fundamental to composability:

**Methodology** = the overarching approach (e.g., Agile). Defined by a methodology plugin. The methodology plugin provides the workflow skeleton with named contribution points (slots) that workflow plugins fill. One and only one methodology plugin may be installed per project.

**Workflows** = the sub-workflows within the methodology. Each workflow plugin provides the complete sub-workflow for one stage of the methodology — including its own state machine (states, transitions, guards, gates) and artifact types. Each workflow is self-contained with no inheritance from other plugins. One workflow plugin per stage defined in the methodology.

Relationships define the flow both **within** a workflow (e.g., task delivers epic, research informs decision) and **between** workflows and the methodology (e.g., discovery outputs feed planning inputs, implementation delivers against planning). The graph engine computes inverses from forward-only declared relationships, making the entire methodology traceable end-to-end.

### 4.3 Plugin Taxonomy

#### Methodology Plugin

| Plugin | Role |
|--------|------|
| `agile-workflow` | Defines the Agile methodology skeleton and stage slots |

Other methodologies could be created as alternative plugins. A project installs exactly one.

#### Workflow Plugins

Each fills one stage slot defined by the methodology:

| Plugin | Methodology Stage | What It Provides |
|--------|------------------|-----------------|
| `agile-discovery` | Discovery | Workflows for ideas, research, personas, pillars, vision, pivots |
| `agile-planning` | Planning | Workflows for planning decisions, planning ideas, planning research, wireframes |
| `agile-documentation` | Documentation | Documentation contribution workflow |
| `software-kanban` | Implementation | Epic, task, milestone workflows + kanban/roadmap views (dual-purpose) |
| `agile-review` | Review | Review contribution workflow |
| `core` | Learning | The learning loop — lessons, decisions, knowledge, rules. The app's USP. Unified and `uninstallable: true` — also provides framework artifact schemas and git hooks/enforcement. |

#### Domain Knowledge Plugins

Provide expertise without defining methodology or workflows. Do not trigger schema recomposition, but rule changes can trigger enforcement config regeneration.

| Plugin | Domain | Notes |
|--------|--------|-------|
| `cli` | CLI tool domain knowledge | |
| `rust` | Rust development patterns | Dual-purpose: knowledge + generates linting infrastructure from engine |
| `svelte` | Svelte development patterns | |
| `tauri` | Tauri development patterns | |
| `typescript` | TypeScript configs and patterns | Dual-purpose: knowledge + generates linting infrastructure from engine |
| `coding-standards` | Code quality rules and tooling | Generates enforcement configs |
| `systems-thinking` | Systems design knowledge | |
| `plugin-dev` | Plugin development knowledge | |

#### Connector Plugins

| Plugin | Target Framework | Output |
|--------|-----------------|--------|
| `claude-code` | Claude Code | A Claude Code Plugin (`.claude/` directory) |

### 4.4 Plugin Installation Constraints

Enforced by `orqa install`:

1. **One methodology plugin** per project. Error if a second is installed.
2. **One workflow plugin per stage** defined by the methodology. Error if two plugins claim the same stage or a workflow plugin targets a nonexistent stage.
3. **Definition plugins** (methodology, workflow) trigger full recomposition of schemas, resolved workflows, and state machines.
4. **Non-definition plugins** (knowledge, views, sidecars, infrastructure) only install their assets — no recomposition triggered.
5. The `orqa-plugin.json` manifest determines what each plugin provides and what actions are needed at install time.

### 4.5 Plugin Content Installation

When `orqa install` runs, plugins copy their content into the project's `.orqa/` directory. Plugin authors determine where in the hierarchy their artifacts are installed — the destination should make sense as part of a holistic, human-readable collection. Two plugins can install into the same category if it makes topical sense.

The `manifest.json` tracks which files came from which plugin (source hash + installed hash) to enable three-way diff: plugin source vs installed baseline vs project copy.

Source workflow definitions are NOT copied to `.orqa/` — they stay in the plugin directories. Only the resolved output gets written to `.orqa/workflows/`.

### 4.6 Composition Pipeline

The composition pipeline runs whenever a definition plugin is installed (via `orqa plugin install` or as part of the dev environment `orqa install`):

1. Read the installed methodology plugin's workflow skeleton
2. Read each installed workflow plugin's contribution manifest (from plugin directories, not copies)
3. Merge contributions into the methodology's stage slots
4. Compose the full JSON schema from all plugin-provided artifact types and state machines
5. Validate the composed result
6. Write resolved workflows to `.orqa/workflows/<stage>.resolved.yaml` — one file per stage, containing all artifact types, state machines, relationships for that stage
7. Write composed schema (`schema.composed.json`) for LSP/MCP validation
8. Write prompt registry for the prompt pipeline

The runtime reads only resolved files. Recomposition happens only on plugin install/update. Connectors watch for changes and regenerate their output in real time.

---

## 5. Governance Artifacts (`.orqa/`)

The `.orqa/` directory should contain what **a fully working app would generate**. It is not a dumping ground for hand-crafted files accumulated during development. Every artifact should be judged against: "would the finished app have created this file, in this format, in this location?"

### 5.1 Target Structure

The target structure reflects the methodology's stages and the engine's artifact categories. It should be human-navigable — organized by purpose, not by hash.

```
.orqa/
  project.json                    # Project configuration
  manifest.json                   # Installed plugin registry (source/installed hashes)
  schema.composed.json            # Generated: composed schema from all definition plugins
  prompt-registry.json            # Generated: knowledge registry for prompt pipeline
  search.duckdb                   # Semantic search index

  workflows/                      # Generated: resolved workflows, one per methodology stage
    methodology.resolved.yaml     # The full resolved methodology
    discovery.resolved.yaml       # Resolved discovery workflow
    planning.resolved.yaml        # Resolved planning workflow
    documentation.resolved.yaml   # Resolved documentation workflow
    implementation.resolved.yaml  # Resolved implementation workflow
    review.resolved.yaml          # Resolved review workflow
    learning.resolved.yaml        # Resolved learning workflow

  discovery/                      # Discovery stage
    ideas/                        # DISCOVERY-IDEA artifacts
    research/                     # DISCOVERY-RESEARCH artifacts
    decisions/                    # DISCOVERY-DECISION artifacts
    personas/                     # PERSONA artifacts
    pillars/                      # PILLAR artifacts
    vision/                       # VISION artifacts
    pivots/                       # PIVOT artifacts
    wireframes/                   # WIREFRAME artifacts

  planning/                       # Planning stage
    ideas/                        # PLANNING-IDEA artifacts
    research/                     # PLANNING-RESEARCH artifacts
    decisions/                    # PLANNING-DECISION artifacts
    wireframes/                   # WIREFRAME artifacts (planning-scoped)

  documentation/                  # Documentation stage
    <categorized by topic>/       # Organized into meaningful subdirectories by plugin authors
      *.md                        # Full documentation (DOC artifacts)
      knowledge/                  # Agent-consumable chunks derived from the docs (KNOW artifacts)

  implementation/                 # Implementation stage
    milestones/                   # MS artifacts
    epics/                        # EPIC artifacts
    tasks/                        # TASK artifacts
    ideas/                        # Implementation-scoped ideas

  learning/                       # Learning stage
    lessons/                      # IMPL (lesson) artifacts
    decisions/                    # PRINCIPLE-DECISION artifacts (overarching architecture)
    rules/                        # RULE artifacts
```

**Key organizational principle:** The directory structure maps to methodology stages. Artifacts live within the stage they belong to, organized by type within each stage. This mirrors the navigation structure in the app.

**Key differences from current state:**

- **Stage-first organization** — directories map to methodology stages, not artifact types
- No `process/` nesting — gone entirely
- No `delivery/` — replaced by `implementation/` (the methodology stage name)
- No `agents/` directory — agents are ephemeral (generated and discarded), not tracked
- No `grounding/` directory — grounding content becomes `tier: always` knowledge in plugins
- No source workflow definitions — only resolved output (sources stay in plugin directories)
- Decisions split by level: `planning/decisions/` (tactical) and `learning/decisions/` (architectural/principle)
- Knowledge lives WITH documentation — knowledge is documentation split into agent-consumable chunks with injection metadata
- Wireframes are their own artifact type, not DOC
- Resolved workflows named by stage, one per stage
- Composed schema and prompt registry are explicit generated artifacts

### 5.2 Relationships Define Flow

Relationships are the connective tissue of the governance model:

- **Within a workflow:** relationships define the flow between artifacts (e.g., task delivers epic, research informs decision)
- **Between workflows and methodology:** relationships define how each workflow's outputs connect to the broader methodological flow (e.g., discovery outputs feed planning inputs, implementation delivers against planning)

The graph engine computes inverses from forward-only declared relationships. The relationship types are semantic bonds that make the entire methodology traceable end-to-end.

---

## 6. Agent Architecture (Target State)

### 6.1 Base Roles

Eight base agent roles define high-level responsibilities, permissions, and behavioral boundaries. These are stable definitions provided by the methodology plugin:

| Role | Responsibility | Permission Scope |
|------|---------------|-----------------|
| **Orchestrator** | Coordinates work, delegates tasks, reads summaries | Read-only, delegation |
| **Implementer** | Writes code, runs tests | Source code, shell access |
| **Reviewer** | Verifies quality, produces verdicts | Read-only, checks only |
| **Researcher** | Investigates questions, gathers information | Read-only, creates research artifacts |
| **Writer** | Creates and edits documentation | Documentation only |
| **Planner** | Designs approaches, maps dependencies | Plans and delivery artifacts |
| **Designer** | Creates UI/UX designs, component structures | Design artifacts, component code |
| **Governance Steward** | Maintains governance artifacts, ensures process compliance | `.orqa/` artifacts only |

Each base role defines:

- Behavioral boundaries (what the agent may/may not do)
- Tool constraints (which tools it can use)
- Artifact scope (what it can create/edit)
- Permission scope (enforced by the generated tool-native plugin)

### 6.2 Task-Specific Agent Generation

The agents that get used in practice are **generated on a bespoke basis** for each task. The engine generates them by composing:

```
Base Role + Workflow Context + Domain Knowledge = Task-Specific Agent
```

**Base Role** (from methodology plugin): defines permissions and boundaries.

**Workflow Context** (from active workflow): provides workflow-specific instructions. An Implementer in the implementation workflow gets different context than an Implementer in the documentation workflow.

**Domain Knowledge** (from knowledge plugins): composed into the agent based on task scope, file paths, and subject matter. Selected at delegation time, not predefined.

The **engine** provides the generation functionality (prompt pipeline, knowledge injection, role composition). The **connector** provides translation so the generated agents are defined in the third-party tool's native format (e.g., Claude Code's agent structure).

Agents are **ephemeral** — generated and discarded, not tracked as artifacts in the graph. The base role definitions in the methodology plugin are the canonical source, and the prompt pipeline is the set process for composing them into runtime specialists. There is no agent artifact type, no agent workflow, and no AGENT-*.md files.

### 6.3 Token Budgets

Token budgets apply to the generated task-specific agents — they constrain the output of the prompt pipeline, not the base role definitions.

| Generated Agent Type | Total Budget |
|---------------------|-------------|
| Orchestrator | 2,500 tokens |
| Implementer | 2,800 tokens |
| Reviewer | 1,900 tokens |
| Researcher | 2,100 tokens |
| Writer | 1,800 tokens |
| Planner | 2,500 tokens |
| Designer | 1,800 tokens |
| Governance Steward | 1,800 tokens |

The generator balances what needs to be **embedded knowledge** (in the system prompt, within budget) vs what should be **instructions to retrieve documentation via MCP** (on-demand). The general principle is accuracy over speed — err on the side of giving agents what they need to be correct, even if it means more MCP lookups.

---

## 7. Prompt Generation Pipeline (Target State)

```
Plugin Registry -> Schema Assembly -> Section Resolution -> Token Budgeting -> Prompt Output
```

| Stage | What Happens |
|-------|-------------|
| **Plugin Registry** | All installed plugins register prompt contributions at install time |
| **Schema Assembly** | For a (base role, workflow, task) tuple, collect applicable prompt sections |
| **Section Resolution** | Resolve references to compressed summaries; follow cross-refs depth 1 |
| **Token Budgeting** | Measure against budget; trim P3 first, then P2, then P1; never trim P0 |
| **Prompt Output** | Static core at TOP (cached), dynamic content at BOTTOM (changes per turn) |

The pipeline balances embedded knowledge vs MCP retrieval instructions based on priority, token budget, and task relevance. Accuracy is preferred over speed — on-demand retrieval latency is acceptable if it produces better outcomes.

---

## 8. Connector Architecture (Target State)

### 8.1 What a Connector Is

A connector is a special OrqaStudio plugin with two responsibilities:

1. **Generate** a tool-native plugin from the composed methodology, workflows, rules, and coding standards
2. **Watch** for changes to plugins, rules, and composition and **regenerate** in real time

The generated output goes directly where the third-party tool expects it (e.g., `.claude/` at the project root for Claude Code). Once generated, the third-party tool interacts with the engine directly (via CLI/MCP). The connector is not in the runtime path — it is a live generation pipeline.

The connector source lives in its own top-level directory alongside app, daemon, plugins, etc. It does NOT live inside `.orqa/`.

### 8.2 What the Generated Plugin Should Contain

| Component | Purpose |
|-----------|---------|
| Permission configuration | Role-scoped file access — works WITHOUT bypass permissions |
| Agent definitions | Generated from base roles + workflow context, in the tool's native format |
| Slash commands | Thin wrappers exposing OrqaStudio actions |
| Hook scripts | Marshal events to the engine (via CLI/MCP), apply responses — THIN |
| hooks.json | Generated from plugin hook declarations, not static |
| Validation rules | Generated from engine's artifact validation |

Git hooks and linting configs are NOT part of the generated tool-native plugin. Those come from their respective OrqaStudio plugins (core, coding-standards, typescript, rust) which install enforcement infrastructure directly.

The generated plugin enforces workflow constraints and agent permissions. Agents get scoped permissions matching their role — preventing them from modifying files outside their artifact scope.

### 8.3 What the Connector Source Should NOT Contain

| Anti-Pattern | Why It's Wrong | Where It Belongs |
|-------------|---------------|-----------------|
| Rule evaluation logic | Business logic | Engine enforcement crate |
| Knowledge injection algorithms | Business logic | Engine prompt crate |
| Artifact validation beyond format | Business logic | Engine enforcement crate |
| Prompt generation/assembly | Business logic | Engine prompt crate |
| Impact analysis logic | Business logic | Engine graph crate |
| Departure detection heuristics | Business logic | Engine enforcement crate |
| Knowledge artifacts | Workflow knowledge | Methodology plugin |
| Custom telemetry endpoints | Should use unified logging | Engine logging library |

The connector's code should be generation, translation, and file-watching logic only. If it contains `if/else` trees, scoring algorithms, or domain-specific heuristics, it has exceeded its role. The generated hooks should be thin: receive event -> call engine (via CLI/MCP) -> apply response.

### 8.4 Development Strategy

Because we have been building OrqaStudio while simultaneously applying it to Claude Code, business logic has leaked into the connector. The circular dependency — building OrqaStudio with OrqaStudio while OrqaStudio is still being defined — has polluted the codebase. The path forward:

1. **Disconnect Claude Code** from the development process
2. **Hand-write the target Claude Code Plugin** — the ideal output that the connector should generate. This becomes a test fixture.
3. **Work backwards** — build the connector and engine infrastructure that would generate that ideal plugin.
4. **Test for completion:** turn on the generated version, turn off the hand-written one, verify no functionality is lost.

The same target-first approach applies to git hooks, linting configs, and validation rules: hard-code the target, then build the generation pipeline to produce it.

---

## 9. State Machine Design (Target State)

- Each workflow plugin owns its complete state machine for its artifact types
- The engine provides the state machine evaluation engine and primitives
- State machines are YAML, validated by the composed JSON schema
- States have categories: `planning`, `active`, `review`, `completed`, `terminal`
- Guards are declarative: field checks, relationship checks, graph queries
- Human gates are five-phase sub-workflows: GATHER, PRESENT, COLLECT, EXECUTE, LEARN
- No inheritance — each workflow plugin's state machine is self-contained
- Relationships between artifacts define flow within and across workflows

---

## 10. Enforcement Tooling (Target State)

Enforcement should be machine-applied, not just documented standards that agents are asked to follow. The more mechanical enforcement exists, the more trust can be placed in agents to work autonomously.

### 10.1 Enforcement Layers

| Layer | What It Enforces | Mechanism |
|-------|-----------------|-----------|
| **JSON Schema** (composed) | Artifact structure, required fields, valid types, valid relationships | LSP validation against composed schema |
| **Git hooks** | Pre-commit checks, relationship validation, artifact format, affected tests | Generated by core plugin from engine rules |
| **Linting configs** | Coding standards (eslint, clippy, prettier) | Generated by coding-standards/typescript/rust plugins |
| **Artifact validation** | Frontmatter correctness, ID format, type-location consistency, relationship validity, knowledge size (500-2000 tokens) | Engine enforcement crate, exposed via MCP/LSP |
| **Agent permissions** | File access scoped to role, artifact scope constraints | Generated tool-native plugin permission model |
| **Workflow guards** | Transition prerequisites, gate requirements, AC verification | Engine's state machine evaluation |

### 10.2 Validation Timing

Validation is enforced at two levels:

- **Write time (LSP):** immediate feedback while editing — frontmatter correctness, knowledge size constraints, schema compliance
- **Commit time (pre-commit):** catches anything that slipped through — artifact validation, relationship validity, affected tests

Pre-commit runs tests **scoped to what's changed** — not the full test suite, but tests affected by the staged changes.

### 10.3 Purpose-Specific Guards

Workflow guards should mechanically enforce acceptance criteria, not just check for green CI:

- A task that implements a function should verify that **a test exists that defines the function's purpose AND it passes** — not just that all tests pass
- AC enforcement should trace from the task's deliverable to its verification
- The more that can be mechanically verified, the less human review overhead is needed

### 10.4 The Learning Loop Hardens Enforcement

The learning loop should feed directly into enforcement evolution:

1. **Lesson captured:** "this mistake happened"
2. **Analysis:** "can we mechanically prevent this?"
3. **If yes:** create or update a guard, test, validation rule, or linting config
4. **Result:** the system hardens itself against past failures over time

Every lesson is a candidate for a new mechanical guard. Avenues for more mechanical enforcement should always be explored through the learning loop. The enforcement tooling grows organically from real failures — this is the Learning Through Reflection pillar in action.

---

## 11. Key Decisions

| Decision | Resolution | Reference |
|----------|-----------|-----------|
| Engine structure | Rust library crates per functional domain, consumed by all access layers | This document |
| Language boundary | Rust for all libs/CLI/daemon. TypeScript for frontend only. | This document |
| Daemon purpose | Persistent process: file watchers, MCP/LSP, system tray. Consumes engine crates. | This document |
| Access patterns | App+sidecar, connector, CLI, and daemon are peer consumers of engine crates | This document |
| Connector output | Generates to `.claude/` (or equivalent). Watches and regenerates on changes. | This document |
| Methodology vs Workflows | Methodology = overarching approach. Workflows = sub-workflows per stage. | This document |
| Decision levels | Two distinct types: `principle-decision` and `planning-decision` | This document |
| Base roles | 8 fixed roles; task-specific agents generated at runtime, ephemeral not tracked | This document |
| Agent artifacts | Removed. Agents are ephemeral. No agent type, workflow, or AGENT-*.md files. | This document |
| Core plugin | Unified (learning stage + framework schemas + git hooks). Uninstallable. | This document |
| Wireframes | Own artifact type, visible in planning navigation | This document |
| Resolved workflows | One file per stage, named by purpose, not per artifact type | This document |
| Source workflows | Stay in plugin directories. NOT copied to .orqa/. | This document |
| Storage | Data lives with the process that manages it. Storage traits in engine. | This document |
| Tool executor | Engine-level via MCP for sidecars. Connectors map to native tools. | This document |
| Git integration | Engine concern, not external infrastructure. | This document |
| Telemetry | Unified logger library. Future split into metrics + logger. | This document |
| Accuracy over speed | Core product principle for all trade-offs | This document |
| Workflow inheritance | No inheritance. Plugin owns complete state machine. | AD-1ef9f57c |
| Guard language | Declarative only. Code hooks for complex cases. | AD-1ef9f57c |
| Business logic boundary | Engine crates, not MCP/LSP. Protocols are access methods. | AD-1ef9f57c |
| Backwards compatibility | None during pre-release. `orqa migrate` for breaking changes. | AD-1ef9f57c |
| Summary generation | Author writes summaries. `orqa summarize` generates drafts. | AD-1ef9f57c |
| Relationship storage | Forward-only. Task stores `delivers: epic`; graph computes inverses. | CLAUDE.md |
| Session state | `.state/` not `tmp/`. Operational data, not disposable. | AD-8727f99a |

---

## 12. Proposed Codebase Structure

The directory layout should make architectural purposes self-evident:

```
orqastudio-dev/
  engine/                       # NOT a directory — engine crates live in libs/

  libs/                         # Rust library crates (engine functional domains)
    graph/                      # Artifact relationships, traceability
    workflow/                   # State machine evaluation, guards, actions
    prompt/                     # Prompt generation pipeline
    search/                     # Semantic search, ONNX embeddings
    enforcement/                # Rule evaluation, artifact validation, config generation
    plugin/                     # Plugin system, composition, installation
    agent/                      # Base roles, task-specific agent generation
    brand/                      # Brand assets, icons, design tokens

  daemon/                       # Persistent Rust process
    src/                        # File watchers, MCP/LSP servers, system tray

  app/                          # Desktop application (engine consumer)
    src/                        # SvelteKit frontend (TypeScript)
    src-tauri/                  # Tauri backend (Rust, thin wrapper around engine crates)

  cli/                          # Rust CLI tool (thin wrapper around engine crates)

  connectors/                   # Connector plugins (generation pipelines)
    claude-code/                # Generates Claude Code Plugin to .claude/

  plugins/                      # OrqaStudio plugins organized by type
    methodology/                # Methodology plugins (one at a time)
      agile-workflow/
    workflows/                  # Workflow plugins (one per stage)
      agile-discovery/
      agile-planning/
      agile-documentation/
      agile-review/
      software-kanban/
      core/                     # Learning stage + framework schemas + enforcement
    knowledge/                  # Domain knowledge plugins
      cli/
      rust/
      svelte/
      tauri/
      typescript/
      coding-standards/
      systems-thinking/
      plugin-dev/
  sidecars/                     # LLM provider integrations (top-level — unique purpose)
    claude-agent-sdk/

  models/                       # ONNX models for local semantic search
  templates/                    # Project scaffolding templates for orqa init
  scripts/                      # Migration and maintenance scripts
  infrastructure/               # Deployment tooling (Forgejo setup)
```

---

## 13. Migration Plan

### Principle: Target States First

Before building the generation pipelines, **hand-write the target outputs** as test fixtures. These represent what the finished system would produce. Development validates against these targets. The targets are only replaced by generated output once the generation code is complete and validated.

**Protection rule:** Target state files are protected during development. They must NOT be overwritten by work-in-progress code. A target is only replaced when the code that generates it produces output that matches or exceeds the hand-written version.

**Zero tech debt enforcement:** Every migration phase must leave zero legacy behind. No deprecated code, no stale artifacts, no commented-out blocks, no backwards-compatibility shims, no "follow-up" tasks that defer cleanup. If a file, function, artifact, or config doesn't serve the target architecture, it is deleted in the same phase that replaces it. Legacy code and artifacts left in the codebase WILL influence future agent behavior in the wrong direction — this is not theoretical, it is the exact problem this migration is solving.

**Regeneration safeguard:** Generation pipelines (connector, enforcement plugins) must support a **dry-run mode** controlled by an environment variable (`ORQA_DRY_RUN=true`). When dry-run is enabled, pipelines write their output to a comparison directory (e.g., `.state/dry-run/`) instead of overwriting live files. This allows generated output to be compared and validated against the hand-written targets without affecting the in-progress migration. The environment variable is set to `true` for the duration of the migration and switched to `false` only when the pipeline is validated. This applies to: connector generation (`.claude/`), enforcement config generation (git hooks, eslint, clippy, markdownlint, prettier), and resolved workflow generation.

### Phase 1: Establish Target States and Migration Enforcement

Hand-write the target outputs as test fixtures. These are the FIRST thing built — everything else validates against them. The enforcement tooling must be active before any other migration work begins.

#### Step 1: Target Schema + Validation Script (PREREQUISITE)

This is the foundation — without schema validation, nothing else can be verified:

1. **Write `targets/schema.composed.json`** — the full composed schema for all artifact types, relationships, valid statuses
2. **Write a temporary validation script** (`scripts/validate-artifacts.mjs` or similar) that checks governance artifacts against the target schema. This is a stopgap — it gets replaced by the engine's enforcement crate later. It must validate: required frontmatter fields per type, ID format, type-location consistency, relationship target existence, status validity, knowledge size constraints.
3. **Verify the script works** by running it against the current `.orqa/` directory — it should report the known issues from the audit.

#### Step 2: Install Enforcement Configs

Copy target enforcement configs from `targets/enforcement/` into the live project:

- `targets/enforcement/eslint/eslint.config.js` -> `app/eslint.config.js`
- `targets/enforcement/clippy/clippy.toml` -> workspace root `clippy.toml`
- `targets/enforcement/clippy/workspace-lints.toml` -> apply to `Cargo.toml` `[workspace.lints]`
- `targets/enforcement/markdownlint/.markdownlint.json` -> project root `.markdownlint.json`
- `targets/enforcement/prettier/.prettierrc` -> project root `.prettierrc`
- `targets/enforcement/prettier/.prettierignore` -> project root `.prettierignore`
- Install required dependencies (`prettier`, `prettier-plugin-svelte`, `prettier-plugin-tailwindcss`, `markdownlint-cli2`)

#### Step 3: Migration `.claude/` Instance

Write the migration-specific `.claude/` directory with hooks that call enforcement tooling **directly** (not thin daemon wrappers — the daemon doesn't exist yet):

- Hooks invoke eslint, clippy, markdownlint, the validation script, and scoped tests directly
- Artifact validation hook calls the temporary validation script against the target schema
- Hooks are more aggressive than the post-migration thin wrappers — they enforce everything at every step because the migration is critical
- Agent definitions include full migration context (architecture references, target state awareness, phase plan)
- **Artifact schema validation is DISABLED in hooks until Phase 6 (Content Cleanup) and Phase 7 (Governance Artifact Migration) are complete.** Enabling it before artifacts are fixed would block every commit. The validation script exists and can be run manually to track progress, but the pre-commit hook skips it via an `ORQA_SKIP_SCHEMA_VALIDATION=true` environment variable. This variable is removed when the artifacts are fixed and validation should be enforced.

#### Step 4: Remaining Targets

With enforcement in place, write the remaining targets:

| Target | What It Defines | Location |
|--------|----------------|----------|
| **Target Claude Code Plugin** | The ideal `.claude/` output the connector should generate | `targets/claude-code-plugin/` |
| **Target `.orqa/` structure** | Governance artifacts as the app would generate them | Applied directly to `.orqa/` |
| **Target resolved workflows** | One per stage, fully composed | `targets/workflows/` |
| **Target plugin manifests** | orqa-plugin.json for each plugin with correct taxonomy fields | `targets/plugin-manifests/` |

Additionally, the actual governance artifacts in `.orqa/` must be corrected to match the target state — correct artifact types, correct frontmatter, correct locations, correct relationships. The LSP/MCP then validates against the end-goal, not WIP, and can accurately identify issues.

These targets serve as:

1. **Test fixtures** — the generation pipeline must produce output matching these
2. **Validation benchmarks** — LSP/MCP validate against the target schema, not WIP
3. **Development reference** — developers see what the end state looks like
4. **Living data** — the `.orqa/` artifacts are correct and complete, ready for the app to display

See **Appendix A** for the detailed target state specifications.

### Phase 2: Engine Extraction

Extract business logic from Tauri backend and CLI into Rust library crates:

1. **Types and traits first** — shared type definitions, storage traits
2. **Graph crate** — artifact reader, relationship engine, traceability
3. **Enforcement crate** — rule evaluation, artifact validation (absorb app/.githooks/ logic)
4. **Search crate** — semantic search (already partially extracted)
5. **Workflow crate** — state machine evaluation, status transitions, guards
6. **Plugin crate** — installation, composition, schema generation
7. **Prompt crate** — prompt generation pipeline (absorb connector's prompt-injector and knowledge-injector)
8. **Agent crate** — base role definitions, task-specific agent generation
9. **Stream loop abstraction** — design proper traits for sidecar communication and event delivery
10. **Absorb app/tools/ engine logic** — `verify-pipeline-integrity.mjs`, `verify-installed-content.mjs`, `lint-relationships.mjs` implement engine-level business logic

### Phase 3: Daemon

Build the daemon as a standalone Rust process:

1. System tray icon with context menu (process status, app launch)
2. File watchers for plugin/rule/workflow changes
3. MCP server (consuming engine crates)
4. LSP server (consuming engine crates, validating against composed schema)
5. Unified logging (future: split into metrics + logger)

### Phase 4: Connector Cleanup

Refactor the connector to be pure generation + watching:

1. Move prompt classification logic to engine prompt crate
2. Move knowledge injection logic to engine prompt crate
3. Move context preservation logic to engine
4. Replace direct MCP IPC with daemon calls
5. Move connector knowledge artifacts to methodology plugin
6. Implement `generator.ts` (primary connector job) and `watcher.ts` (live regeneration)
7. Generate hooks.json from plugin declarations
8. Delete legacy `artifact-bridge.ts`
9. `.claude/` directory becomes connector output — stop hand-maintaining `.claude/agents/` and `.claude/CLAUDE.md`
10. Validate generated output against target Claude Code Plugin

### Phase 5: Plugin Manifest Standardization

Update all plugin manifests to support the architecture:

1. Add `purpose` field (methodology, workflow, knowledge, connector, infrastructure, sidecar)
2. Add `stage_slot` field for workflow plugins
3. Standardize category vocabulary to match taxonomy
4. Declare content installation targets (where in `.orqa/` hierarchy)
5. Implement installation constraint enforcement in `orqa install`
6. Standardize schema field naming — `title` not `name` across all artifact type schemas
7. Fix missing files referenced by manifests (KNOW-3f307edb, review-checklist.md) — either create in plugin source or remove references
8. Fix software-kanban contribution workflow missing from manifest
9. Fix agile-documentation inconsistent workflow declaration format (flat string -> structured object)
10. Rename plugins for clarity — names must make taxonomy self-evident (e.g., `agile-workflow` -> `agile-methodology`)

### Phase 6: Content Cleanup (Zero Dead Weight)

Review and clean ALL governance content. Nothing survives that isn't accurate, relevant, and forward-compatible.

#### Scripts

- Remove every script that isn't forward-compatible with the target architecture
- Dead migration scripts, obsolete tooling, one-time fixes that have run — all deleted
- Specifically: `migrate-artifact-ids.mjs`, `standardise-ids.mjs`, `fix-duplicate-frontmatter-keys.mjs`, `fix-missing-inverses.mjs`, `link-skills-to-docs.mjs`, `remove-inverse-relationships.mjs`, `migrate-types.mjs`, `rebuild-artifacts.mjs` — all completed migrations, delete
- Migration manifests (`id-migration-manifest.json`, `id-standardise-manifest.json`, `migration-manifest.json`) — delete or archive to `.state/migrations/`

#### Documentation and Knowledge

- Review all project documentation for accuracy — update or remove anything stale
- Review all knowledge artifacts for accuracy and duplication — merge duplicates, remove obsolete
- Ensure documentation and knowledge are sourced from the correct plugins (not orphaned project copies of plugin content)
- Organize into domain subdirectories for human navigation

#### Ideas

- Combine/group ideas that are thematically the same
- Archive ideas that are no longer relevant to the architecture

#### Decisions

- Review every decision for accuracy against the current architecture
- Archive decisions that were superseded or no longer apply
- Split into `principle-decision` and `planning-decision` types

#### Epics and Tasks

- Archive all epics/tasks that aren't about the path forward
- Ensure remaining epics/tasks align with this migration plan
- Clean status values (consistent quoting, valid statuses)

#### Lessons

- Review for ongoing relevance — archive lessons about superseded approaches
- Any valid lesson that can be guarded mechanically becomes a mechanical guard immediately — no recurrence threshold needed at this stage
- Convert applicable lessons into enforcement rules, validation checks, or workflow guards

#### Rules

- Review all rules for accuracy against the current architecture
- Classify every rule as **mechanical** (enforced by tooling) or **advisory** (guidance for agents/humans)
- Rules must support filtering/grouping by this classification
- Remove rules that contradict the plugin-composed architecture or are made redundant by it

#### .state/ Cleanup

- Delete empty team directories (e.g., `fix-sot/`)
- Establish cleanup policy: team findings are ephemeral — promote valuable content to governance artifacts, then delete
- CLI should provide `orqa dev clean-teams [--age <days>]` to prune stale team directories

#### Legacy File Removal

- Delete `validation_stderr.txt` (root)
- Delete `tmp/` directory (superseded by `.state/`)
- Delete `app/WORKING-DOCUMENT.md` (legacy)
- Remove vendored `node_modules` from `integrations/claude-agent-sdk/` (add to `.gitignore`)
- Remove legacy CLI aliases (no backwards compatibility per architecture)
- Fix CLI version hardcoding in `cli.ts` to read from `package.json` dynamically

### Phase 7: Governance Artifact Migration

Restructure `.orqa/` to match target structure:

1. Remove `process/` nesting — promote categories to top-level
2. Remove `agents/` directory — delete AGENT-*.md files
3. Remove `grounding/` directory — migrate to `tier: always` knowledge in plugins
4. Remove SKILL.md files from knowledge
5. Categorize knowledge and documentation into domain subdirectories
6. Fix wireframe artifact type (doc -> wireframe)
7. Fix personas directory (move DOC to documentation/)
8. Standardize frontmatter (title not name, required status, consistent quoting)
9. Regenerate resolved workflows as one-per-stage

### Phase 8: Codebase Restructure

Move directories to match the proposed structure:

1. Move engine crates to `libs/`
2. Create `daemon/` top-level directory
3. Restructure `plugins/` into taxonomy subdirectories
4. Move CLI to top-level `cli/`
5. Move claude-agent-sdk to top-level `sidecars/`
6. Remove sync-bridge (aspirational, not needed now)
7. Remove all dead scripts identified in Phase 5
8. Update all import paths, Cargo workspace, and package.json references
9. Update templates — remove `process/` prefix from content paths, add missing template types (methodology, workflow, knowledge-only, infrastructure, connector, sidecar)
10. Remove `file-audit/` directory (audit working files, not permanent)
11. Update `CLAUDE.md` to reflect the new architecture (or generate it via connector)
12. Reconcile relationship type count (41 in plugins vs 30 stated in CLAUDE.md)

### Phase 9: Frontend Alignment

Review and update the app frontend to work with the target architecture:

#### Navigation Structure

- **Dashboard** — top-level landing with insight widgets sourced from appropriate plugins. Ensure existing widgets work; detailed redesign is future scope.
- **Methodology stages** — one main nav item per methodology stage (Discovery, Planning, Documentation, Implementation, Review, Learning). Artifacts organized by where they fit in each stage's sub-workflow. Navigation structure generated from the methodology plugin and its stage plugins, not hardcoded.
- **Plugins** — top-level nav item (above Settings). Shows available/installed plugins with filters by category (knowledge, methodology, workflow, sidecar, connector, infrastructure). Surface **plugin groups** that bundle a methodology + all its stage plugins together (e.g., "Agile Software Development" installs methodology + all workflow stages). The `core` plugin is NOT surfaced to users — it's only a plugin for architectural composability, but it IS part of the core framework.
- **Settings** — bottom nav item. Reorganized to reflect the architecture:
  - **Methodology** — dedicated section for the installed methodology plugin. Workflow plugins nested underneath (they serve a specific required purpose within the methodology). Settings pages generated by the methodology and workflow plugins themselves.
  - **Sidecar** — dedicated section for the installed sidecar(s). Required plugin type, gets its own area.
  - **Connector** — dedicated section for installed connector(s). Settings pages generated by the connector plugin.
  - **Plugins** — generic section for all other installed plugins (knowledge, infrastructure, coding-standards, etc.). Grouped together since they're optional extensions.
  - Each plugin generates its own settings pages, which appear in the appropriate section AND are reachable via a link on the installed plugins list.
  - Global vs project settings remain separate but follow the same organizational structure.
  - **Remove the navigation settings page** — navigation is plugin-driven for now. Future: allow users to override the default navigation structure (methodology stages -> workflow artifacts -> custom views) with custom layouts, enabling plugins to create alternate artifact views.

#### Custom Views

- Review the roadmap view to ensure it works with the milestone/epic hierarchy
- Custom views contributed by plugins should render correctly in the new navigation structure

#### Hardcoded Pattern Removal

- Remove hardcoded artifact type prefixes, status values, stage names, and sort orders
- Replace with engine-provided data from composed schema and resolved workflows
- Fix hardcoded sidecar plugin name in StatusBar
- Deduplicate model options (currently in 3 files)

### Phase 10: Validate Against Targets

For each target artifact from Phase 1:

1. Run the generation pipeline
2. Compare generated output against hand-written target
3. If match: replace target with generated version
4. If gap: fix the generation pipeline, do not modify the target
5. Remove `targets/` directory once all generation is validated

### Completion Test

### Phase 11: Post-Migration Documentation

Create proper project documentation and knowledge artifacts for all architecture content that remains relevant post-migration:

1. Convert each split architecture file (`targets/claude-code-plugin/.claude/architecture/*.md`) into proper `.orqa/` documentation and knowledge artifacts
2. Documentation files (DOC) for human-readable reference, organized by topic
3. Knowledge files (KNOW) for agent-consumable chunks with injection metadata (tier, roles, paths, tags)
4. Ensure the documentation/knowledge hierarchy is complete — every architectural concept has both a doc and derived knowledge
5. Remove the `targets/` directory — all targets are now produced by generation pipelines
6. Remove the `file-audit/` directory — audit is complete

### Completion Test

The system is complete when:

- Every target from Phase 1 is produced by a generation pipeline
- The same methodology and principles apply whether working via the app or via Claude Code
- All enforcement is mechanical (generated hooks, linting, validation, permissions)
- The `.orqa/` directory looks like something the finished app would have created
- Agents work without bypass permissions, scoped to their role
- Architecture documentation exists as proper governance artifacts, not just a root-level markdown file

---

## 14. Audit Criteria

When reviewing files against this architecture, each file should be assessed on:

1. **Does it belong in this location?** Would the finished app have created this file here?
2. **Does it have the correct artifact type?** (e.g., PERSONA in personas/, not DOC)
3. **Does it serve the plugin-composed architecture?** Or does it assume the old monolithic model?
4. **Is it a duplicate?** Content installed from a plugin AND also defined manually.
5. **Is it correctly scoped?** Knowledge should be 500-2,000 tokens, atomic, self-contained.
6. **Does it have proper frontmatter?** (id, type, title, description, relationships)
7. **Is it actively used?** Or is it a leftover from a superseded approach?
8. **Does it cross boundaries?** (e.g., connector doing engine's job, app hardcoding governance patterns)
9. **Is it organized for human navigation?** Hash-only filenames in flat directories are not navigable.

---

## Glossary

Precise definitions of terms used throughout this document. When in doubt, these definitions are authoritative.

### System Components

| Term | Definition |
|------|-----------|
| **Engine** | A collection of Rust library crates providing all business logic (graph, workflow, enforcement, search, prompt pipeline, plugin system, agent generation). Not a process — a set of libraries consumed by access layers. |
| **Daemon** | A persistent Rust process that runs file watchers, serves MCP/LSP, manages the system tray, and consumes the engine crates. Outlives the app. |
| **App** | The Tauri desktop application (SvelteKit frontend + Rust backend). A consumer of the engine crates. Provides the custom UI for interacting with the engine. Empty shell without plugins. |
| **CLI** | The `orqa` command-line tool. A thin Rust wrapper around engine crates. An access method, not business logic. |
| **MCP** | Model Context Protocol. An access protocol exposing engine capabilities to LLM tools. NOT an application boundary. |
| **LSP** | Language Server Protocol. An access protocol for editor/IDE validation against the composed schema. NOT an application boundary. |

### Plugin Ecosystem

| Term | Definition |
|------|-----------|
| **OrqaStudio Plugin** | A package within the OrqaStudio ecosystem that provides definitions to the engine, extends the app, or both. Managed by `orqa install`. |
| **Tool-Native Plugin** | The output artifact generated by a connector (e.g., a Claude Code Plugin at `.claude/`). Consumed by the third-party tool. NOT an OrqaStudio plugin. |
| **Methodology Plugin** | An OrqaStudio plugin that defines the overarching approach (e.g., Agile). Provides the workflow skeleton with contribution points. One per project. |
| **Workflow Plugin** | An OrqaStudio plugin that provides the complete sub-workflow for one stage of the methodology. Owns its own state machine. One per stage. |
| **Domain Knowledge Plugin** | An OrqaStudio plugin that provides expertise (knowledge, rules, documentation) without defining methodology or workflows. Does not trigger schema recomposition, but rules changes CAN trigger enforcement regeneration (eslint, clippy, hooks, etc.). |
| **Sidecar** | An OrqaStudio plugin that integrates with an LLM provider to give the app inference capability. The app has no built-in inference — at least one sidecar is required. |
| **Connector** | A special OrqaStudio plugin that generates a tool-native plugin and watches for changes to regenerate it. The generated output goes where the third-party tool expects it (e.g., `.claude/`). |
| **Infrastructure Plugin** | An OrqaStudio plugin that generates enforcement configs (git hooks, linting) from engine rules. |
| **Plugin Group** | A bundled set of plugins installed together as a shortcut. Can be a methodology + all its workflow stages (e.g., "Agile Software Development") OR a set of knowledge plugins for a domain (e.g., "Tauri App" = svelte + tauri + typescript + rust). Groups make project setup easier — they are shortcuts, not a new concept. |
| **Plugin Manifest** | The `orqa-plugin.json` file declaring what a plugin provides, its purpose, stage slot, and installation targets. |

### Methodology and Workflows

| Term | Definition |
|------|-----------|
| **Methodology** | The overarching approach to work (e.g., Agile). Defined by a methodology plugin. Provides named contribution points that workflow plugins fill. |
| **Workflow** | A self-contained sub-workflow for one stage of the methodology (e.g., Discovery, Planning, Implementation). Each workflow plugin owns its complete state machine. NOT "workflow" in the generic sense — specifically a methodology stage's sub-process. |
| **Contribution Point** | A named slot in the methodology skeleton that a workflow plugin fills. The mechanism for composing methodology + workflows. |
| **Resolved Workflow** | The YAML file on disk produced by merging plugin contributions. One per stage. Deterministic, diffable, inspectable. The runtime reads only resolved files. |
| **Composed Schema** | The full JSON schema produced by composing all plugin-provided artifact type definitions. Used by LSP/MCP for validation. Generated whenever a definition plugin is installed — not just during dev environment setup. |
| **Composition Pipeline** | The process that runs when definition plugins are installed (via `orqa plugin install` or as part of `orqa install`): reads methodology + workflow plugins, merges contributions, validates, writes resolved workflows and composed schema. Triggered by any plugin install that affects the schema, not just the dev environment setup command. |

### Artifacts and Governance

| Term | Definition |
|------|-----------|
| **Artifact** | A markdown file with YAML frontmatter and a markdown body. The atomic unit of governance data. Each has an ID, type, title, status, and relationships. |
| **Artifact Type** | A category of artifact defined by a plugin's schema (e.g., `task`, `epic`, `knowledge`, `lesson`). Each type has an ID prefix, required fields, valid statuses, and valid relationships. |
| **Frontmatter** | The YAML header of an artifact file (between `---` delimiters). Contains structured metadata: id, type, title, description, status, created, updated, relationships. |
| **Relationship** | A typed, directional link between two artifacts (e.g., `task delivers epic`). Stored forward-only — the graph engine computes inverses. |
| **Knowledge** | Documentation split into agent-consumable chunks (500-2,000 tokens each) with injection metadata (tier, roles, paths, tags). Lives alongside its parent documentation. Knowledge IS documentation — just sized and tagged for the prompt pipeline. |
| **Rule** | A behavioral constraint. Classified as **mechanical** (enforced by tooling — guards, hooks, validation) or **advisory** (guidance for agents/humans). |
| **Lesson** | An implementation lesson captured through the learning loop. If it can be mechanically guarded, it becomes a mechanical rule immediately. |
| **Principle Decision** | An overarching architecture or approach decision that shapes the entire system. Examples: "no backwards compatibility during pre-release", "daemon is the business logic boundary", "Rust for all libs, TypeScript only for frontend", "accuracy over speed as a core principle". These rarely change and have wide-reaching consequences. |
| **Planning Decision** | An implementation-level decision about how to solve a specific category of problem. Examples: "use three-way diff for plugin content management", "pre-commit hooks run scoped tests not the full suite", "knowledge lives alongside its parent documentation", "use PreToolUse hooks for file access enforcement since Claude Code agents don't support path-level permissions". These are tactical and may evolve as implementation progresses. |
| **State Machine** | A YAML-defined set of states, transitions, guards, and actions for an artifact type. Owned by the workflow plugin. Evaluated by the engine. |
| **Guard** | A declarative prerequisite on a state transition (field check, relationship check, graph query). Evaluated mechanically by the engine. |
| **Gate** | A human decision point in a workflow. Five-phase sub-workflow: GATHER, PRESENT, COLLECT, EXECUTE, LEARN. |
| **State Category** | A classification of states that enables cross-cutting UI treatment: `planning`, `active`, `review`, `completed`, `terminal`. |

### Agents

| Term | Definition |
|------|-----------|
| **Base Role** | One of 8 fixed agent role definitions (Orchestrator, Implementer, Reviewer, Researcher, Writer, Planner, Designer, Governance Steward). Defines permissions, tool constraints, and artifact scope. Lives in the methodology plugin. |
| **Task-Specific Agent** | A bespoke agent generated at runtime by composing Base Role + Workflow Context + Domain Knowledge. Ephemeral — generated and discarded, not tracked as artifacts. |
| **Agent Team** | The hub-spoke orchestration model where an orchestrator delegates to ephemeral background workers. In the Claude Code connector, this uses Claude Code's team infrastructure (TeamCreate, TaskCreate, Agent, SendMessage, TaskUpdate, TeamDelete). In the app, the same philosophy applies — ephemeral task-scoped agents coordinated by a persistent orchestrator, implemented through the engine's agent generation and sidecar integration. The pattern is universal, not connector-specific. |
| **Prompt Pipeline** | The five-stage process that generates task-specific agent prompts: Plugin Registry -> Schema Assembly -> Section Resolution -> Token Budgeting -> Prompt Output. |
| **Token Budget** | The maximum token allocation for a generated agent's system prompt. Enforced by the prompt pipeline. Balances embedded knowledge vs MCP retrieval instructions. |
| **Injection Tier** | How knowledge is delivered to agents: `always` (compressed summary at spawn), `stage-triggered` (when workflow matches), `on-demand` (via MCP semantic search). |

### Enforcement

| Term | Definition |
|------|-----------|
| **Mechanical Enforcement** | Rules enforced by tooling (guards, hooks, validation, linting). Cannot be bypassed by agents. The more mechanical enforcement exists, the more autonomy agents can have. |
| **Advisory Enforcement** | Rules communicated to agents via prompts. Depends on agent compliance. Should be converted to mechanical enforcement where possible. |
| **Target State** | A hand-written artifact representing what the finished generation pipeline would produce. Used as a test fixture and validation benchmark. Protected during the migration — only replaced by validated generated output. Post-migration, the dry-run flag (`ORQA_DRY_RUN=true`) remains available to safely validate engine changes against live output without risking in-flight work. |
| **Test Fixture** | A target state artifact that generation pipeline output is compared against. If generated output matches, the target is replaced. If not, the pipeline is fixed. |

---

## Appendix A: Target State Specifications (Phase 1 Detail)

Each target is a hand-written artifact that represents what the finished generation pipeline would produce. These are built FIRST and everything else validates against them.

### A.1 Target Claude Code Plugin (`targets/claude-code-plugin/`)

The ideal `.claude/` directory that the connector should generate. This is what Claude Code consumes.

```
targets/claude-code-plugin/
  .claude/                        # Project-level config
    settings.json                 # Project-level Claude Code settings
    CLAUDE.md                     # Generated orchestrator prompt (from engine prompt pipeline)
    agents/                       # Generated from 8 base roles + workflow context
      implementer.md              # Role: source code + shell access
      reviewer.md                 # Role: read-only, can run checks, produces verdicts
      researcher.md               # Role: read-only, creates research artifacts only
      writer.md                   # Role: documentation only, no source code
      planner.md                  # Role: plans and delivery artifacts
      designer.md                 # Role: design artifacts, component code
      governance-steward.md       # Role: .orqa/ artifacts only
      orchestrator.md             # Role: read-only, delegation, reads summaries
  plugin/                         # The Claude Code Plugin PACKAGE (installed via /plugin)
    .claude-plugin/               # Plugin manifest directory
      plugin.json                 # Name, version, author — ONLY file in this directory
    skills/                       # At plugin root (sibling of .claude-plugin/, per CC spec)
      orqa/SKILL.md               # Main OrqaStudio skill (routes to subcommands)
      orqa-save/SKILL.md          # Save context to governance artifacts
      orqa-create/SKILL.md        # Create new governance artifact
      orqa-validate/SKILL.md      # Run validation against composed schema
    hooks/                        # At plugin root
      hooks.json                  # Generated from plugin hook declarations
    scripts/                      # At plugin root
      pre-tool-use.mjs            # Validate artifact operations before execution
      post-tool-use.mjs           # Track completions, update telemetry
      user-prompt-submit.mjs      # Classify prompt, inject workflow context via daemon
      session-start.mjs           # Initialize session, load project context via daemon
      stop.mjs                    # Save session state via daemon
      pre-compact.mjs             # Preserve critical context before compaction
      subagent-stop.mjs           # Review subagent output via daemon
      teammate-idle.mjs           # Team coordination — check task list, assign work
      task-completed.mjs          # Team coordination — verify AC, assign next task
```

**Key characteristics:**

- **Agent teams, not bare subagents** — the orchestrator uses TeamCreate/TaskCreate/Agent(team_name)/SendMessage/TaskUpdate/TeamDelete for hub-spoke coordination. Agent definitions enable this by defining the roles that get spawned into teams.
- Agent files use YAML frontmatter (`name`, `description`, `tools`/`disallowedTools`, `model`, `maxTurns`, `skills`). The markdown body becomes the system prompt.
- **No file-level path permissions in agent definitions** — Claude Code does not support glob-pattern path restrictions in agent frontmatter. File access enforcement uses `PreToolUse` hooks that validate paths before tool execution via the daemon.
- **Plugin agents cannot set `permissionMode`, `hooks`, or `mcpServers`** — silently ignored for security. Permission enforcement flows through hooks instead.
- All hooks are thin: receive event -> call daemon/CLI -> apply response. No business logic. Timeout values in seconds (not milliseconds).
- CLAUDE.md is generated from the prompt pipeline, not hand-written
- hooks.json is generated from plugin declarations, not static
- The Claude Code Plugin is a separate PACKAGE (in `plugin/`) installed via `/plugin`. `.claude-plugin/` contains ONLY `plugin.json`. Skills, hooks, and scripts are at the plugin root (siblings of `.claude-plugin/`), per the Claude Code plugin spec. `${CLAUDE_PLUGIN_ROOT}` resolves to the plugin root directory.
- hooks/ and scripts/ are at plugin root, NOT inside .claude-plugin/
- Uses skills/ (current CC standard), not commands/ (legacy)
- No git hooks or linting configs — those come from other plugins

**Each agent file should contain (as frontmatter + markdown body):**

```yaml
---
name: implementer
description: "Implements code changes. Reads task, reads knowledge, writes code, runs checks."
model: sonnet                   # or opus for complex tasks — engine decides at generation time
tools: "Read,Write,Edit,Bash,Grep,Glob,Agent,TaskCreate,TaskUpdate,TaskGet,TaskList,SendMessage"
disallowedTools: ""
maxTurns: 50
skills:
  - orqa-validate
---

[Generated system prompt from engine prompt pipeline]

Role: Implementer
Behavioral boundaries: ...
Artifact scope: source code, tests, configs
Knowledge summary: [compressed, within token budget]
On-demand retrieval: Use MCP search tools for detailed knowledge...
```

**PreToolUse hook for file access enforcement:**

The daemon evaluates whether the current agent role is allowed to access the target path. The hook calls the daemon with the tool name, file path, and agent role. The daemon returns approve/deny based on the role's artifact scope rules.

### A.2 Target JSON Schema (`targets/schema.composed.json`)

The full composed schema that the plugin composition pipeline would produce. All artifact types from all installed plugins, unified into one schema.

**Must define for each artifact type:**

- `id_prefix` — the prefix for artifact IDs (e.g., `TASK`, `EPIC`, `KNOW`)
- `type` — the artifact type name (e.g., `task`, `epic`, `knowledge`)
- Required frontmatter fields (id, type, title, description, status, created, updated, relationships)
- Optional frontmatter fields per type
- Valid status values (from the type's state machine)
- Valid relationship types (with from/to constraints)
- Which methodology stage the type belongs to

**Artifact types to include (from all current plugins):**

| Stage | Artifact Types |
|-------|---------------|
| Methodology (agile) | — (no artifacts, defines the skeleton) |
| Discovery | `discovery-idea`, `discovery-research`, `persona`, `pillar`, `vision`, `pivot` |
| Planning | `planning-idea`, `planning-research`, `planning-decision`, `wireframe` |
| Documentation | `doc` |
| Implementation | `epic`, `task`, `milestone` |
| Review | — (uses contribution workflow, no unique types) |
| Learning | `lesson`, `knowledge`, `rule`, `principle-decision` |
| Cross-cutting | `planning-decision` (from both planning and learning) |

**Relationship types must include:**

- All 41 types currently defined across plugins (reconciled, deduplicated)
- Each with `from` and `to` constraints (which artifact types can participate)
- Direction semantics (forward-only, graph computes inverses)

### A.3 Target `.orqa/` Structure

Applied directly to the live `.orqa/` directory. This IS the governance data — not a copy in `targets/`.

See Section 5.1 for the directory structure. The work here is:

1. **Restructure directories** — remove `process/` nesting, promote categories to top-level
2. **Fix every artifact** — correct type, correct frontmatter (`title` not `name`, required `status`), consistent YAML quoting
3. **Remove legacy** — delete all AGENT-*.md, SKILL.md, grounding docs
4. **Split decisions** — create `principles/` and `planning/` subdirectories with correct types
5. **Categorize knowledge** — organize into domain subdirectories
6. **Categorize documentation** — organize into topic subdirectories
7. **Fix wireframes** — change type from `doc` to `wireframe`
8. **Fix personas** — move DOC-1ff7a9ba to documentation/
9. **Clean delivery/discovery** — archive stale items, combine duplicate ideas
10. **Validate relationships** — ensure all targets exist, types are valid

The result must pass validation against the target JSON schema (A.2).

### A.4 Target Enforcement Configs (`targets/enforcement/`)

What the enforcement plugins should generate.

```
targets/enforcement/
  githooks/
    pre-commit                  # Shell script orchestrator
    post-commit                 # Post-commit actions

  eslint/
    base.config.js              # TypeScript base (from typescript plugin)
    svelte.config.js            # Svelte extension (from svelte plugin)
    app.config.js               # App config that imports from plugin bases

  clippy/
    clippy.toml                 # Generated from enforcement rules

  prettier/
    .prettierrc                 # Generated formatting config

  markdownlint/
    .markdownlint.json          # Generated markdown linting rules for governance artifacts

  tsconfig/
    base.json                   # Base TypeScript config (from typescript plugin)
    app.json                    # App-specific config extending base (bundler, DOM, noEmit)
    library.json                # Library config extending base (NodeNext, declarations)
```

**Pre-commit target checks:**

1. Artifact frontmatter validation (required fields, ID format, type matches location)
2. Relationship validation (targets exist, types valid, from/to constraints)
3. Schema compliance (against composed schema)
4. Lint checks (delegated to eslint/clippy)
5. Tests affected by staged changes (scoped, not full suite)
6. Knowledge size constraints (500-2000 tokens)
7. Status value validity (must be from workflow-defined values)

**Post-commit target actions:**

1. Auto-push to Forgejo

**ESLint target rules (key non-default rules):**

- `@typescript-eslint/no-unused-vars`: error (allow `_` prefix)
- `@typescript-eslint/no-explicit-any`: warn
- Import organization rules
- Svelte-specific: a11y rules, component naming

**Clippy target rules:**

- Generated from coding-standards plugin enforcement rules
- Reflects actual project coding standards, not just defaults

**Markdownlint target rules:**

- Frontmatter validation (YAML structure, required fields per artifact type)
- Consistent heading hierarchy
- Line length limits appropriate for governance artifacts
- Link validation (internal artifact references resolve)
- Code block language tags required
- Configured to work with OrqaStudio's YAML frontmatter + markdown body artifact format

### A.5 Target Resolved Workflows (`targets/workflows/`)

One YAML file per methodology stage, fully composed from plugin contributions.

```
targets/workflows/
  methodology.resolved.yaml     # The agile methodology skeleton with stage definitions
  discovery.resolved.yaml       # All discovery artifact types + state machines
  planning.resolved.yaml        # All planning artifact types + state machines
  documentation.resolved.yaml   # Documentation artifact types + state machines
  implementation.resolved.yaml  # Epic, task, milestone types + state machines
  review.resolved.yaml          # Review contribution workflow
  learning.resolved.yaml        # Lesson, decision, knowledge, rule types + state machines
```

**Each stage file must contain:**

- All artifact types for that stage (with full schema definitions)
- Complete state machine per artifact type (states, transitions, guards, actions)
- State categories mapped (planning, active, review, completed, terminal)
- Human gate definitions where applicable
- Relationship types relevant to the stage
- Contribution point metadata (which plugin contributed this content)

### A.6 Target Plugin Manifests (`targets/plugin-manifests/`)

Corrected `orqa-plugin.json` for each plugin showing the required taxonomy fields.

**Every manifest must include:**

```json
{
  "name": "@orqastudio/plugin-<name>",
  "description": "...",
  "version": "...",
  "purpose": ["methodology" | "workflow" | "knowledge" | "connector" | "infrastructure" | "sidecar"],
  "stage_slot": "<stage-name>",        // workflow plugins only
  "affects_schema": true | false,
  "affects_enforcement": true | false,
  "category": "<taxonomy-category>",   // matches ARCHITECTURE.md taxonomy
  "uninstallable": true | false,
  "provides": {
    "schemas": [...],
    "workflows": [...],                // structured objects, never flat strings
    "knowledge": [...],
    "rules": [...],
    "roles": [...],
    "views": [...],
    "enforcement_mechanisms": [...]
  }
}
```

**Manifests to produce (one per plugin):**

| Plugin | Purpose | Stage Slot | Affects Schema | Affects Enforcement |
|--------|---------|-----------|---------------|-------------------|
| `agile-methodology` (renamed) | `methodology` | — | yes | no |
| `core` | `workflow` | `learning` | yes | yes |
| `agile-discovery` | `workflow` | `discovery` | yes | no |
| `agile-planning` | `workflow` | `planning` | yes | no |
| `agile-documentation` | `workflow` | `documentation` | yes | no |
| `agile-review` | `workflow` | `review` | yes | no |
| `software-kanban` | `workflow` | `implementation` | yes | no |
| `cli` | `knowledge` | — | no | no |
| `rust` | `knowledge`, `infrastructure` | — | no | yes |
| `svelte` | `knowledge` | — | no | no |
| `tauri` | `knowledge` | — | no | no |
| `typescript` | `knowledge`, `infrastructure` | — | no | yes |
| `coding-standards` | `infrastructure` | — | no | yes |
| `systems-thinking` | `knowledge` | — | no | no |
| `plugin-dev` | `knowledge` | — | no | no |
| `claude-code` | `connector` | — | no | no |
