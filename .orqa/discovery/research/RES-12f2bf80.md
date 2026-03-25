---
id: RES-12f2bf80
type: research
title: "Persistent Dev Team Model — Composable Agent Collaboration"
status: active
category: process
created: 2026-03-24
updated: 2026-03-24
description: "Design proposal for persistent agent team structure that mirrors an IRL dev team. Team roles defined by software plugin, expertise consumed from language plugins via knowledge artifacts."
tags:
  - agent-teams
  - collaboration
  - plugin-architecture
relationships:
  - target: RES-138eff6e
    type: related
    rationale: "Token efficiency research informs team sizing and model selection"
---

# Persistent Dev Team Model for OrqaStudio

## Executive Summary

The current approach of spawning disposable agents per task is failing on four fronts:
1. **Zero context continuity** — Each agent starts from scratch, re-learning patterns already captured in knowledge artifacts
2. **Resource contention** — Parallel Rust compilation exhausts memory; agents don't know about each other's resource needs
3. **Poor shutdown discipline** — Agents don't terminate cleanly, consuming resources for hours
4. **Sequential blocking** — Related tasks (e.g., MCP fix followed by LSP fix) can't share learnings or coordinate timing

An IRL development team works differently: persistent specialists, built-in domain expertise, natural handoffs, sequential quality. This proposal redesigns the agent model to mirror that reality.

**Key Design Principle: Composable Team Structure**

The team is defined in the **Software plugin** (`plugins/software/agents/`) — not in the framework. This design principle enables **portability across any tech stack**:

- **OrqaStudio project** (Rust + Svelte): Team agents consume Rust + Svelte plugins
- **Python + Django + React project**: Same team agents consume Python + Django + React plugins
- **Go + API + Vue project**: Same team agents consume Go + Vue plugins
- **Any future stack**: Drop in the right language/framework plugins; team structure unchanged

Team roles (Backend Lead, Frontend Lead, Integration Lead, Governance Lead) are stable specializations. Expertise sources (knowledge from plugins) are interchangeable. This is composability: one team structure, swappable expertise layers.

---

## Part 0: Plugin-Based Architecture

### 0.1 The Composability Model

Team agents live in `plugins/software/agents/`, not `.orqa/process/agents/` (framework). This separation enables portability:

| Context | Team Location | Knowledge Sources | Benefit |
|---------|---------------|-------------------|---------|
| **OrqaStudio** (Rust + Svelte + TypeScript + Tauri) | `plugins/software/agents/` | `plugins/rust/knowledge/`, `plugins/svelte/knowledge/`, `plugins/typescript/knowledge/`, `plugins/tauri/knowledge/` | Backend Lead implements Rust, Frontend Lead implements Svelte |
| **Python + Django + React** | `plugins/software/agents/` (same agents) | `plugins/python/knowledge/`, `plugins/django/knowledge/`, `plugins/react/knowledge/` | Same Backend/Frontend/Integration Leads; different expertise sources |
| **Go + API + Vue** | `plugins/software/agents/` (same agents) | `plugins/go/knowledge/`, `plugins/api/knowledge/`, `plugins/vue/knowledge/` | Team roles unchanged; knowledge swapped |

The team is a **role-based abstraction**. What it specializes in changes with the language/framework plugins loaded.

### 0.2 Knowledge Plugin Architecture

**Language plugins** provide domain expertise. On OrqaStudio:
- `plugins/rust/` — Rust async, error handling, testing, clippy standards
- `plugins/svelte/` — Svelte 5 runes, reactive state, component patterns
- `plugins/typescript/` — TypeScript strict mode, type safety, eslint
- `plugins/tauri/` — Tauri v2 IPC patterns, command registration, error serialization

**The software plugin** (`plugins/software/`) defines team roles that consume these knowledge sources via relationship edges in agent frontmatter.

**Backend Lead example:**

```yaml
# plugins/software/agents/AGENT-backend-lead.md
title: Backend Lead
skills:
  - orqa-code-search
  - composability
knowledge:
  - orqa-backend-best-practices    # From plugins/software/knowledge/
  - orqa-error-composition
  - orqa-domain-services
  - orqa-ipc-patterns
  - orqa-repository-pattern
  - rust-async-patterns            # From plugins/rust/knowledge/
  - tauri-v2-integration           # From plugins/tauri/knowledge/
```

The Backend Lead **references** language plugin knowledge, not duplicates it. On a Python project, the same role would reference `python-async-patterns` and `django-integration` instead.

### 0.3 Plugin Load Order at Session Start

1. **Framework agents** (`.orqa/process/agents/`) — Orchestrator, universal roles
2. **Language plugins** (`plugins/rust/`, `plugins/svelte/`, `plugins/typescript/`, etc.) — Provide knowledge and specialists
3. **Software plugin** (`plugins/software/agents/`) — Team roles that consume knowledge from layers 1 & 2

Team agents depend on language plugins. Without the right language plugins, team agents cannot function correctly.

**On OrqaStudio:** Rust + Svelte + TypeScript + Tauri plugins are always loaded
**On a Python + Django + React project:** Python + Django + React plugins would be loaded instead
**The team agents themselves need not change.**

---

## Part 1: Current State Analysis

### 1.1 Existing Agent Structure

**Role-based agents (universal roles):**
- `Orchestrator` — coordination only, no implementation
- `Implementer`, `Reviewer`, `Researcher`, `Planner`, `Writer`, `Designer`
- `Governance Steward` — governance artifacts

**Project-specific specialists (layer on universal roles):**
- `OrqaStudio Rust Backend Specialist` — backend domain knowledge
- `OrqaStudio Svelte Frontend Specialist` — frontend domain knowledge
- `OrqaStudio Integration Specialist` — cross-boundary (IPC, streaming, type safety)

**Governance enforcement:**
- `Governance Enforcer` — mechanical enforcement for rules

**Plugin-provided specialists** (from @orqastudio/plugin-rust, plugin-svelte, etc.):
- `Rust Specialist`
- `Svelte Specialist`

### 1.2 Knowledge Architecture

Knowledge is organized in `.orqa/process/knowledge/` by domain:

**Backend domains:**
- `orqa-domain-services` — 3 service shapes, constructor injection
- `orqa-ipc-patterns` — Tauri command → IPC type → TS interface → store chain
- `orqa-repository-pattern` — Trait-based repos, SQLite, migrations
- `orqa-error-composition` — OrqaError enum, From impls, error flow
- `orqa-backend-best-practices` — Umbrella conventions

**Frontend domains:**
- `orqa-store-patterns` — Runes-based stores, reactive state
- `orqa-store-orchestration` — Multi-store coordination, side effects
- `svelte5-best-practices` — Runes only, no Svelte 4 patterns
- `component-extraction` — Component purity, prop interfaces

**Integration domains:**
- `orqa-streaming` — Agent SDK → sidecar → backend → UI pipeline
- `orqa-ipc-patterns` (shared with backend)

**Governance domains:**
- `orqa-governance` — Artifact graph, relationships, lifecycle
- `orqa-documentation` — Documentation-first workflow

### 1.3 Codebase Technical Domains

**Backend subsystems** (app/backend/src-tauri/src/):
- `commands/` — Tauri command handlers, IPC entry points
- `domain/` — Domain logic, business rules, core types
- `repo/` — Data access layer, SQLite repositories
- `search/` — Search engine integration (ONNX, DuckDB)
- `servers/` — MCP server, LSP server implementations
- `sidecar/` — Sidecar protocol, streaming pipeline
- `plugins/` — Plugin system integration
- `hooks/` — Pre-commit, validation hooks
- `cli_tools/` — CLI integration

**Frontend subsystems** (app/ui/src/):
- `lib/components/` — Reusable component library
- `lib/stores/` — Reactive state management
- `lib/services/` — Services that call backend commands
- `lib/plugins/` — Plugin integration layer
- `routes/` — Pages and layouts
- `lib/workers/` — Web workers for heavy computation

**Shared libraries** (libs/):
- `cli/` — CLI implementation (TypeScript, Node.js)
- `mcp-server/` — MCP server (Rust + ONNX search)
- `lsp-server/` — LSP server (Rust)
- `search/` — Search engine (Rust, ONNX, DuckDB)
- `validation/` — Validation engine (Rust)
- `sdk/` — TypeScript SDK for external tools
- `graph-visualiser/` — Graph visualization (TypeScript/Svelte)
- `svelte-components/` — Shared component library (published)
- `types/` — Shared TypeScript types

**Cross-cutting concerns:**
- `connectors/claude-code/` — Claude Code plugin (TypeScript)
- `plugins/` — First-party plugins (software, cli, claude, svelte, tauri)
- `.orqa/` — Governance (rules, knowledge, agents, epics, tasks, ideas)

### 1.4 Current Failure Modes

**Problem 1: Disposable agents lose context**
```
Session N: Backend agent builds feature A (learns 5 patterns)
Session N+1: Different backend agent builds feature B (relearns same 5 patterns)
```
→ Knowledge exists in artifacts but not in agent continuity

**Problem 2: Parallel compilation kills quality**
```
Backend agent A: Starts rustc for feature X (uses 400MB RAM)
Backend agent B: Starts rustc for feature Y (same 400MB)
System: OOM — builds fail, timeouts, no clear error
```
→ Resource safety rule exists but no enforcement at team level

**Problem 3: Agents don't shut down**
```
Orchestrator: "Delegate feature X to Backend agent"
Backend agent: Works, marks task done, doesn't terminate
Orchestrator: Spawns new agent for feature Y
Background: Both agents alive, competing for resources
```
→ TeamDelete supposed to happen but doesn't reliably

**Problem 4: No handoff discipline**
```
Backend finishes IPC layer (command + type)
Frontend agent doesn't know it exists yet
Frontend agent writes its own version of the type
Integration specialist catches mismatch post-hoc
```
→ Parallel work creates rework, not progress

---

## Part 2: Recommended Team Structure

### 2.1 Persistent Role-Based Team

An OrqaStudio dev session should maintain a **persistent team of 4-6 specialists** with clear ownership. Rather than spawning agents per task, the team persists across the session, sharing context and expertise.

**Core team (always active):**

| Role | Agent | Reads Knowledge From | Owns |
|------|-------|----------------------|------|
| **Orchestrator** | `.orqa/process/agents/` | Framework layer | Process coordination, governance, vision |
| **Backend Lead** | `plugins/software/agents/` | Rust + Tauri plugins + OrqaStudio software plugin | Rust backend subsystems, IPC layer, error handling |
| **Frontend Lead** | `plugins/software/agents/` | Svelte + TypeScript plugins + OrqaStudio software plugin | Svelte frontend, stores, reactive state |
| **Integration Lead** | `plugins/software/agents/` | Rust + Svelte + TypeScript plugins + OrqaStudio software plugin | Cross-boundary contracts, IPC types, streaming |
| **Governance Lead** | `.orqa/process/agents/` | Framework + OrqaStudio software plugin | .orqa/ artifacts, graph integrity, artifact lifecycle |

**Key difference from previous structure:**
- Backend/Frontend/Integration/Governance agents live in **software plugin**, not framework
- They CONSUME knowledge from language plugins (Rust, Svelte, TypeScript, Tauri)
- On a Python + React project, same team roles consume Python + React plugin knowledge
- Orchestrator and universal roles stay in framework

**Specialist pool (spawned as needed):**

| Role | When Used | From | Owns |
|------|-----------|------|------|
| **Reviewer** | After implementation phase | Framework | Quality gates, verification |
| **Planner** | Before implementation | Framework | Design, dependencies, architecture |
| **Researcher** | During planning | Framework | Investigation, options evaluation |
| **Writer** | Parallel with implementation | Framework | Documentation, specs |
| **Designer** | UI/UX work | Framework or software plugin | Component design, interaction patterns |

### 2.2 Team Lifecycle

**Session start:**
1. Orchestrator spawns core team (4-5 agents)
2. Each team member reads its knowledge artifacts
3. Orchestrator briefs team on current scope
4. Team stays active for entire session

**During work:**
- Core team handles most tasks
- Specialist pool agents spawn for specific needs (planning, review)
- Specialists terminate after their work; core team persists

**Session end:**
- Orchestrator coordinates final verification
- TeamDelete removes all agents cleanly
- Session state documents what's running for next session

### 2.3 Communication Model

**Sync points (via SendMessage):**
- Backend finishes IPC layer → notifies Integration Lead + Frontend Lead
- Frontend finishes store → notifies Integration Lead
- Integration Lead verifies full chain before both sign off
- Each domain validates its own code before handoff

**Sequential handoffs:**
```
Planner: "Here's the design"
         ↓
Backend Lead: "Building IPC layer + command"
         ↓
Integration Lead: "Wiring up the contract"
         ↓
Frontend Lead: "Building store + component"
         ↓
Integration Lead: "Verifying end-to-end"
         ↓
Reviewer: "Quality gate"
```

**Parallel safety zones:**
- Frontend can work on unrelated components while backend works on commands
- Governance work runs in parallel with feature work (different code areas)
- CLI can advance independently of app development

---

## Part 3: Expertise Mapping

### 3.1 Backend Lead (Rust Backend Specialist)

**Owns:** `app/backend/src-tauri/src/{domain,commands,repo,sidecar,cli_tools}`

**Knowledge loaded:**
- `orqa-backend-best-practices` — Umbrella conventions
- `orqa-error-composition` — OrqaError enum, From impls, error flow
- `orqa-domain-services` — 3 service shapes, injection, composition
- `orqa-ipc-patterns` — Full request chain, IPC contracts
- `orqa-repository-pattern` — Traits, repos, migrations

**Skills:**
- Rust async patterns, error handling, type safety
- Tauri v2 integration
- SQLite/DuckDB schema design
- IPC contract design

**Responsibilities:**
1. Implement domain logic and services
2. Define Tauri commands with proper error handling
3. Build repository implementations
4. Write Rust side of IPC contracts
5. Verify `make check` passes (clippy, rustfmt, tests)

**Boundaries:**
- Does NOT touch frontend code
- Does NOT design UI or stores
- Does NOT coordinate across teams (Orchestrator does that)

### 3.2 Frontend Lead (Svelte Frontend Specialist)

**Owns:** `app/ui/src/{lib/components,lib/stores,lib/services,routes}`

**Knowledge loaded:**
- `orqa-frontend-best-practices` — Component purity, stores, runes
- `svelte5-best-practices` — Runes only, no Svelte 4 patterns
- `component-extraction` — Shared components, prop interfaces
- `orqa-store-patterns` — Reactive stores, side effects
- `orqa-store-orchestration` — Multi-store coordination

**Skills:**
- Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`)
- Strict TypeScript (no `any`)
- shadcn-svelte components
- Tailwind CSS design system
- Store architecture

**Responsibilities:**
1. Build reusable components
2. Implement reactive stores
3. Wire backend calls via `invoke()`
4. Handle loading/error/empty states
5. Design UX flow
6. Verify `npm run check` passes (type check, lint, tests)

**Boundaries:**
- Does NOT touch backend Rust code
- Does NOT write complex business logic in stores
- Does NOT call backend directly from display components

### 3.3 Integration Lead (Integration Specialist)

**Owns:** IPC contracts, streaming pipeline, cross-boundary type safety

**Knowledge loaded:**
- `orqa-ipc-patterns` — Full 4-layer request chain
- `orqa-streaming` — Agent SDK → sidecar → backend → UI
- `orqa-error-composition` — Error flow across boundary
- `orqa-store-patterns` — Frontend consumption of IPC responses
- Cross-domain knowledge (backend + frontend)

**Skills:**
- Tauri invoke/listen patterns
- Type contract verification
- NDJSON protocol (streaming)
- Error serialization
- End-to-end wiring

**Responsibilities:**
1. Design IPC types and contracts
2. Verify Rust types match TypeScript interfaces
3. Wire streaming pipeline for long-running operations
4. Verify error flow from backend to UI
5. Conduct integration tests (end-to-end wiring)
6. Produce evidence of complete chains (not stubs)

**Boundaries:**
- Does NOT implement backend logic
- Does NOT implement frontend components
- Focuses purely on contracts and wiring

### 3.4 Governance Lead (Governance Steward)

**Owns:** `.orqa/` artifact graph, schema compliance, relationships

**Knowledge loaded:**
- `orqa-governance` — Artifact graph, lifecycle, relationships
- `orqa-documentation` — Documentation gates, specs
- Graph schema validation
- Artifact integrity

**Skills:**
- Artifact graph schema
- Relationship types and direction
- YAML frontmatter validation
- Bidirectional relationship maintenance
- Documentation-first workflow

**Responsibilities:**
1. Create and maintain all `.orqa/` artifacts
2. Enforce graph integrity (bidirectional edges)
3. Manage artifact lifecycle (status transitions)
4. Maintain documentation gates
5. Capture lessons and promote to rules
6. Verify artifact config in `project.json`

**Boundaries:**
- Does NOT modify implementation code
- Does NOT make architecture decisions (records them)
- Does NOT review feature work (Reviewer does that)

### 3.5 Orchestrator

**Owns:** Process coordination, task delegation, governance gates

**Knowledge loaded:**
- `decision-tree` — Pillar alignment, vision
- All active rules (injected at session start)
- Artifact graph topology

**Skills:**
- Process coordination
- Task decomposition
- Governance enforcement
- Status reporting
- Conflict resolution

**Responsibilities:**
1. Discover pillars and rules at session start
2. Evaluate work against pillar gates
3. Create and delegate tasks
4. Verify dependencies before unblocking work
5. Enforce artifact completion gates
6. Report status accurately
7. Trace artifacts to affected systems (RULE-b2584e59)

**Boundaries:**
- Does NOT write implementation code
- Does NOT implement features
- Does NOT touch `.orqa/` directly (Governance Lead does)
- Does NOT shut down agents (TeamDelete does)

---

## Part 4: Workflow Model

### 4.1 Sequential Feature Implementation

```
Planner:
├─ Read epic, understand requirements
├─ Interview domain experts (Backend/Frontend leads)
├─ Design 4-layer architecture
├─ Produce acceptance criteria
└─ Present to user for approval

Backend Lead:
├─ Read plan, understand IPC contract
├─ Implement domain services
├─ Implement Tauri command
├─ Implement error handling
└─ Notify Integration Lead: "IPC layer ready"

Integration Lead:
├─ Verify Rust IPC type definition
├─ Design TypeScript interface
├─ Verify round-trip serialization
└─ Notify Frontend Lead: "Contract ready"

Frontend Lead:
├─ Implement store consuming the command
├─ Implement component displaying the data
├─ Handle loading/error/empty states
└─ Notify Integration Lead: "Frontend ready"

Integration Lead:
├─ Wire store → component
├─ Test end-to-end (real data, not stubs)
├─ Verify error flow
└─ Report: "Feature wired and tested"

Reviewer:
├─ Run `make check` on backend
├─ Run `npm run check` on frontend
├─ Verify acceptance criteria
├─ Produce PASS/FAIL verdict
└─ Report findings

Governance Lead:
└─ Capture any patterns as lessons → rules
```

**Key principle:** Each person/role completes ONE thing fully before handing to the next. No parallel work on the same feature's layers — parallel work happens on *different* features.

### 4.2 Parallel Safety Zones

**These CAN happen in parallel:**
- Backend working on feature A while Frontend works on unrelated component in feature B
- Governance work on rules while development happens (different code areas)
- CLI development while app development (separate subsystems)
- Planner designing next epic while Backend implements current epic
- Researcher investigating options while current feature is in code review

**These MUST be sequential:**
- Backend Rust compilation → Frontend build (resource contention)
- Backend IPC layer → Frontend store implementation (dependency)
- Any feature layer → code review (dependency)
- Code review → next feature start (gating)

### 4.3 Resource Safety Enforcement

**Compilation resource contention:**
- Only ONE agent compiles Rust at a time
- Frontend-only work (Svelte) happens while Rust is building
- Agent starting a compilation notifies team via SendMessage
- Other agents wait for notification before starting compilation

**Context continuity:**
- Core team persists across session (5+ hour typical)
- No re-spawning unless explicitly needed
- Specialists spawn briefly, terminate cleanly

**Clean shutdown:**
- Orchestrator coordinates final TaskUpdates
- Orchestrator calls TeamDelete (not individual shutdowns)
- Agents do NOT stay alive between tasks

---

## Part 5: Knowledge Requirements

### 5.1 Knowledge Loading at Task Start

Each agent loads its knowledge before ANY work begins:

**Backend Lead loads:**
```yaml
knowledge:
  - orqa-code-search        # Universal search wrapper
  - composability           # Universal composability philosophy
  - orqa-backend-best-practices
  - orqa-error-composition
  - orqa-domain-services
  - orqa-ipc-patterns
  - orqa-repository-pattern
```

**Frontend Lead loads:**
```yaml
knowledge:
  - orqa-code-search
  - composability
  - orqa-frontend-best-practices
  - svelte5-best-practices
  - component-extraction
  - orqa-store-patterns
  - orqa-store-orchestration
```

**Integration Lead loads:**
```yaml
knowledge:
  - orqa-code-search
  - composability
  - orqa-ipc-patterns
  - orqa-streaming
  - orqa-error-composition
  - orqa-store-patterns
  - orqa-backend-best-practices  # For verifying backend side
  - orqa-frontend-best-practices # For verifying frontend side
```

**Governance Lead loads:**
```yaml
knowledge:
  - orqa-code-search
  - composability
  - orqa-governance
  - orqa-documentation
```

### 5.2 Knowledge Deduplication

The orchestrator injects knowledge once per session. Core team members don't re-load the same knowledge when switching tasks.

**Example:**
- Session starts, Backend Lead loads 7 knowledge artifacts
- Backend Lead implements feature A
- Backend Lead handles new task in feature B
- Knowledge is already loaded; no re-injection (dedup cache)
- If knowledge *changes* during session (unlikely), agent must be respawned

---

## Part 6: Team Size and Composition

### 6.1 Minimum Viable Team

**For most development sessions:**
- **1 Orchestrator** — coordination
- **1 Backend Lead** — Rust implementation
- **1 Frontend Lead** — Svelte implementation
- **1 Integration Lead** — cross-boundary contracts
- **1 Governance Lead** — .orqa/ artifacts

**Total: 5 agents, persistent for entire session**

### 6.2 When to Spawn Specialists

Spawn specialist agents for bounded tasks:

| Trigger | Specialist | Lifetime | Example |
|---------|-----------|----------|---------|
| Implementation complete | Reviewer | 1 task | Verify feature passes `make check` |
| Feature planning phase | Planner | 1 task | Design 4-layer architecture |
| Complex investigation | Researcher | 1 task | Evaluate migration approach |
| Documentation phase | Writer | N tasks | Write specs, docs, guides |
| UI/UX design | Designer | 1 task | Design component interaction |

**Specialist lifecycle:**
1. Orchestrator spawns for specific task
2. Specialist works and produces output
3. Specialist terminates cleanly (TaskUpdate → done)
4. Orchestrator reads findings file
5. Core team continues

### 6.3 Scaling Considerations

**For large epics (3+ weeks):**
- Consider adding a dedicated **CLI specialist** if CLI work is active
- Consider splitting **Frontend Lead** into component specialist + stores specialist
- Keep core team at 5-6; add specialists as needed

**Resource constraints:**
- Never run >1 Rust compilation agent in parallel
- Frontend agents are cheap; safe to parallel with backend planning
- Governance work runs independent of feature work

---

## Part 7: Agent Lifecycle

### 7.1 Session Start

```
User: "Here's a new feature"
     ↓
Orchestrator:
├─ Query pillars, rules, active work
├─ Spawn Backend Lead (persistent)
├─ Spawn Frontend Lead (persistent)
├─ Spawn Integration Lead (persistent)
├─ Spawn Governance Lead (persistent)
├─ Brief team on scope
└─ Create tasks

Each Agent:
├─ Load knowledge from YAML frontmatter
├─ Read Required Reading docs
├─ Report ready
└─ Await first task
```

### 7.2 During Work

```
Orchestrator:
├─ Assign task to Backend Lead
├─ Backend Lead works → notifies Orchestrator
├─ Orchestrator delegates next task
├─ (If planning needed: spawn Planner)
├─ (If review needed: spawn Reviewer)
└─ Core team persists across all tasks

Specialist (if spawned):
├─ Work on bounded task
├─ Produce output (findings file)
├─ TaskUpdate → done
└─ Terminate
```

### 7.3 Session End

```
Orchestrator:
├─ Verify all open tasks
├─ Request final status from each agent
├─ Coordinate final verifications
├─ Write session state
├─ Commit all work
└─ Call TeamDelete

TeamDelete:
├─ Signal all agents to shut down
├─ Wait for clean exit
├─ Kill any stragglers
└─ Clean up resources
```

---

## Part 8: Anti-Patterns to Avoid

### 8.1 From Current Failures

**Anti-pattern 1: Spawn agents per task without team concept**
```
Task 1 → Backend Agent X → Knowledge loaded, work done, dies
Task 2 → Backend Agent Y → Knowledge reloaded, re-learns patterns, dies
Task 3 → Backend Agent Z → Knowledge reloaded AGAIN
```
**Fix:** Persistent Backend Lead carries knowledge across 3 tasks

**Anti-pattern 2: Parallel Rust compilation**
```
Agent A: cargo build (400MB)
Agent B: cargo clippy (400MB)
System: OOM
```
**Fix:** Compilation resource synchronization via SendMessage

**Anti-pattern 3: Stale agents consuming resources**
```
Agent A finishes, doesn't terminate
Agent B finishes, doesn't terminate
After 10 tasks: 10 agents alive, competing for memory
```
**Fix:** Explicit TeamDelete after task batch completion

**Anti-pattern 4: No handoff discipline**
```
Backend: "IPC layer done"
Frontend: "Building store..."
Frontend: writes own version of IPC type (didn't know it existed)
Integration: Finds type mismatch post-hoc
```
**Fix:** Integration Lead validates all contracts before Frontend starts

**Anti-pattern 5: Parallel same-layer work**
```
Backend Agent A: Building Tauri command for feature X
Backend Agent B: Building Tauri command for feature Y
Both try to update app builder → merge conflict, no clear resolution
```
**Fix:** One Backend Lead owns all backend changes sequentially

**Anti-pattern 6: Agent doesn't know the project's quality workflow**
```
Agent: restructures 306 files, runs cargo check, reports "done"
Pre-commit: 20+ rustfmt diffs, 92 eslint errors, clippy warnings
Result: commit blocked, orchestrator spends 30 minutes fixing lint
```
**Fix:** Every agent must know: format → lint → test → commit. This is
project knowledge that agents must load BEFORE starting work. The team
design should ensure agents consume the project's quality workflow from
the coding-standards plugin, not discover it at commit time.

**Anti-pattern 7: Agent uses cargo check instead of orqa check**
```
Agent: "cargo check passes — done"
Reality: rustfmt, clippy, eslint, svelte-check, artifact validation all unchecked
```
**Fix:** Agents must use the project's check command (`orqa check`), not
raw tool commands. The check command runs ALL quality gates. This is
enforced through the coding-standards knowledge artifact.

### 8.2 Resource Contention

**FORBIDDEN:**
- Two Rust compilation agents in same worktree
- Unbounded parallel agent spawning
- Agents staying alive after task completion

**SAFE:**
- Frontend + Planning in parallel (no compilation)
- Governance + Implementation in parallel (different trees)
- Sequential feature layers with handoff notifications

### 8.3 Communication Breakdown

**FORBIDDEN:**
- Agents silently assuming contract details
- No notification when IPC layer ready
- Frontend building before Integration Lead verifies contract

**REQUIRED:**
- SendMessage when layer complete
- Integration Lead validates before next layer
- Clear handoff protocol per workflow

---

## Part 9: Implementation Plan

### 9.1 Plugin Structure Changes

**Create software plugin agent definitions** (`plugins/software/agents/`):

1. **Backend Lead** (AGENT-backend-lead-sw.md)
   - Consumes: Rust + Tauri plugins + OrqaStudio domain knowledge
   - Owns: backend subsystems, Rust implementation, Tauri commands
   - Lives in: `plugins/software/agents/`

2. **Frontend Lead** (AGENT-frontend-lead-sw.md)
   - Consumes: Svelte + TypeScript plugins + OrqaStudio store/component knowledge
   - Owns: frontend subsystems, Svelte components, reactive stores
   - Lives in: `plugins/software/agents/`

3. **Integration Lead** (AGENT-integration-lead-sw.md)
   - Consumes: All language plugins + cross-boundary knowledge
   - Owns: IPC contracts, streaming pipeline, type safety verification
   - Lives in: `plugins/software/agents/`

4. **Governance Lead** (AGENT-governance-lead-sw.md)
   - Consumes: Framework + governance knowledge
   - Owns: .orqa/ artifacts, graph integrity, artifact lifecycle
   - Lives in: `.orqa/process/agents/` (governance is cross-project)

### 9.2 Framework Changes (`.orqa/` artifacts)

**DO NOT modify these — these are framework-level and affect all projects:**

1. **Update Orchestrator agent definition** (`.orqa/process/agents/AGENT-4c94fe14.md`)
   - Add "team management" section
   - Document how to spawn software plugin team
   - Document persistent team protocol
   - Document specialist spawning criteria
   - Document SendMessage handoff protocol

2. **Create team orchestration rule** (`.orqa/process/rules/`)
   - Define persistent team composition
   - Define plugin-based knowledge loading
   - Define handoff protocol
   - Define specialist spawning criteria
   - Define cleanup procedure (TeamDelete)

3. **Update existing rules for team context**
   - RULE-87ba1b81 (agent-delegation) — persistent teams vs disposable agents
   - RULE-99abcea1 (TeamCreate discipline) — team lifecycle
   - Document composability principle (team structure is stack-portable)

### 9.2b Plugin Changes (`plugins/software/` — ProjectSpecific)

**DO modify these — these are project-specific and define the actual team:**

1. **Create Backend Lead agent** (`plugins/software/agents/AGENT-backend-lead-sw.md`)
   - Title: "Backend Lead (OrqaStudio)"
   - Knowledge: Rust + Tauri + OrqaStudio domain knowledge
   - For another project: swap Rust → language, Tauri → framework

2. **Create Frontend Lead agent** (`plugins/software/agents/AGENT-frontend-lead-sw.md`)
   - Title: "Frontend Lead (OrqaStudio)"
   - Knowledge: Svelte + TypeScript + OrqaStudio frontend knowledge
   - For another project: swap Svelte → framework, TypeScript → language

3. **Create Integration Lead agent** (`plugins/software/agents/AGENT-integration-lead-sw.md`)
   - Title: "Integration Lead (OrqaStudio)"
   - Knowledge: All language plugins + cross-boundary knowledge
   - For another project: include new language/framework plugins

4. **Create Governance Lead agent** (`.orqa/process/agents/` not software plugin)
   - Governance is framework-level, doesn't change per project
   - But create an agent definition so it can be part of the team

### 9.3 Process Changes (Orchestrator behavior)

These changes affect how the Orchestrator agent behaves at runtime:

1. **Session start protocol**
   - After discovering pillars, query for software plugin agents
   - Spawn core team from software plugin agents using TeamCreate
   - Each team member loads knowledge from `skills:` frontmatter
   - Brief team on scope before delegating work
   - Document team composition in session state

2. **Task assignment protocol**
   - Assign to core team member when possible
   - Only spawn specialists (Planner, Reviewer, Researcher) for bounded tasks
   - Coordinate handoffs between core team via SendMessage
   - Core team members stay alive until session end

3. **Session end protocol**
   - Verify all tasks done before shutdown
   - Call TeamDelete with all core team agent IDs
   - Document team learnings and patterns in session state
   - Do NOT individual shutdown requests — TeamDelete handles cleanup

### 9.4 Knowledge Integration

**OrqaStudio software plugin knowledge** (`plugins/software/knowledge/`):
- `orqa-backend-best-practices` — OrqaStudio backend conventions, Rust patterns
- `orqa-error-composition` — OrqaError enum patterns, error flow
- `orqa-domain-services` — 3-shape service architecture, injection
- `orqa-ipc-patterns` — 4-layer IPC chain, Tauri + NDJSON
- `orqa-repository-pattern` — Trait-based repo pattern, SQLite migrations
- `orqa-frontend-best-practices` — OrqaStudio frontend conventions, Svelte
- `orqa-store-patterns` — Runes-based store patterns, reactive state
- `orqa-store-orchestration` — Multi-store coordination, side effects
- `orqa-streaming` — Agent SDK → sidecar → backend → UI pipeline
- `orqa-team-protocol` — Handoff expectations, resource safety, SendMessage protocol (NEW)

**Language/framework plugins provide:**
- `plugins/rust/knowledge/` — Rust async, thiserror, clippy, ownership
- `plugins/svelte/knowledge/` — Svelte 5 runes, `$state`/`$derived`/`$effect`
- `plugins/typescript/knowledge/` — Strict TypeScript, type safety, no `any`
- `plugins/tauri/knowledge/` — Tauri v2 commands, IPC patterns, events

**On a different tech stack**, only the plugin sources change. Software plugin knowledge (business domain, architecture patterns, team protocols) stays the same.

### 9.5 Enforcement Mechanisms

1. **TeamCreate + background spawning (existing rule)**
   - Enforce via orchestrator's session start protocol

2. **Persistent team lifecycle (new rule)**
   - Agents stay active until TeamDelete
   - Specialists terminate cleanly after task completion
   - No lingering background processes

3. **Resource synchronization (enhance RULE-87ba1b81)**
   - SendMessage before starting compilation
   - Wait for notification before proceeding
   - Document in team protocol

4. **Handoff protocol verification**
   - Integration Lead validates before unblocking next layer
   - SendMessage confirmation logged
   - Evidence of end-to-end wiring required

---

## Part 9.6: Adapting the Team to Different Tech Stacks

The persistent team model is **stack-agnostic**. To deploy the same team to a different technology stack:

**Step 1: Identify the new tech stack**
```
Current (OrqaStudio): Rust backend + Svelte frontend + TypeScript + Tauri
Target (Example): Python backend + React frontend + TypeScript
```

**Step 2: Identify equivalent language plugins**
```
Rust → Python       (plugins/python/)
Svelte → React      (plugins/react/)
TypeScript → (unchanged, or plugins/typescript-react/)
Tauri → Flask/FastAPI (plugins/fastapi/ or plugins/flask/)
```

**Step 3: No agent changes needed**
The team agents (`AGENT-backend-lead-sw.md`, `AGENT-frontend-lead-sw.md`, etc.) do NOT change. Only their knowledge sources change.

**Step 4: Update agent knowledge references**
```yaml
# BEFORE (OrqaStudio)
Backend Lead knows:
  - rust-async-patterns
  - tauri-v2-integration

# AFTER (Python + FastAPI)
Backend Lead knows:
  - python-async-patterns
  - fastapi-integration
```

**Example adaptation table:**

| Role | OrqaStudio Knowledge | Python + React Equivalent |
|------|---------------------|---------------------------|
| Backend Lead | Rust async + Tauri | Python async + FastAPI |
| Frontend Lead | Svelte 5 runes | React hooks + TypeScript |
| Integration Lead | Tauri IPC + NDJSON | REST API / WebSocket |
| Governance Lead | (framework, unchanged) | (framework, unchanged) |

**Result:** The same 4 team agents, deployed to a completely different tech stack. No role redesign needed. No workflow changes needed. Only knowledge sources swap.

This is the composability principle in action.

---

## Part 10: Transition Timeline

### Phase 1: Documentation (This session)
- Document recommended team structure ✓ (this proposal)
- Update Orchestrator agent definition
- Create team orchestration rule
- Update team member agent definitions with team context

### Phase 2: Enforcement (Next session)
- Orchestrator spawns core team at session start
- Core team reads team protocol rule
- Orchestrator documents handoff expectations
- Test with single feature through 4-layer implementation

### Phase 3: Validation (Following sessions)
- Run multiple features through persistent team model
- Capture handoff issues as lessons
- Refine based on real usage
- Promote stable patterns to rules

### Phase 4: Specialization (Later)
- Define specialist pool criteria
- Document when/how to spawn for bounded tasks
- Refine resource synchronization rules
- Document cleanup procedures

---

## Part 11: Success Metrics

A persistent team model succeeds when:

1. **Context continuity improves** — Backend Lead retains knowledge across 3+ tasks without re-learning
2. **Resource contention drops** — No more compilation failures due to parallel cargo builds
3. **Agent shutdown cleans up** — No zombie agents after task completion
4. **Handoff discipline works** — Features flow 4-layer chain without rework
5. **Session quality improves** — Features complete with fewer rounds of review
6. **Learning accumulates** — Lessons from session N inform session N+1's approaches

---

## Conclusion

OrqaStudio's current agent model treats implementation as a commodity — spawn agent, get work done, die. A persistent team model treats implementation as expertise — maintain specialists, let them grow in domain knowledge, coordinate their handoffs.

The recommended structure is composable and stack-agnostic:

**Core principles:**
- 5 core agents (orchestrator + 4 specialists) persistent for entire session
- Specialist pool for planning, review, research (spawned as needed)
- Clear ownership boundaries per domain
- Handoff protocol via SendMessage
- Clean lifecycle: spawn → work → notify → next → terminate

**Portability:**
- Team agents live in software plugin, consume knowledge from language/framework plugins
- Same Backend/Frontend/Integration/Governance Leads on any tech stack
- Swap language plugins; team roles stay identical
- Python + Django + React project uses same team agents as Rust + Tauri + Svelte

This mirrors how experienced IRL teams work and removes the failure modes that plague disposable-agent approaches. The composability principle makes the investment in team design multiply across different projects.

**Next steps:**
1. Get user feedback on team structure and composability principle
2. Create agents in `plugins/software/agents/` and rule in `.orqa/process/rules/`
3. Update Orchestrator agent definition with team management section
4. Test with real features in next session
