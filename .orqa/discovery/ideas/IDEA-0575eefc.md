---
id: IDEA-0575eefc
type: idea
title: "Artifact ID migration: plugin-specific prefixes"
status: captured
description: "Audit artifact locations across all plugins and dev environment, then migrate IDs to use plugin-specific prefix conventions (e.g., KNOW-GOV-*, RULE-ST-*, AGENT-GOV-*)."
created: "2026-03-22"
relationships:
  - target: "RULE-130f1f63"
    type: "related"
    rationale: "Data integrity requires consistent, traceable IDs"
---

# Artifact ID Migration: Plugin-Specific Prefixes

## Context

Artifacts were migrated from `app/.orqa/process/` to owning plugins (agile-governance,
systems-thinking, software, claude-code connector) on 2026-03-22. Many migrated artifacts
still carry bare IDs (e.g., `KNOW-54823e2d`) instead of plugin-prefixed IDs (e.g.,
`KNOW-SYS-54823e2d`).

**NOTE (2026-03-22):** There are also many older artifacts across all plugins that were
never migrated to the hex ID format — they still use sequential numbering (e.g.,
`KNOW-TS-001`, `KNOW-CS-001`). These also need converting to hex format as part of
this migration. The `scripts/migrate-artifact-ids.mjs` bulk migration script handles
sequential → hex conversion. Run with `--apply` after confirming the manifest.

## Prerequisites

1. **Manual audit** of the artifact table below — confirm every artifact lives in the right plugin
2. Once locations are confirmed, run `orqa id migrate` for each artifact to apply the prefix

## Existing Tooling

- `orqa id migrate <old> <new>` — renames an ID across the entire graph (frontmatter, relationships, body text)
- `scripts/migrate-artifact-ids.mjs` — bulk migration script with dry-run and manifest output

## Current ID Prefix Conventions

| Plugin | Abbreviation | Pattern |
|--------|-------------|---------|
| Rust | RST | `AGENT-RST-*`, `KNOW-RST-*` |
| Svelte | SVE | `AGENT-SVE-*`, `KNOW-SVE-*` |
| CLI | CLI | `KNOW-CLI-*` |
| Software | SW | `KNOW-SW-*` |
| Tauri | TAU | `KNOW-TAU-*` |
| TypeScript | TS | `KNOW-TS-*` |
| Coding Standards | CS | `KNOW-CS-*` |
| Claude Connector | CC | `KNOW-CC-*` |
| Agile Governance | GOV | `AGENT-GOV-*`, `KNOW-GOV-*`, `RULE-GOV-*` |
| Systems Thinking | SYS | `KNOW-SYS-*`, `RULE-SYS-*` |

## Full Artifact Location Audit

### Agents

| Current ID | Title | Location | Needs Prefix? |
|-----------|-------|----------|---------------|
| AGENT-1dab5ebe | Orchestrator | plugins/agile-governance/agents | GOV |
| AGENT-cc255bc8 | Implementer | plugins/agile-governance/agents | GOV |
| AGENT-b0774726 | Reviewer | plugins/agile-governance/agents | GOV |
| AGENT-fb0ce261 | Researcher | plugins/agile-governance/agents | GOV |
| AGENT-caff7bc1 | Planner | plugins/agile-governance/agents | GOV |
| AGENT-ec1b3785 | Writer | plugins/agile-governance/agents | GOV |
| AGENT-c5284fde | Designer | plugins/agile-governance/agents | GOV |
| AGENT-ff44f841 | Governance Steward | plugins/agile-governance/agents | GOV |
| AGENT-bedeffd1 | Installer | plugins/agile-governance/agents | GOV |
| AGENT-GOV-e7f3a2c9 | Enforcer | plugins/agile-governance/agents | Already prefixed |
| AGENT-SVE-spec-c8e4f9a2 | Svelte Specialist | plugins/svelte/agents | Already prefixed |
| AGENT-SVE-b0857607 | Svelte Standards | plugins/svelte/agents | Already prefixed |
| AGENT-RST-spec-a3f7d2b1 | Rust Specialist | plugins/rust/agents | Already prefixed |
| AGENT-RST-4241392c | Rust Standards | plugins/rust/agents | Already prefixed |

### Rules (by location)

**plugins/agile-governance/rules/**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| RULE-532100d9 | Agent Delegation | GOV |
| RULE-7b770593 | Artifact Lifecycle | GOV |
| RULE-9daf29c0 | Documentation-First Implementation | GOV |
| RULE-3eccebf3 | Enforcement Before Code | GOV |
| RULE-57ccb4a3 | Error Ownership | GOV |
| RULE-633e636d | Git Workflow | GOV |
| RULE-6d1c8dc7 | Historical Artifact Preservation | GOV |
| RULE-878e5422 | Honest Reporting | GOV |
| RULE-22783309 | IDs Are Not Priority | GOV |
| RULE-551bde31 | Lessons Learned | GOV |
| RULE-e120bb70 | No Deferred Deliverables | GOV |
| RULE-e9c54567 | No Stubs or Placeholders | GOV |
| RULE-b2753bad | Required Reading | GOV |
| RULE-8035e176 | Structure Before Work | GOV |
| RULE-2f7b6a31 | Artifact Link Format | GOV |
| RULE-f809076f | Tool Access Restrictions | GOV |
| RULE-e352fd0a | Session Management | GOV |
| RULE-98682b5e | Core Graph Firmware Protection | GOV |
| RULE-130f1f63 | Data Integrity | GOV |
| RULE-9bc8c230 | Behavioral Rule Enforcement Plan | GOV |

**plugins/systems-thinking/rules/**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| RULE-9ba80a19 | No Aliases or Hacks | SYS |
| RULE-39169bcd | Pillar Alignment in Documentation | SYS |
| RULE-1f30904a | Root Directory Cleanliness | SYS |
| RULE-d90112d9 | Systems Thinking First | SYS |
| RULE-4d4f540d | UAT Process | SYS |
| RULE-1e8a1914 | Vision Alignment | SYS |
| RULE-a764b2ae | Artifact Schema Compliance | SYS |

**plugins/software/rules/**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| RULE-6c0496e0 | Artifact Config Integrity | SW |
| RULE-303c1cc8 | Plan Mode Compliance | SW |
| RULE-deab6ea7 | Skill Enforcement | SW |
| RULE-11c29c9e | Skill Portability | SW |
| RULE-df24948b | Context Window Management | SW |
| RULE-5ee43922 | User-Invocable Knowledge Semantics | SW |
| RULE-92dba0cb | Provider-Agnostic Tool Capabilities | SW |

**.orqa/process/rules/ (dev environment — project-specific)**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| RULE-65973a88 | Architecture Decisions | No (project-level) |
| RULE-5e03e67b | Code Search Usage | No |
| RULE-b49142be | Coding Standards | No |
| RULE-c71f1c3f | Development Commands | No |
| RULE-6083347d | Dogfood Mode | No |
| RULE-1acb1602 | End-to-End Completeness | No |
| RULE-cb65b5d0 | Reusable Components | No |
| RULE-f10bb5de | Testing Standards | No |
| RULE-89155a7f | Tooltips over title attributes | No |
| RULE-c95f4444 | Data Persistence Boundaries | No |
| RULE-f9d0279c | Automated Knowledge Injection | No |
| RULE-7f416d7d | Tooling Ecosystem Management | No |
| RULE-4f7e2a91 | Real-time Session State Management | No |
| RULE-12e74734 | Enforcement Gap Priority | No |
| RULE-029db175 | Continuous Operation | No |
| RULE-4263a6b3 | Pre-Release Version Tagging | No |
| RULE-9cd980b1 | Honest Status Reporting | No |
| RULE-67b91c13 | Trace Every Artifact to Its Usage Contexts | No |
| RULE-c4fe67a2 | Governance Priority Over Delivery | No |

### Knowledge (by location)

**plugins/agile-governance/knowledge/**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| KNOW-GOV-e4b91f37 | Enforcement Patterns | Already prefixed |
| KNOW-6f33713e | Planning | GOV |
| KNOW-f7476f0a | Research Methodology | GOV |
| KNOW-f5edb34d | Diagnostic Methodology | GOV |
| KNOW-8d76c3c7 | Governance Maintenance | GOV |
| KNOW-8d1c4be6 | Plugin Artifact Usage | GOV |
| KNOW-449b1e02 | Artifact Status Management | GOV |
| KNOW-eea50a65 | Governance Patterns | GOV |
| KNOW-4368d782 | Artifact Audit Methodology | GOV |
| KNOW-250d5d6f | Naming Conventions | GOV |
| KNOW-b08d355c | Schema Validation | GOV |

**plugins/systems-thinking/knowledge/**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| KNOW-54823e2d | Systems Thinking | SYS |
| KNOW-30a419dd | Architectural Evaluation | SYS |
| KNOW-82d32398 | Artifact Relationships | SYS |
| KNOW-f0c40eaf | Composability | SYS |
| KNOW-8c98ea98 | Restructuring Methodology | SYS |
| KNOW-c7fb7c83 | Tech Debt Management | SYS |
| KNOW-323c2803 | Thinking Mode: Debugging | SYS |
| KNOW-1ab0e715 | Thinking Mode: Documentation | SYS |
| KNOW-a4c8f1e2 | Thinking Mode: Dogfood Implementation | SYS |
| KNOW-fda0559b | Thinking Mode: Implementation | SYS |
| KNOW-85e392ea | Thinking Mode: Learning Loop | SYS |
| KNOW-de25b290 | Thinking Mode: Planning | SYS |
| KNOW-1a8eb147 | Thinking Mode: Research | SYS |
| KNOW-83614358 | Thinking Mode: Review | SYS |

**plugins/software/knowledge/**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| KNOW-SW-1d47d8d8 | Software Delivery | Already prefixed |
| KNOW-SW-epic-complete | Epic Completion | Already prefixed |
| KNOW-b453410f | Plugin Development | SW |
| KNOW-e1333874 | First-Party Plugin Dev | SW |
| KNOW-63cc1a00 | Third-Party Plugin Dev | SW |
| KNOW-a2b3c4d5 | Search | SW |
| KNOW-2c8eead6 | Skills Maintenance | SW |
| KNOW-f0efaf83 | Code Quality Review | SW |
| KNOW-353a228b | Component Extraction | SW |
| KNOW-1b805150 | QA Verification | SW |
| KNOW-170c220e | Security Audit | SW |
| KNOW-bcb42347 | Test Engineering | SW |
| KNOW-c6d04755 | UAT Process | SW |
| KNOW-5124e508 | UX Compliance Review | SW |

**connectors/claude-code/knowledge/**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| KNOW-CC-decision-tree | Decision Tree | Already prefixed |
| KNOW-CC-implementer-tree | Implementer Tree | Already prefixed |
| KNOW-CC-reviewer-tree | Reviewer Tree | Already prefixed |
| KNOW-e3a559c9 | Plugin Setup | CC |
| KNOW-82ceb1bd | Project Inference | CC |
| KNOW-0fd23e0b | Project Migration | CC |
| KNOW-e0dec720 | Project Setup | CC |
| KNOW-819789ab | Project Type: Software | CC |

**.orqa/process/knowledge/ (dev environment)**

| Current ID | Title | Needs Prefix? |
|-----------|-------|---------------|
| KNOW-025fc31d | OrqaStudio Architecture | No (project-level) |
| KNOW-5ad0bf1b | Backend Best Practices | No |
| KNOW-282c0305 | Frontend Best Practices | No |
| KNOW-49f495ff | IPC Patterns | No |
| KNOW-2b6147c9 | Repository Pattern | No |
| KNOW-1b990160 | Store Orchestration | No |
| KNOW-65f5aa67 | Store Patterns | No |
| KNOW-3f34e682 | Streaming Pipeline | No |
| KNOW-7a96b952 | Testing Patterns | No |
| KNOW-58611337 | Domain Services | No |
| KNOW-8a821622 | Error Composition | No |
| KNOW-13ec986c | Documentation Authoring | No |
| KNOW-dac84f00 | Enforcement Engine | No |
| KNOW-01a64d58 | Centralized Logging | No |
| KNOW-c60144c1 | (untitled) | No |
| KNOW-12ed4953 | (untitled) | No |

## Steps

1. [ ] User audits this table — confirms each artifact is in the right plugin
2. [ ] Resolve any misplacements (move artifacts between plugins)
3. [ ] Run `orqa id migrate` for each bare-ID artifact to add plugin prefix
4. [ ] Update plugin manifests (orqa-plugin.json) with new IDs
5. [ ] Run `orqa validate --fix` to verify graph integrity
6. [ ] Fix sequential IDs (KNOW-TS-001, KNOW-CS-001/002) to use hex format
