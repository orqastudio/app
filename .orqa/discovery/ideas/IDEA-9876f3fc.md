---
id: IDEA-9876f3fc
type: idea
title: "Artifact migration: location audit, content cleanup, plugin categories"
status: surpassed
description: Audit artifact locations, move to correct plugins, rewrite content to be domain-portable, add plugin category schema (core/enhancement/extension), deduplicate.
created: 2026-03-22
updated: 2026-03-22
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: Structured artifact placement
  - target: PERSONA-c4afd86b
    type: benefits
    rationale: Lead needs artifacts in consistent locations
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
3. **No OrqaStudio CLI commands** (orqa enforce, orqa graph) — reference generic actions
4. **No project-specific config** (core.json, project.json) — these are app-level
5. **CAN reference** general categories: "Discovery artifacts", "Delivery artifacts", "Governance artifacts"
6. **CAN reference** generic types: "rules", "knowledge", "agents", "decisions", "lessons" (these are the governance vocabulary, not project-specific)

## Full Artifact Location Audit

### Agents

| Current ID | Title | Current Location | Action |
|-----------|-------|-----------------|--------|
| AGENT-4c94fe14 | Orchestrator | agile-governance | **MOVE → orqa-core** (framework execution model) |
| AGENT-e5dd38e4 | Implementer | agile-governance | **MOVE → orqa-core** |
| AGENT-8e58cd87 | Reviewer | agile-governance | **MOVE → orqa-core** |
| AGENT-e333508b | Researcher | agile-governance | **MOVE → orqa-core** |
| AGENT-85be6ace | Planner | agile-governance | **MOVE → orqa-core** |
| AGENT-bbad3d30 | Writer | agile-governance | **MOVE → orqa-core** |
| AGENT-0aad40f4 | Designer | agile-governance | **MOVE → orqa-core** |
| AGENT-ae63c406 | Governance Steward | agile-governance | STAY (governance-specific) |
| AGENT-d1be3776 | Installer | agile-governance | **MOVE → orqa-core** (framework role) |
| AGENT-7a06d10e | Enforcer | agile-governance | STAY + RENAME to "Governance Enforcer" |
| AGENT-5de8c14f | Svelte Specialist | svelte | STAY |
| AGENT-6f55de0d | Svelte Standards | svelte | STAY |
| AGENT-065a25cc | Rust Specialist | rust | STAY |
| AGENT-26e5029d | Rust Standards | rust | STAY |

### Rules — agile-governance (currently 20)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-87ba1b81 | Agent Delegation | **MOVE → orqa-core** (framework execution model) |
| RULE-b10fe6d1 | Artifact Lifecycle | **REVIEW** — generic status transitions STAY, governance-specific parts SPLIT |
| RULE-ec9462d8 | Documentation-First | **MOVE → orqa-core** (framework methodology) |
| RULE-4603207a | Enforcement Before Code | STAY (governance enforcement pattern) |
| RULE-0be7765e | Error Ownership | STAY (governance discipline) |
| RULE-f609242f | Git Workflow | **MOVE → future git-integration plugin** |
| RULE-484872ef | Historical Preservation | **MOVE → orqa-core** (framework archival methodology) |
| RULE-5dd9decd | Honest Reporting | STAY (governance discipline) |
| RULE-25baac14 | IDs Are Not Priority | STAY (governance convention) |
| RULE-c603e90e | Lessons Learned | STAY (governance learning loop) |
| RULE-8ee65d73 | No Deferred Deliverables | STAY (governance enforcement) |
| RULE-af5771e3 | No Stubs or Placeholders | **MOVE → coding-standards** (code-level rule) |
| RULE-5965256d | Required Reading | **MOVE → orqa-core** (framework discipline) |
| RULE-d5d28fba | Structure Before Work | **MOVE → orqa-core** (framework methodology) |
| RULE-2f7b6a31 | Artifact Link Format | **MERGE with Artifact Lifecycle** if possible, else STAY |
| RULE-b723ea53 | Tool Access Restrictions | **MOVE → orqa-core** (framework agent model) |
| RULE-30a223ca | Session Management | **MOVE → orqa-core** (framework methodology) |
| RULE-3c2da849 | Core Graph Firmware Protection | STAY (governance protection) |
| RULE-130f1f63 | Data Integrity | **REVIEW** — STAY if generic cross-reference integrity. SPLIT if OrqaStudio-specific |
| RULE-af1cd87d | Behavioral Rule Enforcement Plan | STAY (governance enforcement) |

### Rules — systems-thinking (currently 7)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-c382e053 | No Aliases or Hacks | **MOVE → coding-standards** (code-level rule) |
| RULE-05562ed4 | Pillar Alignment in Documentation | STAY (discovery methodology) |
| RULE-97e96528 | Root Directory Cleanliness | **MOVE → coding-standards** (file structure rule) |
| RULE-43f1bebc | Systems Thinking First | STAY (core methodology) |
| RULE-71352dc8 | UAT Process | **MOVE → software** (delivery process) |
| RULE-1b238fc8 | Vision Alignment | STAY (discovery methodology) |
| RULE-23699df2 | Artifact Schema Compliance | **MOVE → agile-governance** (governance enforcement) |

### Rules — software (currently 7)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-63cc16ad | Artifact Config Integrity | **REVIEW** — may be redundant with well-enforced schemas. Remove if so |
| RULE-dccf4226 | Plan Mode Compliance | STAY (delivery planning) |
| RULE-dd5b69e6 | Skill Enforcement | **MOVE → orqa-core** (framework knowledge model) |
| RULE-205d9c91 | Skill Portability | **MOVE → orqa-core** (framework knowledge model) |
| RULE-ef822519 | Context Window Management | **MOVE → orqa-core** (framework agent methodology) |
| RULE-f23392dc | User-Invocable Knowledge Semantics | **MOVE → orqa-core** (framework knowledge model) |
| RULE-8abcbfd5 | Provider-Agnostic Tool Capabilities | **MOVE → orqa-core** (framework agent model) |

### Rules — dev environment (.orqa/process/rules/) — 19 rules, 14 should move to plugins

Rules that enforce plugin-specific behaviour must travel with their plugin.

**Enforcement authority**: The app is the authority, connectors are adapters
(see [IDEA-e3373f67](IDEA-e3373f67)). Rules declare WHAT is enforced, not HOW.
The app's enforcement engine handles in-app agents natively. Connectors map
the same rules to their native hook systems. Rules do NOT need `requires: connector`.

Rules enforced via a language tool (clippy, ESLint) should live in OR require
the tool's plugin — those tools are the enforcement mechanism, not the connector.

**Enforcement method key:**
- `behavioral` — prompt injection (app: system prompt builder, connector: UserPromptSubmit hook)
- `hook` — action validation (app: enforcement engine, connector: PreToolUse/PostToolUse)
- `lint` — linter rule (provided by language plugin: rust, svelte, typescript)
- `review` — code reviewer agent check
- `tool` — CLI tool (orqa enforce, orqa check — lives with CLI plugin)
- `none` — not yet mechanically enforced (enforcement gap)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-4dbb3612 | Enforcement Gap Priority | **MOVE → agile-governance** | `none` — enforcement gap itself! |
| RULE-d543d759 | Honest Status Reporting | **MOVE → agile-governance** | `behavioral` — prompt injection |
| RULE-145332dc | Governance Priority Over Delivery | **MOVE → agile-governance** | `behavioral` + `hook` (stop hook escalation). App enforces natively; connector adapts to native hooks |
| RULE-b2584e59 | Trace Artifacts to Usage Contexts | **MOVE → agile-governance** | `none` — enforcement gap |
| RULE-2f64cc63 | Continuous Operation | **MOVE → orqa-core** | `behavioral` — prompt injection |
| RULE-8aadfd6c | Real-time Session State Management | **MOVE → orqa-core** | `behavioral` + `hook`. App enforces natively; connector adapts to native hooks |
| RULE-e1f1afc1 | Automated Knowledge Injection | **MOVE → orqa-core** | `hook` (PostToolUse file write). App enforces natively; connector adapts to native hooks |
| RULE-0d29fc91 | Code Search Usage | **MOVE → orqa-core** | `behavioral` — agent prompt. Search provided by orqa-core |
| RULE-f3dca71e | Pre-Release Version Tagging | **MOVE → cli** | `tool` (orqa version check). Rule lives WITH the tool |
| RULE-83411442 | Tooltips over title attributes | **MOVE → coding-standards** | `review` + potential `lint` (ESLint rule via svelte plugin) |
| RULE-eb269afb | Reusable Components | **MOVE → coding-standards** | `review` + `hook` (knowledge injection on component writes). App enforces natively; connector adapts to native hooks |
| RULE-8cb4bd04 | Testing Standards | **MOVE → coding-standards** | `hook` (pre-commit runs make test). Language plugins provide test runners |
| RULE-42d17086 | Tooling Ecosystem Management | **MOVE → coding-standards** | `lint` delegation — rule documents which linter enforces what. Language plugins provide linters |
| RULE-b03009da | End-to-End Completeness | **MOVE → software** | `review` + `hook` (pre-commit). OrqaStudio-specific (four-layer Tauri stack). **NEEDS REWRITE** to be generic "all layers must be updated together" |
| RULE-05ae2ce7 | Architecture Decisions | **SPLIT** — generic "decisions are first-class artifacts" → agile-governance; OrqaStudio AD source of truth details → STAY |
| RULE-9814ec3c | Coding Standards | **SPLIT** — generic enforcement discipline (run linters, no disabling rules, lint-rule alignment) → coding-standards; OrqaStudio Rust+Svelte+TS specifics → STAY |
| RULE-d2c2063a | Development Commands | STAY (OrqaStudio Makefile targets — purely project-specific) |
| RULE-998da8ea | Dogfood Mode | **SPLIT** — generic "editing the app you're running inside" methodology → orqa-core; OrqaStudio-specific restart/sidecar rules → STAY |
| RULE-09a238ab | Data Persistence Boundaries | STAY (OrqaStudio SQLite/file/ephemeral split — purely project-specific) |

### Knowledge — agile-governance (currently 11)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-ee860ed9 | Enforcement Patterns | STAY | GENERIC — teaches methodology |
| KNOW-21d28aa0 | Planning | **MOVE → orqa-core** (framework methodology) | NEEDS REWRITE — has .orqa/ paths, AD IDs |
| KNOW-9ff8c63f | Research Methodology | **MOVE → orqa-core** (framework methodology) | GENERIC |
| KNOW-8564d52c | Diagnostic Methodology | **MOVE → orqa-core** (framework methodology) | GENERIC |
| KNOW-8c359ea4 | Governance Maintenance | STAY but REWRITE | NEEDS REWRITE — mix of generic + OrqaStudio-specific |
| KNOW-0444355f | Plugin Artifact Usage | STAY | REVIEW — may be OrqaStudio-specific |
| KNOW-51de8fb7 | Artifact Status Management | **MOVE → orqa-core** (framework status model) | GENERIC — portable status model |
| KNOW-936e5944 | Governance Patterns | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — .orqa/ structure, core.json |
| KNOW-16e91c20 | Artifact Audit Methodology | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — schema-driven audit |
| KNOW-85a449e7 | Naming Conventions | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — GitHub repo naming, npm scopes |
| KNOW-2bf2b321 | Schema Validation | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — core.json validation |

### Knowledge — systems-thinking (currently 14)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-41849545 | Systems Thinking | STAY | GENERIC |
| KNOW-7fadba3f | Architectural Evaluation | STAY | GENERIC |
| KNOW-1ea9291c | Artifact Relationships | STAY | GENERIC |
| KNOW-0619a413 | Composability | STAY | GENERIC |
| KNOW-a3dcdd05 | Restructuring Methodology | STAY | GENERIC |
| KNOW-d13d80e1 | Tech Debt Management | STAY | GENERIC |
| KNOW-b95ec6e3 | Thinking Mode: Debugging | **MOVE → orqa-core** + add `thinking-mode: debugging` | MOSTLY GENERIC |
| KNOW-bf70068c | Thinking Mode: Documentation | STAY + add `thinking-mode: documentation` | MOSTLY GENERIC |
| KNOW-eeceaabf | Thinking Mode: Dogfood Implementation | STAY + add `thinking-mode: dogfood-implementation` | GENERIC |
| KNOW-f7fb7aa7 | Thinking Mode: Implementation | **MOVE → orqa-core** + add `thinking-mode: implementation` | MOSTLY GENERIC |
| KNOW-83039175 | Thinking Mode: Learning Loop | **MOVE → agile-governance** + add `thinking-mode: learning-loop` | MOSTLY GENERIC |
| KNOW-4a4241a5 | Thinking Mode: Planning | STAY + add `thinking-mode: planning` | MOSTLY GENERIC |
| KNOW-36befd20 | Thinking Mode: Research | STAY + add `thinking-mode: research` | MOSTLY GENERIC |
| KNOW-fd636a56 | Thinking Mode: Review | **MOVE → orqa-core** + add `thinking-mode: review` | MOSTLY GENERIC |

### Knowledge — software (currently 14)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-a700e25a | Software Delivery | STAY but REWRITE | ORQASTUDIO-ONLY — references .orqa/delivery/, artifact IDs. Extract generic delivery pattern |
| KNOW-0188373b | Epic Completion | STAY but REWRITE | ORQASTUDIO-ONLY — references .orqa/, orqa enforce. Extract generic "delivery unit completion" |
| KNOW-2f38309a | Plugin Development | **MOVE → orqa-core** (framework infrastructure) | NEEDS REWRITE — remove OrqaStudio-specific paths, make generic |
| KNOW-e6fee7a0 | First-Party Plugin Dev | **MOVE → orqa-core** (framework infrastructure) | NEEDS REWRITE — remove orqastudio-dev/ refs |
| KNOW-1b7fa054 | Third-Party Plugin Dev | **MOVE → orqa-core** (framework infrastructure) | NEEDS REWRITE — remove project.json specifics |
| KNOW-13348442 | Search | **MOVE → orqa-core** (framework infrastructure) | NEEDS REWRITE — make search methodology generic |
| KNOW-72ca209f | Skills Maintenance | **ARCHIVE** — reactivate when skills.sh integration lands | See IDEA for skills.sh |
| KNOW-91a7a6c1 | Code Quality Review | STAY | GENERIC |
| KNOW-d00093e7 | Component Extraction | STAY but minor rewrite | MOSTLY GENERIC — remove TASK-63276ee5 reference |
| KNOW-1314ac47 | QA Verification | STAY | GENERIC |
| KNOW-45b5f8a8 | Security Audit | STAY | GENERIC |
| KNOW-5f4db8f7 | Test Engineering | STAY | GENERIC |
| KNOW-71352dc8 | UAT Process | STAY | GENERIC |
| KNOW-bec7e87d | UX Compliance Review | STAY | GENERIC |

### Knowledge — connector (currently 6)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-3155cdaa | Decision Tree | **SPLIT** → generic "Agent Decision Methodology" in orqa-core + Claude Code-specific tree in dev .orqa/ | Extract reasoning pattern; keep CC tool names/mappings project-level |
| KNOW-b1593311 | Implementer Tree | **SPLIT** → generic "Implementer Reasoning" in orqa-core + CC-specific tree in dev .orqa/ | Same split pattern |
| KNOW-08fcd847 | Reviewer Tree | **SPLIT** → generic "Reviewer Reasoning" in orqa-core + CC-specific tree in dev .orqa/ | Same split pattern |
| KNOW-f5ee4e0d | Plugin Setup | **MOVE → orqa-core** + add `onboarding: true` | NEEDS REWRITE — make connector-agnostic |
| KNOW-03421ec0 | Project Inference | **SPLIT** → generic "Project Type Detection" in orqa-core (`onboarding: true`) + CC-specific inference rules in dev .orqa/ | Generic: file signature → project type. Specific: .claude/ detection |
| KNOW-4a58e7dd | Project Migration | **SPLIT** → generic "Governance Migration Methodology" in orqa-core + CC-specific .claude/ → .orqa/ steps in dev .orqa/ | Generic: format migration methodology. Specific: Claude Code paths |
| KNOW-2876afc7 | Project Setup | **MOVE → orqa-core** + add `onboarding: true` | NEEDS REWRITE — make connector-agnostic |
| KNOW-d03337ac | Project Type: Software | **MOVE → software** + add `onboarding: true` | Used during project onboarding to configure software delivery |

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
12. [ ] Run `orqa enforce --fix` to verify graph integrity
13. [ ] Register orqa-core in .orqa/project.json and .claude/ symlinks