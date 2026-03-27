# Inventory: .orqa/ Config, Principles, and Documentation

**Researcher:** Agent
**Date:** 2026-03-26
**Scope:** `.orqa/` top-level config, connectors, principles, and documentation

---

## 1. Top-Level Config Files

### project.json

| Field | Value |
|-------|-------|
| Path | `.orqa/project.json` |
| Purpose | Project-level configuration: name, stack, governance stats, artifact tree, excluded paths |

**Structure:**
- `name`: "Orqastudio-dev"
- `dogfood`: false
- `default_model`: "auto"
- `excluded_paths`: node_modules, .git, target, dist, build
- `stack`: languages (javascript, rust, svelte, typescript), frameworks (cargo), package_manager (cargo), has_claude_config (true), has_design_tokens (false)
- `governance`: counts (lessons: 80, decisions: 65, agents: 15, rules: 56, knowledge: 99)
- `artifacts`: 5 top-level groups:
  - **Process** (rules, agents, knowledge, lessons, decisions) in `.orqa/process/`
  - **Principles** (pillars, personas, vision, grounding) in `.orqa/principles/`
  - **Delivery** (milestones, epics, tasks, ideas) in `.orqa/delivery/`
  - **Discovery** (ideas, research, wireframes) in `.orqa/discovery/`
  - **Documentation** (flat) in `.orqa/documentation`
- `artifactLinks`, `statuses`, `delivery.types`, `relationships`, `plugins`: all empty — schema-driven from plugins
- `show_thinking`: false
- `custom_system_prompt`: null

### manifest.json

| Field | Value |
|-------|-------|
| Path | `.orqa/manifest.json` |
| Size | 873 lines |
| Purpose | Plugin installation manifest tracking file hashes (source vs installed) for three-way diff |

**Structure:** `{ plugins: { "<plugin-name>": { version, installed_at, files: { "<path>": { sourceHash, installedHash } } } } }`

**18 plugins tracked:**

| Plugin | Version |
|--------|---------|
| @orqastudio/plugin-agile-workflow | 0.1.4-dev |
| @orqastudio/plugin-cli | (tracked) |
| @orqastudio/plugin-coding-standards | (tracked) |
| @orqastudio/plugin-core-framework | (tracked) |
| @orqastudio/plugin-rust | (tracked) |
| @orqastudio/plugin-software-kanban | (tracked) |
| @orqastudio/plugin-svelte | (tracked) |
| @orqastudio/plugin-tauri | (tracked) |
| @orqastudio/plugin-typescript | (tracked) |
| @orqastudio/plugin-systems-thinking | (tracked) |
| @orqastudio/plugin-githooks | (tracked) |
| @orqastudio/claude-code-connector | (tracked) |
| @orqastudio/plugin-claude | (tracked) |
| @orqastudio/plugin-plugin-dev | (tracked) |
| @orqastudio/plugin-agile-discovery | (tracked) |
| @orqastudio/plugin-agile-planning | (tracked) |
| @orqastudio/plugin-agile-review | (tracked) |
| @orqastudio/plugin-agile-documentation | (tracked) |

### prompt-registry.json

| Field | Value |
|-------|-------|
| Path | `.orqa/prompt-registry.json` |
| Size | 3,654 lines |
| Purpose | Pre-built prompt injection registry for the prompt generation pipeline |

**Structure:** `{ version: 1, built_at, knowledge: [...], sections: [...], contributors: [...], errors: [...] }`

- `knowledge`: 177 entries, each with: id, plugin, source, tier (always/stage-triggered/on-demand), roles, stages, paths, tags, priority, summary, content_file
- `sections`: 2 entries (implementer-role, review-checklist), both plugin-sourced
- `contributors`: 11 entries
- `errors`: 0 entries

### injector-config.json

| Field | Value |
|-------|-------|
| Path | `.orqa/connectors/claude-code/injector-config.json` |
| Purpose | Runtime config for the Claude Code connector |

**Structure:** `{ generated, behavioral_rules, mode_templates: {}, session_reminders }`
- `generated`: "2026-03-26T09:47:03.431Z"
- `behavioral_rules`: Text about lesson artifacts, relationship type validation, epic scoping
- `session_reminders`: Relationship types are schema-driven, session state must include scoped epic

### search.duckdb

| Field | Value |
|-------|-------|
| Path | `.orqa/search.duckdb` |
| Size | ~17.3 MB |
| Purpose | DuckDB database for semantic and regex search (ONNX embeddings) |

---

## 2. .orqa/connectors/

Two files in `.orqa/connectors/claude-code/`:

| Path | Purpose |
|------|---------|
| `.orqa/connectors/claude-code/injector-config.json` | Runtime config for Claude Code connector (described above) |
| `.orqa/connectors/claude-code/hooks/scripts/enforce-background-agents.mjs` | PreToolUse hook that warns (does not block) when Agent tool is called without `run_in_background: true`. Reads stdin JSON, checks for Agent tool calls, emits systemMessage warning referencing RULE-00a8c660 / RULE-532100d9. |

---

## 3. .orqa/principles/

### 3a. vision/ (1 file)

| Path | ID | Type | Title | Status | Created | Description |
|------|----|------|-------|--------|---------|-------------|
| `.orqa/principles/vision/VISION-4893db55.md` | VISION-4893db55 | vision | Product Vision | captured | 2026-03-02 | Comprehensive product vision document. Defines OrqaStudio as an "AI-assisted clarity engine." Covers: mission statement, core principles (clarity before execution, human-led AI, agile as thinking system, artifact-driven reasoning), platform identity (not an AI dev tool), five-layer enforcement architecture (core/project/plugin/community/user), entry modes (problem/idea/goal/chaos), the agile learning loop, three interaction layers (artifact/insight/reasoning), dogfooding principle, AI provider integration, data ownership, and 10 key differentiators. |

### 3b. pillars/ (3 files)

| Path | ID | Type | Title | Status | Created | Gate Questions | Relationships |
|------|----|------|-------|--------|---------|----------------|---------------|
| `.orqa/principles/pillars/PILLAR-c9e0a695.md` | PILLAR-c9e0a695 | pillar | Clarity Through Structure | active | 2026-03-09 | 5 gate questions about visible governance, structured knowledge, understanding-before-action, surfacing hidden info, mechanical enforcement | upholds VISION-4893db55; served-by 15 agents |
| `.orqa/principles/pillars/PILLAR-2acd86c1.md` | PILLAR-2acd86c1 | pillar | Learning Through Reflection | active | 2026-03-09 | 5 gate questions about lesson capture, metrics, retrospective feedback, knowledge accumulation, enforcement gap action | upholds VISION-4893db55; served-by 7 agents |
| `.orqa/principles/pillars/PILLAR-a6a4bbbb.md` | PILLAR-a6a4bbbb | pillar | Purpose Through Continuity | active | 2026-03-13 | 5 gate questions about orientation toward purpose, preventing knowledge loss, explicit scope drift, cognitive burden reduction, visible enforcement gaps | upholds VISION-4893db55; served-by 6 agents |

Each pillar includes: what the pillar means, examples of work that serves it, anti-patterns, relationship to other pillars, and conflict resolution guidance (pillars are equal, conflicts flagged to user).

### 3c. personas/ (4 files)

| Path | ID | Type | Title | Status | Created | Description | Type Match? |
|------|----|------|-------|--------|---------|-------------|-------------|
| `.orqa/principles/personas/DOC-1ff7a9ba.md` | DOC-1ff7a9ba | doc | User Personas | (none) | 2026-03-02 | Full personas document with detailed profiles for Alex (The Lead), Sam (The Practitioner), Jordan (The Independent). Includes demographics, goals, pain points, workflows, design implications, and a comparison table. | **FLAG: type is `doc`, not `persona`. File is in the personas/ directory but uses DOC- prefix and `type: doc` in frontmatter.** |
| `.orqa/principles/personas/PERSONA-c4afd86b.md` | PERSONA-c4afd86b | persona | Alex -- The Lead | active | 2026-03-07 | Primary persona. Brief summary pointing to full document. served-by 3 agents. | Yes |
| `.orqa/principles/personas/PERSONA-477971bf.md` | PERSONA-477971bf | persona | Sam -- The Practitioner | active | 2026-03-07 | Secondary persona. Brief summary pointing to full document. served-by 11 agents. | Yes |
| `.orqa/principles/personas/PERSONA-2721ae35.md` | PERSONA-2721ae35 | persona | Jordan -- The Independent | active | 2026-03-07 | Tertiary persona. Brief summary pointing to full document. served-by 2 agents. | Yes |

**Note:** DOC-1ff7a9ba is the detailed personas reference document. The three PERSONA- files are structured graph nodes that link to agent definitions. The DOC file's type (`doc`) does not match its directory (`personas/`).

### 3d. grounding/ (5 files)

All 5 files share: type=doc, status=captured, created=2026-03-14, relationships=[].

| Path | ID | Title | Target Roles | Content Summary |
|------|-----|-------|-------------|-----------------|
| `.orqa/principles/grounding/DOC-a0490c49.md` | DOC-a0490c49 | Product Purpose -- Agent Grounding | Orchestrator, Planner, Writer | What OrqaStudio is, what each pillar demands, what good looks like (steering, documentation, planning), what goes wrong (losing purpose, treating governance as overhead, optimizing for throughput over clarity) |
| `.orqa/principles/grounding/DOC-bdb520ae.md` | DOC-bdb520ae | Research Principles -- Agent Grounding | Researcher | Source credibility tiers (T1-T3), version matching, confidence levels (Confirmed/Likely/Uncertain/Speculative), artifacts in RES-NNN.md, what goes wrong (opinions as findings, skipping verification, omitting confidence, chat instead of artifacts) |
| `.orqa/principles/grounding/DOC-40b1498a.md` | DOC-40b1498a | Design Principles -- Agent Grounding | Designer | Design for Alex first, UX-first principle, all component states (loading/error/empty/loaded), design language (no emoji, Lucide icons, shadcn tooltips), what goes wrong (designing for devs, adding cognitive load, ignoring state coverage) |
| `.orqa/principles/grounding/DOC-ebf19a16.md` | DOC-ebf19a16 | Code Principles -- Agent Grounding | Implementer, Reviewer | Rust owns domain logic, invoke() is the only bridge, Result everywhere (no unwrap/expect/panic), components display / pages fetch, Svelte 5 runes only, what goes wrong (shortcuts, boundary violations, coding without system understanding) |
| `.orqa/principles/grounding/DOC-0ea4c263.md` | DOC-0ea4c263 | Artifact Principles -- Agent Grounding | Orchestrator, Writer, Researcher, Governance Steward | Graph is the point, orphaned artifacts are failures, every relationship has an inverse, status transitions are gates, frontmatter is the contract, what goes wrong (no relationships, treating frontmatter as paperwork, breaking graph under volume) |

---

## 4. .orqa/documentation/

### 4a. Root Documentation (20 files)

| Path | ID | Type | Title/Name | Category | Description |
|------|----|------|------------|----------|-------------|
| `DOC-13c73ecf.md` | DOC-13c73ecf | doc | Tauri Development Guide | how-to | Developing with Rust and Tauri v2 -- IPC patterns, error handling, testing, coding standards |
| `DOC-1f4aba8f.md` | DOC-1f4aba8f | doc | Three-Layer Enforcement Model | governance | LSP real-time diagnostics, behavioral rules, pre-commit hard gates, demotion model |
| `DOC-22783288.md` | DOC-22783288 | doc | CLI Architecture | architecture | How orqa CLI works: three protocol modes (MCP, LSP, direct), daemon as only long-running service |
| `DOC-2372ed36.md` | DOC-2372ed36 | doc | Rust Development Guide (name) | how-to | Plugin: @orqastudio/plugin-rust |
| `DOC-4554ff3e.md` | DOC-4554ff3e | doc | Software Delivery Guide (name) | how-to | Plugin: @orqastudio/plugin-software-kanban |
| `DOC-586bfa9a.md` | DOC-586bfa9a | doc | Knowledge Auto-Injection | architecture | How knowledge flows into agents: declared injection, semantic search, deduplication, hook enforcement |
| `DOC-7062bce9.md` | DOC-7062bce9 | doc | TypeScript Plugin Skills | plugin | Reference for TypeScript plugin skills -- advanced type patterns |
| `DOC-7068f40a.md` | DOC-7068f40a | doc | Documentation Placement Guide | architecture | Where to write docs and knowledge: plugin dirs for production, .orqa/ for dev |
| `DOC-8cf6ef38.md` | DOC-8cf6ef38 | doc | Dev Environment Setup Guide | onboarding | Submodules, npm linking, multi-repo workflow |
| `DOC-9505a5b5.md` | DOC-9505a5b5 | doc | Tauri Plugin Setup | onboarding | Tauri dev plugin install, toolchain requirements, config generation |
| `DOC-a06f2a63.md` | DOC-a06f2a63 | doc | Svelte Plugin Setup | onboarding | Svelte dev plugin install, dependencies, config generation |
| `DOC-a16b7bc7.md` | DOC-a16b7bc7 | doc | Demoted Rule Stability Tracking | governance | Demotion lifecycle, stability counter, violation logging, auto-delete trigger |
| `DOC-af962d42.md` | DOC-af962d42 | doc | License Policy | reference | Which licenses apply to which components, audit compliance |
| `DOC-db794473.md` | DOC-db794473 | doc | OrqaStudio CLI Reference (name) | reference | Plugin: @orqastudio/plugin-cli |
| `DOC-dd5062c9.md` | DOC-dd5062c9 | doc | Shared Validation Engine | architecture | Single TypeScript lib consumed by 3 adapters (LSP, CLI, pre-commit), schema-driven |
| `DOC-e16aea3b.md` | DOC-e16aea3b | doc | OrqaStudio Agentic Workflow and Enforcement Pipeline | architecture | Core concept: agents, knowledge flow, artifact graph, three-layer enforcement stack |
| `DOC-e89753ad.md` | DOC-e89753ad | doc | OrqaStudio CLI Commands | development | Complete reference: all commands, subcommands, options, usage patterns |
| `DOC-ecc181cb.md` | DOC-ecc181cb | doc | README Standards | reference | Canonical README structure, badges, banner, sections, audit process |
| `DOC-f0a1c9b5.md` | DOC-f0a1c9b5 | doc | Versioning System Guide | how-to | Single-version ecosystem, VERSION file, sync process, dev tag convention |
| `DOC-fd1d12bb.md` | DOC-fd1d12bb | doc | Svelte Development Guide | how-to | Svelte 5 development: runes, component patterns, testing, coding standards |

### 4b. platform/ Documentation (36 files)

| Path | ID | Type | Title/Name | Category |
|------|----|------|------------|----------|
| `platform/DOC-06224bf6.md` | DOC-06224bf6 | doc | Product Governance | concept |
| `platform/DOC-23175cea.md` | DOC-23175cea | doc | IPC Command Catalog | reference |
| `platform/DOC-248d74e2.md` | DOC-248d74e2 | doc | Project Configuration (`.orqa/project.json`) | reference |
| `platform/DOC-266182d2.md` | DOC-266182d2 | doc | Plugin Manifest Schema Reference | reference |
| `platform/DOC-28344cd7.md` | DOC-28344cd7 | doc | Artifact Framework | reference |
| `platform/DOC-31bcfa5c.md` | DOC-31bcfa5c | doc | Design System | reference |
| `platform/DOC-36befd20.md` | DOC-36befd20 | doc | Thinking Mode: Research | platform |
| `platform/DOC-39ea442c.md` | DOC-39ea442c | doc | Prompt Pipeline Architecture | architecture |
| `platform/DOC-4a4241a5.md` | DOC-4a4241a5 | doc | Thinking Mode: Planning | platform |
| `platform/DOC-54594c57.md` | DOC-54594c57 | doc | Priority Assessment | concept |
| `platform/DOC-61ecc85e.md` | DOC-61ecc85e | doc | Status & Workflow | reference |
| `platform/DOC-68a7420e.md` | DOC-68a7420e | doc | The Agentic Development Team | reference |
| `platform/DOC-6d71f083.md` | DOC-6d71f083 | doc | Plugin Artifact Usage Guide | concept |
| `platform/DOC-712f8c56.md` | DOC-712f8c56 | doc | Interaction Patterns | reference |
| `platform/DOC-743f9c71.md` | DOC-743f9c71 | doc | Tech Debt Management Guide (name) | how-to |
| `platform/DOC-7b9b45f0.md` | DOC-7b9b45f0 | doc | Schema Validation Reference (name) | reference |
| `platform/DOC-7bdef310.md` | DOC-7bdef310 | doc | Delegation Reference | reference |
| `platform/DOC-83039175.md` | DOC-83039175 | doc | Thinking Mode: Learning Loop | platform |
| `platform/DOC-939d8636.md` | DOC-939d8636 | doc | Glossary & Domain Model | reference |
| `platform/DOC-ae447f88.md` | DOC-ae447f88 | doc | Artifact Relationships Reference (name) | reference |
| `platform/DOC-b11d4f61.md` | DOC-b11d4f61 | doc | Enforcement Architecture | architecture |
| `platform/DOC-b4099ea3.md` | DOC-b4099ea3 | doc | Information Architecture | architecture |
| `platform/DOC-b95ec6e3.md` | DOC-b95ec6e3 | doc | Thinking Mode: Debugging | platform |
| `platform/DOC-bad8e26f.md` | DOC-bad8e26f | doc | Core Platform Knowledge Catalog | platform |
| `platform/DOC-bcd7fef4.md` | DOC-bcd7fef4 | doc | Error Taxonomy | reference |
| `platform/DOC-bf647454.md` | DOC-bf647454 | doc | Getting Started | onboarding |
| `platform/DOC-bf70068c.md` | DOC-bf70068c | doc | Thinking Mode: Documentation | platform |
| `platform/DOC-c43c7d5d.md` | DOC-c43c7d5d | doc | Content Ownership: Docs, Agents, Knowledge, and Rules | concept |
| `platform/DOC-d9cc1f84.md` | DOC-d9cc1f84 | doc | Orchestration | architecture |
| `platform/DOC-e3a0462c.md` | DOC-e3a0462c | doc | SQLite Schema | reference |
| `platform/DOC-e42efeaf.md` | DOC-e42efeaf | doc | Repository and Package Naming Conventions (name) | reference |
| `platform/DOC-e6fb92b0.md` | DOC-e6fb92b0 | doc | Plugin Architecture | architecture |
| `platform/DOC-ec909ab0.md` | DOC-ec909ab0 | doc | Tool Definitions | reference |
| `platform/DOC-f6c4ac69.md` | DOC-f6c4ac69 | doc | Artifact Workflow | how-to |
| `platform/DOC-f7fb7aa7.md` | DOC-f7fb7aa7 | doc | Thinking Mode: Implementation | platform |
| `platform/DOC-fc36aeec.md` | DOC-fc36aeec | doc | Software Project Setup Guide | onboarding |
| `platform/DOC-fd636a56.md` | DOC-fd636a56 | doc | Thinking Mode: Review | platform |

**Thinking Mode docs (7):** Research, Planning, Learning Loop, Debugging, Documentation, Implementation, Review. All paired with KNOW- counterparts via `synchronised-with` relationships.

### 4c. project/ Documentation (31 files)

| Path | ID | Type | Title | Category |
|------|----|------|-------|----------|
| `project/DOC-05d5eaee.md` | DOC-05d5eaee | doc | How To: Build an OrqaStudio Plugin | how-to |
| `project/DOC-05f59d04.md` | DOC-05f59d04 | doc | Licensing & Ethics | concept |
| `project/DOC-07f98a90.md` | DOC-07f98a90 | doc | Search Engine Architecture | architecture |
| `project/DOC-156f2188.md` | DOC-156f2188 | doc | Artifact Graph SDK | development |
| `project/DOC-1a4f41f7.md` | DOC-1a4f41f7 | doc | Dependency License Compatibility | how-to |
| `project/DOC-2c94f7ba.md` | DOC-2c94f7ba | doc | Svelte Component Tree | architecture |
| `project/DOC-2f2abff9.md` | DOC-2f2abff9 | doc | Wireframe Serving Infrastructure | architecture |
| `project/DOC-32aa55c8.md` | DOC-32aa55c8 | doc | Lesson Promotion Pipeline Architecture | architecture |
| `project/DOC-39e2fb81.md` | DOC-39e2fb81 | doc | Streaming Pipeline | architecture |
| `project/DOC-3d8ed14e.md` | DOC-3d8ed14e | doc | Core Application Architecture | architecture |
| `project/DOC-431006b6.md` | DOC-431006b6 | doc | Contributing | onboarding |
| `project/DOC-52b00632.md` | DOC-52b00632 | doc | MCP Host Interface | architecture |
| `project/DOC-5cdfb8a6.md` | DOC-5cdfb8a6 | doc | Lesson Dashboard UI Spec | (none) |
| `project/DOC-5e486816.md` | DOC-5e486816 | doc | OrqaStudio -- Brand Guidelines | reference |
| `project/DOC-6097aad6.md` | DOC-6097aad6 | doc | Hook Execution Semantics | development |
| `project/DOC-6d9cd337.md` | DOC-6d9cd337 | doc | Responsive Behavior | reference |
| `project/DOC-6e417645.md` | DOC-6e417645 | doc | MVP Feature Specification | (none) |
| `project/DOC-7ac153d1.md` | DOC-7ac153d1 | doc | Enforcement Panel UI Spec | (none) |
| `project/DOC-8cba3805.md` | DOC-8cba3805 | doc | Governance Bootstrap | architecture |
| `project/DOC-8d2e5eef.md` | DOC-8d2e5eef | doc | Agent Team Structure | architecture |
| `project/DOC-9010239f.md` | DOC-9010239f | doc | Process Metrics | reference |
| `project/DOC-921ab420.md` | DOC-921ab420 | doc | Rust Module Architecture | architecture |
| `project/DOC-96e32382.md` | DOC-96e32382 | doc | User Journeys | concept |
| `project/DOC-9814ec3c.md` | DOC-9814ec3c | doc | Coding Standards | reference |
| `project/DOC-9e1f1ebf.md` | DOC-9e1f1ebf | doc | Go-To-Market Strategy | concept |
| `project/DOC-ba2f6335.md` | DOC-ba2f6335 | doc | How To: Write Rust Tests in OrqaStudio | how-to |
| `project/DOC-bb4d4ae3.md` | DOC-bb4d4ae3 | doc | First-Run Setup Wizard | architecture |
| `project/DOC-d2c2063a.md` | DOC-d2c2063a | doc | Development Commands | reference |
| `project/DOC-db5b37dc.md` | DOC-db5b37dc | doc | Development Workflow | how-to |
| `project/DOC-ddba21f4.md` | DOC-ddba21f4 | doc | Centralized Logging Guide | how-to |
| `project/DOC-dff91641.md` | DOC-dff91641 | doc | Plugin-Canonical Architecture Guide | architecture |
| `project/DOC-f5bd63b4.md` | DOC-f5bd63b4 | doc | Dev Controller and OrqaDev Dashboard | architecture |
| `project/DOC-fdb8bc16.md` | DOC-fdb8bc16 | doc | How To: Write Frontend Tests in OrqaStudio | how-to |
| `project/DOC-ffad3f6b.md` | DOC-ffad3f6b | doc | Sub-Agent Support Architecture | architecture |

---

## File Counts Summary

| Area | File Count |
|------|-----------|
| Top-level config | 4 (project.json, manifest.json, prompt-registry.json, search.duckdb) |
| Connectors | 2 (injector-config.json, enforce-background-agents.mjs) |
| Principles: vision | 1 |
| Principles: pillars | 3 |
| Principles: personas | 4 (1 DOC + 3 PERSONA) |
| Principles: grounding | 5 |
| Documentation: root | 20 |
| Documentation: platform | 36 |
| Documentation: project | 31 |
| **Total** | **106** |

---

## Observations (factual only)

1. **Frontmatter inconsistency:** Some docs use `title` field, others use `name` field. Files using `name` instead of `title`: DOC-2372ed36, DOC-4554ff3e, DOC-db794473, DOC-743f9c71, DOC-7b9b45f0, DOC-ae447f88, DOC-e42efeaf.

2. **DOC in personas directory:** DOC-1ff7a9ba has `type: doc` but lives in `.orqa/principles/personas/`. The other 3 files in that directory correctly use `type: persona` and PERSONA- prefixed IDs.

3. **Missing status field:** Many documentation files lack a `status` field in frontmatter. Some have `status: captured`, some have `status: active`, and many have none at all.

4. **Missing category field:** A few docs (DOC-5cdfb8a6, DOC-6e417645, DOC-7ac153d1) have no `category` field.

5. **Plugin-sourced docs:** Some root-level docs (DOC-2372ed36, DOC-4554ff3e, DOC-db794473) have a `plugin` field in frontmatter, indicating they were installed from plugins rather than created locally.

6. **Documentation categories in use:** architecture, reference, how-to, concept, onboarding, governance, development, platform, plugin.

7. **Thinking Modes:** 7 thinking mode docs in platform/ (Research, Planning, Learning Loop, Debugging, Documentation, Implementation, Review), all with `synchronised-with` relationships to KNOW- counterparts.
