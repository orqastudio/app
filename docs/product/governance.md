---
title: "Product Governance"
description: "Product governance criteria defining how features are evaluated against the two-pillar framework."
category: product
tags: []
created: 2026-03-02
updated: 2026-03-07
---

**Date:** 2026-03-07

> OrqaStudio™ is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

## Feature Governance Framework

Every feature, task, and implementation must pass the Two-Pillar Test before work begins.

### The Two-Pillar Test

**Pillar 1: Clarity Through Structure** — Does this feature make thinking, standards, or decisions more visible and structured?

- Does it make governance artifacts visible and manageable?
- Does it produce or organise structured knowledge (plans, decisions, rules)?
- Does it enforce a workflow that ensures understanding precedes action?
- Does it surface what would otherwise be hidden in files, terminal output, or people's heads?

**Pillar 2: Learning Through Reflection** — Does this feature help the system or its users improve over time?

- Does it capture lessons, discoveries, or patterns?
- Does it track metrics or outcomes that show improvement (or regression)?
- Does it feed retrospectives back into the governance framework?
- Does it accumulate knowledge across sessions so each cycle starts from a better position?

### Rejection Criteria

Reject any feature that:

- Serves neither pillar
- Adds complexity without improving clarity or learning
- Is a generic tool feature with no connection to structured thinking or reflective improvement
- Cannot articulate how it makes the work more visible and structured (Pillar 1) or how it makes the process smarter over time (Pillar 2)

### Pillar Conflict Resolution

When Pillar 1 (Clarity Through Structure) and Pillar 2 (Learning Through Reflection) conflict, **Pillar 1 takes priority**. You cannot improve a process that isn't visible and structured. Governance must be solid before the learning loop can meaningfully operate on it.

## Foundational Principles

The following are immutable without explicit user approval:

1. **Two-Pillar framework** — Every feature serves at least one pillar
2. **Primary audience** — Structured thinkers: product managers, tech leads, and anyone who needs to turn complex situations into structured understanding
3. **Human approval gate** — Plans must be approved by a human before execution begins. AI assists and executes, but humans authorise.
4. **Technology stack** — Tauri v2 + Svelte 5 + Rust + SQLite
5. **IPC boundary** — Tauri commands are the only frontend-backend interface
6. **Composability** — External integrations (AI providers, MCP servers) connect through provider-agnostic interfaces. The Rust core speaks a neutral protocol; provider-specific logic lives in swappable sidecar processes. Claude is the primary provider, but the architecture must not hard-couple to any single provider.
7. **UX-first design** — User journeys drive backend requirements; the UI should be approachable for leads and PMs, not just developers
8. **Documentation-first workflow** — Document → Approve → Implement → Verify
9. **Error propagation** — Result types everywhere, no unwrap in production

## Governance Artifact Format

Governance artifacts are native `.claude/` files stored on disk. OrqaStudio reads and writes these files through its UI. The same files are read by the Claude Code CLI. OrqaStudio's SQLite database is a derived cache, not the source of truth.

This means:
- `.orqa/rules/*.md`, `.orqa/agents/*.md`, `.orqa/skills/`, `.orqa/hooks/`, and `CLAUDE.md` are the canonical governance format
- Users can edit these files in OrqaStudio, in a text editor, or through a Claude Code CLI session — all changes are reflected everywhere
- OrqaStudio adds a visual management layer on top of this file-based governance; it does not replace the file format with a proprietary one

## Bootstrap Phase — CLI-Only Governance

> **This section documents the current state.** The governance framework uses native `.claude/` artifacts managed through the Claude Code CLI. OrqaStudio will supplement this with a visual UI once the MVP is functional.

The current process uses:

- **Claude Code CLI** as the orchestrator (`.claude/CLAUDE.md`)
- **15 specialised agents** defined as `.orqa/agents/*.md` files
- **20 rules** enforced via `.orqa/rules/*.md`
- **11 hookify files** in `.claude/` for real-time code/command enforcement
- **3 hooks** in `.orqa/hooks/` (session start, skill loading, pre-commit)
- **Git worktree workflow** for task isolation

This is the same framework used in the Alvarez project, adapted for OrqaStudio's Tauri/Svelte/Rust stack. The governance works, but it is invisible — artifacts live in dotfiles and terminal output. This is exactly the problem OrqaStudio exists to solve: making this governance visible (Pillar 1) and improving it over time through structured reflection (Pillar 2).

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

Until these criteria are met, governance is managed through direct `.claude/` file editing and CLI sessions. After these criteria are met, both OrqaStudio and the CLI continue to operate on the same `.claude/` artifacts — OrqaStudio adds the visual layer, the CLI remains fully functional.

## Decision Process

1. Feature request received
2. Pillar alignment verified (reject if neither)
3. Architecture decision check (new AD needed?)
4. Documentation written and approved
5. Implementation planned (UX-first)
6. Implementation delegated to agents
7. Three-reviewer verification gate
8. Merge and deploy
