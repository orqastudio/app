---
id: DOC-af4d6184
type: doc
title: Agent Team Structure
description: "How the OrqaStudio development agent team is organised: specialist dev team agents (Rust, Svelte, Integration), the multi-role Governance Steward, Plugin Developer, and how the orchestrator delegates work across them."
category: architecture
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: KNOW-af4d6183
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
---

# Agent Team Structure

## Overview

OrqaStudio uses a team of specialised agents, each with domain-specific knowledge. The core framework defines universal roles (Implementer, Reviewer, Researcher, etc.), and plugins extend these with specialists that carry the right knowledge for their domain.

This document describes the team composition for the OrqaStudio development project and how the orchestrator delegates work.

## Universal Roles (Core Framework)

The core framework plugin (`@orqastudio/plugin-core-framework`) defines the universal agent roles that every project gets:

| Role | Agent ID | Purpose |
|------|----------|---------|
| Orchestrator | AGENT-1dab5ebe | Coordinates work, delegates to specialists, manages artifact lifecycle |
| Implementer | AGENT-cc255bc8 | Generic implementation — base role for specialist implementers |
| Reviewer | AGENT-ec1b3785 | Independent quality verification — PASS/FAIL verdicts |
| Researcher | AGENT-fb0ce261 | Investigation and analysis — produces findings, not changes |
| Planner | AGENT-caff7bc1 | Design and dependency mapping — produces plans, not code |
| Writer | AGENT-b0774726 | Documentation authoring — produces docs, not implementation |
| Designer | AGENT-bedeffd1 | Interface and experience design — UI/UX work |

## Specialist Agents

### Implementation Specialists (Plugin-Provided)

These agents inherit from the generic Implementer and carry domain-specific knowledge via `employs` relationships:

| Specialist | Plugin | Domain | Key Knowledge |
|-----------|--------|--------|--------------|
| **Rust Specialist** | `@orqastudio/plugin-software` | Rust backend, Tauri commands, domain services | Error composition, repository patterns, async Rust |
| **Svelte Specialist** | `@orqastudio/plugin-svelte` | Svelte 5 frontend, shadcn-svelte, stores | Runes patterns, component purity, store orchestration |
| **Integration Specialist** | `@orqastudio/plugin-tauri` | Cross-boundary wiring, IPC contracts | Tauri IPC patterns, type consistency, end-to-end flows |

### Governance Steward (Core Framework)

The Governance Steward (AGENT-fbdfa9ef) is a multi-role agent that handles all artifact-touching work:

| Role | When Used | What It Does |
|------|-----------|-------------|
| **Governance Steward** | Creating/updating rules, knowledge, decisions, lessons | Ensures schema compliance, relationship integrity, correct placement |
| **Documentation Maintainer** | Documentation needs updating or pairing | Keeps docs accurate, enforces the doc+knowledge pairing rule |
| **Artifact Auditor** | Schema violations or structural problems found | Finds and fixes invalid statuses, missing relationships, wrong placements |

The orchestrator delegates to this agent whenever governance artifacts need to be created, updated, or audited. The agent always queries schemas via MCP before writing frontmatter.

### Plugin Developer (Core Framework)

The Plugin Developer (AGENT-34a6e2cd) handles all plugin lifecycle work:

- Scaffolding new plugins
- Adding components to existing plugins (hooks, agents, knowledge, skills, docs)
- Managing plugin manifests (`orqa-plugin.json`)
- Running `orqa install` to sync content
- Testing plugin changes

### Enforcement Specialists (Plugin-Specific)

| Agent | Plugin | Purpose |
|-------|--------|---------|
| **Governance Enforcer** | `@orqastudio/plugin-agile-governance` | Designs mechanical enforcement for governance rules |
| **Tauri Standards Agent** | `@orqastudio/plugin-tauri` | Scoped task agent for Tauri v2 standards checks |

## How the Orchestrator Delegates

### Task-to-Agent Mapping

The orchestrator selects the specialist based on which code areas or artifact types the task touches:

| Task Touches | Delegate To |
|-------------|------------|
| `backend/src-tauri/` (Rust code) | Rust Specialist |
| `ui/src/` (Svelte/TypeScript) | Svelte Specialist |
| Both backend and frontend | Integration Specialist |
| `.orqa/` artifacts (rules, knowledge, docs) | Governance Steward |
| Plugin directories (`plugins/`) | Plugin Developer |
| Enforcement mechanisms (hooks, validators) | Governance Enforcer |
| Code review for any area | Generic Reviewer |

### Parallel Execution

Not all agents can run in parallel safely. The key constraint is **compilation resources**:

| Agent Type | Resource Weight | Parallel Safety |
|-----------|----------------|-----------------|
| Rust Specialist | Heavy (cargo compilation) | Only with lightweight agents |
| Svelte Specialist | Light (svelte-check) | Safe with any agent |
| Governance Steward | None (file operations) | Safe with any agent |
| Plugin Developer | None (file operations) | Safe with any agent |
| Generic Reviewer | Heavy (runs full check suite) | Run alone |

**Rule:** Never run two agents that trigger `cargo` compilation in the same worktree simultaneously.

### Knowledge Injection

When the orchestrator delegates to a specialist, two things happen:

1. **Declared knowledge** — the agent's `employs` relationships are followed, loading base knowledge
2. **Semantic knowledge** — the task description is searched against the knowledge corpus for additional relevant knowledge

See RULE-f9d0279c for the full knowledge auto-injection model.

## Agent Teams

The orchestrator uses Claude Code's agent teams (via `TeamCreate`) for all delegated work. Even single-task delegations use teams because:

- The orchestrator stays available for conversation while agents work in the background
- Structured findings are written to `tmp/team/<team-name>/task-<id>.md`
- The user can steer or add context while work is in progress

## Related Documents

- [KNOW-af4d6183](KNOW-af4d6183) — Agent-facing knowledge pair for this documentation page
- [RULE-f9d0279c](RULE-f9d0279c) — Knowledge Auto-Injection (how agents receive domain knowledge)
