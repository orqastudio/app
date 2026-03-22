---
id: DOC-01ddd8aa
title: Artifact Framework
category: reference
description: "Schema definitions, lifecycle rules, and governance for all structured artifacts in .orqa/."
created: 2026-03-07
updated: 2026-03-18
sort: 11
relationships:
  - target: RULE-65973a88
    type: documents
  - target: AD-e156310d
    type: documents
  - target: AD-10586f26
    type: documents
  - target: AD-21d35c1d
    type: documents
  - target: AD-c8535011
    type: documents
  - target: AD-6dfbba70
    type: documents
  - target: AD-2783985c
    type: documents
  - target: AD-774cc3d0
    type: documents
  - target: AD-a334623b
    type: documents
  - target: AD-e513c9e4
    type: documents
  - target: MS-eea45fa8
    type: documents
  - target: MS-654badde
    type: documents
  - target: EPIC-6787bb93
    type: documents
  - target: EPIC-fe75b52c
    type: documents
  - target: EPIC-642234ba
    type: documents
  - target: EPIC-e045ab6d
    type: documents
  - target: RULE-7b770593
    type: documents
    rationale: Source-of-truth for artifact lifecycle that RULE-7b770593 enforces
  - target: RULE-a764b2ae
    type: documents
    rationale: Defines the artifact schemas and frontmatter contracts that RULE-a764b2ae validates
  - target: RULE-532100d9
    type: documents
---

> OrqaStudio is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

This document defines the artifact types, schemas, relationships, and lifecycle that form OrqaStudio's structured thinking layer. All artifacts are markdown documents with YAML frontmatter, stored in `.orqa/` and browsable in the UI.

The **single source of truth** for artifact types and relationships is `libs/types/src/platform/core.json`. No artifact types or relationship keys are hardcoded in any code path. Everything described here derives from that canonical config.

---

## Design Principles

### Markdown-First

Artifacts are markdown documents. The underlying UX model is **browsable markdown with structured frontmatter**. Every artifact must be readable and useful in its raw form — in OrqaStudio's UI, in a text editor, or on GitHub.

Richer views (kanban boards, dashboards, graph visualisations, priority matrices) are **optional layers built on top** of the markdown. They project different views of the same underlying data. The markdown is the source of truth.

### Frontmatter as Structured Data

YAML frontmatter provides machine-parseable metadata that enables:

- **Indexing** — OrqaStudio parses frontmatter into the artifact node graph for search, filtering, and relationship queries
- **Connections** — The `relationships` array creates a navigable web of typed relationships
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
| "$derived(() => expr) causes infinite reactivity loops in Svelte 5" | "Derived Rune Infinite Loop Pitfall" |

Descriptions can be longer and more technical — they appear in detail views, not navigation.

### Documentation-First Gate

Documentation is not an afterthought — it is a **gate for work**. Every epic declares:

- **`docs-required`** — what documentation must exist before implementation begins
- **`docs-produced`** — what documentation this work will create or update on completion

This creates an automated traceability chain: research validates ideas, decisions capture architectural choices, implementation produces doc updates. At every stage, the system records what was decided and why.

### Progressive Enhancement

The same artifact works at three levels of sophistication:

1. **File on disk** — readable in any text editor, discoverable with `find` or `grep`
2. **Browsable in UI** — OrqaStudio renders the markdown with syntax highlighting and navigation
3. **Interactive views** — dashboards, boards, and graphs project frontmatter data into richer interfaces

Each level adds capability without replacing the previous one.

---

## Governance Classification

All governance artifacts (agents, skills, rules) carry a `layer` field that determines their scope and editability.

### Five Layers

| Layer | Meaning | Ships With | Editable By User |
|-------|---------|------------|------------------|
| `core` | Platform principles — applies to ALL projects | The app | No (updated centrally) |
| `project` | Project-specific enforcement and patterns | The project's `.orqa/` | Yes |
| `plugin` | 1st party official extensions | Installed via plugin system | Yes (but managed externally) |
| `community` | Community-contributed extensions | Installed via community registry | Yes (but managed externally) |
| `user` | User-only personal workflows | User's local config | Yes (private to user) |

### Governance Concept Definitions

| Concept | Definition | Test |
|---------|-----------|------|
| **Agent** | A portable role you delegate work to. Has a distinct workflow and deliverable type. | "I need someone to do X" |
| **Knowledge** | Domain context or methodology injected into an agent's context. Shapes how work is done. Not user-invocable — loaded by the orchestrator before delegating. | "The person doing X needs to know Y" |
| **Skill** | A user-invocable slash command. Triggered explicitly by the user to execute a defined process. Not injected automatically — selected intentionally. | "I want to run process Y right now" |
| **Rule** | A constraint that must be followed. Binary: compliant or not. | "Anyone doing anything must follow Z" |
| **Lesson** | A learned pattern from experience. Promoted to rules/knowledge at recurrence threshold. | "We learned W the hard way" |

### Agent vs Knowledge Decision Framework

| Question | If Yes | If No |
|----------|--------|-------|
| Would you hire a different person for this? | Agent | Knowledge |
| Does it have a unique deliverable type? | Agent | Knowledge |
| Does it work across any project domain? | Agent | Knowledge |
| Is it a lens or methodology injected before delegation? | Knowledge | Agent |
| Can it be loaded by multiple different roles? | Knowledge | Agent |

---

## Artifact Types

Artifact types are defined in `core.json`. They fall into three management layers per AD-c8535011. **Platform** types ship with the app (immutable). **Project** types are created by the user and AI. **Plugin** types are installed from the ecosystem.

### Core Types (from core.json)

| Type | ID Prefix | Icon | Purpose |
|------|-----------|------|---------|
| **Vision** | `VISION` | eye | The product vision — what OrqaStudio is and why it exists |
| **Pillar** | `PILLAR` | columns-3 | Guiding principle that all work is evaluated against |
| **Persona** | `PERSONA` | users | Target user archetype that ideas must benefit |
| **Idea** | `IDEA` | lightbulb | Candidate for future work, needs validation |
| **Decision** | `AD` | scale | Architecture decision record — what was decided and why |
| **Rule** | `RULE` | shield | Constraint that must be followed — binary: compliant or not |
| **Lesson** | `IMPL` | book-open | Learning capture from implementation |
| **Knowledge** | `KNOW` | book-open | Domain context injected into an agent's context before delegation |
| **Agent** | `AGENT` | bot | Portable role that work is delegated to |
| **Document** | `DOC` | file-text | Platform or project documentation |
| **Pivot** | `PIVOT` | refresh-cw | Revision to a foundational artifact (vision, pillar, persona) |
| **Bug** | `BUG` | bug | Defect requiring investigation and fix |

### Project Types (from project.json)

These types are defined in the project's delivery hierarchy, not in core.json:

| Type | ID Prefix | Purpose |
|------|-----------|---------|
| **Milestone** | `MS` | Strategic goal with a gate question |
| **Epic** | `EPIC` | Trackable body of work within a milestone |
| **Task** | `TASK` | Individual implementation unit within an epic |
| **Research** | `RES` | Investigation, design exploration, or spike |

### When to Use Each Type

| Type | Use This When | Not This |
|------|--------------|----------|
| **Pillar** | Defining a guiding principle that all work is evaluated against | Don't use for specific constraints — that's a rule |
| **Milestone** | Defining a strategic goal that groups related epics with a gate question | Don't use for individual features — that's an epic |
| **Epic** | Scoping a trackable body of work with deliverables and acceptance criteria | Don't use for investigation — that's research |
| **Task** | Tracking an individual implementation unit within an epic | Don't use for standalone work — tasks always belong to an epic |
| **Idea** | Capturing a future possibility that needs investigation before committing | Don't use for approved work — promote to epic first |
| **Research** | Investigating a question, exploring options, auditing existing state | Don't use for implementation plans — that goes in the epic body |
| **Decision** | Recording an architectural or process choice with rationale | Don't use for investigation — that's research |
| **Lesson** | Capturing a reusable pattern discovered during implementation | Don't use for process changes — that's a rule update |

---

## Directory Structure

Navigation sections in `project.json` are views into the graph. The canonical directory layout:

```
.orqa/
├── project.json                    # Project config, navigation, statuses, delivery hierarchy
├── principles/
│   ├── pillars/                    # PILLAR-NNN — guiding principles
│   ├── vision/                     # VISION-NNN — product vision
│   ├── personas/                   # PERSONA-NNN — target user archetypes
│   └── grounding/                  # Grounding artifacts
├── discovery/
│   ├── ideas/                      # IDEA-NNN — candidates for future work
│   ├── research/                   # RES-NNN — investigations and explorations
│   └── wireframes/                 # UI wireframes and sketches
├── delivery/
│   ├── milestones/                 # MS-NNN — strategic goals
│   ├── epics/                      # EPIC-NNN — trackable work units
│   └── tasks/                      # TASK-NNN — individual implementation units
├── process/
│   ├── decisions/                  # AD-NNN — architecture decision records
│   ├── rules/                      # RULE-NNN — enforceable constraints
│   ├── lessons/                    # IMPL-NNN — implementation learnings
│   ├── knowledge/                  # KNOW-NNN — domain context injected into agents
│   └── agents/                     # AGENT-NNN — portable roles
└── documentation/
    ├── platform/                   # DOC-NNN — platform docs (ship with app)
    └── project/                    # DOC-NNN — project-specific docs
```

Platform governance artifacts (core agents, core knowledge, core rules) ship in `app/.orqa/process/`. Project-scoped artifacts live in the project's `.orqa/`.

---

## Relationship Vocabulary

All connections use the `relationships` frontmatter array with `target` and `type` fields. There are no standalone connection fields — no `milestone:`, `epic:`, `depends-on:`, `blocks:`, `research-refs:`, or `pillars:` fields. Everything is a typed relationship.

### Connection Format

```yaml
relationships:
  - target: PILLAR-569581e0
    type: grounded
    rationale: Anchored to Clarity Through Structure
  - target: AD-c8535011
    type: driven-by
    rationale: Motivated by the three-layer artifact model decision
```

### Platform Relationships (from core.json)

Organised by semantic category. Every forward relationship has a defined inverse. The integrity engine validates that all relationships are bidirectional and that targets resolve to existing artifacts.

#### Foundation

Relationships anchoring the foundational layer — vision, pillars, personas, and changes to them.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `upholds` | `upheld-by` | pillar | vision | Pillar supports the vision |
| `grounded` | `grounded-by` | idea | pillar | Idea anchored to a foundational principle |
| `benefits` | `benefited-by` | idea | persona | Idea serves a target persona |
| `revises` | `revised-by` | pivot | vision, persona, pillar | Pivot revises a foundational artifact |

#### Lineage

Relationships establishing artifact succession — one thing becoming or spawning another.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `crystallises` | `crystallised-by` | idea | decision | Idea crystallises into an architecture decision |
| `spawns` | `spawned-by` | idea | research | Idea spawns an investigation |
| `merged-into` | `merged-from` | idea | idea | Ideas consolidated — multiple become one |

#### Governance

Relationships where decisions and rules direct behaviour.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `drives` | `driven-by` | decision | epic | Decision motivates a body of delivery work |
| `governs` | `governed-by` | decision | rule | Decision establishes governance |
| `enforces` | `enforced-by` | rule | decision | Rule enforces a decision |
| `codifies` | `codified-by` | rule | lesson | Rule codifies a lesson into enforceable governance |

#### Knowledge-Flow

Relationships where knowledge flows between artifacts.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `informs` | `informed-by` | research | decision | Research findings inform an architecture decision |
| `teaches` | `taught-by` | lesson | decision | Lesson teaches a future decision |
| `guides` | `guided-by` | research | epic | Research guides a body of delivery work |
| `cautions` | `cautioned-by` | lesson | epic | Lesson cautions a body of delivery work |
| `documents` | `documented-by` | doc | epic, decision, rule, milestone | Document describes an artifact for human reference |

#### Observation

Relationships where agents monitor and use capabilities.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `observes` | `observed-by` | agent | epic, task, decision, rule, milestone | Agent monitors an artifact |
| `employs` | `employed-by` | agent | knowledge | Agent employs a knowledge artifact |

#### Synchronisation

Paired content kept in sync.

| Forward | Inverse | From | To | Description |
|---------|---------|------|----|-------------|
| `synchronised-with` | `synchronised-with` | knowledge, doc | knowledge, doc | Paired content — agent-facing and human-facing versions |

### Project Relationships (from project.json)

Projects can define additional relationships beyond the platform vocabulary.

| Forward | Inverse | Description |
|---------|---------|-------------|
| `depends-on` | `depended-on-by` | Work item depends on another work item |

### Delivery Hierarchy Relationships

The delivery hierarchy uses the `delivers` / `delivered-by` relationship pair, configured in `project.json`:

- **Task** `delivers` **Epic**
- **Epic** `delivers` **Milestone**

### Constraint Summary

Several relationships carry type constraints enforced by the integrity engine:

| Constraint | Rule |
|------------|------|
| `grounded` / `grounded-by` | Only targets pillars |
| `enforces` | Only from rules, only to decisions |
| `drives` / `driven-by` | Only from decisions |
| `observes` / `observed-by` | Only from agents |
| `upholds` / `upheld-by` | Only from pillars to vision |
| `benefits` / `benefited-by` | Only from ideas to personas |
| `revises` / `revised-by` | Only from pivots to vision/persona/pillar |

Some relationships also carry **required** constraints (e.g., every rule must `enforces` at least one decision, every agent must `employs` at least one knowledge artifact, every pillar must `upholds` the vision, every idea must be `grounded` and `benefits` a persona).

---

## Status Model

All artifact types share the same 12-status vocabulary, defined in `project.json` (see AD-10586f26). Types use subsets of these statuses appropriate to their lifecycle.

### The 12 Canonical Statuses

| Status | Icon | Allowed Transitions |
|--------|------|---------------------|
| `captured` | circle-dot | exploring, ready, archived |
| `exploring` | search | ready, captured, archived |
| `ready` | check-circle | prioritised, exploring, archived |
| `prioritised` | arrow-up-circle | active, ready, archived |
| `active` | loader | review, hold, blocked |
| `hold` | pause-circle | active, archived |
| `blocked` | x-circle | active, archived |
| `review` | eye | completed, active |
| `completed` | check-circle-2 | archived |
| `surpassed` | fast-forward | archived |
| `archived` | archive | (terminal) |
| `recurring` | repeat | archived |

### Status Flow

```
captured → exploring → ready → prioritised → active → review → completed
                                                ↓         ↑
                                              hold ───────┘
                                                ↓
                                             blocked ─────┘

completed → archived
surpassed → archived
recurring → archived
```

### Auto-Transition Rules

The status model supports graph-query-based auto-transitions. For example:

- When all dependencies of a `blocked` artifact reach `completed`, the artifact automatically transitions to `active`.

---

## Prioritisation Framework

### Dimensions

Priority is computed from scored dimensions. Dimensions are configurable per project via `project.json`.

**Default dimensions for OrqaStudio:**

| Dimension | Scale | Weight | What It Measures |
|-----------|-------|--------|-----------------|
| `pillar` | 0-5 | x3 | Alignment with product pillars |
| `impact` | 1-5 | x2 | How much this changes the daily user experience |
| `dependency` | 1-3 | x3 | How many other items are blocked |
| `effort` | 1-5 | divisor | Implementation size. Higher = lower score |

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

---

## ID Conventions

All artifact IDs use their type prefix followed by a zero-padded number that auto-increments within the type. The prefix is defined by `idPrefix` in `core.json`.

| Type | Pattern | Examples |
|------|---------|----------|
| Vision | `VISION-NNN` | VISION-001 |
| Pillar | `PILLAR-NNN` | PILLAR-569581e0, PILLAR-cdf756ff |
| Persona | `PERSONA-NNN` | PERSONA-cda6edd6 |
| Idea | `IDEA-NNN` | IDEA-7035530f, IDEA-14f3874c |
| Decision | `AD-NNN` | AD-e513c9e4, AD-c6abc8e6 |
| Rule | `RULE-NNN` | RULE-532100d9, RULE-a764b2ae |
| Lesson | `IMPL-NNN` | IMPL-eb748de2, IMPL-f27a1550 |
| Knowledge | `KNOW-NNN` | KNOW-30a419dd, KNOW-c7fb7c83 |
| Agent | `AGENT-NNN` | AGENT-c5284fde, AGENT-ff44f841 |
| Document | `DOC-NNN` | DOC-001, DOC-01ddd8aa |
| Pivot | `PIVOT-NNN` | PIVOT-001 |
| Bug | `BUG-NNN` | BUG-001 |
| Milestone | `MS-NNN` | MS-654badde, MS-eea45fa8 |
| Epic | `EPIC-NNN` | EPIC-e045ab6d, EPIC-2362adfc |
| Task | `TASK-NNN` | TASK-58a9d218 |
| Research | `RES-NNN` | RES-4124820a |

IDs are stable — never reused after deletion. The next ID is determined by scanning existing files in the directory.

---

## Body Templates

Body templates define the minimum required sections for each artifact type's markdown body (everything below the frontmatter delimiter).

| Type | Required Sections | Optional Sections |
|------|-------------------|-------------------|
| **Pillar** | What This Pillar Means, Examples of Work That Serves This Pillar, Anti-Patterns, Conflict Resolution | — |
| **Milestone** | Context, Epics | Completion Criteria |
| **Epic** | Context, Implementation Design, Tasks | Out of Scope |
| **Task** | What, How, Verification | — |
| **Idea** | Motivation | Sketch |
| **Decision** | Decision, Rationale, Consequences | — |
| **Lesson** | Pattern, Fix | Git Evidence |
| **Rule** | Related Rules | FORBIDDEN |
| **Research** | — | — (intentionally freeform) |

---

## Field Ordering Convention

YAML frontmatter fields follow a consistent content hierarchy across all artifact types:

1. **Identity** — `id`, `title`
2. **Classification** — `layer`, `status`, `priority`, `category`, `scope`
3. **Description** — `description`, `gate`
4. **Lifecycle** — `created`, `updated`, `deadline`
5. **Relationships** — `relationships` array (the only connection mechanism)
6. **Scoring** — `scoring` block
7. **Operational** — `assignee`, `acceptance`, `recurrence`, `sources`

---

## Traceability Web

The `relationships` array creates a navigable web across all artifact types:

```
Vision ←── upholds ── Pillar ←── grounded ── Idea ── benefits ──→ Persona
                                                ↓
                                          crystallises
                                                ↓
Research ←── spawns ── Idea          Decision ── drives ──→ Epic ── delivers ──→ Milestone
    │                                    ↑                    ↑
    └── informs ──→ Decision             │                    │
                                    governs              guided-by ── Research
                                         ↓                    │
                                       Rule              cautioned-by ── Lesson
                                         │
                                    codifies
                                         ↓
                                       Lesson ── teaches ──→ Decision
```

### Documentation Traceability Chain

Every stage of work self-documents its decisions:

```
Idea captured
  → Idea spawns research (spawns / spawned-by)
  → Research informs decision (informs / informed-by)
  → Idea crystallises into decision (crystallises / crystallised-by)
  → Decision drives epic (drives / driven-by)
  → Research guides epic (guides / guided-by)
  → Tasks deliver epic (delivers / delivered-by)
  → Lessons teach future decisions (teaches / taught-by)
  → Rules codify lessons (codifies / codified-by)
  → Rules enforce decisions (enforces / enforced-by)
```

At any point, you can trace backwards: "Why does this rule exist?" follows `codifies` to lesson, `enforces` to decision, `driven-by` to epic, `crystallised-by` to idea. This is the automated documentation process — the system records what was decided at each stage and why.

---

## Key References

| Artifact | What It Defines |
|----------|----------------|
| `libs/types/src/platform/core.json` | Canonical types, relationships, semantics, and constraints |
| `project.json` | Navigation, statuses, delivery hierarchy, project relationships |
| AD-c8535011 | Three-layer artifact model (app-fixed, app-required, project-scoped) |
| AD-21d35c1d | Canonical relationship vocabulary (relationships are the only connections) |
| AD-10586f26 | Canonical status model (12 universal statuses) |
| AD-e156310d | Four enforcement layers (app, scanners, integrity, git hooks) |
| RULE-7b770593 | Artifact lifecycle enforcement |
| RULE-a764b2ae | Artifact schema compliance validation |

---
