---
id: RULE-026
title: Skill Enforcement
description: "Three-tier skill loading: agent portable skills, orchestrator-injected project skills, and context-resolving wrappers."
status: active
created: 2026-03-07
updated: 2026-03-07
layer: canon
scope: system
---
Every agent MUST have a `skills:` list in its YAML frontmatter. Skills load in three tiers (AD-028).

## Three-Tier Model

| Tier | What | Where Declared | Loaded By |
|------|------|---------------|-----------|
| **Tier 1** | Portable language/framework skills + wrappers | Agent YAML `skills:` frontmatter | Agent self-loads on task start |
| **Tier 2** | Project-specific `orqa-*` skills | Orchestrator injection table | Orchestrator adds to delegation prompt |
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
| `canon` | Platform skill — portable across projects | Loaded based on agent YAML `skills:` list (Tier 1) |
| `project` | Project-specific — captures THIS codebase's patterns | Injected by orchestrator based on task scope (Tier 2) |
| `plugin` | Ecosystem skill — installed from external source | Loaded same as canon |

Agent `scope` determines which agents are loaded for a project type:

| Scope | Loaded When |
|-------|-------------|
| `general` | Always — every project type |
| `software-engineering` | Software projects only |
| `governance` | When governance work is needed |

## Universal Skills (Tier 1)

These MUST appear in every agent's `skills:` YAML frontmatter:

- `code-search` — Context-aware search wrapper. Resolves to `chunkhound` (CLI) or `orqa-native-search` (App) at Tier 3.
- `orqa-composability` — Meta-skill that shapes how all code is structured. Universal across all agents.

The orchestrator loads `code-search`, `orqa-composability`, and `planning` on every session.

## Project Skills (Tier 2 — Orchestrator-Injected)

These are NOT on agent YAML frontmatter. The orchestrator injects them based on task scope. All project skills have `layer: project` in their SKILL.md:

| Skill | Domain | Injected When Task Touches |
|-------|--------|---------------------------|
| `orqa-ipc-patterns` | Tauri IPC, Channel<T>, command registration | `src-tauri/src/commands/` |
| `orqa-domain-services` | Domain service anatomy, composition | `src-tauri/src/domain/` |
| `orqa-repository-pattern` | SQLite repos, migrations, queries | `src-tauri/src/repo/`, `db.rs` |
| `orqa-error-composition` | OrqaError flow, From impls | `src-tauri/src/commands/`, `src-tauri/src/domain/` |
| `orqa-streaming` | Agent SDK → sidecar → Rust → Svelte pipeline | `sidecar/src/`, streaming code |
| `orqa-store-patterns` | Svelte 5 rune stores, reactive data flow | `ui/lib/stores/`, `ui/lib/components/` |
| `orqa-store-orchestration` | Multi-store coordination, $effect wiring | `ui/lib/stores/` |
| `orqa-governance` | Artifacts, scanning, lessons, rules | `.orqa/` |
| `orqa-testing` | Test commands, patterns, mock boundaries | Test-related work |
| `orqa-native-search` | Embedded DuckDB + ONNX search engine | `src-tauri/src/search/` |

When delegating, the orchestrator includes: "Load these project skills before starting: [list]"

## Context Resolution (Tier 3)

The `code-search` wrapper skill detects the runtime context and resolves to the correct search implementation:

| Available Tools | Context | Resolved Skill |
|----------------|---------|---------------|
| `mcp__chunkhound__*` | CLI (Claude Code) | `chunkhound` |
| `search_regex`, `search_semantic`, `code_research` as Tauri commands | App (OrqaStudio) | `orqa-native-search` |
| Neither | Fallback | Use Grep/Glob, note in task summary |

## Portable Skills (Tier 1 — Agent-Declared)

| Skill | Purpose | On Which Roles |
|-------|---------|---------------|
| `code-search` | Code search wrapper (mandatory) | ALL roles |
| `orqa-composability` | Composability philosophy (mandatory) | ALL roles |
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
| `rust-async-patterns` | Rust async/await, error handling | Implementer (backend), Reviewer (code quality) |
| `svelte5-best-practices` | Svelte 5 runes, components | Implementer (frontend), Designer, Reviewer (code quality) |
| `tailwind-design-system` | Tailwind CSS utilities | Designer, Reviewer (UX tasks) |
| `typescript-advanced-types` | Strict TypeScript patterns | Implementer (frontend) |
| `tauri-v2` | Tauri commands, plugins, security | Implementer (backend), Reviewer (security) |

## Rule Status Awareness

Rules carry a `status` field in their YAML frontmatter: `active` or `inactive`. When loading rules for enforcement:

- **`status: active`** — The rule is enforced. Agents must comply.
- **`status: inactive`** — The rule is NOT enforced. Agents should skip it. The file is preserved for historical reference.
- **Missing status field** — Treat as `active` (backwards compatibility).

The orchestrator and all agents MUST check rule status before applying enforcement. Inactive rules are not loaded into agent context.

## Audit

- The orchestrator periodically audits that agent Tier 1 skill lists contain only portable skills + universal wrappers
- No `orqa-*` skills (except `orqa-composability`) should appear in agent YAML frontmatter
- The injection table in the orchestrator is the single source of truth for Tier 2 loading
- All skill changes are documented in `.orqa/documentation/process/skills-log.md`

## App-Managed Loading

In OrqaStudio, skills are loaded via the `load_skill` tool and managed by the app's process enforcement layer. The app tracks which skills each agent has loaded and can enforce loading before task execution begins. The YAML frontmatter `skills:` declarations remain authoritative for CLI usage, where agents self-load skills based on their frontmatter lists.

## Related Rules

- RULE-023 (required-reading) — docs that agents must load (complementary to skills)
- RULE-005 (chunkhound-usage) — code search usage and context detection
- RULE-001 (agent-delegation) — orchestrator must delegate to agents, not implement directly
