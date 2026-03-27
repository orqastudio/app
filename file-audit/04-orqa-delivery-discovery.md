# Inventory: .orqa/ Delivery, Discovery, and Resolved Workflows

**Date:** 2026-03-26
**Scope:** `.orqa/delivery/`, `.orqa/discovery/`, `.orqa/workflows/*.resolved.yaml`

---

## 1. .orqa/delivery/milestones/ (3 files)

| File | Title | Status | Created | Gate |
|------|-------|--------|---------|------|
| MS-063c15b9.md | Foundation & Scaffold | completed | 2026-03-02 | All pre-build research, architecture decisions, product definition, UX design, technical design, and initial scaffold complete |
| MS-21d5096a.md | MVP | exploring | 2026-03-07 | Can a new user install, open a project, and get value within 10 minutes? |
| MS-b1ac0a20.md | Dogfooding | active | 2026-03-07 | Can we use this app instead of the terminal for governance management? |

### Milestone Details

**MS-063c15b9 (Foundation & Scaffold):** Retroactively created. Covers 7 phases (0a-2b) completed before the artifact framework existed. All completion criteria checked off. No forward epic references -- all work predates the framework.

**MS-b1ac0a20 (Dogfooding):** Lists ~30 epics across P1 (Critical Path), P1 (Retroactive - Completed), P2 (Enablers), P3 (Polish). 7 completion criteria, none checked. Active milestone.

**MS-21d5096a (MVP):** Lists 13 epics across P1-P3. 6 completion criteria, none checked. Prerequisites: Dogfooding milestone complete.

### Structural observations

- All 3 milestones have frontmatter with: id, type, title, description, status, created, updated, gate, relationships
- Status values used: completed, active, exploring
- Foundation milestone has a `## Epics (Retroactive)` inline table since epics predate the framework
- Dogfooding and MVP milestones reference epics by ID link
- Relationships array is empty `[]` on all three

---

## 2. .orqa/delivery/epics/ (128 files)

### Status Distribution

| Status | Count |
|--------|-------|
| completed | 61 (58 quoted + 3 unquoted) |
| captured | 39 (21 quoted + 18 unquoted) |
| active | 19 (12 quoted + 7 unquoted) |
| review | 4 |
| surpassed | 2 |
| exploring | 2 |
| ready | 1 |

### Sample Epics Read (5)

| File | Title | Status | Priority | Horizon | Relationships |
|------|-------|--------|----------|---------|---------------|
| EPIC-2451d1a9 | Architecture alignment | active | P0 | active | depends-on x3, implements x1, fulfils x1 |
| EPIC-0497a1be | Business logic deduplication -- daemon delegation model | review | P1 | active | fulfils x1, grounded x2 |
| EPIC-0777d74e | Process Visibility Dashboard | captured | P1 | next | fulfils x1 |
| EPIC-d45b4dfd | Artifact Graph SDK and Structural Integrity | completed | P1 | null | fulfils x1 |
| EPIC-e24086ed | Code quality audit and enforcement alignment | captured | P2 | next | fulfils x1, grounded x2 |

### Epic Frontmatter Fields

Common fields: id, type, title, description, status, priority, created, updated, horizon, scoring (impact/urgency/complexity/dependencies), relationships[]

- **scoring** block present on most epics with 4 numeric sub-fields
- **horizon** values observed: active, next, null, later
- **relationships** use typed targets with rationale: fulfils, grounded, depends-on, implements
- Larger epics include: `## Context`, `## Implementation Design`, `## Tasks` (table), `## Out of Scope`, `## Dependency Chain`
- Simpler captured epics have minimal body with placeholder text

### YAML Quoting Inconsistency

Status values appear both quoted (`"completed"`) and unquoted (`completed`). Same for other string fields. Not a parse error but inconsistent authoring.

---

## 3. .orqa/delivery/tasks/ (731 files)

### Status Distribution

| Status | Count |
|--------|-------|
| completed | 581 (443 quoted + 138 unquoted) |
| active | 40 (37 unquoted + 3 quoted) |
| captured | 46 (29 unquoted + 17 quoted) |
| ready | 44 (24 quoted + 20 unquoted) |
| blocked | 10 (8 unquoted + 2 quoted) |
| surpassed | 9 (6 quoted + 3 unquoted) |
| archived | 1 |

### Sample Tasks Read (5)

| File | Title | Status | Key Fields |
|------|-------|--------|------------|
| TASK-a30d8521 | Knowledge gap detection in governance audit | blocked | priority, scoring, acceptance[], relationships (delivers + depends-on) |
| TASK-7a3000fd | First governed session test | captured | relationships (delivers + depends-on), numbered acceptance criteria in body |
| TASK-84b816cd | Dashboard components use shared library | completed | priority, scoring, acceptance[], assignee: null |
| TASK-ccb0269c | Plugin packaging -- dual-manifest | active | relationships (delivers + depends-on), acceptance criteria in body |
| TASK-bbd43489 | Migrate scope fields to relationships array | completed | priority, scoring, acceptance[], assignee: null |

### Task Frontmatter Fields

Two structural patterns observed:

1. **Scored tasks:** id, type, title, description, status, priority, scoring{}, created, updated, assignee, acceptance[], relationships[]
2. **Simple tasks:** id, type, title, status, created, updated, relationships[] (no scoring, no acceptance in frontmatter)

- Acceptance criteria appear either in frontmatter `acceptance:` array or in body `## Acceptance Criteria` section
- Body structure: `## What`, `## How`, `## Verification`, `## Lessons` (or `## Scope` for simpler tasks)
- Relationships typically include `delivers` (to epic) and optionally `depends-on` (to other tasks)

---

## 4. .orqa/delivery/ideas/ (12 files)

All 12 ideas have status: `captured`.

### All Files

| File | Title (from sample reads) |
|------|--------------------------|
| IDEA-028dace7 | File-opening protocol for .orqa artifacts |
| IDEA-09a60c2e | (not read) |
| IDEA-102f7014 | (not read) |
| IDEA-14669b52 | (not read) |
| IDEA-22474bf5 | (not read) |
| IDEA-638bd0f0 | App chat panel uses MCP+LSP (same as CLI connector) |
| IDEA-6ebb15b1 | (not read) |
| IDEA-825e6182 | (not read) |
| IDEA-87cd7dcb | Resolve npm audit vulnerabilities before MVP |
| IDEA-8f905f6f | (not read) |
| IDEA-a7882323 | (not read) |
| IDEA-e68b6a47 | (not read) |

### Delivery Ideas Characteristics

- type: `discovery-idea` (same as discovery ideas -- NOT `delivery-idea`)
- Frontmatter: id, type, title, description, status, priority, created, updated, horizon, relationships[]
- Relationships: grounded (to pillar), benefits (to persona)
- Bodies are brief: `## What` + optional `## Approach options` or `## Dev environment impact`
- These appear to be ideas that relate to implementation/delivery concerns but use the same artifact type as discovery ideas

---

## 5. .orqa/discovery/ideas/ (160 files)

### Status Distribution

| Status | Count |
|--------|-------|
| captured | 116 |
| surpassed | 28 |
| completed | 16 |

### Sample Discovery Ideas Read (5)

| File | Title | Status | Notable |
|------|-------|--------|---------|
| IDEA-0066e754 | Move all service ports above 10000 | captured | Has a current/proposed ports table |
| IDEA-01bb7bda | AI-assisted artifact backfill tooling | captured | Has research-needed[] field, horizon: active |
| IDEA-057fce3f | Plugin lifecycle events | captured | Brief, ~20 lines |
| IDEA-174fa5c8 | OrqaStudio Cloud -- Forgejo-based git hosting | captured | Very long (~170 lines), extensive design |
| IDEA-6669df10 | Tools as plugins -- runtime tool registration | surpassed | Short, marked surpassed |

### Discovery vs Delivery Ideas

Both use type: `discovery-idea`. The separation is purely directory-based:
- `.orqa/delivery/ideas/` -- 12 files, all captured, focused on implementation/delivery concerns
- `.orqa/discovery/ideas/` -- 160 files, mix of statuses, focused on product/feature concepts

Ideas vary dramatically in size: from ~15 lines (simple captures) to ~170 lines (full design documents with architecture diagrams, deployment options, and cross-references).

### Common Fields

- id, type (discovery-idea), title, status, description, created, updated
- Optional: priority, horizon, research-needed[], scoring{}
- relationships[]: grounded (to pillar), benefits (to persona), realises (to epic)

---

## 6. .orqa/discovery/research/ (80 files)

### Status Distribution

| Status | Count |
|--------|-------|
| completed | 60 |
| active | 12 |
| surpassed | 8 |

### All Research Files

```
RES-0bbae4c4.md    RES-0e971367.md    RES-12f2bf80.md    RES-138eff6e.md
RES-156f2188.md    RES-16fd5aea.md    RES-1a888755.md    RES-206222e3.md
RES-22e4c59c.md    RES-25f55a8d.md    RES-27120af2.md    RES-295282cc.md
RES-29cf5ac5.md    RES-2c959f47.md    RES-2d91e2c2.md    RES-2f1648f5.md
RES-32832ff2.md    RES-35e37496.md    RES-44b34cba.md    RES-45894924.md
RES-4ae0750b.md    RES-4dbf04d7.md    RES-525533a7.md    RES-535fb6f8.md
RES-5435cae9.md    RES-62c3f0ce.md    RES-63cda7a7.md    RES-6a16de02.md
RES-6d8c494c.md    RES-6d9cafb9.md    RES-72f6f6b6.md    RES-797972a7.md
RES-7b24ff49.md    RES-7e032412.md    RES-7ee1a770.md    RES-80a476c7.md
RES-8746b9f3.md    RES-8c29af5d.md    RES-8d203b37.md    RES-8f54c71d.md
RES-8fee4dad.md    RES-93c7bb99.md    RES-94a3a6ca.md    RES-96c4417a.md
RES-98ab39e0.md    RES-9986caa9.md    RES-999def94.md    RES-9a4dc908.md
RES-9af5a942.md    RES-9e03dcdc.md    RES-9f754719.md    RES-a105065c.md
RES-a2a77d0c.md    RES-a4f9dc97.md    RES-a5d10705.md    RES-ac474863.md
RES-aefc9e44.md    RES-b0268020.md    RES-b484fa9f.md    RES-b666c725.md
RES-b7062d7b.md    RES-bb4d4ae3.md    RES-bd4d7ea3.md    RES-c42a077f.md
RES-c46ece96.md    RES-cc3e38db.md    RES-d067c8c2.md    RES-d2baf8c4.md
RES-d6d344c9.md    RES-d6e8ab11.md    RES-da6ca6a6.md    RES-e7cf8d7b.md
RES-e9566e49.md    RES-ef35d2de.md    RES-f27cfd70.md    RES-f664f528.md
RES-f66a29ad.md    RES-f72b1a22.md    RES-f7bd7ab1.md    RES-fbe69e04.md
```

### Sample Research Read (5 headers)

| File | Title | Type | Status | Guides |
|------|-------|------|--------|--------|
| RES-d6e8ab11 | Agent Team Design v2: Plugin-Composed Workflows... | discovery-research | active | Key reference in CLAUDE.md |
| RES-0bbae4c4 | Rebrand: Forge to OrqaStudio | discovery-research | completed | EPIC-0bbae4c4, EPIC-4fb5e9e1 |
| RES-156f2188 | Artifact Graph SDK | discovery-research | completed | EPIC-d45b4dfd |
| RES-44b34cba | Native Search Engine Implementation | discovery-research | completed | EPIC-7f3119b1 |
| RES-f72b1a22 | UX Polish Sprint | discovery-research | completed | EPIC-f72b1a22 |

### Key Reference Documents

Per CLAUDE.md, the following are designated key references:
- **RES-d6e8ab11** -- Agent Team Design v2 (status: active, category: architecture)

### Research Frontmatter

- id, type (discovery-research), title, description, status, created, updated
- Optional: category
- relationships[]: guides (to epic)
- Bodies are intentionally freeform (per body template rules)

---

## 7. .orqa/discovery/wireframes/ (5 files)

| File | Title | Sort | Status | Freshness |
|------|-------|------|--------|-----------|
| DOC-6c91572c.md | Wireframe: Core Layout | 1 | captured | Stale: PaneForge references outdated (now shadcn-svelte Resizable), dashboard section reflects earlier vision |
| DOC-65a3c4e8.md | Wireframe: Conversation View | 2 | captured | Partially stale: DiffView.svelte does not exist, TypingIndicator renamed to StreamingIndicator, approval flow not implemented |
| DOC-93a0f6c1.md | Wireframe: Artifact Browser | 3 | captured | Partially stale: fs.watch API reference outdated, viewer has additional panels not shown |
| DOC-4ac7f17a.md | Wireframe: Dashboard Views | 4 | captured | Significantly outdated: Scanner/Metrics/Learning dashboards never built as described; dashboard uses narrative flow layout |
| DOC-796d7f01.md | Wireframe: Settings & Onboarding | 5 | captured | Partially outdated: Onboarding wizard has more steps than shown; discovery conversation flow not implemented |

### Wireframe Characteristics

- All type: `doc`, all status: `captured`
- All created: 2026-03-02, updated: 2026-03-15
- Use PlantUML Salt notation for wireframes
- Each has a `<!-- FRESHNESS NOTE -->` comment documenting known staleness
- Informed by: Information Architecture, Frontend Research (RES-80a476c7)
- `sort` field controls display ordering
- No relationships (empty array)
- Comprehensive: include element tables, keyboard navigation, state indicators, flow diagrams

---

## 8. .orqa/workflows/ -- Resolved Workflows (24 files)

All files are auto-generated by `orqa plugin install`. Each is a `*.resolved.yaml` file.

### Summary Table

| Filename | Artifact Type | Source Plugin | States | Transitions | Contributions |
|----------|---------------|-------------- |--------|-------------|---------------|
| agile-methodology.resolved.yaml | epic | @orqastudio/plugin-agile-workflow | 33 | 66 | 6 (all slots filled) |
| delivery.resolved.yaml | epic | @orqastudio/plugin-agile-workflow | 21 | 41 | 3 of 6 filled |
| epic.resolved.yaml | epic | @orqastudio/plugin-software-kanban | 12 | 20 | 0 |
| milestone.resolved.yaml | milestone | @orqastudio/plugin-software-kanban | 12 | 23 | 0 |
| task.resolved.yaml | task | @orqastudio/plugin-software-kanban | 15 | 17 | 0 |
| research.resolved.yaml | research | @orqastudio/plugin-software-kanban | 8 | 14 | 0 |
| idea.resolved.yaml | idea | @orqastudio/plugin-agile-workflow | 10 | 17 | 0 |
| discovery-idea.resolved.yaml | discovery-idea | @orqastudio/plugin-agile-discovery | 10 | 17 | 0 |
| discovery-decision.resolved.yaml | discovery-decision | @orqastudio/plugin-agile-discovery | 8 | 15 | 0 |
| discovery-research.resolved.yaml | discovery-research | @orqastudio/plugin-agile-discovery | 8 | 14 | 0 |
| persona.resolved.yaml | persona | @orqastudio/plugin-agile-discovery | 4 | 5 | 0 |
| pillar.resolved.yaml | pillar | @orqastudio/plugin-agile-discovery | 4 | 5 | 0 |
| pivot.resolved.yaml | pivot | @orqastudio/plugin-agile-discovery | 6 | 9 | 0 |
| vision.resolved.yaml | vision | @orqastudio/plugin-agile-discovery | 5 | 7 | 0 |
| planning-decision.resolved.yaml | planning-decision | @orqastudio/plugin-agile-planning | 6 | 12 | 0 |
| planning-idea.resolved.yaml | planning-idea | @orqastudio/plugin-agile-planning | 6 | 11 | 0 |
| planning-research.resolved.yaml | planning-research | @orqastudio/plugin-agile-planning | 5 | 7 | 0 |
| wireframe.resolved.yaml | wireframe | @orqastudio/plugin-agile-planning | 8 | 13 | 0 |
| agent.resolved.yaml | agent | @orqastudio/plugin-core-framework | 8 | 15 | 0 |
| decision.resolved.yaml | decision | @orqastudio/plugin-core-framework | 8 | 15 | 0 |
| doc.resolved.yaml | doc | @orqastudio/plugin-core-framework | 8 | 15 | 0 |
| knowledge.resolved.yaml | knowledge | @orqastudio/plugin-core-framework | 8 | 15 | 0 |
| lesson.resolved.yaml | lesson | @orqastudio/plugin-core-framework | 9 | 16 | 0 |
| rule.resolved.yaml | rule | @orqastudio/plugin-core-framework | 9 | 18 | 0 |

**Total: 24 resolved workflows, 247 states, 399 transitions**

### Composition Model

Two workflows use composition via contribution points:

1. **delivery.resolved.yaml** -- 3 of 6 contribution points filled by @orqastudio/plugin-software-kanban (planning-methodology, implementation-workflow, review-process). 3 unfilled: discovery-artifacts, documentation-standards, learning-pipeline.

2. **agile-methodology.resolved.yaml** -- All 6 contribution points filled by 5 different plugins:
   - @orqastudio/plugin-agile-discovery (discovery-artifacts)
   - @orqastudio/plugin-agile-planning (planning-methodology)
   - @orqastudio/plugin-agile-documentation (documentation-standards)
   - @orqastudio/plugin-software-kanban (implementation-workflow)
   - @orqastudio/plugin-agile-review (review-process)
   - @orqastudio/plugin-core-framework (learning-pipeline)

### Workflow Source Plugins

| Plugin | Workflows Provided |
|--------|-------------------|
| @orqastudio/plugin-agile-workflow | 2 (delivery, agile-methodology skeletons, idea) |
| @orqastudio/plugin-software-kanban | 4 (epic, milestone, task, research) |
| @orqastudio/plugin-agile-discovery | 6 (discovery-idea, discovery-decision, discovery-research, persona, pillar, pivot, vision) |
| @orqastudio/plugin-agile-planning | 4 (planning-decision, planning-idea, planning-research, wireframe) |
| @orqastudio/plugin-core-framework | 6 (agent, decision, doc, knowledge, lesson, rule) |

### Common Structural Elements

- All files begin with `# AUTO-GENERATED -- do not edit manually.`
- Header comments list: resolved-by, skeleton source, contributions, resolved-at timestamp
- Fields: name, version (all 0.1.0), artifact_type, plugin, description, initial_state
- States have: category (planning/active/review/completed/terminal), description, optional on_enter actions
- on_enter actions: append_log, set_field
- Transitions have: from, to, event, description, optional guards[], actions[], gate
- Guards use field_check with operator: not_empty
- Gates use structured_review pattern with gather/present/collect/learn phases
- Composed workflows have contribution_points[] with name, stage, description, required, filled_by
- Some workflows include migration{} maps for status migration

---

## Aggregate Statistics

| Category | Count |
|----------|-------|
| Milestones | 3 |
| Epics | 128 |
| Tasks | 731 |
| Delivery Ideas | 12 |
| Discovery Ideas | 160 |
| Research | 80 |
| Wireframes | 5 |
| Resolved Workflows | 24 |
| **Total artifacts** | **1,143** |
| **Total workflow states** | **247** |
| **Total workflow transitions** | **399** |
