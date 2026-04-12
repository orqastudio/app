# OrqaStudio

OrqaStudio is an **opinionated clarity engine** that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection. It is not an AI development tool — it is cognitive infrastructure for structured reasoning that happens to use AI as its reasoning partner. Software development is its first domain, but the product's identity is rooted in principled structured thinking, not code generation.

## Product Pillars

Every feature and decision is evaluated against these three pillars. When pillars conflict, the conflict is flagged to the user — agents do not prioritise one pillar over another.

### Clarity Through Structure

Making thinking, standards, and decisions visible and structured.

- Does this make governance artifacts visible and manageable?
- Does it produce structured knowledge (plans, decisions, rules)?
- Does it enforce understanding before action?
- Does the system mechanically enforce its own structural rules?

### Learning Through Reflection

The system and its users improve over time through structured retrospection.

- Does this capture lessons, discoveries, or patterns?
- Does it track metrics showing improvement (or regression)?
- Does it feed retrospectives back into the governance framework?
- Are discovered enforcement gaps acted on immediately, not deferred?

### Purpose Through Continuity

The system actively maintains coherence between intention and action.

- Does this help users stay oriented toward their original purpose?
- Does this prevent knowledge, decisions, or context from being silently lost?
- Does this make scope drift visible and require explicit approval?
- Does this reduce the user's cognitive burden rather than adding to it?

## Design Principles

| # | Principle | Constraint |
|---|-----------|------------|
| P1 | Plugin-Composed Everything | No governance pattern hardcoded in engine. Plugins provide definitions, engine provides capabilities. |
| P2 | One Context Window Per Task | Each agent spawns fresh for a single task. No persistent agents, no accumulated context. |
| P3 | Generated, Not Loaded | System prompts are generated from plugin registries and workflow state, not loaded from disk. |
| P4 | Declarative Over Imperative | State machines, guards, and workflows are YAML declarations validated by JSON Schema. |
| P5 | Token Efficiency as Architecture | 2-4x overhead ratio. Per-agent prompts: 1,500-4,000 tokens. |
| P6 | Hub-Spoke Orchestration | Persistent orchestrator coordinates ephemeral task-scoped workers via structured summaries. Orchestrator delegates review to a Reviewer agent and reads the verdict — it does not self-assess. |
| P7 | Resolved Workflow Is a File | After plugin composition, the resolved workflow is a deterministic YAML file on disk. |

**Core product principles:** Accuracy over speed. Mechanical enforcement enables autonomy. The learning loop hardens the system.

## Artifact System — Navigation Index

OrqaStudio's `.orqa/` directory is the governance artifact system. All artifacts are structured markdown with YAML frontmatter, typed relationships, and lifecycle states.

### Discovery (product definition)

- **Vision:** `.orqa/discovery/vision/VISION-4893db55.md` — product mission, identity, entry modes, interaction model
- **Pillars:** `.orqa/discovery/pillars/PILLAR-*.md` — 3 pillars with gate questions and conflict resolution
- **Personas:** `.orqa/discovery/personas/PERSONA-*.md` — Alex (Lead), Sam (Practitioner), Jordan (Independent)
- **Ideas:** `.orqa/discovery/ideas/` — 156 captured ideas
- **Research:** `.orqa/discovery/research/` — 68 research artifacts

### Documentation

- **Architecture:** `.orqa/documentation/architecture/` — 19 docs covering core architecture, plugins, agents, connectors, state machines, enforcement, codebase structure
- **Platform:** `.orqa/documentation/platform/` — 37 docs covering thinking modes, enforcement architecture, artifact workflows, orchestration, design system, tool definitions, information architecture
- **Project:** `.orqa/documentation/project/` — 35 docs covering Rust module architecture, coding standards, streaming pipeline, search engine, brand guidelines, go-to-market
- **Guides:** `.orqa/documentation/guides/` — 9 development guides (Rust, Svelte, Tauri, delivery, environment setup)
- **Reference:** `.orqa/documentation/reference/` — CLI reference, standards, licensing

### Implementation

- **Epics:** `.orqa/implementation/epics/` — 132 epics tracking feature delivery
- **Milestones:** `.orqa/implementation/milestones/` — 3 milestones

### Learning (the system gets smarter)

- **Decisions:** `.orqa/learning/decisions/` — 70 project decisions with rationale
- **Lessons:** `.orqa/learning/lessons/` — 84 implementation lessons with recurrence tracking
- **Rules:** `.orqa/learning/rules/` — 60 promoted rules (lessons that recurred enough to become enforcement)

### Planning

- **Plans:** `.orqa/planning/` — active plans including SurrealDB storage migration
- **Wireframes:** `.orqa/planning/wireframes/` — 5 wireframe specs

### Key Architecture Docs (read before implementation work)

| Doc | Title | Read when... |
|-----|-------|-------------|
| `DOC-62969bc3` | Core Architecture | ...starting any architectural work |
| `DOC-41ccf7c4` | Plugin Architecture | ...working on plugins, composition, taxonomy |
| `DOC-b951327c` | Agent Architecture | ...working on agent roles, prompt pipeline |
| `DOC-4d531f5e` | Connector Architecture | ...working on the Claude Code connector or tool integration |
| `DOC-70063f55` | State Machine & Enforcement | ...working on workflows, guards, enforcement |
| `DOC-fd3edf48` | Governance Artifacts | ...modifying `.orqa/` structure |
| `DOC-80a4cf76` | Key Decisions | ...making a new architectural decision |
| `DOC-762facfb` | Codebase Structure | ...adding or moving files |

## Thinking Modes

The orchestrator routes work through thinking modes based on what the user is asking for. Each mode has a dedicated platform doc with activation signals, workflow steps, and quality criteria.

| Mode | When | Platform Doc |
|------|------|-------------|
| **Research** | Investigate, explore, compare, understand | `DOC-36befd20` |
| **Planning** | Design approach, map dependencies, sequence work | `DOC-4a4241a5` |
| **Implementation** | Build, fix, add, refactor — concrete deliverables | `DOC-f7fb7aa7` |
| **Review** | Check, validate, audit — PASS/FAIL verdict, not fixes | `DOC-fd636a56` |
| **Documentation** | Write, update, organise docs and knowledge | `DOC-bf70068c` |
| **Debugging** | Reproduce, isolate, diagnose failures | `DOC-b95ec6e3` |
| **Learning Loop** | Capture observations, promote lessons to rules | `DOC-83039175` |

Read the relevant thinking mode doc before starting work — it defines the workflow and quality expectations for that mode.

## Team Discipline

### Always Use Agent Teams

ALL work MUST use team infrastructure:

1. `TeamCreate` — create a named team before spawning agents
2. `TaskCreate` — create tasks within the team for tracking
3. `Agent` — spawn agents with `run_in_background: true` and `team_name` set
4. `TaskUpdate` — agents mark tasks complete, orchestrator verifies via findings file
5. `TeamDelete` — clean up after committing work

Never spawn a bare Agent without a team. Never run agents in the foreground.

### Hub-Spoke Coordination

- The orchestrator coordinates. It does NOT implement.
- Delegate ALL implementation to background agents via teams.
- Read structured summaries from findings files — do not accumulate agent output in context.
- Stay available for conversation with the user.

### Agent Delegation

| Task Type | Agent Role | Model |
|-----------|-----------|-------|
| Code changes, tests, configs | Implementer | sonnet |
| Quality verification, AC checks | Reviewer | sonnet |
| Investigation, information gathering | Researcher | sonnet |
| Documentation creation/editing | Writer | sonnet |
| Approach design, dependency mapping | Planner | opus |
| UI/UX design, component structure | Designer | sonnet |
| `.orqa/` artifact maintenance | Governance Steward | sonnet |

### Role-Based Tool Constraints

| Role | Can Edit | Can Run Shell | Artifact Scope |
|------|----------|---------------|----------------|
| Orchestrator | No | No | Read-only, delegation |
| Implementer | Yes | Yes | Source code only |
| Reviewer | No | Yes (checks only) | Read-only, produces verdicts |
| Researcher | No | No | Creates research artifacts only |
| Writer | Yes | No | Documentation only |
| Planner | Yes | No | Delivery artifacts only |
| Designer | Yes | No | Design artifacts, component code |
| Governance Steward | Yes | No | `.orqa/` artifacts only |

### Mandatory Independent Review

Every completed task MUST be reviewed by a Reviewer agent. The orchestrator spawns the implementer, reads findings, spawns a separate Reviewer to verify each acceptance criterion with evidence, reads the verdict, and only accepts on PASS. Self-assessment is not review.

### Completion Gate

Before creating a new team: read all findings files, verify every acceptance criterion is DONE or FAILED (not "deferred"), ensure Reviewer PASS verdicts, fix failures or get explicit user approval to defer, commit all changes, `TeamDelete`, then proceed.

### No Autonomous Decisions

When an agent encounters ambiguity: check architecture docs → raise to orchestrator → escalate to user. Agents do NOT improvise — unclear requirements are signals to escalate.

## Key Design Decisions

- **Forward-only relationships** — task stores `delivers: epic`, epic does NOT store `delivered-by: task`. Graph computes inverses.
- **Plugin-composed everything** — no governance patterns hardcoded in core.
- **Daemon is business logic boundary** — MCP/LSP are access protocols, not application boundaries.
- **32 relationship types** — semantic precision creates structure. Each type is a unique bond. Narrow from/to constraints.
- **No backwards compatibility** — pre-release, breaking changes expected, data migrated via `orqa migrate`.
- **.state/ not tmp/** — session state and metrics are operational data, not disposable.

## Zero Tech Debt

- **Delete legacy code** — do not comment it out, do not wrap it in feature flags
- **No backwards compatibility** — pre-release, breaking changes are expected
- **No "we'll fix this later"** — if it doesn't match the architecture, fix it now
- **Every file justifies its existence** against the architecture

## Current Project Focus

**Read `.claude/PLAN-mvp.md` before starting any work.** It defines the MVP scope, the three irreducible capabilities, six work streams, and what is explicitly post-MVP. All work should be evaluated against this plan.

### MVP Path

The MVP targets non-technical thinkers (PMs, consultants, researchers) for a public beta download. Three irreducible capabilities: the artifact graph (on SurrealDB), the learning loop, and the methodology layer. Philosophy: build a solid foundation that scales, not a minimum viable product that ships fast.

### Dual Representation (Keep In Sync)

The MVP scope is represented in two places that MUST stay aligned:

| Location | Purpose | Consumer |
|----------|---------|----------|
| `.claude/PLAN-mvp.md` | Working plan — guides Claude sessions | Claude Code / Cowork |
| `.orqa/implementation/milestones/MS-21d5096a.md` | Artifact graph representation | OrqaStudio's artifact system |
| `.orqa/implementation/milestones/MS-b1ac0a20.md` | Dogfooding waypoint (subset of MVP) | OrqaStudio's artifact system |

When MVP scope changes: update PLAN-mvp.md first (it's the source of truth for now), then update the milestones to match. When the connector is built and PLAN becomes a proper artifact type, the `.orqa/` artifacts become the source of truth and the `.claude/` plan becomes generated output.

### Artifact Cleanup (Pre-Migration — Active)

The `.orqa/` artifacts need cleanup before SurrealDB migration. **Read `.claude/BRIEF-artifact-cleanup.md` before starting cleanup work.** It defines five phases: structural fixes, architecture doc refresh, epic realignment, learning artifacts, and discovery/planning. Each phase has explicit acceptance criteria and must be reviewed before the next begins.

### SurrealDB Migration (Stream 1 — Critical Path)

Replacing the markdown-based artifact system with **SurrealDB as source of truth**.

- **Storage migration plan:** `.orqa/planning/PLAN-storage-migration.md`
- **Graph DB PoC:** `engine/graph-db/` — standalone SurrealDB proof-of-concept crate
- **Three deployment tiers:** local (embedded SurrealDB + SQLite), self-hosted (SurrealDB server + Postgres), cloud-hosted
- **Git becomes automatic background infrastructure** for version history and audit trails
- **P7 revised:** "Resolved Workflow Is a Record" — deterministic SurrealDB record, exportable as file via `orqa export`

### Migration Task Lists (if resuming migration work)

| File | Phases | Tasks |
|------|--------|-------|
| `.claude/tasks/migration-tasks-phase1-3.md` | Phase 1-3 (targets, engine, daemon) | 68 |
| `.claude/tasks/migration-tasks-phase4-5.md` | Phase 4-5 (connectors, plugins) | ~70 |
| `.claude/tasks/migration-tasks-phase6-8.md` | Phase 6-8 (content, governance, restructure) | ~70 |
| `.claude/tasks/migration-tasks-phase9-11.md` | Phase 9-11 (frontend, validation, docs) | ~70 |

## Autonomous Execution

Work continuously without stopping. Do not ask "shall I proceed?" or "ready for the next task?". The user will interrupt if they want to steer. Silence means continue.

The ONLY acceptable reasons to pause:

1. A genuine blocker you cannot resolve
2. A destructive/irreversible action that could lose user work

## Git Workflow

- Commit at natural boundaries (task completion, phase completion)
- No `--no-verify` — fix errors, don't skip hooks
- After Rust changes: rebuild and restart daemon
- Agents do NOT commit — the orchestrator commits after reviewing findings

## Session Protocol

1. Read this file
2. Check `.state/session-state.md` for previous session context
3. Check `git status` and `git stash list`
4. Resume from where the previous session left off
5. Begin working immediately — do not summarise what you are about to do
6. Write session state to `.state/session-state.md` periodically
