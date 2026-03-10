---
title: "System vs Project Artifacts"
description: "Distinction between system-level and project-level artifacts and their management boundaries."
created: 2026-03-05
updated: 2026-03-08
---

**Date:** 2026-03-05

OrqaStudio™ is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans. It distinguishes between **system artifacts** and **project artifacts**. System artifacts encode the methodology — how to run a structured thinking and development process — and are shared across every project that uses OrqaStudio. Project artifacts encode project-specific knowledge — the technology stack, architecture decisions, and conventions for one particular codebase.

This separation ensures that governance methodology is portable. A new project does not start from scratch; it inherits proven process from the system and adds its own technical context on top.

---

## System vs Project Classification

### System Artifacts (Methodology)

System artifacts describe how to do managed agentic development correctly. They apply universally — any project using OrqaStudio benefits from them, regardless of the technology stack.

| Artifact | Location | What It Contains |
|----------|----------|-----------------|
| System rules | `.orqa/rules/` | Behavioral constraints that apply to all agents on all projects: no stubs, error ownership, end-to-end completeness, documentation-first, vision alignment. Canon rules are app-managed and non-editable. |
| System agents | `.orqa/agents/` | Generic role definitions: backend-engineer, frontend-engineer, code-reviewer, etc. Role defines *how* the agent works; project knowledge is injected via skills. Canon agents are app-managed. |
| System skills | `.orqa/skills/` | Technology patterns that are portable: Svelte 5 runes, Rust async, Tauri v2, ChunkHound, planning methodology. Canon skills are app-managed. |
| Process docs | `.orqa/documentation/process/` | Governance framework, orchestration model, DoR/DoD, content governance |
| AGENTS.md | `AGENTS.md` | Cross-agent project instructions (imported by orchestrator configuration) |
| Orchestrator config | `CLAUDE.md` (bootstrap) / `.orqa/agents/orchestrator.md` (app-managed) | Orchestrator operating model, delegation protocol, and worktree lifecycle. During the bootstrap phase, this lives at `.claude/CLAUDE.md` via symlink to `.orqa/`. |

### Project Artifacts (Technical Context)

Project artifacts describe the specific codebase the agents are working on. They are generated or configured per project and live in the project's repository.

| Artifact | Location | What It Contains |
|----------|----------|-----------------|
| Architecture decisions | `.orqa/decisions/` | Technology stack choices and architecture decision records (AD-NNN) for this project |
| Product docs | `.orqa/documentation/product/` | Vision, personas, roadmap, MVP specification — specific to this product |
| Development docs | `.orqa/documentation/development/` | Coding standards, getting-started guide — specific to this stack |
| UI specs | `.orqa/documentation/ui/` | Component inventory, design system, interaction patterns — specific to this app |
| Lessons | `.orqa/lessons/*.md` | Mistakes observed in this codebase, not universal |
| SQLite schema | `.orqa/documentation/architecture/sqlite-schema.md` | This project's database design |
| Project config | `.orqa/project.json` | Per-project OrqaStudio settings (scan paths, artifact types, priority dimensions, promotion threshold, etc.) |

---

## System Rules — Rationale

The following rules are classified as system-level because they apply to every managed agentic development project, regardless of technology:

| Rule | Classification Rationale |
|------|--------------------------|
| `vision-alignment.md` | The pillar framework is OrqaStudio's methodology. Any project using OrqaStudio must pass features against active pillars. |
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
| `pillar-alignment-docs.md` | Documentation pillar alignment applies to any project following OrqaStudio governance. |
| `skill-enforcement.md` | The skill loading protocol applies to all agents. |
| `required-reading.md` | Required Reading protocol applies to all agents. |
| `root-cleanliness.md` | Project root discipline applies to any project. |
| `reusable-components.md` | The shared component library principle applies to any frontend project. (Component names are Orqa-specific but the principle is system-level.) |
| `testing-standards.md` | 80%+ coverage and test isolation requirements apply universally. |

### Rules That Are System-Level But Reference Project Technology

Some system rules reference technology choices (Rust, Svelte 5, SQLite) because OrqaStudio itself is built on those choices. For other projects, the technology-specific content would differ while the principle remains the same.

| Rule | System Principle | Project-Specific Content |
|------|-----------------|--------------------------|
| `architecture-decisions.md` | Read architecture decisions before coding | The specific decisions (Tauri v2, Svelte 5) are Orqa-specific |
| `coding-standards.md` | Read coding standards before coding | The Rust/Svelte details are Orqa-specific |
| `vision-alignment.md` | Features must serve the product vision | The Two Pillars are Orqa-specific |

---

## System Agents

All agent definitions in `.orqa/agents/` are system-level. They define generic roles, not technology-specific implementations. The technology context is provided by the skills each agent loads.

| Agent | Role (Generic) | Project Context Injected Via |
|-------|---------------|------------------------------|
| `backend-engineer` | Implements backend features | `rust-async-patterns`, `tauri-v2` skills + Required Reading: `.orqa/documentation/architecture/` |
| `frontend-engineer` | Implements frontend components | `svelte5-best-practices`, `typescript-advanced-types` skills + Required Reading: `.orqa/documentation/ui/` |
| `code-reviewer` | Reviews code quality | All technology skills + coding standards doc |
| `documentation-writer` | Creates and maintains docs | `architecture` skill + governance docs |
| `data-engineer` | Designs database layer | `rust-async-patterns` skill + schema docs |
| `agent-maintainer` | Maintains governance framework | All skills + all governance docs |

An agent definition that says "write Rust code this way" is encoding project knowledge. The correct pattern is: the agent says "write backend code according to the coding standards", and the `rust-async-patterns` skill provides the Rust-specific guidance.

---

## System Skills vs Project Skills

| Type | Examples | Where It Lives |
|------|----------|---------------|
| **System skills** (technology patterns, portable) | `chunkhound`, `planning`, `svelte5-best-practices`, `rust-async-patterns`, `tauri-v2`, `architecture` | `.orqa/skills/` |
| **Project skills** (would be project-specific knowledge) | "How OrqaStudio's session model works", "The governance bootstrap flow" | Does NOT exist as a skill — lives in `.orqa/documentation/` and is Referenced Reading in agent files |

Project knowledge should not be encoded as skills. Skills are portable technology patterns. Project-specific context belongs in documentation, referenced by agent Required Reading lists.

---

## Project Initialization Flow

When OrqaStudio opens a new project for the first time:

1. **Scan** — The governance scanner walks the project directory looking for:
   - Existing `.claude/` artifacts (rules, agents, hooks, skills)
   - Other tool governance files (Cursor rules, Copilot instructions, Continue config)
   - Documentation structure (`docs/`, `README.md`, ADRs)

2. **Compare** — The scanner compares what it found against the system artifact baseline:
   - Which system rules are present? Which are missing?
   - Which system agents are defined? Are they up to date?
   - Does the project have architecture docs? Product docs?

3. **Propose** — The AI provider (via sidecar) generates a recommendation set:
   - "Copy these system rules — they apply universally"
   - "These Cursor rules could be translated to OrqaStudio governance format"
   - "These architecture decisions should be created based on the README"
   - "The following governance docs are missing — here are starter templates"

4. **Apply** — The user reviews and approves recommendations. OrqaStudio writes the approved artifacts to `.orqa/` (the source of truth). If CLI compatibility is needed (e.g., Claude Code), symlinks are created in `.claude/` pointing to the corresponding `.orqa/` paths.

The output is a project that has both the system methodology baseline and its own technical context layer.

---

## CLI Compatibility Model

OrqaStudio governance artifacts live under `.orqa/` as the source of truth. When CLI tool compatibility is needed, OrqaStudio creates symlinks in the tool-specific location pointing to `.orqa/` paths. This means CLI tools read the same artifacts that OrqaStudio manages — there is no sync layer.

| Artifact | Works in Claude Code CLI |
|----------|--------------------------|
| `.orqa/rules/*.md` | Yes — via `.claude/rules/` symlinks |
| `.orqa/agents/*.md` | Yes — via `.claude/agents/` symlinks |
| `.orqa/hooks/*.sh` | Yes — via `.claude/hooks/` symlinks |
| `.orqa/hooks/hookify.*.local.md` | Yes — via `.claude/hookify.*.local.md` symlinks |
| `.orqa/lessons/*.md` | Yes — readable by agents via file tools |
| `.orqa/documentation/` | Yes — readable by agents via file tools |

Symlink creation is a project configuration option, not a requirement. If a user is not using external CLI tools, no symlinks are necessary — OrqaStudio handles all enforcement natively.

OrqaStudio adds visual management, dashboards, and enhanced UX on top of these file-based artifacts. It never creates vendor lock-in — all artifacts are standard markdown on disk.

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | Defining what is system-scoped vs project-scoped makes governance visible and portable. The initialization flow ensures new projects start with proven methodology rather than an empty governance structure. |
| Learning Through Reflection | The system/project distinction ensures that lessons learned in one project can inform the system baseline — promoted patterns become system-level rules applicable across all projects. |

---

## Related Documents

- `.orqa/documentation/process/content-governance.md` — Content ownership model (what belongs in docs, rules, skills, agents)
- `.orqa/documentation/architecture/enforcement.md` — Enforcement engine with system vs project scope
- `.orqa/documentation/architecture/governance-bootstrap.md` — Governance bootstrap scan, compare, propose, apply workflow
- `.orqa/documentation/product/vision.md` — Product vision that defines the Two Pillars
