# OrqaStudio Architecture Working Document

*Created 2026-03-15 — for holistic discussion across sessions*

---

## The Foundation

OrqaStudio is a structured systems thinking partner. It provides a framework where humans and AI agents collaborate to solve problems, and then applies the solutions back into the same framework so they can be implemented — by an implementation agent, or a human.

Everything is a node on a graph. Nodes connect through typed relationships. The graph IS the thinking made visible. The framework enforces systems thinking — not through guidelines that agents forget under pressure, but through mechanical constraints that make unstructured work impossible.

The full loop: structured thinking → problem solving with AI → solutions applied to the framework → implementation → learning fed back into the framework.

Critically, this loop compounds. Lessons become rules. Patterns become skills. Decisions become governance. The more someone works with their thinking partner, the better the partner gets at transposing their thoughts into process. The framework isn't static — it evolves with the project, capturing not just what was decided but HOW the team thinks and works. Over time, the system learns the user's reasoning patterns and encodes them as enforceable structure.

## Types vs Instances

A **type** is a definition — what kind of artifact this is, what statuses it allows, what relationships it can have. An **instance** is a specific artifact — PILLAR-001 "Clarity Through Structure", EPIC-045 "Dashboard Redesign". The type is the schema; the instance is the data.

This distinction matters because types and instances have different rules about what's fixed and what's configurable.

## Three Layers of Artifacts

Artifacts, their content, and their relationships exist in three layers based on how much control the project has:

### Layer 1: App-Fixed (Immutable)

Type definitions, content, and relationships are all hardcoded into the app. These are the definitions that make OrqaStudio work as a product. They cannot be edited through the app. Pre-commit hooks block file-based edits.

**What lives here:**
- The canonical type definitions themselves (what "pillar" means, what "idea" means)
- The canonical relationship definitions (what `informs` means, what `delivers` means)
- The canonical status vocabulary and their semantics
- Platform documentation (how to use OrqaStudio)
- Platform skills (how agents interact with OrqaStudio)

These are defined internally using the same structures as project-level definitions — they're just hardcoded rather than configurable.

### Layer 2: App-Required, Project-Authored (Required Existence, Configurable Content)

The app requires certain artifact instances to exist for the project to function, but the **content** is authored by the project. These define the project's purpose and identity within the framework.

| Type | Why Required | Content Is... |
|---|---|---|
| Pillars | Project must have guiding principles | What THIS project believes |
| Vision | Project must have direction | Where THIS project is going |
| Personas | Project must know who it serves | Who THIS project serves |
| Grounding | Agents must have behavioral anchors | What grounds THIS project's agents |

The app enforces that these exist. A new project can't function without at least defining its pillars and vision. But what those pillars SAY is entirely up to the project.

**Required relationships at this layer:**
- Agents must be `grounded-by` at least one pillar or grounding artifact
- Rules must be `governed-by` at least one decision

### Layer 3: Project-Scoped (Fully Configurable)

Types, instances, content, and relationships are all defined by the project. The app enforces the rules the project configures, but doesn't prescribe what those rules are.

**Core types with project-scoped instances** — the types are canonical but instances are created freely:

| Type | Purpose |
|---|---|
| Ideas | Entry point for any change — to the project, process, or principles |
| Decisions | Resolutions that direct work and authorise enforcement |
| Rules | Standards we enforce |
| Lessons | What we've learned |
| Skills | Knowledge we teach |
| Agents | Roles that do work |

**Project-defined types** — the types themselves are defined by the project:

| Category | Examples | Defined In |
|---|---|---|
| Discovery types | Research, wireframes, client discovery, experiments | `project.json` |
| Delivery types | Milestones, epics, tasks, phases, workstreams | `project.json` |

**Project relationships** — domain-specific connections between project types. Software: `depends-on` between tasks. Research: `replicates` between experiments. Defined in `project.json`, enforced by the app with the same rigour as canonical relationships.

**Project statuses** — if delivery/discovery types need statuses beyond the canonical vocabulary, they define them at the project level.

Labels, icons, hierarchy depth, and status aliases are all project-configurable. A research project might use a flask icon for experiments and a microscope for observations.

### How the Layers Compose

The app combines all three layers into a single enforcement model. Canonical type definitions and project type definitions use the same internal structure. Canonical relationships and project relationships are enforced by the same engine. The layers aren't separate systems — they're the same system with different levels of configurability.

```
┌─────────────────────────────────────────────────┐
│ Layer 1: App-Fixed                              │
│ Type definitions, canonical relationships,      │
│ status vocabulary, platform docs/skills         │
│ Immutable. Pre-commit hooks protect.            │
├─────────────────────────────────────────────────┤
│ Layer 2: App-Required, Project-Authored         │
│ Pillars, Vision, Personas, Grounding instances  │
│ Must exist. Content is the project's.           │
│ Required relationships enforced.                │
├─────────────────────────────────────────────────┤
│ Layer 3: Project-Scoped                         │
│ Ideas, decisions, rules, lessons, skills,       │
│ agents + project-defined discovery/delivery     │
│ types, relationships, statuses                  │
│ Fully configurable. App enforces config.        │
└─────────────────────────────────────────────────┘
```

**Ideas** are the universal entry point at Layer 3. Where an idea goes depends on what it is — it might evolve into delivery work, or drive a decision that governs a rule. The idea is always preserved as the record of original thinking.

**Decisions** are the bridge. They appear in multiple views because they serve multiple roles.

Labels and icons on core types are configurable per project for contextual clarity. A consulting project might call "Rules" → "Standards" and "Lessons" → "Insights." The canonical type key stays the same.

## Relationships

Relationships are the ONLY way artifacts connect. No standalone frontmatter fields for connections. No `epic: EPIC-045`. Only relationships.

There are two levels: canonical relationships that the app ships with and enforces universally, and project relationships that are defined per-project for domain-specific connections.

### Canonical Relationships (App-Level)

These exist in every project. They represent universal ways that thinking connects. Projects can alias the display labels but the underlying semantics are fixed.

| Relationship | Inverse | Meaning | Example |
|---|---|---|---|
| `informs` | `informed-by` | Supplementary context | Research informs an idea |
| `evolves-into` | `evolves-from` | Thinking becomes action | Idea evolves into an epic |
| `drives` | `driven-by` | Decision directs work | Decision drives an epic's design |
| `governs` | `governed-by` | Decision authorises enforcement | Decision governs a rule |
| `delivers` | `delivered-by` | Work contributes to a goal | Task delivers to an epic |
| `enforces` | `enforced-by` | Mechanical enforcement | Rule enforces a standard |
| `grounded` | `grounded-by` | Foundational anchor | Agent grounded by a pillar |
| `observes` | `observed-by` | Learning captured | Lesson observes a pattern |
| `merged-into` | `merged-from` | Ideas converged | Idea merged into a new idea |
| `synchronised-with` | `synchronised-with` | Must stay aligned | Skill synchronised with its documentation |

The old relationship types that existed at the app level are replaced by this canonical vocabulary. For each, the migration must determine whether the existing usage maps to a canonical relationship or becomes a project-level relationship:

| Old Type | Migration Path | Reasoning |
|---|---|---|
| `scoped-to` / `scoped-by` | → canonical `enforces` | Rule scope = what it enforces |
| `contains` / `belongs-to` | → canonical `delivers` | Containment is contribution |
| `documents` / `documented-by` | → canonical `informs` | Documentation informs its subject |
| `practices` / `practiced-by` | → canonical `grounded` | Agent practicing a skill = grounded by it |
| `verifies` / `verified-by` | → canonical `enforces` | Verification is enforcement |
| `depends-on` | → project relationship | Domain-specific delivery dependency |

Some connections that used old canonical types might belong as project relationships instead. The migration epic must audit each usage and route it correctly.

### Project Relationships (Project-Level)

These are defined per-project in `project.json` for domain-specific connections between project types. They only exist when the project defines them.

| Domain | Relationship | Inverse | Meaning |
|---|---|---|---|
| Software | `depends-on` | `depended-on-by` | Must complete before this can start |
| Research | `replicates` | `replicated-by` | Experiment reproduces another |
| Consulting | `escalates-to` | `escalated-from` | Issue raised to higher authority |

Project relationships follow the same rules as canonical ones — bidirectional, app-enforced, used in graph queries. The difference is they're defined by the project, not shipped with the app.

**Blocking is inferred, not stored.** If artifact A `depends-on` artifact B and B isn't completed, A is blocked. The graph query tells you that. There is no `blocked-by` relationship — blocking is a derived state from dependency + status.

### Standalone Fields to Migrate

All existing standalone connection fields become relationships:

| Field | On Type | Becomes |
|---|---|---|
| `epic` | Tasks | `delivers → EPIC-NNN` |
| `milestone` | Epics | `delivers → MS-NNN` |
| `depends-on` | Tasks | `depends-on → TASK-NNN` (project relationship) |
| `promoted-to` | Ideas/Lessons | `evolves-into → RULE-NNN` |
| `surpassed-by` | Research | `evolves-into` (the newer work) |
| `supersedes` / `superseded-by` | Decisions | `evolves-into` / `evolves-from` |
| `pillars` | Ideas/Epics | `grounded-by → PILLAR-NNN` (already migrated) |
| `research-refs` | Epics | `informed-by → RES-NNN` |
| `docs-required` / `docs-produced` | Epics | `informs` / `informed-by` to doc artifacts |

## How Ideas Evolve

An idea doesn't "promote" — it **evolves**. And it can evolve in multiple directions:

1. Idea → `evolves-into` → delivery artifact (epic, task, experiment)
2. Idea → `evolves-into` → decision → `governs` → rule
3. Idea → `evolves-into` → decision → `drives` → delivery artifact
4. Idea → status moves to `completed` (the thinking is done, the output is connected)

Research and other discovery types `inform` ideas and decisions. They're supplementary input, not the driver.

## Sections Are Graph Filters

The nav sections don't categorise artifacts. They filter the graph by relationship patterns. An artifact appears in every section its relationships qualify it for.

| Section | Shows | Label for Decisions |
|---|---|---|
| **Principles** | Pillars, Vision, Personas, Grounding | — |
| **Discovery** | Ideas, Decisions, project discovery types | "Decisions" (all) |
| **Learning** | Rules, Lessons, Skills, Agents, + decisions with `governs` edges | "Governing Decisions" |
| **Delivery** | Project delivery types + decisions with `drives` edges | "Driving Decisions" |

Decisions appear in Discovery (where they're made), Learning (when they govern rules), and Delivery (when they drive work). The contextual label explains WHY the decision appears in each section.

## State Machine

Status is a property of each node. Transitions are graph queries. Like types and relationships, statuses have two levels.

### Canonical Statuses (App-Level)

These represent universal stages of thinking. They exist in every project. The underlying concept is fixed; the display label is aliasable per project.

| Status | Meaning |
|---|---|
| `captured` | Recorded, not yet explored |
| `exploring` | Under investigation |
| `ready` | Explored, awaiting prioritisation |
| `prioritised` | Chosen for action |
| `active` | Work in progress |
| `hold` | Paused deliberately |
| `blocked` | Can't proceed (inferred from dependencies, or set manually for external blockers) |
| `review` | Done, awaiting validation |
| `completed` | Finished |
| `surpassed` | Superseded by newer thinking (merged ideas, outdated research) |
| `archived` | Didn't go anywhere — preserved but inactive (explored and abandoned) |
| `recurring` | Completed but expected to repeat |

**`surpassed` vs `archived`**: Different meanings. `surpassed` = thinking evolved, something better exists. `archived` = thinking was explored but didn't lead anywhere. The `merged-into` relationship tells you how something was surpassed; `archived` has no successor.

**Status subsets per type**: Each type declares which statuses it allows. A task doesn't need `exploring`. A milestone doesn't need `recurring`. The app enforces the restriction — you can't set a status that isn't in the type's allowed subset.

**Display aliases**: Projects configure how statuses appear in the UI. The canonical key is always stored; the alias is a rendering concern. A research project might display `exploring` as "Investigating" and `active` as "In Lab."

### Project Statuses (Project-Level)

If a project's delivery or discovery types need statuses that aren't universal thinking stages, they define them in `project.json`. These are scoped to the types that use them and don't pollute the canonical vocabulary.

### Transitions and Auto-Rules

**Auto-rules are graph queries:**
```json
{
  "condition": "all-related-in-status",
  "relationship": "delivers",
  "status": "completed",
  "target": "review"
}
```

"When all nodes connected via `delivers` relationships are in `completed` status → propose transitioning this node to `review`."

Auto-rules can reference both canonical and project relationships. A project might define: "when all `depends-on` targets are `completed` → auto-unblock this artifact."

The state machine isn't a separate system — it's rules about which transitions are valid given the current graph state.

## Views Are Graph Queries

| View | What it renders |
|---|---|
| **Roadmap** | Graph filtered to delivery types, grouped by hierarchy, status as columns |
| **Dashboard** | Aggregate queries: health, status distribution, attention needed, trends |
| **Scratchpad** | Ideas and discovery types on a spatial canvas — each item IS a graph node |
| **Artifact viewer** | Single node with its relationships |
| **Full graph** | Everything |

## Enforcement

The process is enforced by app code, not AI instructions. Invalid graph states are mechanically impossible. This applies to BOTH canonical and project-configured elements — the app enforces project relationships, statuses, and type constraints with the same rigour as canonical ones. Configuration doesn't mean optional enforcement.

### What the App Enforces

| Constraint | Scope | Enforcement |
|---|---|---|
| Relationships must be bidirectional | Canonical + project | App creates inverse automatically |
| Relationships must use valid types | Canonical + project | App rejects undefined relationship types |
| Statuses must be from allowed vocabulary | Canonical + project | App rejects invalid values for each type |
| Status transitions must be valid | Canonical + project | App won't allow invalid transitions |
| Delivery artifact must connect to parent | Project-configured | App rejects creation without required relationship |
| Child can't be further along than parent | Project-configured | App flags immediately, blocks until resolved |
| Type-specific status subsets | Canonical + project | App rejects statuses not in the type's allowed list |

This is not behavioral guidance for AI. This is the app refusing to accept invalid state — like a database rejecting bad SQL. If enforcement depends on AI remembering rules, enforcement fails under pressure (proven this session).

Project relationships defined in `project.json` get the same mechanical enforcement as canonical ones. When a project defines `depends-on` for tasks, the app enforces bidirectionality, validates the relationship type, and uses it in graph queries — exactly like `delivers` or `informs`.

### Four Enforcement Layers

| Layer | When | Human needed? |
|---|---|---|
| **App enforcement** | At action time | No — invalid states impossible |
| **Automated scanners** | On file change (watcher) | No — objective transitions auto-applied |
| **Integrity checks** | On scan/refresh | Yes — flags judgment-required issues |
| **Git hooks** | At commit time | Yes — last line of defence |

**App enforcement** makes invalid states impossible. You can't create them. This covers both canonical constraints (relationship bidirectionality, status vocabulary) and project-configured constraints (delivery hierarchy, type-specific rules).

**Automated scanners** run on file watcher events. When an artifact changes on disk, the scanner:
1. Reads the changed file's relationships
2. Evaluates connected nodes for objective state changes
3. Auto-applies transitions that are unambiguous (no human judgment needed)

Examples:
- All tasks on an epic are `completed` → scanner auto-moves epic to `review`
- A dependency completes → scanner auto-unblocks the waiting task
- A lesson hits recurrence threshold → scanner auto-moves to `review`

These are the configurable auto_rules from the state machine, triggered by file watcher events rather than manual scans. Auto-rules can reference both canonical and project relationships.

**Integrity checks** detect states that are technically valid but semantically wrong — situations where code can identify the problem but only a human can decide the fix. Examples:
- Body text references an artifact with no corresponding relationship
- Lesson has recurred 3 times (promote to rule?)
- All tasks completed on an epic (move to review?)
- Child artifact further along than parent (advance parent or move child?)

**Git hooks** are the last line of defence for when changes happen outside the app — text editors, CLI agents, manual file editing. They run the same integrity checks at commit time. Additionally, git hooks enforce Layer 1 immutability — blocking commits that modify app-fixed artifacts (canonical type definitions, platform docs/skills).

### Idea Merging

Ideas should be mergeable. When multiple ideas converge into one concept:
1. A new idea is created
2. The source ideas connect to the new idea via `merged-into` / `merged-from` relationships
3. Source ideas move to `surpassed` status (thinking evolved, originals preserved)
4. The new idea inherits context through the relationships — the full provenance chain is visible

This preserves change history. You can always trace back to the original ideas that led to the merged concept.

AI rules/skills become documentation: they teach agents HOW to work with the system, not how to ENFORCE it. The app enforces. The agent operates within the enforced boundaries.

## Documentation and Skills Are Separate But Synchronised

Documentation and skills are different artifacts written for different audiences:

| | Documentation | Skill |
|---|---|---|
| **Audience** | Humans | Agents |
| **Tone** | Narrative, explanatory | Concise, actionable |
| **Structure** | Chapters, sections, examples | Rules, patterns, do/don't |
| **Format** | Long-form readable | Context-window efficient |

They cover the same knowledge but are authored differently. You can't make one markdown file serve both audiences well.

**Synchronisation through relationships:** A skill and its corresponding documentation page are connected via a relationship (e.g., `synchronised-with`). The integrity checker flags when one is modified without the other — preventing the drift we've been fighting.

The graph ensures they stay aligned. When documentation changes, the connected skill is flagged for review. When a skill is updated, its documentation is flagged too.

**But skills are broader than documentation.** Not every skill has a corresponding doc page. Skills fall into three categories:

| Category | Example | Synchronised with docs? |
|---|---|---|
| **Project skills** | "How our artifact statuses work" | Yes — same knowledge, two renderings |
| **Domain skills** | Rust patterns, Svelte 5, testing methodology | No — portable expertise, not project-specific |
| **Platform skills** | How OrqaStudio works | No — shipped with the app, uneditable |

**Documentation has two categories too:**

| Category | Example | Editable? |
|---|---|---|
| **Platform documentation** | How to use OrqaStudio, how statuses work, what the roadmap shows | No — shipped with app |
| **Project documentation** | Architecture docs, how-to guides, specifications | Yes — project-specific |

The symmetry:
- Platform docs ↔ Platform skills (same knowledge, human vs agent, app-shipped)
- Project docs ↔ Project skills (same knowledge, human vs agent, synchronised)
- Domain skills have no doc counterpart (portable expertise, not project or platform)

## Open Questions — Resolved

1. **Existing relationship types** → All old canonical types replaced by the new vocabulary. Each existing usage audited during migration: maps to canonical relationship or becomes project relationship. See Relationships section.

2. **`surpassed` vs `archived`** → Different statuses with different meanings. `surpassed` = thinking evolved, something better exists (merged ideas, outdated research). `archived` = explored but didn't lead anywhere, no successor. See State Machine section.

3. **Status restriction per type** → Yes. Canonical vocabulary with type-specific allowed subsets. Display labels aliasable per project; underlying concepts stay canonical. App enforces the restriction.

4. **`depends-on` replacement** → `depends-on` is a project-level relationship, not canonical. Blocking is inferred state (dependency + incomplete status), not a relationship. No `blocked-by` type needed.

5. **Research as core vs project** → Project-specific. Migrated now as part of the migration epic. Principles aren't partial — you can't migrate half a principle.

6. **Agent skills during migration** → Domain skills (coding standards, composability, Svelte 5, Tauri v2, Rust patterns) loaded by implementation agents. They're portable expertise needed by this project, loaded explicitly while governance is disconnected.

## Key Design Principles

1. **The graph is the only data structure.** No standalone fields, no side channels.
2. **Sections are views, not categories.** The graph doesn't have sections. Views filter by relationship patterns.
3. **Two layers for everything.** Types, relationships, and statuses each have a canonical app-level (universal, aliasable) and a project-level (domain-specific, fully configurable). The line is clear: canonical = universal thinking concepts; project = domain-specific work patterns.
4. **The app enforces both layers.** Project-configured relationships, statuses, and constraints get the same mechanical enforcement as canonical ones. Configuration doesn't mean optional enforcement.
5. **Display is configurable, concepts are canonical.** Projects alias labels, icons, and status names for their domain. The app stores and enforces the canonical key. The UI renders the alias.
6. **Blocking is derived, not stored.** Dependencies are structural relationships. Whether something is blocked is inferred from dependency status, not stored as a separate relationship.
7. **AI knows the system through app-shipped docs.** Uneditable conventions, loaded into agent context.
8. **Project rules are project-specific.** Editable by the project, enforce project standards.

---

## Key Architectural Decisions

- **AD-049**: Status represented by icons, colors reserved for artifact types
- **AD-050**: Status transitions are config-driven
- **AD-051**: Three-layer artifact model — app-fixed, app-required/project-authored, project-scoped
- **AD-052**: Canonical relationship vocabulary — relationships only, two layers (canonical + project)
- **AD-053**: Canonical status model — universal vocabulary, type subsets, display aliases, surpassed vs archived
- **AD-054**: Four enforcement layers — app, scanners, integrity, git hooks (enforces both canonical + project)
- **IDEA-105**: Delivery pipeline as a future plugin
- **IDEA-106**: Sections are graph filters — Principles, Discovery, Learning, Delivery
- **IDEA-107**: App-shipped platform knowledge — immutable Layer 1 documentation and skills

## Session Artifacts Created

### Epics Completed
- EPIC-064: Enforcement bootstrapping (15 tasks)
- EPIC-073: UAT round 3 (19+ tasks)
- EPIC-074: Dashboard redesign (5 tasks)
- EPIC-075: Documentation reorganisation (6 tasks)
- EPIC-077: Automated status transitions (5 tasks)
- EPIC-078: Configuration-driven delivery pipeline (5 tasks)

### Epics Created (not started)
- EPIC-076: Graph analysis with Cytoscape.js (6 tasks)

### Ideas Captured
- IDEA-095 through IDEA-107 (13 ideas)

---

## Reconnection Instructions

To restore the full governance system after this holistic discussion:

1. **Restore CLAUDE.md, rules, agents, skills:**

   ```bash
   cp .claude/_backup/CLAUDE.md.bak .claude/CLAUDE.md
   mv .claude/_backup/rules .claude/rules
   mv .claude/_backup/agents .claude/agents
   mv .claude/_backup/skills .claude/skills
   rmdir .claude/_backup
   ```

2. **Or just start a new session:**
   The orqastudio-claude-plugin's `session-start.sh` hook recreates these
   from `.orqa/` source of truth on every session start. The `_backup`
   directory can then be deleted.

3. **Clean up:**

   ```bash
   rm WORKING-DOCUMENT.md  # Delete this file when discussion is complete
   ```
