# Phase 2: Governance Artifact Gap Analysis

**Date:** 2026-03-26
**Inputs:** Phase 1 inventories (02, 03, 04), ARCHITECTURE.md Section 5
**Scope:** `.orqa/` directory structure, artifact types, organization, frontmatter, naming

---

## 1. Directory Structure: Current vs Target

### Current Structure

```
.orqa/
  project.json
  manifest.json
  prompt-registry.json          # Not in target -- should this exist?
  search.duckdb

  connectors/                   # Not in target structure
    claude-code/
      injector-config.json
      hooks/scripts/enforce-background-agents.mjs

  process/                      # Extra nesting layer -- NOT in target
    agents/                     # 19 AGENT-*.md files -- SHOULD NOT EXIST
    decisions/                  # 70 AD-*.md files -- flat, no subcategorization
    knowledge/                  # 114 KNOW-*.md + 5 SKILL.md subdirectories
    lessons/                    # 84 IMPL-*.md files
    rules/                     # 59 RULE-*.md files
    workflows/                 # 14 workflow YAML files (source definitions)

  principles/
    vision/                    # 1 VISION file -- matches target
    pillars/                   # 3 PILLAR files -- matches target
    personas/                  # 3 PERSONA + 1 DOC (type mismatch)
    grounding/                 # 5 DOC files -- SHOULD NOT EXIST as directory

  documentation/
    (root)                     # 20 DOC files -- flat, no categorization
    platform/                  # 36 DOC files -- partial categorization
    project/                   # 31 DOC files -- partial categorization

  delivery/
    milestones/                # 3 files -- matches target
    epics/                     # 128 files -- matches target
    tasks/                     # 731 files -- matches target
    ideas/                     # 12 files -- matches target

  discovery/
    ideas/                     # 160 files -- matches target
    research/                  # 80 files -- matches target
    wireframes/                # 5 files -- matches target

  workflows/                   # 24 resolved YAML files
```

### Target Structure (from ARCHITECTURE.md 5.1)

```
.orqa/
  project.json
  manifest.json
  schema.composed.json          # MISSING -- does not exist
  search.duckdb

  workflows/                    # Resolved workflows, named by PURPOSE
  principles/                   # vision/, pillars/, personas/ (PERSONA type only)
  decisions/                    # Split: principles/ and planning/ subdirectories
  knowledge/                    # Categorized by domain subdirectories
  rules/
  documentation/                # Categorized by topic subdirectories
  lessons/
  delivery/                     # milestones/, epics/, tasks/, ideas/
  discovery/                    # ideas/, research/, wireframes/
```

### Gap Summary: Directory Structure

| Gap | Current | Target | Impact |
|-----|---------|--------|--------|
| `process/` nesting | All process artifacts nested under `.orqa/process/` | Artifact categories are top-level within `.orqa/` | HIGH -- extra nesting adds navigation friction, contradicts target |
| `agents/` directory exists | 19 AGENT-*.md files in `.orqa/process/agents/` | No `agents/` directory -- base roles live in methodology plugin; task agents generated at runtime | HIGH -- entire directory is legacy |
| `grounding/` directory exists | 5 DOC files in `.orqa/principles/grounding/` | No `grounding/` -- content becomes `tier: always` knowledge in appropriate plugin | MEDIUM -- content needs migration to knowledge |
| `connectors/` directory | Runtime config + hook scripts | Not in target structure | NEEDS CLARIFICATION -- connector output may belong elsewhere |
| `prompt-registry.json` exists | Pre-built prompt injection registry | Not in target structure | NEEDS CLARIFICATION -- may be a generated runtime artifact |
| `schema.composed.json` missing | Does not exist | Should exist as generated artifact | MEDIUM -- composition pipeline not yet producing this |
| Decisions not split | Flat `process/decisions/` with 70 files | `decisions/principles/` and `decisions/planning/` | MEDIUM -- needs content review and categorization |
| Knowledge not categorized | 114 files flat in `process/knowledge/` | Categorized by domain subdirectories | HIGH -- 114 hash-named files are not human-navigable |
| Documentation partially categorized | 20 root + 36 platform/ + 31 project/ | Categorized by topic subdirectories | MEDIUM -- platform/project is partial, root is uncategorized |

---

## 2. Artifact Type Mismatches

### DOC in personas/ directory

| File | Current Type | Expected Type | Location |
|------|-------------|---------------|----------|
| `DOC-1ff7a9ba.md` | `doc` | Should be `persona` or removed from personas/ | `.orqa/principles/personas/` |

The file uses `type: doc` and a `DOC-` prefix but sits in the `personas/` directory. Per ARCHITECTURE.md: "Personas directory contains only PERSONA artifacts (no DOC artifacts)." This file is a detailed user personas reference document that should either:
- Be converted to individual PERSONA artifacts (the 3 PERSONA files already exist and reference it)
- Be moved to `documentation/` as reference material

### Grounding docs should be knowledge

| File | Current Type | Should Be | Reason |
|------|-------------|-----------|--------|
| `DOC-a0490c49.md` | doc (grounding) | `tier: always` knowledge in a plugin | ARCHITECTURE.md: "grounding content becomes tier: always knowledge in the appropriate plugin" |
| `DOC-bdb520ae.md` | doc (grounding) | `tier: always` knowledge in a plugin | Same |
| `DOC-40b1498a.md` | doc (grounding) | `tier: always` knowledge in a plugin | Same |
| `DOC-ebf19a16.md` | doc (grounding) | `tier: always` knowledge in a plugin | Same |
| `DOC-0ea4c263.md` | doc (grounding) | `tier: always` knowledge in a plugin | Same |

These 5 files are role-specific grounding content (Product Purpose, Research Principles, Design Principles, Code Principles, Artifact Principles). They map to agent roles and should become knowledge artifacts with `tier: always` and appropriate `roles` field, installed via the relevant plugin.

### Wireframes typed as DOC

| File | Current Type | Expected Type |
|------|-------------|---------------|
| `DOC-6c91572c.md` | doc | wireframe |
| `DOC-65a3c4e8.md` | doc | wireframe |
| `DOC-93a0f6c1.md` | doc | wireframe |
| `DOC-4ac7f17a.md` | doc | wireframe |
| `DOC-796d7f01.md` | doc | wireframe |

All 5 wireframes in `.orqa/discovery/wireframes/` use `type: doc` with `DOC-` prefixes. They live in the wireframes directory and there is a `wireframe.resolved.yaml` workflow. These should use `type: wireframe` with `WIRE-` or similar prefixes to match their location and purpose.

---

## 3. Legacy Artifacts

### AGENT-*.md files (19 files) -- ENTIRE CATEGORY IS LEGACY

Per ARCHITECTURE.md Section 6.4:

> | Old Pattern | New Pattern |
> |------------|-------------|
> | Monolithic `AGENT-*.md` specialist definitions | Base roles + generated task-specific agents |

The entire `.orqa/process/agents/` directory contains 19 files that represent the old monolithic agent definition model. In the target architecture:

- **Base roles** (Orchestrator, Implementer, Reviewer, Researcher, Writer) are defined in the methodology plugin
- **Task-specific agents** are generated at runtime by the prompt pipeline
- **Specialist agents** (Rust Specialist, Svelte Specialist, etc.) are replaced by domain knowledge injection

**Files to remove/migrate:**

| Category | Files | Migration Path |
|----------|-------|---------------|
| Core base roles (8) | AGENT-4c94fe14 (Orchestrator), AGENT-e5dd38e4 (Implementer), AGENT-8e58cd87 (Reviewer), AGENT-e333508b (Researcher), AGENT-bbad3d30 (Writer), AGENT-85be6ace (Planner), AGENT-0aad40f4 (Designer), AGENT-d1be3776 (Installer) | Move to methodology plugin as base role definitions |
| Plugin specialists (6) | AGENT-065a25cc (Rust Specialist), AGENT-5de8c14f (Svelte Specialist), AGENT-26e5029d (Rust Standards), AGENT-6f55de0d (Svelte Standards), AGENT-65b56a0b (Tauri Standards), AGENT-7a06d10e (Governance Enforcer) | Replaced by domain knowledge injection at runtime |
| Plugin Developer (1) | AGENT-ce86fb50 | Move to plugin-dev plugin as a role variant |
| Governance Steward (1) | AGENT-ae63c406 | Move to methodology plugin as base role definition |
| Project-specific (3) | AGENT-336e4d7d (Integration), AGENT-867da593 (Rust Backend), AGENT-e5a1b6bf (Svelte Frontend) | Replaced by generated task-specific agents |

### SKILL.md files in knowledge/ (5 subdirectories)

| Directory | SKILL.md | Issue |
|-----------|----------|-------|
| `process/knowledge/diagnostic-methodology/` | Root cause analysis | These use an old skill format (name/description/user-invocable in YAML, body content). Not standard KNOW- artifacts. |
| `process/knowledge/governance-context/` | Artifact graph reading | Same |
| `process/knowledge/planning/` | Documentation-first planning | Same |
| `process/knowledge/plugin-setup/` | Plugin setup for Claude Code | Same |
| `process/knowledge/search/` | Unified MCP search | Same |

Per the user's feedback memories, skills and knowledge are separate concerns:
- `skills/` = user-facing commands (connector output)
- `knowledge/` = agent-internal domain information

These SKILL.md files in knowledge/ conflate the two. The content should either:
- Become standard KNOW- knowledge artifacts (if agent-internal)
- Move to the connector's generated skill definitions (if user-facing commands)

---

## 4. Organizational Problems

### Flat hash-named files -- not human-navigable

Per ARCHITECTURE.md Section 12 audit criterion 9: "Hash-only filenames in flat directories are not navigable."

| Directory | File Count | Format | Problem |
|-----------|-----------|--------|---------|
| `process/knowledge/` | 114 | `KNOW-<8hex>.md` | Cannot browse by topic. Must open files to find what you need. |
| `process/decisions/` | 70 | `AD-<8hex>.md` | Cannot distinguish principle decisions from planning decisions |
| `process/rules/` | 59 | `RULE-<8hex>.md` | Cannot browse by enforcement area |
| `process/lessons/` | 84 | `IMPL-<8hex>.md` | Cannot browse by topic or maturity |
| `documentation/` (root) | 20 | `DOC-<8hex>.md` | Uncategorized, sitting alongside categorized platform/ and project/ |

The target architecture calls for:
- **Knowledge:** categorized by domain subdirectories
- **Decisions:** split into principles/ and planning/ subdirectories
- **Documentation:** categorized by topic subdirectories

### Documentation subcategorization incomplete

Current state:
- `documentation/` (root): 20 files -- NO subcategorization
- `documentation/platform/`: 36 files -- partial categorization (by layer)
- `documentation/project/`: 31 files -- partial categorization (by layer)

The root 20 files include a mix of categories (how-to, governance, architecture, onboarding, reference, plugin). They should be incorporated into a proper topic-based categorization scheme.

The platform/project split is reasonable but incomplete -- the target says "categorized by topic" which would mean subcategories like `documentation/architecture/`, `documentation/how-to/`, `documentation/reference/`, etc.

---

## 5. Frontmatter Issues

### `name` vs `title` inconsistency

Two naming conventions exist across artifact types:

| Convention | Where Used | Examples |
|------------|-----------|----------|
| `title` field | Most artifacts (epics, tasks, decisions, agents, pillars, personas, vision) | Standard |
| `name` field | Some docs, some knowledge, SKILL.md files | DOC-2372ed36, DOC-4554ff3e, DOC-db794473, DOC-743f9c71, DOC-7b9b45f0, DOC-ae447f88, DOC-e42efeaf |
| Both `name` and `title` | Some knowledge files | Inconsistent |

ARCHITECTURE.md Section 12 criterion 6 requires: "id, type, title, description, relationships". The canonical field is `title`. Files using `name` instead need to be migrated.

### Missing `status` field

Many documentation files lack a `status` field entirely:

- Some have `status: captured`
- Some have `status: active`
- Many have no status at all

Per the artifact lifecycle workflows (`doc.workflow.yaml`), all docs should have a status with initial state `captured`. Missing status means the artifact is outside the workflow system.

### Inconsistent YAML quoting

Status values appear both quoted (`"completed"`) and unquoted (`completed`) across epics and tasks. While both parse correctly, this inconsistency suggests different authoring sources (manual vs automated) and should be normalized.

### Knowledge frontmatter schema inconsistency

Knowledge files use wildly inconsistent schemas:

| Field Set | Count (approx) | Description |
|-----------|----------------|-------------|
| New format | ~40 | Has `tier`, `roles`, `paths`, `tags`, `priority`, `summary` |
| Old format | ~60 | Missing injection metadata entirely |
| Mixed | ~14 | Has some but not all injection fields |

The target architecture requires knowledge to have injection metadata for the prompt pipeline. Files without `tier`, `roles`, `paths`, and `tags` cannot participate in intelligent knowledge injection.

---

## 6. Resolved Workflows: Current vs Target Naming

### Current resolved workflows (24 files)

Named by artifact type, one per artifact type managed by each plugin:

```
agile-methodology.resolved.yaml    epic.resolved.yaml
delivery.resolved.yaml             milestone.resolved.yaml
idea.resolved.yaml                 task.resolved.yaml
research.resolved.yaml             discovery-idea.resolved.yaml
discovery-decision.resolved.yaml   discovery-research.resolved.yaml
persona.resolved.yaml              pillar.resolved.yaml
pivot.resolved.yaml                vision.resolved.yaml
planning-decision.resolved.yaml    planning-idea.resolved.yaml
planning-research.resolved.yaml    wireframe.resolved.yaml
agent.resolved.yaml                decision.resolved.yaml
doc.resolved.yaml                  knowledge.resolved.yaml
lesson.resolved.yaml               rule.resolved.yaml
```

### Target resolved workflows (from ARCHITECTURE.md 5.1)

Named by PURPOSE (methodology stage), not by artifact type:

```
methodology.resolved.yaml          # The full resolved methodology
discovery.resolved.yaml            # Resolved discovery workflow
planning.resolved.yaml             # Resolved planning workflow
documentation.resolved.yaml        # Resolved documentation workflow
implementation.resolved.yaml       # Resolved implementation workflow
review.resolved.yaml               # Resolved review workflow
learning.resolved.yaml             # Resolved learning workflow
```

### Gap

| Issue | Details |
|-------|---------|
| **Naming convention** | Current names by artifact type (24 files). Target names by purpose (7 files). |
| **Granularity** | Current has one resolved file per artifact type. Target has one per methodology stage. |
| **Lifecycle workflows** | Current has standalone lifecycle workflows (agent, decision, doc, knowledge, lesson, rule). Target does not list these separately. |
| **Missing purpose names** | No `implementation.resolved.yaml`, no `learning.resolved.yaml`, no `methodology.resolved.yaml` |
| **Name conflicts** | `delivery.resolved.yaml` exists but target expects no file by that name |

This is likely an engine-level change (how `orqa install` names resolved outputs), not a manual file rename. The current naming reflects the actual composition model -- artifact-level resolution -- while the target describes a higher-level purpose-based view.

---

## 7. Content That Should Not Exist

### Duplicated content

| Type | Description | Evidence |
|------|-------------|----------|
| Grounding docs duplicating knowledge | 5 grounding DOC files overlap with existing KNOW- files (e.g., artifact principles exist as both DOC-0ea4c263 grounding and KNOW-e3432947 Plugin-Canonical Architecture) | Content redundancy between principles/grounding/ and process/knowledge/ |
| DOC personas duplicating PERSONA artifacts | DOC-1ff7a9ba (User Personas doc) contains the same information as PERSONA-c4afd86b + PERSONA-477971bf + PERSONA-2721ae35 | The 3 PERSONA files are brief summaries that point to the DOC |
| Documentation DOC files duplicating knowledge | Some DOC files cover the same topic as KNOW- files (e.g., DOC-586bfa9a "Knowledge Auto-Injection" and KNOW-586bfa9a share the same hash) | Same content exists as both doc and knowledge artifact |

### Potentially obsolete artifacts

| Artifact | Reason |
|----------|--------|
| 5 wireframe DOC files | All marked with FRESHNESS NOTEs documenting staleness. Some "significantly outdated." |
| `prompt-registry.json` | 3,654 lines. Not in target structure. May be superseded by runtime prompt pipeline. |
| `connectors/` directory | May need restructuring if connector generates output elsewhere |
| SKILL.md files in knowledge subdirectories | Legacy skill format conflating knowledge and user-facing commands |

---

## 8. Decision Artifact Organization

### Current state

70 `AD-*.md` files in a flat directory (`process/decisions/`), all typed `discovery-decision`.

### Target state

Split into two levels:

```
decisions/
  principles/    # High-level: overarching architecture and approaches
  planning/      # Implementation-level: how to solve specific categories of problems
```

### Classification needed

From the sampled decisions, rough categorization:

| Category | Examples | Approx Count |
|----------|---------|--------------|
| **Principle decisions** (architecture/approach) | AD-e711446e (Vision Evolution), AD-2d58941b (Error Propagation via Result Types), AD-c6c2d9fb (Rule promotion requires enforcement), AD-8727f99a (tmp to .state rename) | ~30-40 |
| **Planning decisions** (implementation/tactical) | AD-45cfe1d1 (Config-Driven Artifact Scanning), AD-7fa3f280 (Task-First Audit Trail), AD-fc4e9013 (Max Subscription Auth) | ~30-40 |

A full content review of all 70 decisions is needed to classify them. The type field may also need updating -- `discovery-decision` is the current type for all, but the target may need `principle-decision` and `planning-decision` as distinct types.

---

## 9. Knowledge Organization

### Current state

114 KNOW-*.md files in a flat directory with 5 SKILL.md subdirectories. No categorization by domain.

### Target state

Categorized by domain subdirectories under `knowledge/`. Based on the inventory, these domain categories emerge:

| Proposed Category | Example Files | Approx Count |
|-------------------|--------------|--------------|
| `platform/` | Core framework, artifact graph, enforcement, governance, workflows | ~25 |
| `development/rust/` | Rust patterns, testing, clippy, Tauri | ~10 |
| `development/frontend/` | Svelte 5, stores, component patterns | ~10 |
| `development/typescript/` | TypeScript patterns, plugin skills | ~5 |
| `methodology/` | Planning, research methodology, delivery discipline, thinking modes | ~15 |
| `documentation/` | Documentation authoring, README standards | ~5 |
| `plugin/` | Plugin development, plugin-canonical architecture | ~5 |
| `integration/` | IPC, CLI, search, streaming, hooks | ~10 |
| `standards/` | Coding standards, naming conventions, design system | ~10 |
| `project/` | Project-specific architecture, component tree, module architecture | ~19 |

A full content review is needed to accurately categorize all 114 files. Many knowledge files have `tags` fields that could inform automatic categorization.

### Injection metadata gaps

For the prompt pipeline to work correctly, every knowledge file needs:
- `tier`: always / stage-triggered / on-demand
- `roles`: which agent roles receive this knowledge
- `paths`: file path patterns that trigger injection
- `tags`: semantic tags for search
- `summary`: compressed summary for token-efficient injection
- `priority`: P0-P3 for token budgeting

Currently ~40 files have the new format, ~60+ are missing injection metadata entirely.

---

## TARGET .orqa/ STRUCTURE

Based on the gap analysis, here is the proposed target directory tree with migration notes:

```
.orqa/
  project.json                         # KEEP as-is
  manifest.json                        # KEEP as-is
  schema.composed.json                 # CREATE -- generated by composition pipeline
  search.duckdb                        # KEEP as-is

  # --- Generated artifacts ---

  workflows/                           # KEEP location, CHANGE naming convention
    methodology.resolved.yaml          # RENAME from agile-methodology.resolved.yaml
    discovery.resolved.yaml            # CONSOLIDATE discovery artifact workflows
    planning.resolved.yaml             # CONSOLIDATE planning artifact workflows
    documentation.resolved.yaml        # NEW (currently no separate file)
    implementation.resolved.yaml       # CONSOLIDATE epic/task/milestone workflows
    review.resolved.yaml               # NEW (currently no separate file)
    learning.resolved.yaml             # CONSOLIDATE lesson/decision/knowledge/rule workflows

  # --- User-authored principles ---

  principles/
    vision/                            # KEEP -- 1 file, matches target
      VISION-4893db55.md
    pillars/                           # KEEP -- 3 files, match target
      PILLAR-c9e0a695.md
      PILLAR-2acd86c1.md
      PILLAR-a6a4bbbb.md
    personas/                          # FIX -- remove DOC-1ff7a9ba, keep 3 PERSONA files
      PERSONA-c4afd86b.md
      PERSONA-477971bf.md
      PERSONA-2721ae35.md

  # --- Process artifacts (promoted from process/ nesting) ---

  decisions/                           # MOVE from process/decisions/, SPLIT into:
    principles/                        # Architecture/approach decisions (~30-40 files)
      AD-e711446e.md                   # Example: Vision Evolution
      AD-2d58941b.md                   # Example: Error Propagation
      ...
    planning/                          # Implementation/tactical decisions (~30-40 files)
      AD-45cfe1d1.md                   # Example: Config-Driven Scanning
      AD-fc4e9013.md                   # Example: Auth approach
      ...

  knowledge/                           # MOVE from process/knowledge/, CATEGORIZE into:
    platform/                          # Core framework, governance, enforcement
    development/                       # Rust, Svelte, TypeScript, Tauri patterns
    methodology/                       # Planning, research, thinking modes
    standards/                         # Coding standards, naming, design system
    plugin/                            # Plugin development, architecture
    project/                           # Project-specific architecture knowledge

  rules/                               # MOVE from process/rules/ -- 59 files
    RULE-*.md

  documentation/                       # RESTRUCTURE from flat + platform/ + project/
    architecture/                      # Architecture docs from all 3 current locations
    reference/                         # Reference docs
    how-to/                            # How-to guides
    onboarding/                        # Getting started, setup guides
    concept/                           # Conceptual docs
    platform/                          # Platform-specific docs (thinking modes etc)

  lessons/                             # MOVE from process/lessons/ -- 84 files
    IMPL-*.md

  # --- Delivery tracking ---

  delivery/                            # KEEP as-is
    milestones/                        # 3 files
    epics/                             # 128 files
    tasks/                             # 731 files
    ideas/                             # 12 files

  # --- Discovery work ---

  discovery/                           # KEEP as-is
    ideas/                             # 160 files
    research/                          # 80 files
    wireframes/                        # 5 files (FIX type from doc to wireframe)
```

### What Gets Removed

| Item | Reason | Migration |
|------|--------|-----------|
| `process/` directory | Extra nesting layer not in target | Contents promoted to top-level `.orqa/` |
| `process/agents/` (19 files) | Legacy monolithic agent definitions | Base roles -> methodology plugin. Specialists -> domain knowledge. Project-specific -> runtime generation. |
| `principles/grounding/` (5 files) | Grounding content -> knowledge | Convert to `tier: always` knowledge artifacts in appropriate plugins |
| `DOC-1ff7a9ba.md` in personas/ | Wrong type for location | Move to documentation/ or delete (content exists in 3 PERSONA files) |
| 5 SKILL.md subdirectories in knowledge/ | Legacy skill format | Convert to standard KNOW- files or move to connector skill generation |
| `prompt-registry.json` | Not in target structure | Clarify: generated runtime artifact or replaced by pipeline? |
| `connectors/` directory | Connector output structure TBD | May move to generated plugin output directory |

### What Gets Created

| Item | Reason |
|------|--------|
| `schema.composed.json` | Generated composed schema for LSP/MCP validation |
| `decisions/principles/` | Subcategorization of architecture decisions |
| `decisions/planning/` | Subcategorization of planning decisions |
| `knowledge/<domain>/` subdirectories | Categorized knowledge for human navigation |
| `documentation/<topic>/` subdirectories | Topic-based documentation organization |

---

## Open Questions

1. **prompt-registry.json** -- Is this a generated runtime artifact that should exist but isn't in the target structure? Or is it replaced by the runtime prompt pipeline? It has 3,654 lines and 177 knowledge entries.

2. **connectors/ directory** -- Where should connector runtime config live in the target structure? The connector generates output (tool-native plugin) but also needs runtime state (injector-config.json, hook scripts).

3. **Resolved workflow naming** -- The target shows 7 purpose-named files, but the current composition model produces 24 artifact-type-named files. Is this a change in the composition engine, or should the current naming be considered correct and the target updated?

4. **Knowledge categorization scheme** -- The proposed domain categories are derived from content analysis. Should the categories match plugin names (since plugins install knowledge), or be independent semantic categories?

5. **Decision type splitting** -- Should `discovery-decision` be split into `principle-decision` and `planning-decision` as distinct artifact types with their own workflows? Or is this purely a directory organization change with the same type?

6. **Source workflow definitions** -- Currently in `process/workflows/` (14 YAML files). These are plugin-sourced contribution/lifecycle definitions, not resolved workflows. Where should these live in the target? They are not the same as the resolved `.resolved.yaml` files in `workflows/`.

7. **Wireframe artifact type** -- There is a `wireframe.resolved.yaml` workflow, but wireframe files use `type: doc`. Does the wireframe type exist in the schema? If not, the workflow is unreachable.
