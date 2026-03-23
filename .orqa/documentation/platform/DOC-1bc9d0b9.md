---
id: DOC-1bc9d0b9
title: Product Governance
category: concept
description: "Product governance criteria defining how features are evaluated against the project's active pillars."
created: 2026-03-02
updated: 2026-03-07
sort: 2
relationships:
  - target: RULE-1e8a1914
    type: documents
    rationale: This document defines the feature governance framework and pillar alignment criteria that RULE-1e8a1914 enforces
---

**Date:** 2026-03-07

> OrqaStudio™ is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

## Feature Governance Framework

Every feature, task, and implementation must pass the Pillar Alignment Test before work begins.

### Pillar Alignment Test

Active pillars are defined as structured artifacts in `.orqa/process/pillars/PILLAR-NNN.md`. Each pillar has:

- **`title`** — The principle name
- **`description`** — What the pillar means
- **`gate`** — Questions to evaluate whether work serves this pillar

To evaluate a feature, read each active pillar's `gate` questions and check if the feature can answer "yes" to at least one question from at least one pillar. Pillars are equal in importance — when they conflict, flag to the user.

### Rejection Criteria

Reject any feature that:

- Does not serve any active pillar (cannot answer "yes" to any pillar's gate questions)
- Adds complexity without serving a pillar's intent
- Cannot articulate which pillar(s) it serves and how
- Is a generic tool feature with no connection to any pillar

### Pillar Conflict Resolution

When pillars conflict, the pillar with the lower `priority` number takes precedence. Check each pillar's `priority` field in its frontmatter.

## Foundational Principles

The following are immutable without explicit user approval:

1. **Pillar framework** — Every feature serves at least one active pillar defined in `.orqa/process/pillars/`
2. **Primary audience** — Structured thinkers: product managers, tech leads, and anyone who needs to turn complex situations into structured understanding
3. **Human approval gate** — Plans must be approved by a human before execution begins. AI assists and executes, but humans authorise.
4. **Technology stack** — Tauri v2 + Svelte 5 + Rust + SQLite
5. **IPC boundary** — Tauri commands are the only frontend-backend interface
6. **Composability** — External integrations (AI providers, MCP servers) connect through provider-agnostic interfaces. The Rust core speaks a neutral protocol; provider-specific logic lives in swappable sidecar processes. The architecture must not hard-couple to any single AI provider.
7. **UX-first design** — User journeys drive backend requirements; the UI should be approachable for leads and PMs, not just developers
8. **Documentation-first workflow** — Document → Approve → Implement → Verify
9. **Error propagation** — Result types everywhere, no unwrap in production

## Governance Artifact Format

Governance artifacts are markdown files stored on disk under `.orqa/` (the default root, configurable via project settings). OrqaStudio reads and writes these files through its UI. OrqaStudio's SQLite database is a derived cache, not the source of truth — the `.orqa/` files on disk are authoritative.

This means:
- All governance artifacts (rules, agents, knowledge, hooks, decisions, lessons, etc.) live as markdown files under `.orqa/`
- Users can edit project-layer files in OrqaStudio, in a text editor, or through any compatible CLI
- OrqaStudio adds a visual management layer on top of this file-based governance; it does not replace the file format with a proprietary one
- For CLI compatibility (e.g., Claude Code), OrqaStudio can optionally create symlinks in `.claude/` that point to the corresponding `.orqa/` paths. This is a project configuration option, not a requirement. Future compatibility layers for other tools (Cursor, Continue, etc.) could follow the same symlink pattern.

### Core Artifacts vs Project Artifacts

Governance artifacts fall into categories based on who manages them and their origin layer:

| Category | Managed By | Editable? | Behaviour on App Update |
|----------|-----------|-----------|------------------------|
| **Core** | App (centrally) | No — warning comment at top of each file | Overwritten with latest version |
| **Project** | User (and AI) | Yes — fully editable | Preserved, never overwritten |
| **Plugin** | Plugin author (1st party official) | No — managed by plugin | Updated with plugin version |
| **Community** | Community contributors | Read-only by default | Updated from community source |
| **User** | Individual user | Yes — fully editable | Preserved, never overwritten |

**Core artifacts** are installed into `.orqa/` when a project is set up and updated when the app updates. They encode the core systems thinking and agile planning philosophy that is OrqaStudio's value proposition. They carry a warning comment discouraging manual editing:

```markdown
<!-- CORE ARTIFACT — managed by OrqaStudio. Manual edits will be overwritten on app update. -->
```

**Project artifacts** are created and managed by the user and AI. They extend and customise the core framework for the specific project context. They are additive — they can add new rules, new agent behaviours, and new artifact types, but they cannot weaken or override core rules.

## Governance Concept Taxonomy

OrqaStudio's governance framework is built from five concept types. Each serves a distinct purpose — conflating them creates confusion.

| Concept | What It Is | What It Does | Examples |
|---------|-----------|-------------|---------|
| **Agent** | A role — defines *how* someone works | Specifies process, ownership boundaries, required reading, tool access | Orchestrator, Implementer, Reviewer |
| **Knowledge** | Knowledge — defines *what* someone knows | Provides domain expertise, patterns, decision frameworks | `rust-async-patterns`, `diagnostic-methodology`, `project-setup` |
| **Rule** | A constraint — defines *what must or must not happen* | Enforces standards, prevents known failure patterns | `no-stubs`, `error-ownership`, `documentation-first` |
| **Hook** | Automation — runs at process events | Executes checks, loads context, enforces gates | Session start, pre-commit, knowledge loading |
| **Lesson** | A learned pattern — captures *what was discovered* | Records mistakes, non-obvious behaviors, reusable insights | `[IMPL-eb748de2](IMPL-eb748de2): unwrap causes panics in sidecar` |

### Agent vs Knowledge Decision Framework

When adding new capability to the governance framework, use this decision tree:

1. **Does it define a new way of working (process, ownership, delegation)?** → Agent
2. **Does it teach domain knowledge that multiple roles could use?** → Knowledge
3. **Does it constrain what can or cannot be done?** → Rule
4. **Does it automate something at a process event?** → Hook
5. **Does it capture something discovered during work?** → Lesson

**Key principle:** Agents are portable across projects. Knowledge artifacts make agents domain-specific. A single Implementer agent becomes a Backend Engineer, Frontend Engineer, or Data Engineer depending on which knowledge is loaded.

## Core Governance Artifacts

The following artifacts are shipped with OrqaStudio as the core layer. They are the same for every project and every industry:

| Artifact | Path | Purpose |
|----------|------|---------|
| 7 universal agent definitions | `.orqa/process/agents/*.md` | Universal roles: Orchestrator, Researcher, Planner, Implementer, Reviewer, Writer, Designer |
| Core enforcement rules | `.orqa/process/rules/*.md` | Behavioral constraints: no stubs, error ownership, documentation-first, etc. |
| Core knowledge | `.orqa/process/knowledge/` | Domain knowledge: diagnostic-methodology, code-quality-review, security-audit, etc. |
| Lifecycle hooks | `.orqa/process/hooks/` | Process automation: session start, pre-commit |
| Artifact framework | `.orqa/documentation/about/artifact-framework.md` | How artifacts are created, tracked, and promoted |

These artifacts collectively implement the agile learning loop (chaos → clarity → execution → reflection → improved clarity) as executable, enforceable governance.

### Knowledge-Based Specialisation

Universal roles load domain knowledge at runtime. The same 7 roles adapt to any project by loading different knowledge artifacts:

| Role | + Knowledge | Becomes |
|------|-------------|---------|
| Implementer | `rust-async-patterns`, `tauri-v2` | Backend specialist |
| Implementer | `svelte5-best-practices`, `tailwind-design-system` | Frontend specialist |
| Reviewer | `code-quality-review` | Code quality reviewer |
| Reviewer | `security-audit` | Security reviewer |
| Reviewer | `test-engineering` | Test engineer |

Project-specific rules, additional knowledge artifacts, and domain-specific artifact configurations are project artifacts and live alongside the core set, clearly separated.

## Bootstrap Phase — CLI-Only Governance

> **This section documents the current state.** The governance framework uses native `.orqa/` artifacts managed through the Claude Code CLI. OrqaStudio will supplement this with a visual UI once the MVP is functional. The CLI-only phase is, by design, bootstrapping the core layer — the same core layer that will ship with the app and be installed into every new project.

The current process uses:

- **Claude Code CLI** as the orchestrator (via `.claude/` symlinks to `.orqa/`)
- **7 universal agent roles** defined as `.orqa/process/agents/*.md` files, specialised via knowledge artifacts
- **20+ rules** enforced via `.orqa/process/rules/*.md`
- **30+ knowledge artifacts** in `.orqa/process/knowledge/` (core + project-specific)
- **Hooks** in `.orqa/process/hooks/` (session start, pre-commit)
- **Git worktree workflow** for task isolation

The governance works, but it is invisible — artifacts live in dotfiles and terminal output. This is exactly the problem OrqaStudio exists to solve: making governance visible and structured, and improving it over time through structured reflection. See `.orqa/process/pillars/` for the active pillar definitions.

Everything being developed in this bootstrap phase is destined to become the core layer. The agents, rules, hooks, and artifact framework defined here are not OrqaStudio-specific — they represent the reusable, opinionated framework that ships with every new project OrqaStudio creates.

### The Dogfooding Milestone

Once the MVP delivers a working conversation UI with basic Claude integration, **OrqaStudio's UI becomes the primary way to manage governance for its own development.** The CLI remains available for all development tasks. This means:

1. **Process artifacts become visible** — Agents, rules, knowledge, and docs are browsed and edited in OrqaStudio's UI, supplementing the raw file access that the CLI provides
2. **Learning loops are active** — Lessons and retrospective entries are captured through OrqaStudio's interface, not manual markdown editing
3. **Governance is visible through OrqaStudio** — Scanner results, verification gates, and compliance checks surface in the dashboard, supplementing terminal output

### Why Dogfooding Matters

Using OrqaStudio to build OrqaStudio creates a natural feedback loop:

- **Deficiencies surface immediately** — If a feature is missing or painful, the team experiences it daily
- **Features are validated by real use** — Every new feature is tested in production (on this project) before being considered "done"
- **Priority is self-evident** — The most painful gaps in the tool become the highest-priority features organically
- **The learning loop bootstraps itself** — OrqaStudio's reflective learning capabilities improve OrqaStudio's own governance, which improves OrqaStudio

### Transition Criteria

OrqaStudio's UI becomes the primary governance management tool when it can:

- [ ] Display and edit process artifacts (agents, rules, knowledge) through its UI
- [ ] Run a conversation with Claude and display streaming responses
- [ ] Show scanner/verification results in the UI
- [ ] Capture lessons through the interface
- [ ] Persist session history in SQLite
- [ ] Detect and display Claude Code CLI status and version

Until these criteria are met, governance is managed through direct file editing and CLI sessions. After these criteria are met, OrqaStudio provides the visual layer while the CLI (via `.claude/` symlinks) continues to operate on the same `.orqa/` artifacts.

## Plugin Architecture

Plugins extend OrqaStudio's artifact system and display system without modifying the core layer. They allow the framework to serve different industries and use cases while keeping the core enforcement model intact.

### What Plugins Can Do

**Extend the artifact system:**
- Register new artifact types in `project.json` under the `artifacts` key
- Register artifact groups that organise multiple types into a named UI section
- Add new artifacts to both the built-in set and plugin-defined types

**Extend the display system:**
- Provide custom views for artifact types (Gantt charts, kanban boards, graph visualisations, timelines)
- Customise how the app displays and navigates artifacts

### What Plugins Cannot Do

- Override or weaken core rules
- Modify the underlying enforcement model
- Change how the agile learning loop operates

### Plugin Structure

A plugin is:
1. One or more entries in `project.json`'s `artifacts` array — defining types, groups, paths, labels, and icons
2. Files dropped into the registered paths — the artifact files themselves
3. Optionally, a custom view component registered for a specific artifact key

The app requires certain canonical artifact keys to exist (`lessons`, `rules`, `decisions`, `milestones`, `epics`, `tasks`, `ideas`, `agents`). Their paths, labels, icons, and grouping are configurable. Everything else is a project or plugin extension.

### Domain Adaptation

Plugins are how OrqaStudio adapts to different domains without forking the product:

| Domain | Likely plugin additions |
|--------|------------------------|
| Software development | CI/CD status views, code metrics dashboards |
| Healthcare | Patient pathway templates, regulatory compliance artifact types |
| Consulting | Engagement frameworks, stakeholder map artifact types |
| Personal planning | Habit tracking views, journaling artifact types |

The core framework — structured thinking, agile learning loop, lesson capture, quality enforcement — applies to all of them. The plugin layer is what makes the experience feel native to each domain.

## Governance Hub (Multi-Tool Projects)

When a project uses multiple AI tools (Claude Code, Cursor, Copilot, Aider, etc.), each tool has its own configuration format for instructions, rules, and context. Without coordination, these configurations drift apart — rules added in one tool are missing in another, leading to inconsistent behaviour.

OrqaStudio can act as a **governance hub** for such projects: a single source of truth for rules, agent instructions, and process standards that distributes governance to each tool in its native format.

**This is a capability, not the product's identity.** OrqaStudio is a clarity engine for structured thinking. The governance hub activates when a project's context calls for it — when multiple AI tools need to share the same standards.

### How It Works

```
.orqa/process/rules/       ← Single source of truth
    │
    ├── .claude/rules/        ← Symlinks (Claude Code reads these)
    ├── .cursorrules          ← Generated (Cursor reads this)
    ├── .github/copilot-*.md  ← Generated (Copilot reads this)
    └── .aider.conf.yml       ← Generated (Aider reads this)
```

1. **Rules live in `.orqa/`** — One canonical set of governance artifacts
2. **Tool-specific configs are derived** — Either symlinked or generated from the canonical set
3. **Changes flow outward** — Edit in `.orqa/`, tool configs update automatically
4. **Each tool reads its native format** — No tool needs to know about `.orqa/`

### Coexistence Model

| Tool | Native Config | Integration Method |
|------|-------------|-------------------|
| **Claude Code** | `.claude/`, `CLAUDE.md`, `AGENTS.md` | Symlinks: `.claude/` → `.orqa/` |
| **Cursor** | `.cursorrules`, `.cursor/rules/` | Generated from `.orqa/process/rules/` |
| **GitHub Copilot** | `.github/copilot-instructions.md` | Generated from `.orqa/process/rules/` |
| **Aider** | `.aider.conf.yml`, `CONVENTIONS.md` | Generated from `.orqa/process/rules/` |

Claude Code is the deepest integration because both tools use the same markdown-based governance format — symlinks mean both read the same files with no generation step needed. For other tools, OrqaStudio generates their native config from `.orqa/` content.

### Drift Reconciliation

When governance changes in `.orqa/`, derived configs may be stale. OrqaStudio handles this through:

1. **Manual sync** — User triggers regeneration from the OrqaStudio UI
2. **File watcher** (future) — Detects changes in `.orqa/` and regenerates affected configs
3. **Commit hook** (future) — Regenerates configs as part of the pre-commit check

### Setup Flow

When OrqaStudio is added to a project that already uses AI tools:

1. **Detect** — `project-inference` skill scans for existing tool configurations
2. **Import** — `project-migration` skill reads existing configs and maps them to `.orqa/` artifacts
3. **Deduplicate** — Existing governance that matches core rules is linked, not duplicated
4. **Link** — Symlinks and generation configs are set up
5. **Verify** — User reviews the imported governance and confirms

## Decision Process

1. Feature request received
2. Pillar alignment verified (reject if neither)
3. Architecture decision check (new AD needed?)
4. Documentation written and approved
5. Implementation planned (UX-first)
6. Implementation delegated to agents
7. Three-reviewer verification gate
8. Merge and deploy
