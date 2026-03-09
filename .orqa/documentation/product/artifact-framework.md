---
title: "Artifact Framework"
description: "Schema definitions, lifecycle rules, and governance for all structured artifacts in .orqa/."
category: product
tags: [artifacts, governance, structured-thinking]
created: 2026-03-07
updated: 2026-03-07
---

**Date:** 2026-03-07

> OrqaStudio™ is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

This document defines the artifact types, schemas, connections, and lifecycle that form OrqaStudio's structured thinking layer. All artifacts are markdown documents with YAML frontmatter, stored in `.orqa/` and browsable in the UI.

---

## Design Principles

### Markdown-First

Artifacts are markdown documents. The underlying UX model is **browsable markdown with structured frontmatter**. Every artifact must be readable and useful in its raw form — in OrqaStudio's UI, in a text editor, or on GitHub.

Richer views (kanban boards, dashboards, graph visualizations, priority matrices) are **optional layers built on top** of the markdown. They project different views of the same underlying data. The markdown is the source of truth.

### Frontmatter as Structured Data

YAML frontmatter provides machine-parseable metadata that enables:

- **Indexing** — OrqaStudio can parse frontmatter into SQLite for search, filtering, and aggregation
- **Connections** — Fields like `milestone`, `epic`, `depends-on` create a navigable web of relationships
- **Status tracking** — Workflow states are frontmatter fields, not separate databases
- **Priority scoring** — Dimension scores in frontmatter feed the prioritisation framework

### Title and Description Guidelines

Artifact titles appear in sidebar navigation, list views, and breadcrumbs. They must be **human-descriptive** and **concise**.

| Rule | Guideline |
|------|-----------|
| **Length** | Target under 40 characters. Hard limit at 50. |
| **Style** | Describe the outcome or concept, not the implementation |
| **No code** | Avoid function names, variable names, file paths, or technical syntax |
| **No process verbs** | Prefer nouns/outcomes over "Audit X", "Update Y", "Fix Z" (unless that IS the outcome) |
| **Capitalisation** | Title Case for all artifact titles |

**Examples:**

| Too Long / Too Technical | Better |
|--------------------------|--------|
| "Rename sdk_session_id to provider_session_id across stack" | "Provider-Neutral Session ID Naming" |
| "Update all agent definitions for three-tier skill loading" | "Three-Tier Skill Loading for Agents" |
| "Using copies instead of symlinks for .claude/ causes divergence" | "Symlinks Prevent Governance Divergence" |
| "$derived(() => expr) causes infinite reactivity loops in Svelte 5" | "Derived Rune Infinite Loop Pitfall" |

Descriptions can be longer and more technical — they appear in detail views, not navigation.

### Documentation-First Gate

Documentation is not an afterthought — it is a **gate for work**. Every epic declares:

- **`docs-required`** — what documentation must exist and be approved before implementation begins
- **`docs-produced`** — what documentation this work will create or update on completion

This creates an automated traceability chain: research validates ideas, epics capture implementation design, implementation produces architecture decisions and doc updates. At every stage, the system records what was decided and why, so you can trace back to understand how you got here and inform what comes next.

This is a key differentiator of OrqaStudio as a platform. Projects built with OrqaStudio self-document their decision history through the artifact framework. The documentation isn't separate from the work — it IS part of the work.

### Deadlines

Some projects operate under time constraints. The artifact framework supports optional deadlines on milestones and epics. When a deadline is present:

- It adds **urgency context** to prioritisation — a P2 item with an imminent deadline should be treated as effectively P1
- It enables **time-based views** — timeline projections, deadline warnings, schedule risk analysis
- It remains **optional** — projects without hard deadlines (like conceptual or research projects) simply omit the field

Deadlines are a reality of commercial practice. The framework accommodates them without requiring them.

### Progressive Enhancement

The same artifact works at three levels of sophistication:

1. **File on disk** — readable in any text editor, discoverable with `find` or `grep`
2. **Browsable in UI** — OrqaStudio renders the markdown with syntax highlighting and navigation
3. **Interactive views** — dashboards, boards, and graphs project frontmatter data into richer interfaces

Each level adds capability without replacing the previous one. A user who never opens the interactive views still has full access to all knowledge through the markdown.

---

## Governance Classification

All governance artifacts (agents, skills, rules, hooks) carry a `layer` field that determines their scope and editability.

### Three Layers

| Layer | Meaning | Ships With | Editable By User |
|-------|---------|------------|------------------|
| `canon` | Platform principles — applies to ALL projects | The app | No (updated centrally) |
| `project` | Project-specific enforcement and patterns | The project's `.orqa/` | Yes |
| `plugin` | Ecosystem-extensible contributions | Installed via skills CLI or plugin system | Yes (but managed externally) |

### Governance Concept Definitions

Each governance concept type has a distinct purpose. See AD-029 for the full decision.

| Concept | Definition | Test |
|---------|-----------|------|
| **Agent** | A portable role you delegate work to. Has a distinct workflow and deliverable type. | "I need someone to do X" |
| **Skill** | Domain knowledge or methodology loaded into an agent's context. Shapes how work is done. | "The person doing X needs to know Y" |
| **Rule** | A constraint that must be followed. Binary: compliant or not. | "Anyone doing anything must follow Z" |
| **Hook** | An automated action triggered by an event. Mechanical enforcement. | "When event E happens, automatically do A" |
| **Lesson** | A learned pattern from experience. Promoted to rules/skills at recurrence threshold. | "We learned W the hard way" |

### Agent vs Skill Decision Framework

| Question | If Yes → | If No → |
|----------|----------|---------|
| Would you hire a different person for this? | Agent | Skill |
| Does it have a unique deliverable type? | Agent | Skill |
| Does it work across any project domain? | Agent | Skill |
| Is it a lens or methodology applied to existing work? | Skill | Agent |
| Can it be loaded by multiple different roles? | Skill | Agent |

### Universal Agent Roles

Agents are portable roles that work across any project type. Domain-specific capability is loaded via skills. See AD-029.

| Role | Purpose |
|------|---------|
| **Orchestrator** | Coordinates work, enforces process, manages governance |
| **Researcher** | Investigates, gathers information, analyses findings |
| **Planner** | Designs approaches, evaluates tradeoffs, maps structure |
| **Implementer** | Does the work — whatever "work" means in this domain |
| **Reviewer** | Checks quality, compliance, and correctness |
| **Writer** | Creates documentation, communications, and records |
| **Designer** | Designs experiences, interfaces, and structures |

### Frontmatter Fields

```yaml
# Agents
layer: canon        # canon | project | plugin
scope: general      # general | software-engineering | governance

# Skills
layer: project      # canon | project | plugin

# Rules
layer: canon        # canon | project

# Hooks
layer: canon        # canon | project
```

### Classification Decisions

- **Canon skills** are portable (work across any project): `planning`, `architecture`, `svelte5-best-practices`, etc.
- **Project skills** are project-specific (capture THIS codebase's patterns): `orqa-ipc-patterns`, `orqa-store-patterns`, etc.
- **Canon rules** enforce platform principles: `documentation-first`, `honest-reporting`, `systems-thinking`, etc.
- **Project rules** enforce project-specific conventions: `development-commands` (make targets), `dogfood-mode`.

---

## Artifact Types

### Overview

Artifact types fall into three management layers. **Canon** artifacts are managed by the app (installed and updated centrally, not user-editable). **Project** artifacts are created and managed by the user and AI. **Plugin** artifacts are installed from the ecosystem. All artifact instances live under `.orqa/` regardless of layer.

| Type | ID Pattern | Directory | Layer | Purpose |
|------|-----------|-----------|-------|---------|
| **Pillar** | `PILLAR-NNN` | `.orqa/planning/pillars/` | Project | Guiding principle that features are evaluated against |
| **Milestone** | `MS-NNN` | `.orqa/milestones/` | Project | Strategic goal with gate question |
| **Epic** | `EPIC-NNN` | `.orqa/epics/` | Project | Trackable work unit within a milestone |
| **Task** | `TASK-NNN` | `.orqa/tasks/` | Project | Individual implementation unit within an epic |
| **Idea** | `IDEA-NNN` | `.orqa/ideas/` | Project | Candidate for future work, needs validation |
| **Lesson** | `IMPL-NNN` | `.orqa/lessons/` | Project | Learning capture from implementation |
| **Research** | `RES-NNN` | `.orqa/research/` | Project | Investigation, design exploration, or implementation plan — produces decisions |
| **Rule** | `RULE-NNN` | `.orqa/governance/rules/` | Canon/Project | Constraint that must be followed — binary: compliant or not |
| **Decision** | `AD-NNN` | `.orqa/decisions/` | Project | Architecture decision record — captures what was decided and why |

### Type Definitions (When to Use Each)

| Type | Use This When | NOT This |
|------|--------------|----------|
| **Pillar** | Defining a guiding principle that the project evaluates all work against. Every feature must serve at least one active pillar. | Don't use for specific constraints — that's a rule. Pillars are strategic principles, not enforcement. |
| **Milestone** | Defining a strategic goal that groups related epics. Has a gate question that must be answerable "yes" when complete. | Don't use for individual features — that's an epic. |
| **Epic** | Scoping a trackable body of work with clear deliverables, acceptance criteria, and documentation gates. Titles describe outcomes, not process. | Don't use for investigation — that's research. Don't use for one-off tasks. |
| **Task** | Tracking an individual implementation unit within an epic. Has a specific assignee, acceptance criteria, and scope. | Don't use for standalone work — tasks always belong to an epic. |
| **Idea** | Capturing a future possibility that needs investigation before committing. Must go through the shaped→promoted lifecycle. | Don't use for approved work — promote to epic first. |
| **Research** | Investigating a question, exploring options, auditing existing state. Produces findings that inform decisions or epics. Flat directory, related via YAML fields. | Don't use for implementation plans — that goes in the epic body. Don't use subdirectories — use `milestone:` and `epic-ref:` fields. |
| **Decision** | Recording an architectural or process choice with rationale. Captures what was decided and why, enabling future understanding. | Don't use for investigation — that's research. Decisions are conclusions, not explorations. |
| **Lesson** | Capturing a reusable pattern discovered during implementation. Tracks recurrence and promotes to rules/skills at threshold. | Don't use for process changes — that's a retrospective entry or rule update. |

Lessons and Research already have established schemas (see their respective READMEs). This document defines the schemas for Milestones, Epics, Tasks, and Ideas, and the connections between all types.

### Connections

```
Pillar ──referenced-by──> Epic, Idea (pillars: [PILLAR-NNN])
  │                        └── evaluated against pillar test-questions
  │
Milestone
  └── Epic (milestone: MS-NNN, pillars: [PILLAR-NNN])
        ├── Task (epic: EPIC-NNN)  — inline checklist or separate file
        └── research-refs: []  — design explorations and investigations

Idea ──promote──> Epic (when validated)

Lesson ──promote──> Rule / Skill / Coding Standard

Research ──promote──> Decision (AD-NNN)

Decision ──supersedes──> Decision (when updated)
```

---

## Schemas

### Pillar (`PILLAR-NNN`)

Pillars are the guiding principles that a project evaluates all work against. Every feature, epic, and idea must serve at least one active pillar. Pillars are project-configurable — different projects define different principles. The `priority` field determines conflict resolution order (lower number = higher priority).

```yaml
---
id: PILLAR-001
title: "Clarity Through Structure"
status: active                    # active | inactive
description: >
  Making thinking, standards, and decisions visible and structured.
test-questions:
  - Does this make governance artifacts visible and manageable?
  - Does it produce structured knowledge (plans, decisions, rules)?
  - Does it surface what would otherwise be hidden?
priority: 1                       # Conflict resolution order (1 = highest)
created: 2026-03-09
updated: 2026-03-09
tags: [visibility, structure, governance]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `PILLAR-NNN` identifier |
| `title` | Yes | string | Human-readable pillar name |
| `status` | Yes | enum | `active` (enforced), `inactive` (preserved but not evaluated against) |
| `description` | Yes | string | What this pillar means — used in system prompt injection |
| `test-questions` | Yes | string[] | Questions to evaluate whether work serves this pillar |
| `priority` | Yes | integer | Conflict resolution order (1 = highest priority; wins when pillars conflict) |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `tags` | No | string[] | Freeform tags |

The pillar body contains the full narrative: what the pillar means in practice, examples of work that serves it, anti-patterns that violate it. The `description` and `test-questions` fields are the machine-readable summary used for system prompt injection and scoring.

### Milestone (`MS-NNN`)

Milestones are strategic goals. Each has a gate question that determines when the milestone is complete.

```yaml
---
id: MS-001
title: "Dogfooding"
status: active                    # planning | active | complete
description: >
  OrqaStudio is usable as a daily workspace for building OrqaStudio itself.
created: 2026-03-07
updated: 2026-03-07
deadline: null                    # ISO date or null — optional time constraint
gate: "Can we use this app instead of the terminal for governance management, conversation debugging, and structured thinking about the project?"
epic-count: 10                    # Total epics in this milestone
completed-epics: 0                # Epics with status: done
tags: []
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `MS-NNN` identifier |
| `title` | Yes | string | Human-readable milestone name |
| `status` | Yes | enum | `planning`, `active`, `complete` |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `deadline` | No | date/null | ISO date for time-constrained milestones, null otherwise |
| `gate` | Yes | string | Question that determines completion |
| `description` | Yes | string | What this milestone achieves |
| `epic-count` | No | integer | Total epics belonging to this milestone |
| `completed-epics` | No | integer | Epics with `status: done` |
| `tags` | No | string[] | Freeform tags |

### Epic (`EPIC-NNN`)

Epics are trackable work units. Each belongs to a milestone, may reference research documents that informed its design, and contains tasks. The epic body holds the implementation design — the context, approach, and decisions that would previously have lived in a separate plan document.

```yaml
---
id: EPIC-001
title: "AI Transparency Wiring"
status: draft                     # draft | ready | in-progress | review | done
priority: P1                     # P1 | P2 | P3
milestone: MS-001
description: >
  Wire AI transparency events through the streaming pipeline so the
  user can see system prompts, tool calls, and thinking in real time.
created: 2026-03-07
updated: 2026-03-07
research-refs: []                 # RES-NNN identifiers of research docs that informed this epic
docs-required:                    # Documentation that must exist before work begins
  - docs/architecture/streaming-pipeline.md
docs-produced:                    # Documentation this work creates or updates
  - docs/architecture/streaming-pipeline.md (update with new events)
scoring:                          # Priority dimension scores
  pillar: 5
  impact: 5
  dependency: 3
  effort: 2
  score: 17.5                    # Computed: (pillar*3 + impact*2 + dependency*3) / effort
tags: [streaming, transparency]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `EPIC-NNN` identifier |
| `title` | Yes | string | Human-readable epic title |
| `status` | Yes | enum | `draft`, `ready`, `in-progress`, `review`, `done` |
| `priority` | Yes | enum | `P1`, `P2`, `P3` — derived from score |
| `milestone` | Yes | string | Milestone ID this belongs to |
| `description` | No | string | Brief description of the epic |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `research-refs` | No | string[] | `RES-NNN` identifiers of research docs that informed this epic |
| `docs-required` | No | string[] | Documentation that must exist before work begins |
| `docs-produced` | No | string[] | Documentation this work will create or update |
| `pillars` | No | string[] | Pillar IDs this epic serves (e.g., `[PILLAR-001, PILLAR-002]`) |
| `scoring` | No | object | Dimension scores for priority calculation (includes computed `score` field) |
| `tags` | No | string[] | Freeform tags |

**Task checklists in epics:** Tasks are listed as markdown checklists in the epic body. When a task needs its own detailed tracking (acceptance criteria, agent assignment, discussion), it graduates to a separate `TASK-NNN.md` file in `.orqa/tasks/`.

### Task (`TASK-NNN`)

Tasks are individual implementation units. Most tasks live as checklist items in their parent epic. Separate task files are created only when the task needs its own detailed tracking.

```yaml
---
id: TASK-001
title: "Emit SystemPromptSent event from stream_commands.rs"
status: todo                      # todo | in-progress | done
epic: EPIC-001
description: >
  Add SystemPromptSent event emission before the sidecar call so the
  frontend can display system prompt content in the conversation.
created: 2026-03-07
updated: 2026-03-07
depends-on: []                    # Task IDs that must be done before this can start
assignee: backend-engineer
skills: [chunkhound, orqa-ipc-patterns, orqa-streaming]
scope:                            # Files/directories affected
  - src-tauri/src/commands/stream_commands.rs
acceptance:                       # What "done" looks like
  - SystemPromptSent event emitted before sidecar call
  - Event carries custom_prompt and governance_prompt
  - Frontend receives and displays the event
tags: []
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `TASK-NNN` identifier |
| `title` | Yes | string | Concise task description |
| `status` | Yes | enum | `todo`, `in-progress`, `done` |
| `epic` | Yes | string | Parent epic ID |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `depends-on` | No | string[] | Task IDs that must be `done` before this task can move to `in-progress` |
| `assignee` | No | string | Agent name |
| `skills` | No | string[] | Skills the assignee should load before starting — enables traceability from plan → task → agent → skills → implementation |
| `scope` | No | string[] | Files/directories affected |
| `acceptance` | No | string[] | Acceptance criteria |
| `tags` | No | string[] | Freeform tags |

**The `skills` field and traceability:** The `skills` field closes the loop from epic to execution. The chain is: **Epic** defines what needs doing → **Task** specifies who does it (`assignee`) and what knowledge they need (`skills`) → **Agent** loads those skills before starting → **Implementation** is done with the right context. Populating `skills` when creating a task ensures no agent picks up work without the codebase knowledge needed to do it well.

### Idea (`IDEA-NNN`)

Ideas are candidates for future work. They need research, validation, and shaping before becoming epics. Each idea captures the concept, its alignment with the vision, and what investigation is needed.

```yaml
---
id: IDEA-001
title: "Multi-Provider Ecosystem"
status: captured                  # captured | exploring | shaped | promoted | archived
pillars: [PILLAR-001]
description: >
  Support additional AI providers through the provider-agnostic sidecar
  interface without changing the Rust core or Svelte UI.
research-needed:                  # What needs investigating before this becomes an epic
  - Provider SDK compatibility assessment
  - Cost model research
  - UX for provider switching
promoted-to: null                 # Epic ID if promoted, null otherwise
tags: [providers, composability]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `IDEA-NNN` identifier |
| `title` | Yes | string | Human-readable idea name |
| `status` | Yes | enum | `captured`, `exploring`, `shaped`, `promoted`, `archived` |
| `pillars` | No | string[] | Pillar IDs this idea serves (e.g., `[PILLAR-001, PILLAR-002]`) |
| `description` | No | string | Brief description of the idea |
| `research-needed` | No | string[] | Questions to answer before promotion |
| `promoted-to` | No | string | Epic ID if promoted, null otherwise |
| `tags` | No | string[] | Freeform tags |

### Decision (`AD-NNN`)

Decisions are architecture decision records. Each captures a significant technical or design choice that constrains future work — what was decided, why, and what the consequences are. Decisions are produced by research and supersede earlier decisions when the situation changes.

```yaml
---
id: AD-001
title: "Use Tauri Channel<T> for streaming IPC"
status: accepted                  # proposed | accepted | superseded | deprecated
description: >
  Use Tauri's Channel<T> mechanism for streaming AI responses from the
  Rust backend to the Svelte frontend.
created: 2026-03-07
updated: 2026-03-07
supersedes: null                  # AD-NNN of the decision this replaces, or null
superseded-by: null               # AD-NNN of the decision that replaced this, or null
tags: [streaming, ipc, tauri]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `AD-NNN` identifier |
| `title` | Yes | string | Human-readable decision title |
| `status` | Yes | enum | `proposed`, `accepted`, `superseded`, `deprecated` |
| `description` | No | string | Brief description of the decision |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `supersedes` | No | string | `AD-NNN` of the decision this replaces, or null |
| `superseded-by` | No | string | `AD-NNN` of the decision that replaced this, or null |
| `tags` | No | string[] | Freeform tags |

The decision body follows the standard structure: **Context** (what situation prompted this decision), **Decision** (what was chosen and why), **Consequences** (what becomes easier, harder, or constrained).

### Research (`RES-NNN`)

Research documents cover investigations, design explorations, architecture spikes, and implementation plans. They replaced the Plan artifact type — the distinction between "investigating something" and "designing an implementation approach" was artificial; both are research activities that produce artifacts referenced by epics and decisions.

Research documents are referenced via `research-refs` on epics, tasks, and decisions using their `RES-NNN` identifier.

```yaml
---
id: RES-001
title: "Streaming IPC Options"
status: complete                  # draft | complete | surpassed
description: >
  Investigation of streaming IPC mechanisms for real-time AI response
  delivery from the Rust backend to the Svelte frontend.
created: 2026-03-07
updated: 2026-03-07
milestone: MS-001                 # Milestone this research serves
surpassed-by: null                # RES-NNN of the doc that supersedes this, or null
tags: [streaming, ipc]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `RES-NNN` identifier |
| `title` | Yes | string | Human-readable research title |
| `status` | Yes | enum | `draft`, `complete`, `surpassed` |
| `description` | No | string | Brief description of what is being investigated |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `milestone` | No | string | Milestone ID this research serves |
| `surpassed-by` | No | string | `RES-NNN` of the research doc that supersedes this, or null |
| `tags` | No | string[] | Freeform tags |

The research body follows the structure: **Question** (what is being investigated), **Research Findings** (what was discovered), **Options Evaluated** (alternatives considered), **Recommendation** (what to do and why).

### Lesson (`IMPL-NNN`)

Lessons capture implementation learnings — unexpected behaviours, non-obvious patterns, and mistakes that should not be repeated. Lessons feed the promotion pipeline: when a lesson recurs enough, it is promoted to a rule, skill update, or coding standard.

```yaml
---
id: IMPL-001
title: "Use typed error enums instead of String errors in Tauri commands"
category: error-handling
status: active
description: >
  Tauri commands returning Result<T, String> lose error context.
  Use thiserror-derived enums for structured error propagation.
recurrence: 0
promoted_to: null
created: 2026-03-07
updated: 2026-03-07
tags: [rust, error-handling, tauri]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `IMPL-NNN` identifier |
| `title` | Yes | string | Human-readable lesson title (no code references) |
| `category` | Yes | string | Lesson category for grouping |
| `status` | Yes | enum | `active`, `recurring`, `promoted` — reflects promotion pipeline state |
| `description` | No | string | Brief description of the lesson |
| `recurrence` | Yes | integer | How many times this lesson has recurred (triggers promotion at threshold) |
| `promoted_to` | No | string | Target artifact if promoted (rule name, skill name, etc.) |
| `created` | Yes | date | ISO date of creation |
| `updated` | Yes | date | ISO date of last update |
| `tags` | No | string[] | Freeform tags |

### Rule (`RULE-NNN`)

Rules enforce coding standards, process requirements, and project conventions. They are loaded as context for agents and verified during code review. Rules may be promoted from lessons (`IMPL-NNN`) when a pattern recurs — the `promoted_from` field traces this lineage.

```yaml
---
id: RULE-006
slug: coding-standards
layer: canon
status: active
title: "Coding Standards"
description: "Enforces Rust and TypeScript coding standards including formatting, linting, error handling, and test coverage."
promoted_from: null               # IMPL-NNN if promoted from a lesson, null otherwise
tags: [coding, standards, quality]
---
```

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Auto-incrementing `RULE-NNN` identifier |
| `slug` | Yes | string | Human-readable slug (original filename without `.md`) for reference |
| `layer` | Yes | enum | `canon` (platform), `project` (project-specific), `plugin` (ecosystem) |
| `status` | Yes | enum | `active` (enforced) or `inactive` (preserved but not enforced) |
| `title` | Yes | string | Human-readable rule title |
| `description` | Yes | string | Brief description of what the rule enforces |
| `promoted_from` | No | string | `IMPL-NNN` if promoted from a lesson, null otherwise |
| `tags` | No | string[] | Freeform tags |

---

## Field Ordering Convention

YAML frontmatter fields follow a consistent content hierarchy across all artifact types. This ordering is not arbitrary — it reflects reading priority: identity first, then classification, then description, then lifecycle metadata, then relationships, then tags.

### Ordering Principles

1. **Identity** — `id`, `title` (who is this?)
2. **Classification** — `status`, `priority`, `category`, `milestone`, `epic`, `pillar` (what kind of thing is it?)
3. **Description** — `description` (what is it about?)
4. **Lifecycle** — `created`, `updated`, `deadline` (when?)
5. **Relationships** — `depends-on`, `research-refs`, `docs-required`, `docs-produced`, `research-needed`, `promoted-to`, `supersedes`, `superseded-by` (what connects to what?)
6. **Scoring** — `scoring` block (how important?)
7. **Operational** — `assignee`, `skills`, `scope`, `acceptance`, `gate`, `epic-count`, `completed-epics`, `recurrence`, `promoted_to` (how is it managed?)
8. **Tags** — `tags` (always last)

### Per-Type Field Order

| Type | Field Order |
|------|------------|
| **Milestone** | id, title, status, description, created, updated, deadline, gate, epic-count, completed-epics, tags |
| **Pillar** | id, title, status, description, test-questions, priority, created, updated, tags |
| **Epic** | id, title, status, priority, milestone, pillars, description, created, updated, research-refs, docs-required, docs-produced, scoring, tags |
| **Task** | id, title, status, epic, description, created, updated, depends-on, assignee, skills, scope, acceptance, tags |
| **Idea** | id, title, status, pillars, description, research-needed, promoted-to, tags |
| **Lesson** | id, title, category, description, recurrence, promoted_to, tags |
| **Rule** | id, slug, layer, status, title, description, promoted_from, tags |
| **Decision** | id, title, status, description, created, updated, supersedes, superseded-by, tags |
| **Research** | id, title, status, description, created, updated, milestone, surpassed-by, tags |

---

## Status Workflows

These are defaults. OrqaStudio's long-term goal is configurable status workflows per project.

### Milestone

```
planning ──> active ──> complete
```

### Epic

```
draft ──> ready ──> in-progress ──> review ──> done
```

### Task

```
todo ──> in-progress ──> done
```

### Idea

```
captured ──> exploring ──> shaped ──> promoted ──> (becomes Epic)
                                  └──> archived    (rejected or deferred)
```

### Research

```
draft ──> complete ──> surpassed (when a newer doc supersedes it)
```

### Lesson (existing)

```
active ──> promoted (recurrence >= threshold)
```

### Decision

```
proposed ──> accepted ──> superseded
                      └──> deprecated
```

---

## Prioritisation Framework

### Dimensions

Priority is computed from scored dimensions. Dimensions are configurable per project via `.orqa/project.json`.

**Default dimensions for OrqaStudio:**

| Dimension | Scale | Weight | What It Measures |
|-----------|-------|--------|-----------------|
| `pillar` | 0-5 | x3 | Alignment with product pillars. 0=neither, 3=one pillar, 5=both. |
| `impact` | 1-5 | x2 | How much this changes the daily user experience. |
| `dependency` | 1-3 | x3 | How many other items are blocked. 1=independent, 3=critical path. |
| `effort` | 1-5 | divisor | Implementation size. Higher = lower score. |

### Scoring Formula

```
score = (pillar * 3 + impact * 2 + dependency * 3) / effort
```

### Priority Bands

| Band | Score Range | Meaning |
|------|-------------|---------|
| **P1** | >= 10.0 | Critical path — milestone blocked without this |
| **P2** | 5.0 - 9.9 | High value — significant quality improvement |
| **P3** | < 5.0 | Nice to have — improves experience but not blocking |

### Configuration

Priority dimensions, weights, and bands are stored in `.orqa/project.json` under the `priority` key. Different projects can define different dimensions and weights. The formula structure (weighted sum with divisor dimension) is fixed; the dimensions and their weights are configurable.

---

## Directory Structure

```
.orqa/
├── project.json              # Project config including priority dimensions
├── planning/
│   └── pillars/
│       ├── README.md            # What pillars are and how to create them
│       ├── PILLAR-001.md        # Clarity Through Structure
│       └── PILLAR-002.md        # Learning Through Reflection
├── milestones/
│   ├── MS-001.md             # Dogfooding
│   └── MS-002.md             # MVP
├── epics/
│   ├── EPIC-001.md           # AI Transparency Wiring
│   ├── EPIC-002.md           # Settings UI
│   └── ...
├── tasks/                    # Only for tasks that need separate files
│   └── TASK-001.md
├── ideas/
│   ├── IDEA-001.md           # Multi-Provider Ecosystem
│   └── ...
├── lessons/                  # Already exists
│   ├── IMPL-001.md
│   └── ...
├── decisions/                # Architecture decision records
│   └── AD-001.md
├── research/                 # Investigations, design explorations, spikes, implementation plans
│   ├── README.md
│   ├── RES-001.md
│   └── ...
└── icon.svg
```

---

## ID Assignment

All artifact IDs auto-increment within their type:

- `PILLAR-001`, `PILLAR-002`, ...
- `MS-001`, `MS-002`, ...
- `EPIC-001`, `EPIC-002`, ...
- `TASK-001`, `TASK-002`, ...
- `IDEA-001`, `IDEA-002`, ...
- `IMPL-001`, `IMPL-002`, ...
- `AD-001`, `AD-002`, ...
- `RES-001`, `RES-002`, ...
- `RULE-001`, `RULE-002`, ...

IDs are stable — never reused after deletion. The next ID is determined by scanning existing files in the directory.

---

## Artifact Visibility in OrqaStudio

### Core UX Principle

**Markdown documents browsable in the UI is the underlying UX model.** Every `.orqa/` artifact is rendered as a readable document within OrqaStudio's artifact browser. This is the foundational layer — all other interaction modes are optional enhancements built on top.

### Visibility Layers

| Layer | What It Provides | When Built |
|-------|-----------------|------------|
| **Document browser** | Navigate and read `.orqa/` artifacts as rendered markdown | Dogfooding (EPIC-005) |
| **Frontmatter sidebar** | Structured metadata displayed alongside the document | Dogfooding (EPIC-005) |
| **Status filtering** | Filter artifacts by status, priority, milestone, type | MVP |
| **Board views** | Kanban-style board projecting epic/task status | MVP |
| **Priority dashboard** | Scored and ranked backlog with band indicators | MVP |
| **Graph views** | Dependency and connection visualisation | Post-MVP |

### What the Browser Shows

For each artifact type, the browser renders:

- **Title and ID** from frontmatter
- **Status badge** with colour coding
- **Priority band** (P1/P2/P3) for epics
- **Milestone membership** for epics
- **Connection links** — clickable references to related artifacts
- **Full markdown body** — the document content, rendered with syntax highlighting

---

## Traceability Web

The frontmatter fields create a navigable web across all artifact types:

```
Milestone ←── milestone ── Epic ←── epic ── Task
                              │                │
                              │                └── depends-on ──→ Other Tasks (execution order)
                              │
                              ├── research-refs ──→ Research (investigations, designs, plans)
                              │
                              ├── depends-on / blocks ──→ Other Epics
                              │
                              ├── docs-required ──→ Existing Docs (gate)
                              │
                              ├── docs-produced ──→ New/Updated Docs (output)
                              │
                              └── pillars ──→ Pillar (PILLAR-NNN)

Pillar ←── pillars ── Idea
Pillar ←── pillars ── Epic

Idea ──research-needed──→ Research ──→ (validates) ──→ promoted-to ──→ Epic
                                    └── docs-produced ──→ Research artifact

Lesson (IMPL-NNN) ──promoted-to──→ Rule (RULE-NNN) / Skill

Research ──produces──→ Decision (AD-NNN) ──supersedes──→ Earlier Decision
```

### Documentation Traceability Chain

Every stage of work self-documents its decisions:

```
Idea captured
  → Research investigates (produces .orqa/research/ artifact)
  → Research resolves → Architecture Decision recorded (.orqa/decisions/AD-NNN.md, indexed in docs/architecture/decisions.md)
  → Idea promoted → Epic created (references research-refs, lists docs-required)
  → Epic body written with implementation design (docs-required gate satisfied)
  → Implementation begins (produces code + docs-produced artifacts)
  → Completion updates architecture docs, component specs, schemas
  → Lessons captured (.orqa/lessons/) → promoted to rules/skills
```

At any point, you can trace backwards: "Why does this rule exist?" → lesson → epic → research → original idea. This is the automated documentation process — the system records what was decided at each stage and why.

This enables:
- **Impact analysis** — "What breaks if this epic is delayed?" (follow `blocks` links)
- **Progress tracking** — "How far along is Milestone 1?" (count epic statuses)
- **Decision traceability** — "Why did we build this?" (epic → research → decision)
- **Learning loops** — "What did we learn?" (lesson → promoted rule)
- **Documentation readiness** — "Can we start this work?" (check `docs-required` gate)
- **Documentation completeness** — "Did we update all the docs?" (check `docs-produced` output)

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | The artifact framework makes thinking visible and structured. Every idea, research document, and work unit is a first-class, browsable, connected document — not hidden in terminal output or chat history. |
| Learning Through Reflection | Ideas, lessons, and research feed the learning loop. Promotion pipelines (idea→epic, lesson→rule, research→decision) ensure knowledge compounds over time. |
