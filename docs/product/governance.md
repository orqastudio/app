# Product Governance

**Date:** 2026-03-02

## Feature Governance Framework

Every feature, task, and implementation must pass the Two-Pillar Test before work begins.

### The Two-Pillar Test

**Pillar 1: Self-Learning Loop** — Does this feature help the system improve over time?

- Does it capture lessons from agent sessions?
- Does it track metrics or outcomes?
- Does it feed retrospectives back into governance?
- Does it accumulate knowledge across sessions?

**Pillar 2: Process Governance** — Does this feature enforce or surface standards, rules, and workflows?

- Does it make governance more visible?
- Does it make governance more enforceable?
- Does it help the user manage their governance framework?

### Rejection Criteria

Reject any feature that:

- Serves neither pillar
- Adds complexity without improving learning or governance
- Is a generic developer tool feature with no connection to managed agentic development
- Cannot articulate how it makes the system smarter (Pillar 1) or governance more tangible (Pillar 2)

### Pillar Conflict Resolution

When Pillar 1 (Self-Learning Loop) and Pillar 2 (Process Governance) conflict, **Pillar 2 takes priority**. Governance must be solid before the learning loop can meaningfully operate on it.

## Foundational Principles

The following are immutable without explicit user approval:

1. **Two-Pillar framework** — Every feature serves at least one pillar
2. **Primary audience** — Product Managers and Tech Leads, not just developers
3. **Human approval gate** — Implementation plans must be approved by a human (Tech Lead / PM) before coding begins. Agents plan and implement, but humans authorize.
4. **Technology stack** — Tauri v2 + Svelte 5 + Rust + SQLite
5. **IPC boundary** — Tauri commands are the only frontend-backend interface
6. **Composability** — External integrations (AI providers, MCP servers) connect through provider-agnostic interfaces. The Rust core speaks a neutral protocol; provider-specific logic lives in swappable sidecar processes. Claude is the primary provider, but the architecture must not hard-couple to any single provider.
7. **UX-first design** — User journeys drive backend requirements; the UI should be approachable for PMs, not just developers
8. **Documentation-first workflow** — Document → Approve → Implement → Verify
9. **Error propagation** — Result types everywhere, no unwrap in production

## Bootstrap Phase — Temporary Alvarez-Derived Process

> **This section documents a temporary state.** The current development process is inherited from the [Alvarez project](../process/orchestration) and will be replaced by Forge itself once the MVP is functional.

The current process uses:

- **Claude Code CLI** as the orchestrator (`.claude/CLAUDE.md`)
- **15 specialized agents** defined as `.claude/agents/*.md` files
- **20 rules** enforced via `.claude/rules/*.md`
- **11 hookify files** in `.claude/` for real-time code/command enforcement
- **3 hooks** in `.claude/hooks/` (session start, skill loading, pre-commit)
- **Git worktree workflow** for task isolation

This is the same framework used in the Alvarez project, adapted for Forge's Tauri/Svelte/Rust stack. It works, but it's invisible infrastructure — exactly the problem Forge exists to solve.

### The Dogfooding Milestone

Once the MVP delivers a working conversation UI with basic Claude integration (Phase 1 complete), **Forge must become the primary tool for managing its own development.** This means:

1. **Process artifacts become visible** — Agents, rules, skills, and docs are browsed and edited in Forge's UI, not raw files
2. **Learning loops are active** — IMPL lessons and RETRO entries are captured through Forge's interface, not manual markdown editing
3. **Governance is enforced through Forge** — Scanner results, verification gates, and compliance checks surface in the dashboard, not terminal output

### Why Dogfooding Matters

Using Forge to build Forge creates a natural feedback loop:

- **Deficiencies surface immediately** — If a governance feature is missing or painful, the team experiences it daily
- **Features are validated by real use** — Every new feature is tested in production (on this project) before being considered "done"
- **Priority is self-evident** — The most painful gaps in the tool become the highest-priority features organically
- **The learning loop bootstraps itself** — Forge's self-learning capabilities improve Forge's own governance, which improves Forge

### Transition Criteria

The Alvarez-derived process is retired when Forge can:

- [ ] Display and edit process artifacts (agents, rules, skills)
- [ ] Run a conversation with Claude and display streaming responses
- [ ] Show scanner/verification results in the UI
- [ ] Capture IMPL lessons through the interface
- [ ] Persist session history in SQLite

Until these criteria are met, the current CLI-based process remains in effect.

## Decision Process

1. Feature request received
2. Pillar alignment verified (reject if neither)
3. Architecture decision check (new AD needed?)
4. Documentation written and approved
5. Implementation planned (UX-first)
6. Implementation delegated to agents
7. Three-reviewer verification gate
8. Merge and deploy
