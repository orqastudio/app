---
scope: system
---

# Skill Enforcement (NON-NEGOTIABLE)

Every agent MUST have a `skills:` list in its YAML frontmatter that includes ALL skills it needs for its domain.

## Skill Loading Order

When an agent starts a task, it MUST follow this sequence:

1. **Load all declared skills** — Every skill in the agent's `skills:` YAML frontmatter is loaded via `Skill(name)` before any other work begins
2. **Read Required Reading** — Load governing documentation listed in the agent's Required Reading section
3. **Begin implementation** — Only after steps 1-2 are complete

If a skill fails to load, the agent MUST report the failure explicitly. Do NOT silently continue without the skill.

## Universal Skills

- The `chunkhound` skill MUST be in every agent's skill list — it is a universal skill for code search
- The `orqa-composability` skill MUST be in every agent's skill list — it is the meta-skill that shapes how all code is structured
- The orchestrator loads `chunkhound`, `planning`, and `orqa-composability` skills on every session (via CLAUDE.md)

## Project-Level Skills

OrqaStudio has project-specific skills that capture codebase patterns:

| Skill | Domain | Used By |
|-------|--------|---------|
| `orqa-composability` | Composability philosophy, pure functions, pipelines, feature isolation | ALL agents (universal) |
| `orqa-ipc-patterns` | Tauri IPC, Channel<T>, command registration | backend-engineer, frontend-engineer, debugger, systems-architect |
| `orqa-store-patterns` | Svelte 5 rune stores, reactive data flow | frontend-engineer, designer, debugger |
| `orqa-store-orchestration` | Multi-store coordination, $effect wiring, circular dep prevention | frontend-engineer, designer, debugger, systems-architect |
| `orqa-streaming` | Agent SDK → sidecar → NDJSON → Rust → Svelte pipeline | backend-engineer, frontend-engineer, debugger |
| `orqa-domain-services` | Domain service anatomy, command delegation, service composition | backend-engineer, data-engineer, systems-architect, refactor-agent |
| `orqa-repository-pattern` | SQLite repos, migrations, query patterns, connection management | backend-engineer, data-engineer |
| `orqa-error-composition` | OrqaError flow, From impls, frontend error handling | backend-engineer, frontend-engineer, debugger, code-reviewer |
| `orqa-governance` | Artifacts, scanning, lessons, rules, `.orqa/` structure | agent-maintainer, code-reviewer, documentation-writer |
| `orqa-testing` | Test commands, patterns, mock boundaries, file locations | test-engineer, qa-tester, code-reviewer |

## Agent-Specific Skills

- When the orchestrator delegates to an agent, the agent's YAML-declared skills are auto-loaded
- Skill lists should match the agent's Required Reading domains — if an agent reads frontend docs, it should have frontend-related skills
- Generic skills (e.g., `rust-async-patterns`) teach language patterns; project skills (e.g., `orqa-ipc-patterns`) teach THIS codebase's patterns

## Audit

- The `agent-maintainer` periodically audits that agent skill lists match their Required Reading domains
- Missing skills are added; irrelevant skills are removed
- All skill changes are documented in `docs/process/skills-log.md`

## App-Managed Loading

In OrqaStudio, skills are loaded via the `load_skill` tool and managed by the app's process enforcement layer. The app tracks which skills each agent has loaded and can enforce loading before task execution begins. The YAML frontmatter `skills:` declarations remain authoritative for CLI usage, where agents self-load skills based on their frontmatter lists.

## Related Rules

- `required-reading.md` — docs that agents must load (complementary to skills)
- `chunkhound-usage.md` — enforcement of ChunkHound as the preferred search tool
- `agent-delegation.md` — orchestrator must delegate to agents, not implement directly
