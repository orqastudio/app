---
title: "System vs Project Artifacts"
category: product
tags: []
created: 2026-03-05
updated: 2026-03-05
---

# System vs Project Artifacts

**Date:** 2026-03-05

Orqa Studio distinguishes between **system artifacts** and **project artifacts**. System artifacts encode the methodology — how to run a managed agentic development process — and are shared across every project that uses Orqa Studio. Project artifacts encode project-specific knowledge — the technology stack, architecture decisions, and conventions for one particular codebase.

This separation ensures that governance methodology is portable. A new project does not start from scratch; it inherits proven process from the system and adds its own technical context on top.

---

## System vs Project Classification

### System Artifacts (Methodology)

System artifacts describe how to do managed agentic development correctly. They apply universally — any project using Orqa Studio benefits from them, regardless of the technology stack.

| Artifact | Location | What It Contains |
|----------|----------|-----------------|
| System rules | `.claude/rules/` | Behavioral constraints that apply to all agents on all projects: no stubs, error ownership, end-to-end completeness, documentation-first, vision alignment |
| System agents | `.claude/agents/` | Generic role definitions: backend-engineer, frontend-engineer, code-reviewer, etc. Role defines *how* the agent works; project knowledge is injected via skills |
| System skills | `.claude/skills/` | Technology patterns that are portable: Svelte 5 runes, Rust async, Tauri v2, ChunkHound, planning methodology |
| Process docs | `docs/process/` | Governance framework, orchestration model, DoR/DoD, content governance |
| AGENTS.md | `AGENTS.md` | Cross-agent project instructions (imported by CLAUDE.md) |
| CLAUDE.md | `.claude/CLAUDE.md` | Orchestrator configuration |

### Project Artifacts (Technical Context)

Project artifacts describe the specific codebase the agents are working on. They are generated or configured per project and live in the project's repository.

| Artifact | Location | What It Contains |
|----------|----------|-----------------|
| Architecture decisions | `docs/architecture/decisions.md` | Technology stack choices immutable to this project (Tauri v2, SQLite, Svelte 5) |
| Product docs | `docs/product/` | Vision, personas, roadmap, MVP specification — specific to this product |
| Development docs | `docs/development/` | Coding standards, getting-started guide — specific to this stack |
| UI specs | `docs/ui/` | Component inventory, design system, interaction patterns — specific to this app |
| Lessons | `.orqa/lessons/*.md` | Mistakes observed in this codebase, not universal |
| SQLite schema | `docs/architecture/sqlite-schema.md` | This project's database design |
| Project config | `.orqa/config.json` | Per-project Orqa Studio settings (scan paths, promotion threshold, etc.) |

---

## System Rules — Rationale

The following rules are classified as system-level because they apply to every managed agentic development project, regardless of technology:

| Rule | Classification Rationale |
|------|--------------------------|
| `vision-alignment.md` | The Two-Pillar framework is Orqa Studio's methodology. Any project using Orqa Studio must pass features through this test. |
| `documentation-first.md` | Documentation-first is a universal principle of managed agentic development. It does not depend on the tech stack. |
| `no-stubs.md` | Stub prevention applies to any implementation. Not specific to Rust or Svelte. |
| `error-ownership.md` | All errors are the agent's responsibility — universal. |
| `honest-reporting.md` | Honest status reporting applies to any task, any technology. |
| `plan-mode-compliance.md` | Architectural compliance verification before coding applies universally. |
| `git-workflow.md` | Worktree lifecycle and data loss prevention apply to any git-based project. |
| `end-to-end-completeness.md` | The "all layers in the same commit" principle applies to any full-stack app, not just Tauri + Svelte. |
| `lessons-learned.md` | The learning loop structure applies to any managed development process. |
| `no-aliases-or-hacks.md` | Fix root causes, not symptoms — universal. |
| `chunkhound-usage.md` | ChunkHound is the system's code search tool — universal for any project indexed with it. |
| `pillar-alignment-docs.md` | Documentation pillar alignment applies to any project following Orqa Studio governance. |
| `skill-enforcement.md` | The skill loading protocol applies to all agents. |
| `required-reading.md` | Required Reading protocol applies to all agents. |
| `root-cleanliness.md` | Project root discipline applies to any project. |
| `reusable-components.md` | The shared component library principle applies to any frontend project. (Component names are Orqa-specific but the principle is system-level.) |
| `testing-standards.md` | 80%+ coverage and test isolation requirements apply universally. |

### Rules That Are System-Level But Reference Project Technology

Some system rules reference technology choices (Rust, Svelte 5, SQLite) because Orqa Studio itself is built on those choices. For other projects, the technology-specific content would differ while the principle remains the same.

| Rule | System Principle | Project-Specific Content |
|------|-----------------|--------------------------|
| `architecture-decisions.md` | Read architecture decisions before coding | The specific decisions (Tauri v2, Svelte 5) are Orqa-specific |
| `coding-standards.md` | Read coding standards before coding | The Rust/Svelte details are Orqa-specific |
| `vision-alignment.md` | Features must serve the product vision | The Two Pillars are Orqa-specific |

---

## System Agents

All agent definitions in `.claude/agents/` are system-level. They define generic roles, not technology-specific implementations. The technology context is provided by the skills each agent loads.

| Agent | Role (Generic) | Project Context Injected Via |
|-------|---------------|------------------------------|
| `backend-engineer` | Implements backend features | `rust-async-patterns`, `tauri-v2` skills + Required Reading: `docs/architecture/` |
| `frontend-engineer` | Implements frontend components | `svelte5-best-practices`, `typescript-advanced-types` skills + Required Reading: `docs/ui/` |
| `code-reviewer` | Reviews code quality | All technology skills + coding standards doc |
| `documentation-writer` | Creates and maintains docs | `architecture` skill + governance docs |
| `data-engineer` | Designs database layer | `rust-async-patterns` skill + schema docs |
| `agent-maintainer` | Maintains governance framework | All skills + all governance docs |

An agent definition that says "write Rust code this way" is encoding project knowledge. The correct pattern is: the agent says "write backend code according to the coding standards", and the `rust-async-patterns` skill provides the Rust-specific guidance.

---

## System Skills vs Project Skills

| Type | Examples | Where It Lives |
|------|----------|---------------|
| **System skills** (technology patterns, portable) | `chunkhound`, `planning`, `svelte5-best-practices`, `rust-async-patterns`, `tauri-v2`, `architecture` | `.claude/skills/` |
| **Project skills** (would be project-specific knowledge) | "How Orqa Studio's session model works", "The governance bootstrap flow" | Does NOT exist as a skill — lives in `docs/` and is Referenced Reading in agent files |

Project knowledge should not be encoded as skills. Skills are portable technology patterns. Project-specific context belongs in documentation, referenced by agent Required Reading lists.

---

## Project Initialization Flow

When Orqa Studio opens a new project for the first time:

1. **Scan** — The governance scanner walks the project directory looking for:
   - Existing `.claude/` artifacts (rules, agents, hooks, skills)
   - Other tool governance files (Cursor rules, Copilot instructions, Continue config)
   - Documentation structure (`docs/`, `README.md`, ADRs)

2. **Compare** — The scanner compares what it found against the system artifact baseline:
   - Which system rules are present? Which are missing?
   - Which system agents are defined? Are they up to date?
   - Does the project have architecture docs? Product docs?

3. **Propose** — Claude (via sidecar) generates a recommendation set:
   - "Copy these system rules — they apply universally"
   - "These Cursor rules could be translated to Claude Code format"
   - "These architecture decisions should be created based on the README"
   - "The following governance docs are missing — here are starter templates"

4. **Apply** — The user reviews and approves recommendations. Orqa Studio writes the approved artifacts to `.claude/` and `docs/`.

The output is a project that has both the system methodology baseline and its own technical context layer.

---

## CLI Compatibility Model

Orqa Studio governance artifacts are native Claude Code artifacts. Everything Orqa Studio manages works in a plain CLI session without the app running.

| Artifact | Works in CLI without Orqa Studio |
|----------|----------------------------------|
| `.claude/rules/*.md` | Yes — injected into context automatically |
| `.claude/agents/*.md` | Yes — used by orchestrator |
| `.claude/hooks/*.sh` | Yes — executed at lifecycle events |
| `.claude/hookify.*.local.md` | Yes — real-time pattern enforcement |
| `.orqa/lessons/*.md` | PLANNED — readable by agents via file tools |
| `docs/` | Yes — readable by agents via file tools |

Orqa Studio adds visual management, dashboards, and enhanced UX on top of these native artifacts. It never creates vendor lock-in.

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Self-Learning Loop | The system/project distinction ensures that lessons learned in one project can inform the system baseline — promoted patterns become system-level rules applicable across all projects. |
| Process Governance | Defining what is system-scoped vs project-scoped makes governance visible and portable. The initialization flow ensures new projects start with proven methodology rather than an empty governance structure. |

---

## Related Documents

- `docs/process/content-governance.md` — Content ownership model (what belongs in docs, rules, skills, agents)
- `docs/architecture/enforcement.md` — Enforcement engine with system vs project scope
- `docs/architecture/governance-bootstrap.md` — Governance bootstrap scan, compare, propose, apply workflow
- `docs/product/vision.md` — Product vision that defines the Two Pillars
