# Phase 1 Inventory: Methodology and Workflow Plugins

Factual inventory of 7 methodology/workflow plugins. No recommendations or judgments -- facts only.

**Date:** 2026-03-26
**Scope:** `plugins/agile-workflow`, `plugins/core`, `plugins/agile-discovery`, `plugins/agile-planning`, `plugins/agile-documentation`, `plugins/agile-review`, `plugins/software-kanban`

---

## 1. agile-workflow

**Package:** `@orqastudio/plugin-agile-workflow` v0.1.4-dev (private)
**Display Name:** Agile Governance
**Category:** governance
**Role:** core:workflow
**Total files:** 38 (excluding node_modules, dist)

### Manifest Summary (orqa-plugin.json)

- **Schemas:** 0
- **Workflows:** 1 (`workflows/agile-methodology.workflow.yaml`)
- **Relationships:** 20 types across 6 categories (Foundation, Lineage, Governance, Knowledge Flow, Agency, Synchronisation)
- **Agents:** 2 registered
- **Knowledge items:** 8 declared
- **Knowledge declarations:** 15+ with tiers (always, stage-triggered, on-demand) and role constraints
- **Enforcement mechanisms:** 0
- **Content mappings:** None (no `content` section)
- **Behavioral rules:** Lessons vs auto-memory guidance

### Workflow Files (1)

| File | Artifact Type | States | Contribution Points | Description |
|------|--------------|--------|---------------------|-------------|
| `agile-methodology.workflow.yaml` | epic | 7 (discover, plan, document, implement, review, learn, done) | 6 (discovery-artifacts, planning-methodology, documentation-standards, implementation-workflow, review-process, learning-pipeline) | Core delivery skeleton. Transitions with guards and gates. delivery-review gate uses `structured_review` pattern with gather/present/collect/execute/learn phases. |

### Role Files (8)

| File | Model Tier | Can Edit | Can Run Shell | Artifact Scope |
|------|-----------|----------|---------------|----------------|
| `roles/orchestrator.yaml` | opus | true | true | coordination-only |
| `roles/implementer.yaml` | sonnet (complex: opus) | true | true | source-code-only |
| `roles/reviewer.yaml` | sonnet | false | true (checks only) | read-only |
| `roles/researcher.yaml` | sonnet (complex: opus) | false | false | research artifacts; can_search_web: true |
| `roles/planner.yaml` | opus | false | false | delivery artifacts; can_search_web: true |
| `roles/writer.yaml` | sonnet | true | false | documentation-only; can_search_web: true |
| `roles/designer.yaml` | sonnet | true | false | source-code-only |
| `roles/governance_steward.yaml` | sonnet | true | false | .orqa/-artifacts-only |

### Agent Files (2)

| File | Title | Model | Status |
|------|-------|-------|--------|
| `agents/AGENT-7a06d10e.md` | Governance Enforcer | sonnet | active |
| `agents/AGENT-ae63c406.md` | Governance Steward | sonnet | active |

### Knowledge Files (8)

| File | Title | Notes |
|------|-------|-------|
| `knowledge/KNOW-83039175.md` | Thinking Mode: Learning Loop | |
| `knowledge/KNOW-85e392ea.md` | Thinking Mode - Learning Loop | Duplicate topic with KNOW-83039175 (different title format) |
| `knowledge/KNOW-0444355f.md` | Plugin Artifact Usage | |
| `knowledge/KNOW-8d1c4be6.md` | Plugin Artifact Usage | Duplicate title with KNOW-0444355f |
| `knowledge/KNOW-8c359ea4.md` | Governance Maintenance | |
| `knowledge/KNOW-8d76c3c7.md` | Governance Maintenance | Duplicate title with KNOW-8c359ea4 |
| `knowledge/KNOW-ee860ed9.md` | Enforcement Patterns | |
| `knowledge/KNOW-498ca38a.md` | Agile Governance -- Relationship Vocabulary | 20 relationship types documented |

### Rule Files (17)

| File | Title |
|------|-------|
| `rules/RULE-00700241.md` | System Command Safety |
| `rules/RULE-05ae2ce7.md` | Architecture Decisions |
| `rules/RULE-0be7765e.md` | Error Ownership |
| `rules/RULE-145332dc.md` | Governance Priority Over Delivery |
| `rules/RULE-23699df2.md` | Artifact Schema Compliance |
| `rules/RULE-25baac14.md` | IDs Are Not Priority |
| `rules/RULE-3c2da849.md` | Core Graph Firmware Protection |
| `rules/RULE-4603207a.md` | Enforcement Before Code |
| `rules/RULE-4dbb3612.md` | Enforcement Gap Priority |
| `rules/RULE-8ee65d73.md` | No Deferred Deliverables |
| `rules/RULE-af1cd87d.md` | Behavioral Rule Enforcement Plan |
| `rules/RULE-af5771e3.md` | No Stubs or Placeholders |
| `rules/RULE-b10fe6d1.md` | Artifact Lifecycle |
| `rules/RULE-b2584e59.md` | Trace Every Artifact to Its Usage Contexts |
| `rules/RULE-c603e90e.md` | Lessons Learned |
| `rules/RULE-d543d759.md` | Honest Status Reporting |
| `rules/RULE-f609242f.md` | Git Workflow |

### Relationship Types (20)

| Category | Forward | Inverse | From | To |
|----------|---------|---------|------|----|
| Foundation | `upholds` | `upheld-by` | pillar | vision |
| Foundation | `grounded` | `grounded-by` | idea, epic | pillar |
| Foundation | `benefits` | `benefited-by` | idea | persona |
| Foundation | `serves` | `served-by` | agent | pillar, persona |
| Foundation | `revises` | `revised-by` | pivot | vision, persona, pillar |
| Lineage | `crystallises` | `crystallised-by` | idea | decision |
| Lineage | `spawns` | `spawned-by` | idea | research |
| Lineage | `merged-into` | `merged-from` | idea, research | idea, research |
| Governance | `drives` | `driven-by` | decision | epic |
| Governance | `governs` | `governed-by` | decision | rule |
| Governance | `enforces` | `enforced-by` | rule | decision |
| Governance | `codifies` | `codified-by` | rule | lesson |
| Governance | `promoted-to` | `promoted-from` | lesson | rule |
| Knowledge Flow | `informs` | `informed-by` | research | decision, research |
| Knowledge Flow | `teaches` | `taught-by` | lesson | decision |
| Knowledge Flow | `guides` | `guided-by` | research | epic |
| Knowledge Flow | `cautions` | `cautioned-by` | lesson | epic |
| Knowledge Flow | `documents` | `documented-by` | doc | epic, decision, rule, milestone |
| Agency | `employs` | `employed-by` | agent | knowledge |
| Synchronisation | `synchronised-with` | `synchronised-with` | knowledge, doc | knowledge, doc |

---

## 2. core

**Package:** `@orqastudio/plugin-core-framework` v0.1.4-dev (private, BSL-1.1)
**Display Name:** Core Framework
**Category:** framework
**Role:** core:framework
**Uninstallable:** true
**Total files:** 63 (excluding node_modules, dist)

### Manifest Summary (orqa-plugin.json)

- **Schemas:** 6 (decision AD-*, rule RULE-*, lesson IMPL-*, knowledge KNOW-*, agent AGENT-*, doc DOC-*)
- **Workflows:** 7 (6 artifact workflows + 1 contribution)
- **Relationships:** 0 (defined by other plugins)
- **Agents:** 9 registered
- **Knowledge items:** 30+ declared
- **Knowledge declarations:** Extensive, covering thinking modes, search, enforcement, planning, CLI, project setup
- **Enforcement mechanisms:** 4 (behavioral str=1, json-schema str=8, hook str=5, onnx str=4)
- **Content mappings:** agents, knowledge, rules, docs, workflows (source dirs -> `.orqa/process/` targets)

### Workflow Files (7)

| File | Artifact Type | States | Description |
|------|--------------|--------|-------------|
| `workflows/decision.workflow.yaml` | decision | 8 (captured, exploring, active, hold, review, completed, surpassed, archived) | ADR lifecycle |
| `workflows/rule.workflow.yaml` | rule | 9 (adds inactive for demoted rules) | Rule enforcement lifecycle |
| `workflows/lesson.workflow.yaml` | lesson | 9 (adds recurring, promoted) | Lesson-to-rule promotion path |
| `workflows/knowledge.workflow.yaml` | knowledge | 8 | Knowledge accuracy lifecycle |
| `workflows/agent.workflow.yaml` | agent | 8 | Agent configuration lifecycle |
| `workflows/doc.workflow.yaml` | doc | 8 | Document authoring lifecycle |
| `workflows/learning.contribution.workflow.yaml` | epic (contribution) | 5 sub-states (capture, retrospective, pattern-tracking, recurrence-detection, learning_complete) | Contributes to agile-methodology `learning-pipeline` point |

### Agent Files (9)

| File | Title | Model |
|------|-------|-------|
| `agents/AGENT-4c94fe14.md` | Orchestrator | (defined in role YAML) |
| `agents/AGENT-8e58cd87.md` | Reviewer | (defined in role YAML) |
| `agents/AGENT-e333508b.md` | Researcher | (defined in role YAML) |
| `agents/AGENT-85be6ace.md` | Planner | (defined in role YAML) |
| `agents/AGENT-d1be3776.md` | Installer | (defined in role YAML) |
| `agents/AGENT-e5dd38e4.md` | Implementer | (defined in role YAML) |
| `agents/AGENT-bbad3d30.md` | Writer | (defined in role YAML) |
| `agents/AGENT-0aad40f4.md` | Designer | (defined in role YAML) |
| `agents/AGENT-ae63c406.md` | Governance Steward | (defined in role YAML) |

### Knowledge Files (23)

| File | Title |
|------|-------|
| `knowledge/KNOW-b95ec6e3.md` | Thinking Mode: Debugging |
| `knowledge/KNOW-f7fb7aa7.md` | Thinking Mode: Implementation |
| `knowledge/KNOW-fd636a56.md` | Thinking Mode: Review |
| `knowledge/KNOW-21d28aa0.md` | Planning |
| `knowledge/KNOW-13348442.md` | Search |
| `knowledge/KNOW-1f4aba8f.md` | Three-Layer Enforcement Model |
| `knowledge/KNOW-586bfa9a.md` | Knowledge Auto-Injection |
| `knowledge/KNOW-51de8fb7.md` | Artifact Status Management |
| `knowledge/KNOW-22783288.md` | CLI Architecture |
| `knowledge/KNOW-e89753ad.md` | OrqaStudio CLI Commands |
| `knowledge/KNOW-2876afc7.md` | Project Setup |
| `knowledge/KNOW-3d946f9a.md` | Agent Decision Methodology |
| `knowledge/KNOW-dd5062c9.md` | Shared Validation Engine |
| `knowledge/KNOW-b320cae8.md` | Implementer Reasoning Methodology |
| `knowledge/KNOW-e484802a.md` | Reviewer Reasoning Methodology |
| `knowledge/KNOW-9ff8c63f.md` | Research Methodology |
| `knowledge/KNOW-a4e351bc.md` | Governance Migration Methodology |
| `knowledge/KNOW-a16b7bc7.md` | Demoted Rule Stability Tracking |
| `knowledge/KNOW-f5ee4e0d.md` | Plugin Setup |
| `knowledge/KNOW-7c871921.md` | Project Type Detection |
| `knowledge/KNOW-477f2c9c.md` | Agentic Workflow and Enforcement Pipeline |
| `knowledge/KNOW-57365826.md` | Query Artifact Schemas Before Writing Frontmatter |
| `knowledge/KNOW-6d80cf39.md` | Documentation Placement -- Where to Write Docs and Knowledge |

### Rule Files (17)

| File | Title |
|------|-------|
| `rules/RULE-2f64cc63.md` | Continuous Operation |
| `rules/RULE-87ba1b81.md` | Agent Delegation |
| `rules/RULE-d5d28fba.md` | Structure Before Work |
| `rules/RULE-b723ea53.md` | Tool Access Restrictions |
| `rules/RULE-ec9462d8.md` | Documentation-First Implementation |
| `rules/RULE-0d29fc91.md` | Code Search Usage |
| `rules/RULE-49f66888.md` | Self-Hosted Development |
| `rules/RULE-205d9c91.md` | Skill Portability |
| `rules/RULE-8abcbfd5.md` | Provider-Agnostic Tool Capabilities |
| `rules/RULE-ef822519.md` | Context Window Management |
| `rules/RULE-30a223ca.md` | Session Management |
| `rules/RULE-e1f1afc1.md` | Automated Knowledge Injection |
| `rules/RULE-f23392dc.md` | User-Invocable Knowledge Semantics |
| `rules/RULE-5965256d.md` | Required Reading |
| `rules/RULE-8aadfd6c.md` | Real-time Session State Management |
| `rules/RULE-dd5b69e6.md` | Skill Enforcement |
| `rules/RULE-484872ef.md` | Historical Artifact Preservation |

### Documentation Files (8)

| File | Title |
|------|-------|
| `docs/DOC-1f4aba8f.md` | Three-Layer Enforcement Model |
| `docs/DOC-22783288.md` | CLI Architecture |
| `docs/DOC-a16b7bc7.md` | Demoted Rule Stability Tracking |
| `docs/DOC-e89753ad.md` | OrqaStudio CLI Commands |
| `docs/DOC-dd5062c9.md` | Shared Validation Engine |
| `docs/DOC-586bfa9a.md` | Knowledge Auto-Injection |
| `docs/DOC-e16aea3b.md` | OrqaStudio Agentic Workflow and Enforcement Pipeline |
| `docs/DOC-7068f40a.md` | Documentation Placement Guide |

### Schemas (6)

| ID Prefix | Artifact Type | Default Path |
|-----------|--------------|--------------|
| AD-* | decision | `.orqa/process/decisions/` |
| RULE-* | rule | `.orqa/process/rules/` |
| IMPL-* | lesson | `.orqa/process/lessons/` |
| KNOW-* | knowledge | `.orqa/process/knowledge/` |
| AGENT-* | agent | `.orqa/process/agents/` |
| DOC-* | doc | `.orqa/process/docs/` |

---

## 3. agile-discovery

**Package:** `@orqastudio/plugin-agile-discovery` v0.1.4-dev (private)
**Display Name:** Discovery Stage
**Category:** discovery
**Total files:** 10

### Manifest Summary (orqa-plugin.json)

- **Schemas:** 7 (vision VISION-*, pillar PILLAR-*, persona PERSONA-*, pivot PIVOT-*, discovery-idea DISC-IDEA-*, discovery-research DISC-RES-*, discovery-decision DISC-AD-*)
- **Workflows:** 8 (7 artifact workflows + 1 contribution)
- **Relationships:** 6 types (evolves-to x2, drives, pivots, hands-off-to, informs)
- **Agents:** 0
- **Knowledge:** 0
- **Rules:** 0
- **Content mappings:** None

### Workflow Files (8)

| File | Artifact Type | States | Description |
|------|--------------|--------|-------------|
| `workflows/vision.workflow.yaml` | vision | 5 (draft, active, review, revised, archived) | Simple lifecycle |
| `workflows/persona.workflow.yaml` | persona | 4 (active, evolving, review, archived) | Starts active |
| `workflows/pillar.workflow.yaml` | pillar | 4 (active, evolving, review, archived) | Starts active |
| `workflows/pivot.workflow.yaml` | pivot | 6 (proposed, evaluating, approved, in-progress, completed, rejected) | Full decision lifecycle |
| `workflows/discovery-idea.workflow.yaml` | discovery-idea | 10 (captured, evaluating, accepted, exploring, validated, deferred, hold, rejected, merged, archived) | Full lifecycle with merge/hold paths |
| `workflows/discovery-research.workflow.yaml` | discovery-research | 8 (proposed, investigating, reviewing, completed, hold, cancelled, merged, archived) | Research investigation lifecycle |
| `workflows/discovery-decision.workflow.yaml` | discovery-decision | 8 (captured, exploring, active, hold, review, completed, surpassed, archived) | Mirrors core decision workflow |
| `workflows/discovery.contribution.workflow.yaml` | epic (contribution) | 4 sub-states (ideation, investigation, strategic_decision, discovery_complete) | Contributes to agile-methodology `discovery-artifacts` point |

### Schemas (7)

| ID Prefix | Artifact Type | Default Path |
|-----------|--------------|--------------|
| VISION-* | vision | `.orqa/principles/` |
| PILLAR-* | pillar | `.orqa/principles/` |
| PERSONA-* | persona | `.orqa/principles/` |
| PIVOT-* | pivot | `.orqa/principles/` |
| DISC-IDEA-* | discovery-idea | `.orqa/discovery/ideas/` |
| DISC-RES-* | discovery-research | `.orqa/discovery/research/` |
| DISC-AD-* | discovery-decision | `.orqa/discovery/decisions/` |

### Relationship Types (6)

| Forward | Inverse | From | To |
|---------|---------|------|----|
| `evolves-to` | `evolved-from` | discovery-idea | discovery-idea |
| `evolves-to` | `evolved-from` | discovery-research | discovery-research |
| `drives` | `driven-by` | discovery-decision | discovery-idea |
| `pivots` | `pivoted-by` | pivot | vision, pillar, persona |
| `hands-off-to` | `handed-off-from` | discovery-idea | idea |
| `informs` | `informed-by` | discovery-research | discovery-decision |

---

## 4. agile-planning

**Package:** `@orqastudio/plugin-agile-planning` v0.1.4-dev (private)
**Display Name:** Agile Planning
**Category:** methodology
**Total files:** 7

### Manifest Summary (orqa-plugin.json)

- **Schemas:** 4 (planning-idea PLAN-IDEA-*, planning-research PLAN-RES-*, planning-decision PLAN-AD-*, wireframe WF-*)
- **Workflows:** 5 (4 artifact workflows + 1 contribution)
- **Relationships:** 5 types (evolves-to x2, references, implements, visualises)
- **Agents:** 0
- **Knowledge:** 0
- **Rules:** 0
- **Content mappings:** workflows source -> `.orqa/process/workflows` target

### Workflow Files (5)

| File | Artifact Type | States | Description |
|------|--------------|--------|-------------|
| `workflows/planning.contribution.workflow.yaml` | epic (contribution) | 4 sub-states (scope_analysis, estimation, prioritisation, plan_finalised) | Contributes to agile-methodology `planning-methodology` point |
| `workflows/planning-idea.workflow.yaml` | planning-idea | 6 (draft, evaluating, accepted, hold, rejected, archived) | Planning idea lifecycle |
| `workflows/planning-research.workflow.yaml` | planning-research | 5 (proposed, investigating, hold, concluded, archived) | Planning research lifecycle |
| `workflows/planning-decision.workflow.yaml` | planning-decision | 6 (proposed, reviewing, hold, resolved, deferred, archived) | Planning decision lifecycle |
| `workflows/wireframe.workflow.yaml` | wireframe | 8 (draft, review, approved, revision, implementing, implemented, hold, archived) | Standard lifecycle with implementation tracking |

### Schemas (4)

| ID Prefix | Artifact Type | Default Path |
|-----------|--------------|--------------|
| PLAN-IDEA-* | planning-idea | `.orqa/planning/ideas/` |
| PLAN-RES-* | planning-research | `.orqa/planning/research/` |
| PLAN-AD-* | planning-decision | `.orqa/planning/decisions/` |
| WF-* | wireframe | `.orqa/planning/wireframes/` |

### Relationship Types (5)

| Forward | Inverse | From | To |
|---------|---------|------|----|
| `evolves-to` | `evolved-from` | planning-idea | planning-idea |
| `evolves-to` | `evolved-from` | planning-research | planning-research |
| `references` | `referenced-by` | planning-decision | planning-research |
| `implements` | `implemented-by` | planning-decision | planning-idea |
| `visualises` | `visualised-by` | wireframe | planning-idea |

---

## 5. agile-documentation

**Package:** `@orqastudio/plugin-agile-documentation` v0.1.4-dev (private)
**Display Name:** Agile Documentation
**Category:** documentation
**Total files:** 3

### Manifest Summary (orqa-plugin.json)

- **Schemas:** 0
- **Workflows:** 1 (contribution only)
- **Relationships:** 0
- **Agents:** 0
- **Knowledge:** 0
- **Rules:** 0
- **Content mappings:** workflows source -> `.orqa/process/workflows` target

### Workflow Files (1)

| File | Artifact Type | States | Description |
|------|--------------|--------|-------------|
| `workflows/documentation.contribution.workflow.yaml` | epic (contribution) | 3 sub-states (draft_docs, review_docs, publish_docs) | Contributes to agile-methodology `documentation-standards` point. Transitions include skip_review path for simple changes. |

---

## 6. agile-review

**Package:** `@orqastudio/plugin-agile-review` v0.1.4-dev (private, type: module)
**Display Name:** Agile Review
**Category:** stage-definition
**Dependencies:** `@orqastudio/types`
**Total files:** 3

### Manifest Summary (orqa-plugin.json)

- **Schemas:** 0
- **Workflows:** 1 (contribution only)
- **Relationships:** 0
- **Agents:** 0
- **Knowledge:** 0
- **Rules:** 0
- **Content mappings:** None

### Workflow Files (1)

| File | Artifact Type | States | Description |
|------|--------------|--------|-------------|
| `workflows/review.contribution.workflow.yaml` | epic (contribution) | 4 sub-states (code_review, gate_check, acceptance_verification, review_complete) | Contributes to agile-methodology `review-process` point. Multiple rework paths back to implement or code_review. |

---

## 7. software-kanban

**Package:** `@orqastudio/plugin-software-kanban` v0.1.4-dev (private, type: module)
**Display Name:** Software Project
**Category:** delivery
**Dependencies:** `@orqastudio/types`
**Peer Dependencies:** `@orqastudio/sdk`, `@orqastudio/svelte-components`, `svelte ^5`
**Dev Dependencies:** `@sveltejs/vite-plugin-svelte`, `vite`, `typescript`
**Build scripts:** `build` (vite build), `dev` (vite build --watch)
**Total files:** 37 (excluding node_modules, dist)

### Manifest Summary (orqa-plugin.json)

- **Schemas:** 3 (milestone MS-*, epic EPIC-*, task TASK-*)
- **Views:** 1 (roadmap)
- **Widgets:** 2 (pipeline, milestone-context)
- **Workflows:** 4 (3 artifact workflows + 1 contribution)
- **Relationships:** 10 types with status rule constraints
- **Knowledge items:** 12 declared
- **Knowledge declarations:** With tiers, roles, prompt sections, decision tree, navigation, delivery types, artifact links, semantics
- **Content mappings:** knowledge, rules, documentation, prompts, workflows (source dirs -> `.orqa/` targets)
- **Build config:** Present

### Workflow Files (4)

| File | Artifact Type | States | Variants | Description |
|------|--------------|--------|----------|-------------|
| `workflows/milestone.workflow.yaml` | milestone | 11 | None | Guards and milestone-review gate (structured_review pattern) |
| `workflows/epic.workflow.yaml` | epic | 11 | None | Guards and epic-review gate (structured_review pattern) |
| `workflows/task.workflow.yaml` | task | 10 | 4 (quickfix, security, docs-only, hotfix) with selection_rules | task-review gate; variants skip/rearrange states |
| `workflows/implementation.contribution.workflow.yaml` | epic (contribution) | 5 sub-states (task_assignment, development, testing, integration, implementation_complete) | None | Contributes to agile-methodology `implementation-workflow` point |

### Knowledge Files (11 on disk, 12 declared in manifest)

| File | Title | Notes |
|------|-------|-------|
| `knowledge/KNOW-0188373b.md` | Delivery Unit Completion Discipline | |
| `knowledge/KNOW-d00093e7.md` | Component Extraction | |
| `knowledge/KNOW-a700e25a.md` | Software Delivery Management | Uses `name:` instead of `title:` in frontmatter |
| `knowledge/KNOW-1314ac47.md` | QA Verification | |
| `knowledge/KNOW-45b5f8a8.md` | Security Audit | |
| `knowledge/KNOW-5f4db8f7.md` | Test Engineering | |
| `knowledge/KNOW-71352dc8.md` | UAT Process | |
| `knowledge/KNOW-72ca209f.md` | Skills Maintenance | |
| `knowledge/KNOW-91a7a6c1.md` | Code Quality Review | |
| `knowledge/KNOW-bec7e87d.md` | UX Compliance Review | |
| `knowledge/KNOW-d03337ac.md` | Project Type: Software | |
| `knowledge/KNOW-ec6c45a7.md` | Software -- Relationship Vocabulary | 10 relationship types documented |

**Missing file:** `knowledge/KNOW-3f307edb.md` is referenced in orqa-plugin.json knowledge list as "Orqa Testing Patterns" but does not exist on disk.

### Rule Files (3)

| File | Title |
|------|-------|
| `rules/RULE-dccf4226.md` | Plan Mode Compliance |
| `rules/RULE-63cc16ad.md` | Artifact Config Integrity |
| `rules/RULE-71352dc8.md` | UAT Process |

### Documentation Files (1)

| File | Title | Notes |
|------|-------|-------|
| `documentation/DOC-4554ff3e.md` | Software Delivery Guide | Uses `name:` instead of `title:` in frontmatter |

### Prompt Files (1)

| File | Description |
|------|-------------|
| `prompts/implementer-role.md` | Software delivery implementer role definition |

### Schemas (3)

| ID Prefix | Artifact Type | Default Path |
|-----------|--------------|--------------|
| MS-* | milestone | `.orqa/delivery/milestones/` |
| EPIC-* | epic | `.orqa/delivery/epics/` |
| TASK-* | task | `.orqa/delivery/tasks/` |

### Relationship Types (10)

| Forward | Inverse | From | To | Status Rules |
|---------|---------|------|----|-------------|
| `realises` | `realised-by` | idea | epic, task | None |
| `yields` | `yielded-by` | task | lesson | None |
| `addresses` | `addressed-by` | task, epic | lesson | None |
| `delivers` | `delivered-by` | task | epic | Required min 1. Epic -> review when all tasks completed |
| `fulfils` | `fulfilled-by` | epic | milestone | Milestone -> review when all epics completed |
| `depends-on` | `depended-on-by` | task, epic | task, epic | Blocked when any dependency incomplete; unblocked when all completed |
| `reports` | `reported-by` | bug | epic, task, milestone | Required min 1 |
| `fixes` | `fixed-by` | task | bug | None |
| `affects` | `affected-by` | bug | persona | None |
| `implements` | `implemented-by` | task, epic | decision | None |

### Source Code Files (8)

| File | Description |
|------|-------------|
| `src/views/RoadmapView.svelte` | Main roadmap view component |
| `src/views/roadmap.ts` | Roadmap data logic |
| `src/views/StatusKanban.svelte` | Kanban board component |
| `src/views/HorizonBoard.svelte` | Horizon board component |
| `src/views/KanbanCard.svelte` | Card component |
| `src/views/MilestoneCard.svelte` | Milestone card component |
| `src/views/CollapsibleColumn.svelte` | Collapsible column UI component |
| `src/views/DrilldownBreadcrumbs.svelte` | Breadcrumb navigation component |

### Config and Build Files

| File | Description |
|------|-------------|
| `tsconfig.json` | TypeScript configuration |
| `vite.config.ts` | Vite build configuration |
| `.gitignore` | Git ignore patterns |
| `CHANGE-LICENSE` | License change notice |
| `thumbnail.png` | Plugin thumbnail image |

### Dist Output (2)

| File |
|------|
| `dist/plugin-software-kanban.css` |
| `dist/views/roadmap.js` |

---

## Cross-Plugin Summary

### Contribution Workflow Architecture

The agile-methodology skeleton (in agile-workflow) defines 6 contribution points. Each is filled by a stage-definition plugin:

| Contribution Point | Filled By | Sub-States |
|--------------------|-----------|-----------:|
| `discovery-artifacts` | agile-discovery | 4 |
| `planning-methodology` | agile-planning | 4 |
| `documentation-standards` | agile-documentation | 3 |
| `implementation-workflow` | software-kanban | 5 |
| `review-process` | agile-review | 4 |
| `learning-pipeline` | core | 5 |

### Schema Distribution

| Plugin | Schemas | Total Types |
|--------|---------|------------|
| core | decision, rule, lesson, knowledge, agent, doc | 6 |
| agile-discovery | vision, pillar, persona, pivot, discovery-idea, discovery-research, discovery-decision | 7 |
| agile-planning | planning-idea, planning-research, planning-decision, wireframe | 4 |
| software-kanban | milestone, epic, task | 3 |
| agile-workflow | (none) | 0 |
| agile-documentation | (none) | 0 |
| agile-review | (none) | 0 |
| **Total** | | **20** |

### Relationship Distribution

| Plugin | Relationship Types |
|--------|-----------------:|
| agile-workflow | 20 |
| software-kanban | 10 |
| agile-discovery | 6 |
| agile-planning | 5 |
| agile-documentation | 0 |
| agile-review | 0 |
| core | 0 |
| **Total** | **41** |

### File Counts by Plugin

| Plugin | Total Files | Workflows | Knowledge | Rules | Agents | Docs | Roles | Source |
|--------|------------|-----------|-----------|-------|--------|------|-------|--------|
| agile-workflow | 38 | 1 | 8 | 17 | 2 | 0 | 8 | 0 |
| core | 63 | 7 | 23 | 17 | 9 | 8 | 0 | 0 |
| agile-discovery | 10 | 8 | 0 | 0 | 0 | 0 | 0 | 0 |
| agile-planning | 7 | 5 | 0 | 0 | 0 | 0 | 0 | 0 |
| agile-documentation | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| agile-review | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| software-kanban | 37 | 4 | 11* | 3 | 0 | 1 | 0 | 8 |
| **Total** | **161** | **27** | **42** | **37** | **11** | **9** | **8** | **8** |

*software-kanban has 11 knowledge files on disk but 12 declared in manifest (KNOW-3f307edb missing).

### Anomalies Observed

1. **Missing file:** `plugins/software-kanban/knowledge/KNOW-3f307edb.md` referenced in orqa-plugin.json but does not exist on disk.
2. **Duplicate knowledge titles in agile-workflow:**
   - KNOW-83039175 "Thinking Mode: Learning Loop" and KNOW-85e392ea "Thinking Mode - Learning Loop" (same topic, different title format)
   - KNOW-0444355f and KNOW-8d1c4be6 both titled "Plugin Artifact Usage"
   - KNOW-8c359ea4 and KNOW-8d76c3c7 both titled "Governance Maintenance"
3. **Inconsistent frontmatter field names:** software-kanban's KNOW-a700e25a and DOC-4554ff3e use `name:` instead of `title:` for the artifact title field.
4. **Shared agent ID:** AGENT-ae63c406 "Governance Steward" exists in both `plugins/core/agents/` and `plugins/agile-workflow/agents/` with the same ID.
5. **Relationship count note:** Total of 41 relationship types declared across all plugins. The CLAUDE.md states 30 -- the difference comes from agile-discovery (6) and agile-planning (5) contributing types not counted in the original 30.
