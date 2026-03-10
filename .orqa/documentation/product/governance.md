---
id: DOC-039
title: Product Governance
description: Product governance criteria defining how features are evaluated against the project's active pillars.
created: "2026-03-02"
updated: "2026-03-07"
---

**Date:** 2026-03-07

> OrqaStudio™ is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

## Feature Governance Framework

Every feature, task, and implementation must pass the Pillar Alignment Test before work begins.

### Pillar Alignment Test

Active pillars are defined as structured artifacts in `.orqa/planning/pillars/PILLAR-NNN.md`. Each pillar has:

- **`title`** — The principle name
- **`description`** — What the pillar means
- **`test-questions`** — Questions to evaluate whether work serves this pillar
- **`priority`** — Conflict resolution order (lower number = higher priority)

To evaluate a feature, read each active pillar's `test-questions` and check if the feature can answer "yes" to at least one question from at least one pillar.

### Rejection Criteria

Reject any feature that:

- Does not serve any active pillar (cannot answer "yes" to any pillar's test questions)
- Adds complexity without serving a pillar's intent
- Cannot articulate which pillar(s) it serves and how
- Is a generic tool feature with no connection to any pillar

### Pillar Conflict Resolution

When pillars conflict, the pillar with the lower `priority` number takes precedence. Check each pillar's `priority` field in its frontmatter.

## Foundational Principles

The following are immutable without explicit user approval:

1. **Pillar framework** — Every feature serves at least one active pillar defined in `.orqa/planning/pillars/`
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
- All governance artifacts (rules, agents, skills, hooks, decisions, lessons, etc.) live as markdown files under `.orqa/`
- Users can edit project-layer files in OrqaStudio, in a text editor, or through any compatible CLI
- OrqaStudio adds a visual management layer on top of this file-based governance; it does not replace the file format with a proprietary one
- For CLI compatibility (e.g., Claude Code), OrqaStudio can optionally create symlinks in `.claude/` that point to the corresponding `.orqa/` paths. This is a project configuration option, not a requirement. Future compatibility layers for other tools (Cursor, Continue, etc.) could follow the same symlink pattern.

### Canon Artifacts vs Project Artifacts

Governance artifacts fall into two categories based on who manages them:

| Category | Managed By | Editable? | Behaviour on App Update |
|----------|-----------|-----------|------------------------|
| **Canon** | App (centrally) | No — warning comment at top of each file | Overwritten with latest version |
| **Project** | User (and AI) | Yes — fully editable | Preserved, never overwritten |

**Canon artifacts** are installed into `.orqa/` when a project is set up and updated when the app updates. They encode the core systems thinking and agile planning philosophy that is OrqaStudio's value proposition. They carry a warning comment discouraging manual editing:

```markdown
<!-- CANON ARTIFACT — managed by OrqaStudio. Manual edits will be overwritten on app update. -->
```

**Project artifacts** are created and managed by the user and AI. They extend and customise the canon framework for the specific project context. They are additive — they can add new rules, new agent behaviours, and new artifact types, but they cannot weaken or override canon rules.

## Governance Concept Taxonomy

OrqaStudio's governance framework is built from five concept types. Each serves a distinct purpose — conflating them creates confusion.

| Concept | What It Is | What It Does | Examples |
|---------|-----------|-------------|---------|
| **Agent** | A role — defines *how* someone works | Specifies process, ownership boundaries, required reading, tool access | Orchestrator, Implementer, Reviewer |
| **Skill** | Knowledge — defines *what* someone knows | Provides domain expertise, patterns, decision frameworks | `rust-async-patterns`, `diagnostic-methodology`, `project-setup` |
| **Rule** | A constraint — defines *what must or must not happen* | Enforces standards, prevents known failure patterns | `no-stubs`, `error-ownership`, `documentation-first` |
| **Hook** | Automation — runs at process events | Executes checks, loads context, enforces gates | Session start, pre-commit, skill loading |
| **Lesson** | A learned pattern — captures *what was discovered* | Records mistakes, non-obvious behaviors, reusable insights | `IMPL-001: unwrap causes panics in sidecar` |

### Agent vs Skill Decision Framework

When adding new capability to the governance framework, use this decision tree:

1. **Does it define a new way of working (process, ownership, delegation)?** → Agent
2. **Does it teach domain knowledge that multiple roles could use?** → Skill
3. **Does it constrain what can or cannot be done?** → Rule
4. **Does it automate something at a process event?** → Hook
5. **Does it capture something discovered during work?** → Lesson

**Key principle:** Agents are portable across projects. Skills make agents domain-specific. A single Implementer agent becomes a Backend Engineer, Frontend Engineer, or Data Engineer depending on which skills are loaded.

## Canon Governance Artifacts

The following artifacts are shipped with OrqaStudio as the canon layer. They are the same for every project and every industry:

| Artifact | Path | Purpose |
|----------|------|---------|
| 7 universal agent definitions | `.orqa/team/agents/*.md` | Universal roles: Orchestrator, Researcher, Planner, Implementer, Reviewer, Writer, Designer |
| Canon enforcement rules | `.orqa/governance/rules/*.md` | Behavioral constraints: no stubs, error ownership, documentation-first, etc. |
| Canon skills | `.orqa/team/skills/` | Domain knowledge: diagnostic-methodology, code-quality-review, security-audit, etc. |
| Lifecycle hooks | `.orqa/governance/hooks/` | Process automation: session start, pre-commit |
| Artifact framework | `.orqa/documentation/product/artifact-framework.md` | How artifacts are created, tracked, and promoted |

These artifacts collectively implement the agile learning loop (chaos → clarity → execution → reflection → improved clarity) as executable, enforceable governance.

### Skill-Based Specialisation

Universal roles load domain skills at runtime. The same 7 roles adapt to any project by loading different skills:

| Role | + Skills | Becomes |
|------|----------|---------|
| Implementer | `rust-async-patterns`, `tauri-v2` | Backend specialist |
| Implementer | `svelte5-best-practices`, `tailwind-design-system` | Frontend specialist |
| Reviewer | `code-quality-review` | Code quality reviewer |
| Reviewer | `security-audit` | Security reviewer |
| Reviewer | `test-engineering` | Test engineer |

Project-specific rules, additional skills, and domain-specific artifact configurations are project artifacts and live alongside the canon set, clearly separated.

## Bootstrap Phase — CLI-Only Governance

> **This section documents the current state.** The governance framework uses native `.orqa/` artifacts managed through the Claude Code CLI. OrqaStudio will supplement this with a visual UI once the MVP is functional. The CLI-only phase is, by design, bootstrapping the canon layer — the same canon layer that will ship with the app and be installed into every new project.

The current process uses:

- **Claude Code CLI** as the orchestrator (via `.claude/` symlinks to `.orqa/`)
- **7 universal agent roles** defined as `.orqa/team/agents/*.md` files, specialised via skills
- **20+ rules** enforced via `.orqa/governance/rules/*.md`
- **30+ skills** in `.orqa/team/skills/` (canon + project-specific)
- **Hooks** in `.orqa/governance/hooks/` (session start, pre-commit)
- **Git worktree workflow** for task isolation

The governance works, but it is invisible — artifacts live in dotfiles and terminal output. This is exactly the problem OrqaStudio exists to solve: making governance visible and structured, and improving it over time through structured reflection. See `.orqa/planning/pillars/` for the active pillar definitions.

Everything being developed in this bootstrap phase is destined to become the canon layer. The agents, rules, hooks, and artifact framework defined here are not OrqaStudio-specific — they represent the reusable, opinionated framework that ships with every new project OrqaStudio creates.

### The Dogfooding Milestone

Once the MVP delivers a working conversation UI with basic Claude integration (Phase 1 complete), **OrqaStudio's UI becomes the primary way to manage governance for its own development.** The CLI remains available for all development tasks. This means:

1. **Process artifacts become visible** — Agents, rules, skills, and docs are browsed and edited in OrqaStudio's UI, supplementing the raw file access that the CLI provides
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

- [ ] Display and edit process artifacts (agents, rules, skills) through its UI
- [ ] Run a conversation with Claude and display streaming responses
- [ ] Show scanner/verification results in the UI
- [ ] Capture lessons through the interface
- [ ] Persist session history in SQLite
- [ ] Detect and display Claude Code CLI status and version

Until these criteria are met, governance is managed through direct file editing and CLI sessions. After these criteria are met, OrqaStudio provides the visual layer while the CLI (via `.claude/` symlinks) continues to operate on the same `.orqa/` artifacts.

## Plugin Architecture

Plugins extend OrqaStudio's artifact system and display system without modifying the canon layer. They allow the framework to serve different industries and use cases while keeping the core enforcement model intact.

### What Plugins Can Do

**Extend the artifact system:**
- Register new artifact types in `project.json` under the `artifacts` key
- Register artifact groups that organise multiple types into a named UI section
- Add new artifacts to both the built-in set and plugin-defined types

**Extend the display system:**
- Provide custom views for artifact types (Gantt charts, kanban boards, graph visualisations, timelines)
- Customise how the app displays and navigates artifacts

### What Plugins Cannot Do

- Override or weaken canon rules
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

The canon framework — structured thinking, agile learning loop, lesson capture, quality enforcement — applies to all of them. The plugin layer is what makes the experience feel native to each domain.

## Decision Process

1. Feature request received
2. Pillar alignment verified (reject if neither)
3. Architecture decision check (new AD needed?)
4. Documentation written and approved
5. Implementation planned (UX-first)
6. Implementation delegated to agents
7. Three-reviewer verification gate
8. Merge and deploy
