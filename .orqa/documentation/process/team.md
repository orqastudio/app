---
id: DOC-034
title: The Agentic Development Team
description: Roster of specialized agents in the agentic development team with their roles and responsibilities.
created: "2026-03-02"
updated: "2026-03-10"
---

**Date:** 2026-03-02

OrqaStudio™ uses an agentic development model: one orchestrator coordinates, and specialized subagents implement. This page defines the team structure, each agent's role, the skills available to them, and how review gates work.

---

## Team Model

| Role | Participant | Responsibilities |
|------|-------------|-----------------|
| **Product Manager** | User (human) | Defines vision, pillars, priorities. Approves feature scope. Makes decisions when product principles conflict. |
| **Tech Lead** | User (human) | Approves implementation plans before coding begins. Reviews architecture decisions. Provides technical oversight. Final authority on technical approach. |
| **Scrum Master / Dev Lead** | Orchestrator (main AI session) | Reads TODO.md, creates worktrees, delegates to agents, gates on DoR/DoD, merges completed work, runs post-merge verification. |
| **Implementation Team** | Specialized agents | Implement delegated tasks end-to-end. Load skills. Run quality checks. Report results. |

The Product Manager and Tech Lead roles may be filled by the same person if they have the skillset. Both are human gates — no implementation proceeds without explicit user approval of the plan.

The orchestrator does NOT implement code directly. Its job is coordination, delegation, and gating. See [Orchestration](DOC-030) for the full orchestrator protocol.

---

## Agent Directory

All 7 universal roles are defined in `.orqa/team/agents/`. Each role is invoked via the Task tool with the appropriate `subagent_type`, and domain capability is loaded via skills at delegation time (see [AD-029](AD-029)).

| Role | Purpose | Tier 1 Skills | When to Use |
|------|---------|---------------|-------------|
| **Orchestrator** | Coordinates work, enforces process, manages governance | `code-search`, `orqa-composability`, `planning`, `governance-maintenance` | Session coordination, task lifecycle, governance artifact management |
| **Researcher** | Investigates questions, gathers information, analyses findings | `code-search`, `orqa-composability`, `planning` | Before planning, when understanding is needed; produces research documents |
| **Planner** | Designs approaches, evaluates tradeoffs, maps dependencies | `code-search`, `orqa-composability`, `planning`, `architecture` | Before implementation; cross-boundary features; produces implementation plans in epics |
| **Implementer** | Does the work — code, deliverables, whatever "work" means | `code-search`, `orqa-composability` + domain skills loaded by orchestrator | Rust backend, Svelte frontend, database, build, refactoring |
| **Reviewer** | Checks quality, compliance, correctness | `code-search`, `orqa-composability` + review skills loaded by orchestrator | Code quality, QA, UX compliance, security audits |
| **Writer** | Creates documentation, specifications, communications | `code-search`, `orqa-composability`, `planning` | Architecture docs, IPC contracts, component specs, process documentation |
| **Designer** | Designs experiences, interfaces, and structures | `code-search`, `orqa-composability`, `svelte5-best-practices`, `tailwind-design-system` | UI components, styling, visual polish, design system |

Orchestrator-injected Tier 2 skills narrow the Implementer and Reviewer roles to specific domains. See the delegation guide above and [AD-029](AD-029) for the full skill-loading model.

---

## Skill Directory

Skills are domain-specific instruction sets stored in `.orqa/team/skills/`. They follow the open [Agent Skills](https://agentskills.io) standard and must be portable — no OrqaStudio-specific architectural rules in skills.

| Skill | Source | Domain | Loaded By |
|-------|--------|--------|-----------|
| `chunkhound` | Custom | Semantic code search: search_regex, search_semantic, code_research | ALL agents |
| `planning` | Custom | Discuss-Agree-Plan-Approve-Implement-Verify methodology | orchestrator, agent-maintainer, systems-architect |
| `skills-maintenance` | Custom | skills.sh CLI, skill lifecycle, portability rules, audit protocol | agent-maintainer |
| `architecture` | Custom | ADR pattern, data flow mapping, architectural violations | systems-architect, documentation-writer |
| `svelte5-best-practices` | skills.sh registry | Svelte 5 runes ($state, $derived, $effect), component patterns | frontend-engineer, designer, debugger, code-reviewer, qa-tester, ux-reviewer |
| `typescript-advanced-types` | skills.sh registry | Strict TypeScript, readonly types, string literal unions | frontend-engineer, debugger, code-reviewer, test-engineer, refactor-agent, qa-tester, ux-reviewer |
| `tailwind-design-system` | skills.sh registry | Tailwind CSS utilities, theme variables, design tokens | frontend-engineer, designer, ux-reviewer |
| `rust-async-patterns` | skills.sh registry | Rust async/await, tokio, error handling, lifetimes | backend-engineer, data-engineer, debugger, code-reviewer, test-engineer, security-engineer, refactor-agent |
| `tauri-v2` | skills.sh registry | Tauri v2 commands, Channel&lt;T&gt;, plugins, security model | backend-engineer, debugger, devops-engineer, security-engineer, systems-architect |

For full provenance and date-added information, see [Skills Log](DOC-033).

---

## Review Gates

Every task passes through independent review agents before it is marked complete. The implementing agent does not self-certify.

```mermaid
graph LR
    Impl["Implementing Agent"] -->|commits| CR["code-reviewer<br/>Static quality"]
    CR -->|PASS| QA["qa-tester<br/>Functional correctness"]
    QA -->|PASS| UX["ux-reviewer<br/>UX compliance<br/>(if UI-facing)"]
    UX -->|PASS| Done["Task Complete"]
    CR -->|FAIL| Fix["Fix and resubmit"]
    QA -->|FAIL| Fix
    UX -->|FAIL| Fix
    Fix --> CR
```

| Reviewer | Evaluates |
|----------|-----------|
| `code-reviewer` | clippy/rustfmt/ESLint/svelte-check zero errors; no stubs; 80%+ coverage; doc layer compliance; DoD code items |
| `qa-tester` | End-to-end functional correctness from a user perspective; smoke test; DoD smoke test items |
| `ux-reviewer` | Labels match UI specs in `.orqa/documentation/ui/`; all component states handled; shared components used; no jargon in UI; DoD UI items |

Review failures generate entries in [Implementation Lessons](DOC-006). The agent-maintainer promotes recurring failures to rules or standards.

---

## Content Ownership

Each layer of the governance system owns a specific type of content. For the full framework, see [Content Governance](DOC-026). The `agent-maintainer` owns hookify rule files alongside lifecycle hooks.

| Layer | Owns |
|-------|------|
| `.orqa/documentation/` | Standards, IPC contracts, architecture decisions — source of truth |
| `.orqa/team/agents/` | Process: how agents work, what to read, when to delegate |
| `.orqa/team/skills/` | Technology patterns — portable, no OrqaStudio-specific rules |
| `.orqa/governance/rules/` | Behavioral enforcement — applies to all agents automatically |
| `.orqa/governance/hooks/` | Lifecycle hooks — shell scripts triggered by lifecycle events (session start, stop, pre-commit) |
| `.orqa/hookify/` | Hookify rules — action-boundary enforcement that blocks or warns on specific patterns in file edits or bash commands (CLI tools may read from a `.claude/` symlink layer) |

---

## Related Documents

- [Orchestration](DOC-030) -- Orchestrator responsibilities and context discipline
- [Workflow](DOC-035) -- Task lifecycle from start to complete
- [Content Governance](DOC-026) -- The six-layer ownership model
- [Skills Log](DOC-033) -- Full skill inventory with provenance and dates
- [Definition of Ready](DOC-028) -- What must be true before implementation starts
- [Definition of Done](DOC-027) -- What must be true before a task is marked complete
