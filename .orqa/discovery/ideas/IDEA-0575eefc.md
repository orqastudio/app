---
id: IDEA-0575eefc
title: "Artifact migration: location audit, content cleanup, plugin categories"
status: exploring
description: "Audit artifact locations, move to correct plugins, rewrite content to be domain-portable, add plugin category schema (core/enhancement/extension), deduplicate."
created: "2026-03-22"
updated: "2026-03-22"
relationships:
  - target: "RULE-130f1f63"
    type: "related"
    rationale: "Data integrity requires consistent, traceable IDs"
---

# Artifact Migration: Location Audit + Content Cleanup

## Plugin Category Schema

Each plugin declares a `role` in addition to its existing `category`:

| Role | Meaning | Examples |
|------|---------|---------|
| `core:framework` | The agent execution model — universal roles, delegation, sessions, knowledge loading. Every project gets this. | **orqa-core** (NEW) |
| `core:governance` | The governance lifecycle — rules, decisions, lessons, enforcement, integrity | agile-governance |
| `core:discovery` | The reasoning methodology — systems thinking, pillar/vision alignment, thinking modes | systems-thinking |
| `core:delivery` | The delivery lifecycle — milestones, epics, tasks, plan compliance | software |
| `enhancement:delivery` | Extends delivery with domain-specific tooling and coding standards | coding-standards, rust, svelte, tauri, typescript |
| `extension` | General functional addition, not lifecycle-specific | cli, claude-code connector |

**orqa-core** is the operating system. The other core plugins plug into it. You could
swap agile-governance for a different governance model, or systems-thinking for a
different discovery methodology, but orqa-core is always present.

Core plugins can reference the general categories "Discovery", "Delivery", "Governance"
but MUST NOT reference other plugins' specific artifact types or OrqaStudio-specific paths.

## Thinking Mode Tagging

Knowledge artifacts that serve as thinking modes add a frontmatter field:

```yaml
thinking-mode: implementation   # maps to the mode template key
```

This decouples mode detection from filename convention. The prompt-injector
searches semantically via ONNX embeddings, finds the best match regardless
of which plugin the artifact lives in, and injects the corresponding mode
template. Thinking modes live with their owning plugin:

| Thinking Mode | Plugin | Rationale |
|--------------|--------|-----------|
| implementation | orqa-core | Generic how-to-implement |
| debugging | orqa-core | Generic how-to-debug |
| review | orqa-core | Generic how-to-review |
| learning-loop | agile-governance | Governance learning cycle |
| dogfood-implementation | systems-thinking | Self-referential methodology |
| planning | systems-thinking | Discovery methodology |
| research | systems-thinking | Discovery methodology |
| documentation | systems-thinking | Discovery methodology |

## Onboarding Tag

Knowledge artifacts used during project setup add a frontmatter tag:

```yaml
onboarding: true
```

During `orqa install` or project onboarding workflows, the installer can query
for all knowledge artifacts tagged `onboarding: true` across installed plugins
to build a project-type-appropriate setup flow. For example, installing the
software plugin brings in "Project Type: Software" knowledge which guides
milestone/epic/task structure creation.

## Content Portability Rules

1. **No `.orqa/` path references** — use "project governance directory" or similar
2. **No specific artifact IDs** (AD-xxx, RULE-xxx) — reference concepts not IDs
3. **No OrqaStudio CLI commands** (orqa validate, orqa graph) — reference generic actions
4. **No project-specific config** (core.json, project.json) — these are app-level
5. **CAN reference** general categories: "Discovery artifacts", "Delivery artifacts", "Governance artifacts"
6. **CAN reference** generic types: "rules", "knowledge", "agents", "decisions", "lessons" (these are the governance vocabulary, not project-specific)

## Full Artifact Location Audit

### Agents

| Current ID | Title | Current Location | Action |
|-----------|-------|-----------------|--------|
| AGENT-1dab5ebe | Orchestrator | agile-governance | **MOVE → orqa-core** (framework execution model) |
| AGENT-cc255bc8 | Implementer | agile-governance | **MOVE → orqa-core** |
| AGENT-b0774726 | Reviewer | agile-governance | **MOVE → orqa-core** |
| AGENT-fb0ce261 | Researcher | agile-governance | **MOVE → orqa-core** |
| AGENT-caff7bc1 | Planner | agile-governance | **MOVE → orqa-core** |
| AGENT-ec1b3785 | Writer | agile-governance | **MOVE → orqa-core** |
| AGENT-c5284fde | Designer | agile-governance | **MOVE → orqa-core** |
| AGENT-ff44f841 | Governance Steward | agile-governance | STAY (governance-specific) |
| AGENT-bedeffd1 | Installer | agile-governance | **MOVE → orqa-core** (framework role) |
| AGENT-e7f3a2c9 | Enforcer | agile-governance | STAY + RENAME to "Governance Enforcer" |
| AGENT-b2f574e5 | Svelte Specialist | svelte | STAY |
| AGENT-b0857607 | Svelte Standards | svelte | STAY |
| AGENT-e1e47559 | Rust Specialist | rust | STAY |
| AGENT-4241392c | Rust Standards | rust | STAY |

### Rules — agile-governance (currently 20)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-532100d9 | Agent Delegation | **MOVE → orqa-core** (framework execution model) |
| RULE-7b770593 | Artifact Lifecycle | **REVIEW** — generic status transitions STAY, governance-specific parts SPLIT |
| RULE-9daf29c0 | Documentation-First | **MOVE → orqa-core** (framework methodology) |
| RULE-3eccebf3 | Enforcement Before Code | STAY (governance enforcement pattern) |
| RULE-57ccb4a3 | Error Ownership | STAY (governance discipline) |
| RULE-633e636d | Git Workflow | **MOVE → future git-integration plugin** |
| RULE-6d1c8dc7 | Historical Preservation | **MOVE → orqa-core** (framework archival methodology) |
| RULE-878e5422 | Honest Reporting | STAY (governance discipline) |
| RULE-22783309 | IDs Are Not Priority | STAY (governance convention) |
| RULE-551bde31 | Lessons Learned | STAY (governance learning loop) |
| RULE-e120bb70 | No Deferred Deliverables | STAY (governance enforcement) |
| RULE-e9c54567 | No Stubs or Placeholders | **MOVE → coding-standards** (code-level rule) |
| RULE-b2753bad | Required Reading | **MOVE → orqa-core** (framework discipline) |
| RULE-8035e176 | Structure Before Work | **MOVE → orqa-core** (framework methodology) |
| RULE-2f7b6a31 | Artifact Link Format | **MERGE with Artifact Lifecycle** if possible, else STAY |
| RULE-f809076f | Tool Access Restrictions | **MOVE → orqa-core** (framework agent model) |
| RULE-e352fd0a | Session Management | **MOVE → orqa-core** (framework methodology) |
| RULE-98682b5e | Core Graph Firmware Protection | STAY (governance protection) |
| RULE-130f1f63 | Data Integrity | **REVIEW** — STAY if generic cross-reference integrity. SPLIT if OrqaStudio-specific |
| RULE-9bc8c230 | Behavioral Rule Enforcement Plan | STAY (governance enforcement) |

### Rules — systems-thinking (currently 7)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-9ba80a19 | No Aliases or Hacks | **MOVE → coding-standards** (code-level rule) |
| RULE-39169bcd | Pillar Alignment in Documentation | STAY (discovery methodology) |
| RULE-1f30904a | Root Directory Cleanliness | **MOVE → coding-standards** (file structure rule) |
| RULE-d90112d9 | Systems Thinking First | STAY (core methodology) |
| RULE-4d4f540d | UAT Process | **MOVE → software** (delivery process) |
| RULE-1e8a1914 | Vision Alignment | STAY (discovery methodology) |
| RULE-a764b2ae | Artifact Schema Compliance | **MOVE → agile-governance** (governance enforcement) |

### Rules — software (currently 7)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-6c0496e0 | Artifact Config Integrity | **REVIEW** — may be redundant with well-enforced schemas. Remove if so |
| RULE-303c1cc8 | Plan Mode Compliance | STAY (delivery planning) |
| RULE-deab6ea7 | Skill Enforcement | **MOVE → orqa-core** (framework knowledge model) |
| RULE-11c29c9e | Skill Portability | **MOVE → orqa-core** (framework knowledge model) |
| RULE-df24948b | Context Window Management | **MOVE → orqa-core** (framework agent methodology) |
| RULE-5ee43922 | User-Invocable Knowledge Semantics | **MOVE → orqa-core** (framework knowledge model) |
| RULE-92dba0cb | Provider-Agnostic Tool Capabilities | **MOVE → orqa-core** (framework agent model) |

### Rules — dev environment (.orqa/process/rules/) — 19 rules, 14 should move to plugins

Rules that enforce plugin-specific behaviour must travel with their plugin.

**Enforcement authority**: The app is the authority, connectors are adapters
(see [IDEA-a93e9261](IDEA-a93e9261)). Rules declare WHAT is enforced, not HOW.
The app's enforcement engine handles in-app agents natively. Connectors map
the same rules to their native hook systems. Rules do NOT need `requires: connector`.

Rules enforced via a language tool (clippy, ESLint) should live in OR require
the tool's plugin — those tools are the enforcement mechanism, not the connector.

**Enforcement method key:**
- `behavioral` — prompt injection (app: system prompt builder, connector: UserPromptSubmit hook)
- `hook` — action validation (app: enforcement engine, connector: PreToolUse/PostToolUse)
- `lint` — linter rule (provided by language plugin: rust, svelte, typescript)
- `review` — code reviewer agent check
- `tool` — CLI tool (orqa validate, orqa check — lives with CLI plugin)
- `none` — not yet mechanically enforced (enforcement gap)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-12e74734 | Enforcement Gap Priority | **MOVE → agile-governance** | `none` — enforcement gap itself! |
| RULE-9cd980b1 | Honest Status Reporting | **MOVE → agile-governance** | `behavioral` — prompt injection |
| RULE-c4fe67a2 | Governance Priority Over Delivery | **MOVE → agile-governance** | `behavioral` + `hook` (stop hook escalation). App enforces natively; connector adapts to native hooks |
| RULE-67b91c13 | Trace Artifacts to Usage Contexts | **MOVE → agile-governance** | `none` — enforcement gap |
| RULE-029db175 | Continuous Operation | **MOVE → orqa-core** | `behavioral` — prompt injection |
| RULE-4f7e2a91 | Real-time Session State Management | **MOVE → orqa-core** | `behavioral` + `hook`. App enforces natively; connector adapts to native hooks |
| RULE-f9d0279c | Automated Knowledge Injection | **MOVE → orqa-core** | `hook` (PostToolUse file write). App enforces natively; connector adapts to native hooks |
| RULE-5e03e67b | Code Search Usage | **MOVE → orqa-core** | `behavioral` — agent prompt. Search provided by orqa-core |
| RULE-4263a6b3 | Pre-Release Version Tagging | **MOVE → cli** | `tool` (orqa version check). Rule lives WITH the tool |
| RULE-89155a7f | Tooltips over title attributes | **MOVE → coding-standards** | `review` + potential `lint` (ESLint rule via svelte plugin) |
| RULE-cb65b5d0 | Reusable Components | **MOVE → coding-standards** | `review` + `hook` (knowledge injection on component writes). App enforces natively; connector adapts to native hooks |
| RULE-f10bb5de | Testing Standards | **MOVE → coding-standards** | `hook` (pre-commit runs make test). Language plugins provide test runners |
| RULE-7f416d7d | Tooling Ecosystem Management | **MOVE → coding-standards** | `lint` delegation — rule documents which linter enforces what. Language plugins provide linters |
| RULE-1acb1602 | End-to-End Completeness | **MOVE → software** | `review` + `hook` (pre-commit). OrqaStudio-specific (four-layer Tauri stack). **NEEDS REWRITE** to be generic "all layers must be updated together" |
| RULE-65973a88 | Architecture Decisions | **SPLIT** — generic "decisions are first-class artifacts" → agile-governance; OrqaStudio AD source of truth details → STAY |
| RULE-b49142be | Coding Standards | **SPLIT** — generic enforcement discipline (run linters, no disabling rules, lint-rule alignment) → coding-standards; OrqaStudio Rust+Svelte+TS specifics → STAY |
| RULE-c71f1c3f | Development Commands | STAY (OrqaStudio Makefile targets — purely project-specific) |
| RULE-6083347d | Dogfood Mode | **SPLIT** — generic "editing the app you're running inside" methodology → orqa-core; OrqaStudio-specific restart/sidecar rules → STAY |
| RULE-c95f4444 | Data Persistence Boundaries | STAY (OrqaStudio SQLite/file/ephemeral split — purely project-specific) |

### Knowledge — agile-governance (currently 11)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-e4b91f37 | Enforcement Patterns | STAY | GENERIC — teaches methodology |
| KNOW-6f33713e | Planning | **MOVE → orqa-core** (framework methodology) | NEEDS REWRITE — has .orqa/ paths, AD IDs |
| KNOW-f7476f0a | Research Methodology | **MOVE → orqa-core** (framework methodology) | GENERIC |
| KNOW-f5edb34d | Diagnostic Methodology | **MOVE → orqa-core** (framework methodology) | GENERIC |
| KNOW-8d76c3c7 | Governance Maintenance | STAY but REWRITE | NEEDS REWRITE — mix of generic + OrqaStudio-specific |
| KNOW-8d1c4be6 | Plugin Artifact Usage | STAY | REVIEW — may be OrqaStudio-specific |
| KNOW-449b1e02 | Artifact Status Management | **MOVE → orqa-core** (framework status model) | GENERIC — portable status model |
| KNOW-eea50a65 | Governance Patterns | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — .orqa/ structure, core.json |
| KNOW-4368d782 | Artifact Audit Methodology | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — schema-driven audit |
| KNOW-250d5d6f | Naming Conventions | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — GitHub repo naming, npm scopes |
| KNOW-b08d355c | Schema Validation | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — core.json validation |

### Knowledge — systems-thinking (currently 14)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-54823e2d | Systems Thinking | STAY | GENERIC |
| KNOW-30a419dd | Architectural Evaluation | STAY | GENERIC |
| KNOW-82d32398 | Artifact Relationships | STAY | GENERIC |
| KNOW-f0c40eaf | Composability | STAY | GENERIC |
| KNOW-8c98ea98 | Restructuring Methodology | STAY | GENERIC |
| KNOW-c7fb7c83 | Tech Debt Management | STAY | GENERIC |
| KNOW-323c2803 | Thinking Mode: Debugging | **MOVE → orqa-core** + add `thinking-mode: debugging` | MOSTLY GENERIC |
| KNOW-1ab0e715 | Thinking Mode: Documentation | STAY + add `thinking-mode: documentation` | MOSTLY GENERIC |
| KNOW-a4c8f1e2 | Thinking Mode: Dogfood Implementation | STAY + add `thinking-mode: dogfood-implementation` | GENERIC |
| KNOW-fda0559b | Thinking Mode: Implementation | **MOVE → orqa-core** + add `thinking-mode: implementation` | MOSTLY GENERIC |
| KNOW-85e392ea | Thinking Mode: Learning Loop | **MOVE → agile-governance** + add `thinking-mode: learning-loop` | MOSTLY GENERIC |
| KNOW-de25b290 | Thinking Mode: Planning | STAY + add `thinking-mode: planning` | MOSTLY GENERIC |
| KNOW-1a8eb147 | Thinking Mode: Research | STAY + add `thinking-mode: research` | MOSTLY GENERIC |
| KNOW-83614358 | Thinking Mode: Review | **MOVE → orqa-core** + add `thinking-mode: review` | MOSTLY GENERIC |

### Knowledge — software (currently 14)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-1d47d8d8 | Software Delivery | STAY but REWRITE | ORQASTUDIO-ONLY — references .orqa/delivery/, artifact IDs. Extract generic delivery pattern |
| KNOW-e2354dce | Epic Completion | STAY but REWRITE | ORQASTUDIO-ONLY — references .orqa/, orqa validate. Extract generic "delivery unit completion" |
| KNOW-b453410f | Plugin Development | **MOVE → orqa-core** (framework infrastructure) | NEEDS REWRITE — remove OrqaStudio-specific paths, make generic |
| KNOW-e1333874 | First-Party Plugin Dev | **MOVE → orqa-core** (framework infrastructure) | NEEDS REWRITE — remove orqastudio-dev/ refs |
| KNOW-63cc1a00 | Third-Party Plugin Dev | **MOVE → orqa-core** (framework infrastructure) | NEEDS REWRITE — remove project.json specifics |
| KNOW-a2b3c4d5 | Search | **MOVE → orqa-core** (framework infrastructure) | NEEDS REWRITE — make search methodology generic |
| KNOW-2c8eead6 | Skills Maintenance | **ARCHIVE** — reactivate when skills.sh integration lands | See IDEA for skills.sh |
| KNOW-f0efaf83 | Code Quality Review | STAY | GENERIC |
| KNOW-353a228b | Component Extraction | STAY but minor rewrite | MOSTLY GENERIC — remove TASK-e752886d reference |
| KNOW-1b805150 | QA Verification | STAY | GENERIC |
| KNOW-170c220e | Security Audit | STAY | GENERIC |
| KNOW-bcb42347 | Test Engineering | STAY | GENERIC |
| KNOW-c6d04755 | UAT Process | STAY | GENERIC |
| KNOW-5124e508 | UX Compliance Review | STAY | GENERIC |

### Knowledge — connector (currently 6)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-3155cdaa | Decision Tree | **SPLIT** → generic "Agent Decision Methodology" in orqa-core + Claude Code-specific tree in dev .orqa/ | Extract reasoning pattern; keep CC tool names/mappings project-level |
| KNOW-b1593311 | Implementer Tree | **SPLIT** → generic "Implementer Reasoning" in orqa-core + CC-specific tree in dev .orqa/ | Same split pattern |
| KNOW-08fcd847 | Reviewer Tree | **SPLIT** → generic "Reviewer Reasoning" in orqa-core + CC-specific tree in dev .orqa/ | Same split pattern |
| KNOW-e3a559c9 | Plugin Setup | **MOVE → orqa-core** + add `onboarding: true` | NEEDS REWRITE — make connector-agnostic |
| KNOW-82ceb1bd | Project Inference | **SPLIT** → generic "Project Type Detection" in orqa-core (`onboarding: true`) + CC-specific inference rules in dev .orqa/ | Generic: file signature → project type. Specific: .claude/ detection |
| KNOW-0fd23e0b | Project Migration | **SPLIT** → generic "Governance Migration Methodology" in orqa-core + CC-specific .claude/ → .orqa/ steps in dev .orqa/ | Generic: format migration methodology. Specific: Claude Code paths |
| KNOW-e0dec720 | Project Setup | **MOVE → orqa-core** + add `onboarding: true` | NEEDS REWRITE — make connector-agnostic |
| KNOW-819789ab | Project Type: Software | **MOVE → software** + add `onboarding: true` | Used during project onboarding to configure software delivery |

### Knowledge — dev environment (.orqa/process/knowledge/) — 16 project-specific

All STAY — these are OrqaStudio implementation knowledge (architecture, IPC, stores, streaming, etc.)

### Documentation — agile-governance/documentation/ (36 files)

Platform docs migrated from app/.orqa/documentation/platform/. These describe the OrqaStudio
platform specifically. **REVIEW** — most should **MOVE → dev .orqa/documentation/** since they
describe OrqaStudio's implementation, not portable governance methodology.

## Deduplication Candidates

| Artifact A | Artifact B | Overlap | Action |
|-----------|-----------|---------|--------|
| governance-maintenance | orqa-artifact-audit | Both teach auditing | Extract generic custodianship → governance-maintenance. Move OrqaStudio audit → dev .orqa/ |
| artifact-status-management | Artifact Lifecycle rule | Both define status vocabulary | No duplication — knowledge teaches, rule enforces. Good separation |
| planning.md | thinking-mode-planning | Planning methodology vs mode detection | No duplication — complementary |
| enforcement-patterns | Behavioral Rule Enforcement Plan rule | Methodology vs enforcement list | No duplication — knowledge teaches patterns, rule lists enforcement strategies |
| Artifact Link Format rule | Artifact Lifecycle rule | Both about artifact conventions | **MERGE candidate** — link format is a subset of lifecycle conventions |

## Steps

1. [ ] User reviews this table — confirms moves, merges, and rewrites
2. [ ] Create orqa-core plugin (new repo, submodule) — the framework operating system
3. [ ] Add `role` field to orqa-plugin.json schema (core:framework | core:governance | core:discovery | core:delivery | enhancement:delivery | extension)
4. [ ] Move universal agents + framework rules/knowledge to orqa-core
5. [ ] Move coding rules (no stubs, no aliases, root cleanliness, tooltips) to coding-standards
6. [ ] Move OrqaStudio-only knowledge to dev .orqa/
7. [ ] Move platform docs from agile-governance/documentation/ to dev .orqa/documentation/
8. [ ] Rewrite content flagged as NEEDS REWRITE — remove .orqa/ paths, AD IDs, OrqaStudio CLI refs
9. [ ] Merge deduplication candidates (Artifact Link Format into Lifecycle, governance-maintenance split)
10. [ ] Move Thinking Mode: Learning Loop to agile-governance
11. [ ] Tighten thinking-mode-* content — remove artifact type names, use generic categories
12. [ ] Run `orqa validate --fix` to verify graph integrity
13. [ ] Register orqa-core in .orqa/project.json and .claude/ symlinks
