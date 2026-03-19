---
id: RULE-deab6ea7
title: Skill Enforcement
description: "Three-tier skill loading: agent portable skills, orchestrator-injected project skills, and context-resolving wrappers."
status: active
created: 2026-03-07
updated: 2026-03-13
relationships:
  - target: AD-6f0dea5e
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-6f0dea5e
  - target: AD-53e80192
    type: enforces
  - target: DOC-4db3a417
    type: documented-by
  - target: DOC-b10607c0
    type: documented-by
  - target: DOC-e0042602
    type: documented-by
---
Every agent MUST have a `skills:` list in its YAML frontmatter. Agent tool access is declared via `capabilities:` and resolved per provider context — see [RULE-92dba0cb](RULE-92dba0cb). Skills load in three tiers [AD-53e80192](AD-53e80192).

## Three-Tier Model

| Tier | What | Where Declared | Loaded By |
|------|------|---------------|-----------|
| **Tier 1** | Portable skills + wrappers | Agent YAML `skills:` frontmatter | Loaded on task start (by agent, plugin, or app) |
| **Tier 2** | Project-specific skills | Orchestrator injection table | Orchestrator adds to delegation prompt |
| **Tier 3** | Context resolution (CLI vs App) | Wrapper skill logic | Wrapper skill auto-resolves |

## Skill Loading Order

When an agent starts a task, it MUST follow this sequence:

1. **Load Tier 1 skills** — Every skill in the agent's `skills:` YAML frontmatter is loaded via `Skill(name)` before any other work begins
2. **Load Tier 2 skills** — Skills included in the orchestrator's delegation prompt (if any)
3. **Read Required Reading** — Load governing documentation listed in the agent's Required Reading section
4. **Begin implementation** — Only after steps 1-3 are complete

If a skill fails to load, the agent MUST report the failure explicitly. Do NOT silently continue without the skill.

## Layer Classification

Every skill carries a `layer` field in its SKILL.md frontmatter:

| Layer | Meaning | Loading |
|-------|---------|---------|
| `core` | Universal skill — applicable to all project types (governance, systems thinking, search usage) | Loaded based on agent YAML `skills:` list (Tier 1) |
| `setup` | Project setup — used only during new project initialization, inference, and migration | Loaded by orchestrator during project setup workflows |
| `project` | Project-specific — captures THIS project's patterns, conventions, and domain knowledge | Injected by orchestrator based on task scope (Tier 2) |
| `plugin` | Ecosystem skill — installed from external source, 1st party official | Loaded same as core |
| `community` | Community-contributed skill — reviewed but not 1st-party | Loaded same as core; treat with appropriate trust level |
| `user` | Personal workflow skill — user's own private patterns | Loaded same as core; not shared or published |

## Universal Skills (Tier 1)

These MUST appear in every agent's `skills:` YAML frontmatter:

- `orqa-code-search` — Context-aware search wrapper. Resolves to the appropriate search implementation at Tier 3.
- `composability` — Meta-skill that shapes how all code is structured. Universal across all agents.

The orchestrator loads `orqa-code-search`, `composability`, and `planning` on every session.

## Project Skills (Tier 2 — Orchestrator-Injected)

These are NOT on agent YAML frontmatter. The orchestrator injects them based on task scope. The injection table lives in the orchestrator's agent definition.

When delegating, the orchestrator includes: "Load these project skills before starting: [list]"

## Context Resolution (Tier 3)

The `orqa-code-search` wrapper skill detects the runtime context and resolves to the correct search implementation:

| Available Tools | Context | Resolved Skill |
|----------------|---------|---------------|
| `mcp__chunkhound__*` tools available | CLI (Claude Code) | `chunkhound` |
| Native search commands available | App (OrqaStudio) | `orqa-native-search` |
| Neither | Fallback | Use Grep/Glob, note in task summary |

## Portable Skills (Tier 1 — Agent-Declared)

| Skill | Purpose | On Which Roles |
|-------|---------|---------------|
| `orqa-code-search` | Code search wrapper (mandatory) | ALL roles |
| `composability` | Composability philosophy (mandatory) | ALL roles |
| `planning` | Planning methodology | orchestrator, planner, researcher, writer |
| `skills-maintenance` | Skill lifecycle management | orchestrator (governance work) |
| `architecture` | ADR patterns, compliance | planner |
| `diagnostic-methodology` | Root cause analysis | Implementer (debugging tasks) |
| `restructuring-methodology` | Safe incremental refactoring | Implementer (refactoring tasks) |
| `security-audit` | Security auditing methodology | Reviewer (security tasks) |
| `governance-maintenance` | Governance framework custodianship | orchestrator (governance work) |
| `code-quality-review` | Code review methodology | Reviewer (code quality tasks) |
| `qa-verification` | E2E functional verification | Reviewer (QA tasks) |
| `ux-compliance-review` | UX compliance review | Reviewer/Designer (UX tasks) |
| `test-engineering` | Test engineering methodology | Reviewer (testing tasks), Implementer (TDD) |
| `architectural-evaluation` | Architectural compliance | Planner, Reviewer (architecture tasks) |

## Rule Status Awareness

Rules carry a `status` field in their YAML frontmatter: `active` or `inactive`. When loading rules for enforcement:

- **`status: active`** — The rule is enforced. Agents must comply.
- **`status: inactive`** — The rule is NOT enforced. Agents should skip it. The file is preserved for historical reference.
- **Missing status field** — Treat as `active` (backwards compatibility).

The orchestrator and all agents MUST check rule status before applying enforcement. Inactive rules are not loaded into agent context.

## Audit

- The orchestrator periodically audits that agent Tier 1 skill lists contain only core/plugin/community/user portable skills + universal wrappers
- No project-specific skills should appear in agent YAML frontmatter
- The injection table in the orchestrator is the single source of truth for Tier 2 loading
- All skill changes are tracked through the artifact graph (skill YAML frontmatter + relationships)

## App-Managed Loading

In OrqaStudio, skills are loaded via the `load_skill` tool and managed by the app's process enforcement layer. The app tracks which skills each agent has loaded and can enforce loading before task execution begins. The YAML frontmatter `skills:` declarations remain authoritative for CLI usage, where agents self-load skills based on their frontmatter lists.

## Related Rules

- [RULE-b2753bad](RULE-b2753bad) (required-reading) — docs that agents must load (complementary to skills)
- [RULE-5e03e67b](RULE-5e03e67b) (code-search-usage) — code search usage and context detection
- [RULE-532100d9](RULE-532100d9) (agent-delegation) — orchestrator must delegate to agents, not implement directly
- [RULE-f809076f](RULE-f809076f) (tool-access-restrictions) — constrains which tools each role may use
- [RULE-5ee43922](RULE-5ee43922) (user-invocable-skills) — user-invocable skill field semantics
- [RULE-92dba0cb](RULE-92dba0cb) (provider-agnostic-capabilities) — capability → tool mapping replaces tools in agent definitions
