# Phase 2: Plugin Ecosystem Gap Analysis

Comparison of the current plugin ecosystem against ARCHITECTURE.md Section 4 (Plugin Architecture).

**Date:** 2026-03-26
**Inputs:** `file-audit/06-plugins-methodology-workflow.md`, `file-audit/07-plugins-knowledge-infra.md`, `ARCHITECTURE.md` sections 4.1-4.6

---

## 1. Plugin Taxonomy Alignment

ARCHITECTURE.md section 4.3 defines five plugin categories: Methodology, Workflow, Domain Knowledge, Connector, Infrastructure. Each plugin should clearly map to exactly one category (or have dual-purpose explicitly declared).

### Current Category Values in Manifests

| Plugin | Manifest `category` | ARCHITECTURE.md Category | Aligned? |
|--------|---------------------|--------------------------|----------|
| `agile-workflow` | `governance` | Methodology | NO |
| `core` | `framework` | Workflow (Learning stage) | NO |
| `agile-discovery` | `discovery` | Workflow (Discovery stage) | NO |
| `agile-planning` | `methodology` | Workflow (Planning stage) | NO |
| `agile-documentation` | `documentation` | Workflow (Documentation stage) | NO |
| `agile-review` | `stage-definition` | Workflow (Review stage) | NO |
| `software-kanban` | `delivery` | Workflow (Implementation stage) + App Extension (dual-purpose) | PARTIAL |
| `cli` | `tooling` | Domain Knowledge | PARTIAL |
| `rust` | `coding-standards` | Domain Knowledge (dual-purpose: knowledge + infrastructure) | PARTIAL |
| `svelte` | `tooling` | Domain Knowledge | PARTIAL |
| `tauri` | `tooling` | Domain Knowledge | PARTIAL |
| `typescript` | `coding-standards` | Domain Knowledge (dual-purpose: knowledge + infrastructure) | PARTIAL |
| `coding-standards` | `coding-standards` | Domain Knowledge | PARTIAL |
| `systems-thinking` | `thinking` | Domain Knowledge | PARTIAL |
| `plugin-dev` | `development` | Domain Knowledge | PARTIAL |
| `githooks` | `enforcement` | Infrastructure | PARTIAL |

### Gaps Found

**GAP-TAX-1: No consistent taxonomy vocabulary.** The architecture defines 5 categories (methodology, workflow, domain-knowledge, connector, infrastructure). Manifests use 11 different category values: `governance`, `framework`, `discovery`, `methodology`, `documentation`, `stage-definition`, `delivery`, `tooling`, `coding-standards`, `thinking`, `development`, `enforcement`. None of the manifests use the ARCHITECTURE.md vocabulary.

**GAP-TAX-2: `agile-workflow` is miscategorized.** Category is `governance`, role is `core:workflow`. Per ARCHITECTURE.md it is THE methodology plugin. Its category should be `methodology` (and is the only plugin that should have this category). Instead, `agile-planning` has `category: "methodology"` which is wrong — it is a workflow plugin for the planning stage.

**GAP-TAX-3: `core` category `framework` is not in the taxonomy.** ARCHITECTURE.md describes `core` as the workflow plugin for the Learning stage. Its category `framework` and role `core:framework` suggest it occupies a special position, but the architecture says it fills the `learning-pipeline` contribution point like any other workflow plugin. The `uninstallable: true` flag is appropriate for its special status but the category should reflect it is a workflow plugin.

**GAP-TAX-4: Workflow plugins use inconsistent categories.** Five workflow plugins use five different category values: `discovery`, `methodology` (wrong — should be agile-workflow's), `documentation`, `stage-definition`, `delivery`. They should all use `workflow` or equivalent.

**GAP-TAX-5: No `stage_slot` declaration.** Per ARCHITECTURE.md 4.4, each workflow plugin fills one stage slot. The contribution workflow YAML files declare `contributes_to.point` but the `orqa-plugin.json` manifests do not surface which stage slot the plugin fills. The installer would need to parse YAML workflow files to discover this.

---

## 2. orqa-plugin.json Manifest Completeness

### Missing Declaration Fields

**GAP-MAN-1: No `purpose` field.** ARCHITECTURE.md 4.1 defines 7 purposes (methodology-definition, workflow-definition, knowledge/rules, app-extension, sidecar, connector, infrastructure). Manifests do not declare their purpose explicitly. The installer must infer purpose from the combination of `category`, `role`, presence of `provides.schemas`, and `provides.workflows`.

**GAP-MAN-2: No `stage_slot` field on workflow plugins.** The manifest does not declare which methodology stage the plugin fills. This is only discoverable by reading the contribution workflow YAML and finding the `contributes_to.point` field. For `orqa install` to enforce "one workflow plugin per stage" (ARCHITECTURE.md 4.4), it would need to either parse YAML or have this in the manifest.

**GAP-MAN-3: No `affects_schema` boolean.** ARCHITECTURE.md 4.4 distinguishes definition plugins (trigger recomposition) from non-definition plugins (just install assets). Manifests don't declare this. The installer must infer it from `provides.schemas` length > 0.

**GAP-MAN-4: Contribution workflow missing from `provides.workflows` in `software-kanban`.** The file `workflows/implementation.contribution.workflow.yaml` exists on disk and has correct `contributes_to` metadata, but it is NOT listed in the `provides.workflows` array of the manifest. The manifest only lists `milestone`, `epic`, and `task` workflows. The installer cannot discover this contribution without scanning the filesystem.

**GAP-MAN-5: Inconsistent workflow declaration format in `agile-documentation`.** Its `provides.workflows` is a flat string array (`["workflows/documentation.contribution.workflow.yaml"]`) rather than the structured object format (`{artifact_type, path, contribution}`) used by all other plugins. This would break any installer expecting a uniform schema.

### Missing Files Referenced by Manifests

**GAP-MAN-6: `plugins/software-kanban/knowledge/KNOW-3f307edb.md` missing from plugin source.** The manifest declares `KNOW-3f307edb` ("Orqa Testing Patterns") in `provides.knowledge` and references it in `knowledge_declarations`. The file exists at `.orqa/process/knowledge/KNOW-3f307edb.md` (installed copy) but NOT in the plugin's own `knowledge/` directory. This violates the plugin-canonical architecture where the plugin source is the source of truth.

**GAP-MAN-7: `plugins/software-kanban/prompts/review-checklist.md` missing.** The manifest declares a `prompt_sections` entry referencing `prompts/review-checklist.md` but only `prompts/implementer-role.md` exists on disk.

### Inconsistent Frontmatter Fields in Schema Definitions

**GAP-MAN-8: `name` vs `title` inconsistency.** Some schemas use `title` as the display-name field (decision, rule, lesson, knowledge, agent, doc, planning-idea, planning-decision, discovery-idea, discovery-decision), while others use `name` (milestone, epic, task, wireframe, discovery-research, planning-research). Also confirmed: `KNOW-a700e25a.md` and `DOC-4554ff3e.md` in software-kanban use `name:` instead of `title:` in actual artifact frontmatter. There is no schema-level consistency.

---

## 3. Legacy AGENT-*.md Files

ARCHITECTURE.md section 6.4 explicitly states: "Monolithic AGENT-*.md specialist definitions" are the OLD pattern, replaced by "Base roles + generated task-specific agents."

### Complete AGENT-*.md Inventory Across All Plugins

| Plugin | File | Title | Legacy? |
|--------|------|-------|---------|
| `core` | AGENT-4c94fe14.md | Orchestrator | YES — should be generated from `roles/orchestrator.yaml` |
| `core` | AGENT-8e58cd87.md | Reviewer | YES — should be generated from `roles/reviewer.yaml` |
| `core` | AGENT-e333508b.md | Researcher | YES — should be generated from `roles/researcher.yaml` |
| `core` | AGENT-85be6ace.md | Planner | YES — should be generated from `roles/planner.yaml` |
| `core` | AGENT-d1be3776.md | Installer | YES — no role YAML exists for installer |
| `core` | AGENT-e5dd38e4.md | Implementer | YES — should be generated from `roles/implementer.yaml` |
| `core` | AGENT-bbad3d30.md | Writer | YES — should be generated from `roles/writer.yaml` |
| `core` | AGENT-0aad40f4.md | Designer | YES — should be generated from `roles/designer.yaml` |
| `core` | AGENT-ae63c406.md | Governance Steward | YES — should be generated from `roles/governance_steward.yaml` |
| `agile-workflow` | AGENT-7a06d10e.md | Governance Enforcer | YES — no corresponding role YAML |
| `agile-workflow` | AGENT-ae63c406.md | Governance Steward | YES — DUPLICATE of core's |
| `rust` | AGENT-065a25cc.md | Rust Specialist | YES — domain knowledge, not a base role |
| `rust` | AGENT-26e5029d.md | Rust Standards Agent | YES — domain knowledge, not a base role |
| `svelte` | AGENT-5de8c14f.md | Svelte Specialist | YES — domain knowledge, not a base role |
| `svelte` | AGENT-6f55de0d.md | Svelte Standards Agent | YES — domain knowledge, not a base role |
| `tauri` | AGENT-65b56a0b.md | Tauri Standards Agent | YES — domain knowledge, not a base role |
| `plugin-dev` | AGENT-ce86fb50.md | Plugin Developer | YES — domain knowledge, not a base role |

**Total: 17 AGENT-*.md files across 6 plugins.** All are legacy per the architecture. The target state is: 5 base roles (as YAML in the methodology plugin) + task-specific agents generated at runtime by the engine. No static AGENT-*.md files.

**GAP-AGENT-1: core defines an `agent` schema and workflow.** The core plugin defines `agent` as an artifact type with AGENT-* ID prefix, status lifecycle, and a full 8-state workflow. This means the system is designed to CREATE and MANAGE agent artifacts. Per the architecture, fixed agent definitions should not exist. The agent schema/workflow may still be needed for tracking generated agent instances, but the 9 static AGENT-*.md files in core should not exist as artifacts.

**GAP-AGENT-2: Role definitions exist in `agile-workflow` but agent definitions exist in `core`.** The architecture says base roles come from the methodology plugin. Role YAMLs are in `plugins/agile-workflow/roles/` (correct). But the corresponding AGENT-*.md files are in `plugins/core/agents/` — the wrong plugin.

---

## 4. Duplicate Artifacts

### Same Agent ID in Multiple Plugins

**GAP-DUP-1: AGENT-ae63c406 exists in both `core` and `agile-workflow`.**

- `plugins/core/agents/AGENT-ae63c406.md`: Updated 2026-03-24, richer description with 3 sub-roles, references KNOW-e3432947 and KNOW-57365826
- `plugins/agile-workflow/agents/AGENT-ae63c406.md`: Created 2026-03-14, simpler description, references KNOW-936e5944 and KNOW-16e91c20

Both have the same ID, same title ("Governance Steward"), same model (sonnet). The core version is newer and more detailed. The agile-workflow version is stale. When installed, one would overwrite the other depending on installation order.

### Duplicate Knowledge Titles in agile-workflow

**GAP-DUP-2: Three pairs of duplicate-topic knowledge in agile-workflow:**

| Pair | File A | File B | Title |
|------|--------|--------|-------|
| 1 | KNOW-83039175 | KNOW-85e392ea | "Thinking Mode: Learning Loop" / "Thinking Mode - Learning Loop" |
| 2 | KNOW-0444355f | KNOW-8d1c4be6 | Both "Plugin Artifact Usage" |
| 3 | KNOW-8c359ea4 | KNOW-8d76c3c7 | Both "Governance Maintenance" |

In each pair, the manifest's `provides.knowledge` array uses a cross-reference pattern: the `key` field of one entry is the ID of the other entry (e.g., key `85e392ea` maps to id `KNOW-83039175`). This suggests they were intended to be the same artifact but got duplicated.

### Duplicate Knowledge Declarations

**GAP-DUP-3: Duplicate knowledge_declarations IDs in agile-workflow.** Three entries all use `id: "thinking-mode-governance"`:
1. Actual thinking-mode-governance (content_file: KNOW-c89f28b3)
2. Schema-first frontmatter rule (content_file: KNOW-57365826)
3. Plugin-canonical architecture (content_file: KNOW-e3432947)
4. Documentation placement guide (content_file: KNOW-6d80cf39)

The last three should have unique IDs. Duplicate IDs mean the prompt pipeline would only see the last one (or whichever wins in a map).

**GAP-DUP-4: Duplicate knowledge_declarations in software-kanban.** Several pairs of declarations reference the same underlying content with different IDs and content paths:
- `code-quality-review` vs `code-quality-review-91a7a6c1` (both about KNOW-91a7a6c1)
- `security-audit` vs `security-audit-45b5f8a8` (both about KNOW-45b5f8a8)
- `test-engineering` vs `test-engineering-5f4db8f7` (both about KNOW-5f4db8f7)
- `delivery-completion` vs `delivery-unit-completion-discipline` (both about KNOW-0188373b)

One path points to the plugin-local file, the other to `../../.orqa/process/knowledge/`. This duplication means each knowledge item would be injected twice.

---

## 5. Missing Files

| Plugin | Missing File | Referenced In | Expected Content |
|--------|-------------|---------------|-----------------|
| `software-kanban` | `knowledge/KNOW-3f307edb.md` | `provides.knowledge`, `knowledge_declarations` | "Orqa Testing Patterns" |
| `software-kanban` | `prompts/review-checklist.md` | `prompt_sections` | Review stage instructions |

**GAP-MISS-1:** KNOW-3f307edb exists at `.orqa/process/knowledge/KNOW-3f307edb.md` but not in the plugin source. The knowledge_declaration references the installed path (`../../.orqa/process/knowledge/KNOW-3f307edb.md`), which works at runtime but violates plugin-canonical architecture. The plugin should own this file in its `knowledge/` directory.

**GAP-MISS-2:** `prompts/review-checklist.md` does not exist anywhere. The manifest declares it as a `prompt_sections` entry but the file was never created.

---

## 6. Role Definitions

ARCHITECTURE.md section 6.1 defines 5 base roles: Orchestrator, Implementer, Reviewer, Researcher, Writer.

### Current Role Files in agile-workflow/roles/

| Role File | In Architecture? | Permissions Defined? | Tool Constraints? | Artifact Scope? |
|-----------|-----------------|---------------------|-------------------|-----------------|
| `orchestrator.yaml` | YES | Yes (can_edit, can_run_shell, can_search_web) | Yes | coordination-only |
| `implementer.yaml` | YES | Yes | Yes | source-code-only |
| `reviewer.yaml` | YES | Yes | Yes | read-only |
| `researcher.yaml` | YES | Yes | Yes | research artifacts |
| `writer.yaml` | YES | Yes | Yes | documentation-only |
| `planner.yaml` | Not in 5 base roles | Yes | Yes | delivery artifacts |
| `designer.yaml` | Not in 5 base roles | Yes | Yes | source-code-only |
| `governance_steward.yaml` | Not in 5 base roles | Yes | Yes | .orqa/-artifacts-only |

**GAP-ROLE-1: 8 roles defined, architecture specifies 5.** Planner, Designer, and Governance Steward are additional roles not in the ARCHITECTURE.md section 6.1 base roles. These may be intentional extensions, but they represent a divergence from the documented architecture.

**GAP-ROLE-2: Roles are in the correct plugin.** ARCHITECTURE.md says "Base roles are stable definitions provided by the methodology plugin" — and they are in `agile-workflow`, which is the methodology plugin. This is correctly placed.

**GAP-ROLE-3: Role files define `knowledge_composition`.** The role YAML files include `knowledge_composition` blocks specifying what knowledge to inject per stage. This is consistent with the architecture's "Base Role + Workflow Context + Domain Knowledge" composition model. However, the architecture says knowledge selection happens at delegation time, not predefined — the role files predefine it.

---

## 7. Workflow Completeness

### Methodology Skeleton (agile-workflow)

The `agile-methodology.workflow.yaml` defines:
- 7 states (discover, plan, document, implement, review, learn, done) with categories
- 6 contribution points (all `required: true`)
- Transitions with guards and a delivery-review gate
- The gate uses `structured_review` pattern with gather/present/collect/learn phases

This is complete and well-structured.

### Contribution Workflows

| Contribution Point | Filled By | Has `contributes_to`? | In `provides.workflows`? |
|--------------------|-----------|-----------------------|--------------------------|
| `discovery-artifacts` | agile-discovery | YES | YES (contribution: true) |
| `planning-methodology` | agile-planning | YES | YES (contribution: true) |
| `documentation-standards` | agile-documentation | YES | YES (but as string, not object) |
| `implementation-workflow` | software-kanban | YES | **NO — missing from manifest** |
| `review-process` | agile-review | YES | YES (contribution: true) |
| `learning-pipeline` | core | YES | YES (contribution: true) |

**GAP-WF-1: software-kanban contribution workflow not declared in manifest.** The file exists and has correct `contributes_to` metadata, but `orqa install` cannot discover it from the manifest alone.

**GAP-WF-2: agile-documentation workflow declaration is a flat string.** All other plugins use `{artifact_type, path, contribution}` objects. agile-documentation uses a bare string. This breaks schema uniformity.

### Artifact Workflows

All artifact workflows (decision, rule, lesson, knowledge, agent, doc, vision, pillar, persona, pivot, discovery-idea, discovery-research, discovery-decision, planning-idea, planning-research, planning-decision, wireframe, milestone, epic, task) define complete state machines with states, transitions, and categories. The architecture requirement of "no inheritance — each plugin owns its complete state machine" is met.

**GAP-WF-3: No resolved workflow files exist.** ARCHITECTURE.md 4.6 and 5.1 specify that resolved workflows should be written to `.orqa/workflows/<name>.resolved.yaml`. These do not exist yet. The composition pipeline has not been implemented.

---

## 8. Plugin Naming

ARCHITECTURE.md section 4.3 uses specific names in its taxonomy table. Current names vs suggested clarity:

| Current Name | Role per Architecture | Name Self-Evident? | Issue |
|-------------|----------------------|--------------------| ------|
| `agile-workflow` | Methodology plugin | NO | "workflow" implies it is a workflow plugin, not THE methodology plugin |
| `core` | Learning stage workflow | NO | "core" implies platform core, not a specific stage |
| `agile-discovery` | Discovery stage workflow | YES | |
| `agile-planning` | Planning stage workflow | YES | |
| `agile-documentation` | Documentation stage workflow | YES | |
| `agile-review` | Review stage workflow | YES | |
| `software-kanban` | Implementation stage workflow + app extension | PARTIAL | "kanban" suggests views, not methodology stage |
| `coding-standards` | Domain knowledge (+ infrastructure) | YES | |
| `systems-thinking` | Domain knowledge | YES | |
| `githooks` | Infrastructure | YES | |

**GAP-NAME-1: `agile-workflow` is confusing.** As the methodology plugin, a name like `agile-methodology` or `agile-governance-methodology` would be clearer. The current name suggests it is one workflow among many.

**GAP-NAME-2: `core` does not communicate its role.** It serves dual purpose: (a) framework providing schemas for learning-loop artifacts, and (b) the learning-stage workflow plugin. Neither purpose is evident from "core." A name like `core-framework` (its package name) or `learning-loop` would be clearer.

---

## 9. Dual-Purpose Plugins

ARCHITECTURE.md 4.3 notes that some plugins serve dual purposes:
- `software-kanban`: workflow plugin + app extension (views/widgets) — ARCHITECTURE.md lists this explicitly
- `rust`: domain knowledge + generates linting infrastructure
- `typescript`: domain knowledge + generates linting infrastructure

### Current Manifest Declarations

**GAP-DUAL-1: No explicit dual-purpose declaration.** Manifests do not have a field that says "this plugin serves these purposes." The dual nature must be inferred from the combination of `provides.schemas` + `provides.views` (for software-kanban) or `provides.enforcement_mechanisms` + `provides.knowledge` (for rust/typescript). The installer has no declarative way to understand which installation actions are needed.

**GAP-DUAL-2: `core` is the most multi-purpose plugin but not declared as such.** Core provides: 6 schemas, 7 workflows (6 artifact + 1 contribution), 4 enforcement mechanisms, 30+ knowledge items, 9 agents, 8 docs, and content mappings. It serves as both the learning-stage workflow plugin and the framework provider. None of this is declaratively stated.

---

## 10. Installation Constraint Readiness

ARCHITECTURE.md 4.4 specifies `orqa install` must enforce:
1. One methodology plugin per project
2. One workflow plugin per stage
3. Definition plugins trigger full recomposition
4. Non-definition plugins only install assets

### Can the Installer Enforce These From Current Manifests?

**GAP-INST-1: Cannot enforce one-methodology rule.** No manifest declares `purpose: "methodology"`. The installer would need a heuristic like "has `provides.workflows` with a non-contribution workflow AND no `provides.schemas`" — fragile and wrong (agile-workflow has no schemas but core also has workflows without being the methodology plugin). Or it could look for `contribution_points` in workflow YAML — but that requires parsing YAML, not just the JSON manifest.

**GAP-INST-2: Cannot enforce one-per-stage rule.** No manifest declares a `stage_slot` field. The contribution workflow YAML files contain `contributes_to.point` (e.g., `discovery-artifacts`), but this is not surfaced in `orqa-plugin.json`. The installer would need to:
1. Find contribution workflow files in `provides.workflows` where `contribution: true`
2. Parse the YAML file
3. Read `contributes_to.point`
4. Match against the methodology's `contribution_points`

This is feasible but unnecessarily complex. A simple `"stage_slot": "discovery-artifacts"` in the manifest would suffice.

**GAP-INST-3: Cannot distinguish definition vs non-definition plugins.** No `triggers_recomposition` or `affects_schema` boolean. Must be inferred from `provides.schemas.length > 0` — which is a reasonable heuristic but not explicit.

**GAP-INST-4: Workflow format inconsistency blocks automated processing.** The `provides.workflows` array uses structured objects in most plugins but a flat string in agile-documentation. Any installer code would need to handle both formats or fail.

---

## Summary of All Gaps

### Critical (blocks installer/composition pipeline)

| ID | Gap | Affected Plugins |
|----|-----|-----------------|
| GAP-TAX-1 | No consistent taxonomy vocabulary | All 16 |
| GAP-TAX-2 | agile-workflow miscategorized as `governance` | agile-workflow |
| GAP-TAX-4 | agile-planning miscategorized as `methodology` | agile-planning |
| GAP-MAN-1 | No `purpose` field in manifests | All 16 |
| GAP-MAN-2 | No `stage_slot` field | All 6 workflow plugins |
| GAP-MAN-4 | Contribution workflow missing from software-kanban manifest | software-kanban |
| GAP-MAN-5 | Inconsistent workflow declaration format | agile-documentation |
| GAP-WF-1 | software-kanban contribution not in manifest | software-kanban |
| GAP-WF-3 | No resolved workflow files exist | System-wide |
| GAP-INST-1 | Cannot enforce one-methodology rule | System-wide |
| GAP-INST-2 | Cannot enforce one-per-stage rule | System-wide |

### High (data integrity / architecture alignment)

| ID | Gap | Affected Plugins |
|----|-----|-----------------|
| GAP-DUP-1 | AGENT-ae63c406 duplicated across plugins | core, agile-workflow |
| GAP-DUP-2 | 3 duplicate knowledge pairs | agile-workflow |
| GAP-DUP-3 | Duplicate knowledge_declaration IDs | agile-workflow |
| GAP-DUP-4 | Duplicate knowledge_declarations with dual paths | software-kanban |
| GAP-MISS-1 | KNOW-3f307edb missing from plugin source | software-kanban |
| GAP-MISS-2 | review-checklist.md missing | software-kanban |
| GAP-MAN-8 | name vs title inconsistency in schemas | Multiple |
| GAP-AGENT-1 | 17 legacy AGENT-*.md files exist | 6 plugins |
| GAP-AGENT-2 | Roles in agile-workflow but agents in core | core, agile-workflow |

### Medium (clarity / naming / conventions)

| ID | Gap | Affected Plugins |
|----|-----|-----------------|
| GAP-TAX-3 | core category `framework` not in taxonomy | core |
| GAP-TAX-5 | No `stage_slot` in manifest (must parse YAML) | 6 workflow plugins |
| GAP-MAN-3 | No `affects_schema` boolean | All |
| GAP-MAN-6 | Knowledge file in .orqa/ but not in plugin source | software-kanban |
| GAP-NAME-1 | agile-workflow name implies workflow, not methodology | agile-workflow |
| GAP-NAME-2 | core name doesn't communicate learning-stage role | core |
| GAP-ROLE-1 | 8 roles defined, architecture specifies 5 | agile-workflow |
| GAP-ROLE-3 | Roles predefine knowledge composition | agile-workflow |
| GAP-DUAL-1 | No explicit dual-purpose declaration | rust, typescript, software-kanban |
| GAP-DUAL-2 | core multi-purpose not declared | core |
| GAP-INST-3 | Cannot distinguish definition vs non-definition plugins | System-wide |
| GAP-INST-4 | Workflow format inconsistency | agile-documentation |
| GAP-WF-2 | agile-documentation workflow as flat string | agile-documentation |

---

## Recommendations

### R1: Standardize manifest taxonomy (addresses GAP-TAX-1 through GAP-TAX-5, GAP-INST-1 through GAP-INST-4)

Add explicit fields to `orqa-plugin.json`:

```json
{
  "purpose": "methodology" | "workflow" | "domain-knowledge" | "connector" | "infrastructure" | "app-extension" | "sidecar",
  "stage_slot": "discovery-artifacts",       // workflow plugins only
  "triggers_recomposition": true,            // methodology + workflow plugins
  "category": "methodology"                  // use ARCHITECTURE.md vocabulary
}
```

Fix category values:
- `agile-workflow` -> `methodology`
- `agile-planning` -> `workflow` (not `methodology`)
- `agile-discovery`, `agile-documentation`, `agile-review`, `software-kanban` -> `workflow`
- `core` -> `workflow` (or `framework-workflow` if dual status needed)
- Domain knowledge plugins -> `domain-knowledge`
- `githooks` -> `infrastructure`

### R2: Remove all AGENT-*.md files (addresses GAP-AGENT-1, GAP-AGENT-2)

Delete all 17 AGENT-*.md files. The base roles live as YAML in `agile-workflow/roles/`. Domain specialists (Rust, Svelte, Tauri, Plugin Developer) should be knowledge declarations that get composed into task-specific agents at runtime, not static agent definitions.

Remove the `agent` schema and `agent.workflow.yaml` from core if agent artifacts are truly superseded by generated agents.

### R3: Deduplicate knowledge artifacts (addresses GAP-DUP-2, GAP-DUP-3, GAP-DUP-4)

- Merge the 3 duplicate knowledge pairs in agile-workflow into single authoritative files
- Fix duplicate `id: "thinking-mode-governance"` entries — each must have a unique ID
- Remove the dual-path knowledge_declarations in software-kanban — use only plugin-local paths

### R4: Fix missing files and references (addresses GAP-MISS-1, GAP-MISS-2, GAP-MAN-4)

- Create `plugins/software-kanban/knowledge/KNOW-3f307edb.md` from the installed copy
- Create `plugins/software-kanban/prompts/review-checklist.md` or remove the declaration
- Add contribution workflow to software-kanban's `provides.workflows`

### R5: Standardize manifest workflow format (addresses GAP-MAN-5, GAP-WF-2)

Update agile-documentation's `provides.workflows` from a flat string to the structured format used by all other plugins.

### R6: Standardize frontmatter field names (addresses GAP-MAN-8)

Pick one: `title` or `name`. Apply consistently across all schemas. Recommend `title` since it is used by the majority of schemas (10 vs 6).

### R7: Resolve AGENT-ae63c406 duplication (addresses GAP-DUP-1)

Immediate fix: remove the stale copy in agile-workflow (created 2026-03-14) and keep the updated copy in core (2026-03-24). Long-term fix: delete both per R2.

---

## Open Questions

1. **Should `core` be split?** Its dual nature (learning-stage workflow + framework artifact schemas) creates ambiguity. Splitting into `core-framework` (schemas, enforcement, base knowledge) and a learning-stage plugin would align with the architecture, but `core`'s `uninstallable: true` flag suggests it is intentionally unified. This is a design decision, not a clear gap.

2. **Are the 3 extra roles (Planner, Designer, Governance Steward) intentional?** The architecture documents 5 base roles. The codebase implements 8. If intentional, ARCHITECTURE.md section 6.1 should be updated. If not, the extra role files should be removed.

3. **Should agent artifacts survive as a concept?** Core defines an `agent` schema and workflow for managing agent configurations. Even if static AGENT-*.md files are removed, the schema may be useful for tracking dynamically-generated agent instances. This needs a design decision on whether agents are artifacts (tracked in the graph) or ephemeral (generated and discarded).
