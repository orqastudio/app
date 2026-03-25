---
id: KNOW-8d2e5eef
type: knowledge
title: Agent Team Structure
description: |
  How the OrqaStudio development agent team is organised: specialist dev team agents
  (Rust, Svelte, Integration), multi-role artifacts-expert, plugin-developer, and how
  the orchestrator delegates to them. Use when: delegating implementation work,
  choosing which agent to assign a task, or understanding team composition.
status: active
created: 2026-03-24
updated: 2026-03-24
category: methodology
version: 1.0.0
user-invocable: false
relationships:
  - target: DOC-8d2e5eef
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
tier: "stage-triggered"
roles:
  - "implementer"
stages:
  - "implement"
tags:
  - "agents"
  - "teams"
  - "delegation"
priority: "P1"
summary: |
  Agent team structure: hub-spoke orchestration, ephemeral task-scoped agents,
  findings to disk. Teams are the unit of delegation — never spawn bare agents.
---

## Purpose

OrqaStudio uses specialised agents that inherit from the universal roles defined in the core framework. This knowledge describes the team composition and delegation patterns for the development project.

---

## Team Composition

### Specialist Dev Team (Project-Specific)

These agents live in project-specific plugin directories and inherit from the generic Implementer (AGENT-e5dd38e4):

| Agent | Plugin | Specialisation |
|-------|--------|---------------|
| **Rust Specialist** | `@orqastudio/plugin-software` | Rust backend, Tauri commands, domain services, error handling |
| **Svelte Specialist** | `@orqastudio/plugin-svelte` | Svelte 5 runes, shadcn-svelte, stores, component purity |
| **Integration Specialist** | `@orqastudio/plugin-tauri` | Cross-boundary wiring — IPC contracts, type consistency, end-to-end flows |

Each specialist carries domain-specific knowledge via `employs` relationships. The orchestrator chooses the specialist based on which code areas the task touches.

### Governance Steward (Core Framework)

The Governance Steward (AGENT-ae63c406) operates in three roles:

| Role | When Activated | What It Does |
|------|---------------|-------------|
| **Governance Steward** | Creating/updating rules, knowledge, decisions, lessons | Schema compliance, relationship integrity, placement decisions |
| **Documentation Maintainer** | Docs need updating or pairing | Keeps docs accurate, ensures doc+knowledge pairing |
| **Artifact Auditor** | Schema violations, missing relationships, wrong placements found | Finds and fixes structural problems across the artifact graph |

### Plugin Developer (Core Framework)

The Plugin Developer (AGENT-ce86fb50) handles all plugin-related work:
- Scaffolding new plugins
- Adding components (hooks, agents, knowledge, skills, docs) to existing plugins
- Manifest management (`orqa-plugin.json`)
- Plugin installation and testing

### Enforcement Specialists (Plugin-Specific)

| Agent | Plugin | Purpose |
|-------|--------|---------|
| **Governance Enforcer** | `@orqastudio/plugin-agile-workflow` | Designs mechanical enforcement for rules |
| **Tauri Standards Agent** | `@orqastudio/plugin-tauri` | Scoped task agent for Tauri v2 standards checks |

---

## Delegation Patterns

### Which Agent Gets Which Task

| Task Type | Assign To | Why |
|-----------|-----------|-----|
| Rust backend code | Rust Specialist | Deep Rust + Tauri knowledge |
| Svelte frontend code | Svelte Specialist | Deep Svelte 5 + shadcn knowledge |
| Cross-boundary wiring (IPC, types) | Integration Specialist | Understands both sides of the boundary |
| Governance artifacts (rules, knowledge, docs) | Governance Steward | Schema compliance, placement decisions |
| Plugin work (scaffolding, manifests, content) | Plugin Developer | Plugin lifecycle knowledge |
| Enforcement design (hooks, validators) | Governance Enforcer | Enforcement pattern knowledge |
| Code review | Generic Reviewer (AGENT-bbad3d30) | Independent verification |

### Parallel Execution Rules

| Combination | Safe? | Why |
|-------------|-------|-----|
| Svelte Specialist + Governance Steward | Yes | Frontend is lightweight, governance is file-only |
| Rust Specialist + Svelte Specialist | Careful | Rust compilation is heavy; stagger if same worktree |
| Two Rust agents in same worktree | No | Cargo compilation fights for resources |
| Governance Steward + Plugin Developer | Yes | Both are file-only operations |

---

## FORBIDDEN

- Assigning Rust backend work to the Svelte Specialist (wrong domain knowledge)
- Assigning governance artifact creation to an Implementer (use the Governance Steward)
- Running two compilation-heavy agents in the same worktree simultaneously
- Skipping the Governance Steward for artifact work (schema compliance requires specialist knowledge)
