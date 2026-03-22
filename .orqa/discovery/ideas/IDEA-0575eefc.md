---
id: IDEA-0575eefc
type: idea
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
| `core:governance` | Defines the governance lifecycle (rules, decisions, lessons, enforcement) | agile-governance |
| `core:discovery` | Defines the discovery/thinking lifecycle (agents, thinking modes, methodology) | systems-thinking |
| `core:delivery` | Defines the delivery lifecycle (milestones, epics, tasks, research) | software |
| `enhancement:governance` | Extends governance with domain-specific enforcement | coding-standards |
| `enhancement:delivery` | Extends delivery with domain-specific tooling | rust, svelte, tauri, typescript |
| `extension` | General functional addition, not lifecycle-specific | cli, claude-code connector |

Core plugins can reference the general categories "Discovery", "Delivery", "Governance"
but MUST NOT reference other plugins' specific artifact types or OrqaStudio-specific paths.

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
| AGENT-1dab5ebe | Orchestrator | agile-governance | **MOVE → systems-thinking** (universal role, not governance-specific) |
| AGENT-cc255bc8 | Implementer | agile-governance | **MOVE → systems-thinking** |
| AGENT-b0774726 | Reviewer | agile-governance | **MOVE → systems-thinking** |
| AGENT-fb0ce261 | Researcher | agile-governance | **MOVE → systems-thinking** |
| AGENT-caff7bc1 | Planner | agile-governance | **MOVE → systems-thinking** |
| AGENT-ec1b3785 | Writer | agile-governance | **MOVE → systems-thinking** |
| AGENT-c5284fde | Designer | agile-governance | **MOVE → systems-thinking** |
| AGENT-ff44f841 | Governance Steward | agile-governance | **STAY** (governance-specific) |
| AGENT-bedeffd1 | Installer | agile-governance | **MOVE → systems-thinking** (universal role) |
| AGENT-e7f3a2c9 | Enforcer | agile-governance | **STAY + RENAME** to "Governance Enforcer" |
| AGENT-b2f574e5 | Svelte Specialist | svelte | STAY |
| AGENT-b0857607 | Svelte Standards | svelte | STAY |
| AGENT-e1e47559 | Rust Specialist | rust | STAY |
| AGENT-4241392c | Rust Standards | rust | STAY |

### Rules — agile-governance (currently 20)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-532100d9 | Agent Delegation | **MOVE → systems-thinking** (how agents work, not governance) |
| RULE-7b770593 | Artifact Lifecycle | **REVIEW** — if generic (status transitions for any artifact), STAY. If it references governance-specific types, SPLIT into generic lifecycle + governance-specific |
| RULE-9daf29c0 | Documentation-First | **MOVE → systems-thinking** (methodology, not governance enforcement) |
| RULE-3eccebf3 | Enforcement Before Code | STAY (governance enforcement pattern) |
| RULE-57ccb4a3 | Error Ownership | STAY (governance discipline) |
| RULE-633e636d | Git Workflow | **MOVE → future git-integration plugin** (not governance) |
| RULE-6d1c8dc7 | Historical Preservation | **MOVE → systems-thinking** (archival methodology) |
| RULE-878e5422 | Honest Reporting | STAY (governance discipline) |
| RULE-22783309 | IDs Are Not Priority | STAY (governance convention) |
| RULE-551bde31 | Lessons Learned | STAY (governance learning loop) |
| RULE-e120bb70 | No Deferred Deliverables | STAY (governance enforcement) |
| RULE-e9c54567 | No Stubs or Placeholders | **MOVE → software** (code implementation rule) |
| RULE-b2753bad | Required Reading | STAY (governance discipline) |
| RULE-8035e176 | Structure Before Work | **MOVE → systems-thinking** (methodology) |
| RULE-2f7b6a31 | Artifact Link Format | **MERGE with Artifact Lifecycle** if possible, else STAY |
| RULE-f809076f | Tool Access Restrictions | **MOVE → systems-thinking** (agent capability model) |
| RULE-e352fd0a | Session Management | **MOVE → systems-thinking** (methodology) |
| RULE-98682b5e | Core Graph Firmware Protection | STAY (governance protection) |
| RULE-130f1f63 | Data Integrity | **REVIEW** — STAY if generic cross-reference integrity. SPLIT if OrqaStudio-specific |
| RULE-9bc8c230 | Behavioral Rule Enforcement Plan | STAY (governance enforcement) |

### Rules — systems-thinking (currently 7)

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-9ba80a19 | No Aliases or Hacks | **MOVE → software** (code implementation rule) |
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
| RULE-deab6ea7 | Skill Enforcement | **MOVE → agile-governance** (governance framework) |
| RULE-11c29c9e | Skill Portability | **MOVE → agile-governance** (governance framework) |
| RULE-df24948b | Context Window Management | **MOVE → systems-thinking** (agent methodology) |
| RULE-5ee43922 | User-Invocable Knowledge Semantics | **MOVE → systems-thinking** (knowledge model) |
| RULE-92dba0cb | Provider-Agnostic Tool Capabilities | **MOVE → agile-governance** (agent capability model) |

### Rules — dev environment (.orqa/process/rules/) — 19 project-specific

| Current ID | Title | Action |
|-----------|-------|--------|
| RULE-65973a88 | Architecture Decisions | STAY (project-level) |
| RULE-5e03e67b | Code Search Usage | STAY |
| RULE-b49142be | Coding Standards | STAY |
| RULE-c71f1c3f | Development Commands | STAY |
| RULE-6083347d | Dogfood Mode | STAY |
| RULE-1acb1602 | End-to-End Completeness | STAY |
| RULE-cb65b5d0 | Reusable Components | STAY |
| RULE-f10bb5de | Testing Standards | STAY |
| RULE-89155a7f | Tooltips over title attributes | **MOVE → coding-standards** (UI convention) |
| RULE-c95f4444 | Data Persistence Boundaries | STAY |
| RULE-f9d0279c | Automated Knowledge Injection | STAY |
| RULE-7f416d7d | Tooling Ecosystem Management | STAY |
| RULE-4f7e2a91 | Real-time Session State Management | STAY |
| RULE-12e74734 | Enforcement Gap Priority | STAY |
| RULE-029db175 | Continuous Operation | STAY |
| RULE-4263a6b3 | Pre-Release Version Tagging | STAY |
| RULE-9cd980b1 | Honest Status Reporting | STAY |
| RULE-67b91c13 | Trace Artifacts to Usage Contexts | STAY |
| RULE-c4fe67a2 | Governance Priority Over Delivery | STAY |

### Knowledge — agile-governance (currently 11)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-e4b91f37 | Enforcement Patterns | STAY | GENERIC — teaches methodology |
| KNOW-6f33713e | Planning | **MOVE → systems-thinking** (methodology) | NEEDS REWRITE — has .orqa/ paths, AD IDs |
| KNOW-f7476f0a | Research Methodology | **MOVE → systems-thinking** (methodology) | GENERIC |
| KNOW-f5edb34d | Diagnostic Methodology | **MOVE → systems-thinking** (methodology) | GENERIC |
| KNOW-8d76c3c7 | Governance Maintenance | STAY but REWRITE | NEEDS REWRITE — mix of generic + OrqaStudio-specific. Extract generic part |
| KNOW-8d1c4be6 | Plugin Artifact Usage | STAY | REVIEW — may be OrqaStudio-specific |
| KNOW-449b1e02 | Artifact Status Management | STAY | GENERIC — portable status model |
| KNOW-eea50a65 | Governance Patterns | **MOVE → dev .orqa/** (100% OrqaStudio-specific) | ORQASTUDIO-ONLY — describes .orqa/ structure, core.json |
| KNOW-4368d782 | Artifact Audit Methodology | **MOVE → dev .orqa/** (100% OrqaStudio-specific) | ORQASTUDIO-ONLY — schema-driven audit against core.json |
| KNOW-250d5d6f | Naming Conventions | **MOVE → dev .orqa/** (100% OrqaStudio-specific) | ORQASTUDIO-ONLY — GitHub repo naming, npm scopes |
| KNOW-b08d355c | Schema Validation | **MOVE → dev .orqa/** (100% OrqaStudio-specific) | ORQASTUDIO-ONLY — core.json, orqa-plugin.json validation |

### Knowledge — systems-thinking (currently 14)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-54823e2d | Systems Thinking | STAY | GENERIC |
| KNOW-30a419dd | Architectural Evaluation | STAY | GENERIC |
| KNOW-82d32398 | Artifact Relationships | STAY | GENERIC |
| KNOW-f0c40eaf | Composability | STAY | GENERIC |
| KNOW-8c98ea98 | Restructuring Methodology | STAY | GENERIC |
| KNOW-c7fb7c83 | Tech Debt Management | STAY | GENERIC |
| KNOW-323c2803 | Thinking Mode: Debugging | STAY | MOSTLY GENERIC — minor rewrite to remove artifact type names |
| KNOW-1ab0e715 | Thinking Mode: Documentation | STAY | MOSTLY GENERIC |
| KNOW-a4c8f1e2 | Thinking Mode: Dogfood Implementation | STAY | GENERIC |
| KNOW-fda0559b | Thinking Mode: Implementation | STAY | MOSTLY GENERIC |
| KNOW-85e392ea | Thinking Mode: Learning Loop | STAY | MOSTLY GENERIC |
| KNOW-de25b290 | Thinking Mode: Planning | STAY | MOSTLY GENERIC |
| KNOW-1a8eb147 | Thinking Mode: Research | STAY | MOSTLY GENERIC |
| KNOW-83614358 | Thinking Mode: Review | STAY | MOSTLY GENERIC |

### Knowledge — software (currently 14)

| Current ID | Title | Action | Content Status |
|-----------|-------|--------|---------------|
| KNOW-1d47d8d8 | Software Delivery | STAY but REWRITE | ORQASTUDIO-ONLY — references .orqa/delivery/, artifact IDs. Extract generic delivery pattern |
| KNOW-e2354dce | Epic Completion | STAY but REWRITE | ORQASTUDIO-ONLY — references .orqa/, orqa validate. Extract generic "delivery unit completion" |
| KNOW-b453410f | Plugin Development | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — orqa plugin create, .orqa/ structure |
| KNOW-e1333874 | First-Party Plugin Dev | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — orqastudio-dev/, .gitmodules |
| KNOW-63cc1a00 | Third-Party Plugin Dev | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — project.json, software plugin pre-install |
| KNOW-a2b3c4d5 | Search | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — orqa mcp, DuckDB, ONNX engine |
| KNOW-2c8eead6 | Skills Maintenance | **MOVE → dev .orqa/** | ORQASTUDIO-ONLY — npx skills CLI, .orqa/ paths |
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
| KNOW-3155cdaa | Decision Tree | STAY | Connector-specific |
| KNOW-b1593311 | Implementer Tree | STAY | Connector-specific |
| KNOW-08fcd847 | Reviewer Tree | STAY | Connector-specific |
| KNOW-e3a559c9 | Plugin Setup | STAY | Connector-specific |
| KNOW-82ceb1bd | Project Inference | STAY | Connector-specific |
| KNOW-0fd23e0b | Project Migration | STAY | Connector-specific |
| KNOW-e0dec720 | Project Setup | STAY | Connector-specific |
| KNOW-819789ab | Project Type: Software | STAY | Connector-specific |

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
2. [ ] Add `role` field to orqa-plugin.json schema (core:governance | core:discovery | core:delivery | enhancement:governance | enhancement:delivery | extension)
3. [ ] Move artifacts to correct plugins per table above
4. [ ] Move OrqaStudio-only knowledge to dev .orqa/
5. [ ] Rewrite content flagged as NEEDS REWRITE — remove .orqa/ paths, AD IDs, OrqaStudio CLI refs
6. [ ] Merge deduplication candidates
7. [ ] Move platform docs from agile-governance/documentation/ to dev .orqa/documentation/
8. [ ] Run `orqa validate --fix` to verify graph integrity
9. [ ] Tighten thinking-mode-* content — remove artifact type names, use generic categories
