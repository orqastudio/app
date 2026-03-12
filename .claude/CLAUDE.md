# EPIC-053: Graph-Based Knowledge Injection

**This is a temporary CLAUDE.md for working through EPIC-053 tasks.**
**After completion, re-symlink to the new graph-based orchestrator prompt.**

---

## Your Mission

You are implementing EPIC-053 — transforming OrqaStudio's enforcement and context
injection from content-based (inject everything into prompts) to graph-based (inject
the artifact relationship graph, agents read nodes on demand).

The end state: the orchestrator prompt shrinks from ~2000 lines to ~200 lines, the
companion plugin reads the graph for context injection, and the system builds itself
through normal use.

## How to Work

1. **Work through ALL tasks** in this file until every task is marked `done`
2. **Read each task** before starting it: `.orqa/planning/tasks/TASK-NNN.md`
3. **Follow task dependencies** — check `depends-on` before starting; blocked tasks wait
4. **Update task status** as you work: set `status: in-progress` in the task file when starting, `status: done` when complete
5. **Update THIS file's status column** in the task table below as you go — keep it in sync with the task files
6. **Commit at natural boundaries** — don't accumulate large uncommitted batches (commit every 1-3 tasks)
7. **Ask the user** before making architectural decisions not covered by the epic
8. **Parallelise where possible** — tasks without dependencies on each other can be done in parallel
9. **Don't stop** until all tasks are `done` or explicitly blocked on user input

## Task Sequence

Work through these in dependency order. Tasks without dependencies can be parallelised.

### Phase 1: Schema & Graph Foundation

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-208 | Add `docs` and `skills` fields to task schema | — | done |
| TASK-209 | Backfill `docs` field on existing tasks | TASK-208 | done |
| TASK-210 | Backfill `skills` field on existing tasks | TASK-208 | done |

### Phase 2: Content Extraction (can start in parallel with Phase 1)

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-212 | Extract dev server/worktree content to skills or docs | — | done |
| TASK-213 | Extract project-specific requirements to graph artifacts | — | done |

### Phase 3: Orchestrator Rewrite

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-211 | Write graph-based orchestrator prompt (~200 lines) | TASK-208, 209, 210 | done |

### Phase 4: Plugin Graph Integration

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-214 | Plugin reads task `docs` and `skills` for injection | TASK-208 | done |
| TASK-215 | Plugin reads skill `scope` fields for path-based injection | — | done |
| TASK-216 | Plugin extends graph on artifact creation | — | done |

### Phase 5: Web Search & Research Parity

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-217 | Add `web_fetch`/`web_search` capabilities to agents | — | done |
| TASK-218 | Add `sources` field to research schema | — | done |

### Phase 6: Core Protection

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-221 | Pre-commit hook blocks core artifact modifications | TASK-211 | done |

### Phase 6b: Decision Audit

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-222 | Audit existing ADs (AD-001–AD-037) against AD-038/039/040 | — | done |

### Phase 6c: Project Setup Integration

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-223 | Create skill for epic-requirement inference during project setup | — | done |

### Phase 7: Verification

| Task | Title | Depends On | Status |
|------|-------|-----------|--------|
| TASK-219 | E2E test: task with docs/skills → correct injection | TASK-214, 215 | done |
| TASK-220 | Dogfood session with ~200-line prompt | TASK-211, 214 | done |

## Key Files

| File | Purpose |
|------|---------|
| `.orqa/planning/epics/EPIC-053.md` | Epic definition with full design |
| `.orqa/planning/tasks/TASK-208..220.md` | Individual task definitions |
| `.orqa/team/agents/orchestrator.md` | Current orchestrator prompt (will be rewritten) |
| `.orqa/planning/tasks/schema.json` | Task schema (add docs/skills fields) |
| `.orqa/planning/research/schema.json` | Research schema (add sources field) |
| `.orqa/documentation/product/artifact-framework.md` | Artifact schemas doc |
| `.orqa/plugins/orqastudio-claude-plugin/` | Companion plugin (graph integration) |
| `.orqa/team/skills/research-methodology/SKILL.md` | Research best practices skill |

## Project Context

- **Tech stack**: Tauri v2 (Rust) + Svelte 5 + TypeScript + SQLite
- **Governance**: `.orqa/` directory with markdown artifacts and YAML frontmatter
- **Plugin**: Claude Code companion plugin in `.orqa/plugins/orqastudio-claude-plugin/`
- **Skills**: `.orqa/team/skills/` — each skill is a directory with `SKILL.md`
- **Rules**: `.orqa/governance/rules/RULE-NNN.md` — 40 rules with enforcement entries
- **Agents**: `.orqa/team/agents/` — 7 universal roles
- **Dev commands**: Use `make` targets (see `.orqa/documentation/development/commands.md`)
- **Dogfooding**: This project uses itself — `.orqa/project.json` has `dogfood: true`

## Core Graph Protection (RULE-044)

Core graph artifacts are firmware — do NOT modify schema.json files, core skills
(composability, planning, research-methodology, orqa-code-search), or the
orchestrator prompt during normal project work. **Exception**: This IS a dogfood
project, so you CAN modify them for EPIC-053 tasks. But be aware the enforcement
engine will block these writes — the dogfood bypass in the enforcement engine
must be respected.

## Safety Constraints (NON-NEGOTIABLE)

- No `unwrap()` / `expect()` / `panic!()` in Rust production code
- No `--no-verify` on git commits
- No force push to main
- No `any` types in TypeScript
- No Svelte 4 patterns (no `$:`, no `export let`, no `let:`)
- Tauri `invoke()` is the ONLY frontend-backend interface
- Documentation before code — update docs first if implementation changes target state
- Commit to branches, not main (unless governance-only changes)

## The Self-Building Vision

The end state is a system where:

1. The orchestrator prompt just says "read and extend the graph"
2. The companion plugin reads the graph to inject context
3. Agents extend the graph through normal work
4. Each session benefits from richer graph relationships
5. Web search feeds research docs → research informs decisions → decisions improve the graph
6. The plugin architecture layers on top and the system becomes self-building

## Research Best Practices

When using `WebSearch` / `WebFetch` during any task:

- Load the `research-methodology` skill for source verification protocol
- Verify sources against the credibility tier system (T1-T4)
- Cross-reference non-trivial claims with at least one independent source
- Document confidence levels (Confirmed / Likely / Uncertain / Speculative)
- Add structured `sources` entries to research artifacts
- Check version compatibility — a Tauri v1 answer is wrong for Tauri v2

## After All Tasks Complete

1. The new orchestrator prompt lives at `.orqa/team/agents/orchestrator.md`
2. Re-create the symlink: `ln -sf .orqa/team/agents/orchestrator.md .claude/CLAUDE.md`
3. Start a new session to dogfood-test the graph-based prompt
4. Iterate based on what's missing
