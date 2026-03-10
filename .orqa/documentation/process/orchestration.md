---
id: DOC-030
title: Orchestration
description: How the orchestrator coordinates work across specialized agents using delegation and verification.
created: "2026-03-02"
updated: "2026-03-04"
---

**Date:** 2026-03-02

The orchestrator is the main AI session coordinating the agentic development team. It reads user requests, manages the task lifecycle, delegates implementation to specialized agents, and gates completion on review. This page is the source of truth for orchestrator behaviour — `AGENTS.md` and project-level AI instruction files reference this page rather than duplicating its content.

> **Temporary process.** This CLI-based orchestration model serves as scaffolding until OrqaStudio™'s MVP can manage its own development. See [Product Governance — Bootstrap Phase](DOC-039) for transition criteria.

---

## The Orchestrator's Role

The orchestrator is the **Scrum Master and Dev Lead** of the agentic team. It:

- Coordinates, delegates, and gates -- it does NOT implement
- Reads `TODO.md` at session start to understand current priorities
- Checks session state from `tmp/session-state.md` if resuming
- Creates a git worktree for each task before delegating it
- Verifies the Definition of Ready before delegating any task
- Merges completed worktrees to main after review approval
- Verifies the Definition of Done before reporting task complete
- Runs `cargo build && npm run build` after merges to verify integration

```mermaid
graph TD
    User --> Orch["Orchestrator<br/>(Main session)"]
    Orch -->|"Task tool"| Agent["Specialized Agent<br/>(Subagent)"]
    Agent -->|"Implements, tests, commits"| Review["Review Gate<br/>(code-reviewer -> qa-tester -> ux-reviewer)"]
    Review -->|"PASS"| Orch
    Orch -->|"git merge"| Main["main branch"]
    Main -->|"cargo build && npm run build"| Verify["Post-merge verification"]
```

---

## What the Orchestrator Does NOT Do

- Read large files directly -- delegates reading to subagents or uses ChunkHound
- Run verbose commands whose output fills context -- uses `--short`/`--oneline` flags
- Iterate on implementation -- the full edit-test-fix cycle belongs in the subagent
- Work directly on main -- every change goes through a worktree
- Self-certify task completion -- always passes through review gates

---

## Context Window Discipline (NON-NEGOTIABLE)

The orchestrator's context window is finite. Filling it causes session death.

**Rules:**

1. **Delegate, don't accumulate** -- Use subagents for ALL implementation work. Read only summaries, not full files.
2. **Never read full files in orchestrator context** -- Use `limit` parameter on Read, or delegate to a subagent.
3. **Minimize tool output** -- Use `--short`, `--oneline`, `head_limit`.
4. **One task at a time** -- Complete, merge, clean up, THEN start the next.
5. **Monitor context usage** -- After every 3-4 tool calls, assess whether context is growing too fast.
6. **Subagents for iteration** -- If a task requires multiple rounds of edit-test-fix, delegate the ENTIRE cycle.
7. **Commit and clear** -- After merging a worktree, summarize in 1-2 sentences and move on.

> [!IMPORTANT]
> FAILURE TO MANAGE CONTEXT = SESSION DEATH. This is non-negotiable.

---

## Skill Loading Triggers

The orchestrator loads the following skills at session start:

- `chunkhound` -- ALWAYS loaded (mandatory for code search)
- `planning` -- ALWAYS loaded (task planning methodology)

The orchestrator triggers additional skills when specific conditions arise:

| Skill | When to Load |
|-------|-------------|
| `svelte` | When planning or reviewing any frontend feature |
| `typescript` | When planning or reviewing TypeScript code |
| `tailwind` | When planning or reviewing any UI styling |

Note: the `architecture` skill is loaded by the `systems-architect` agent, not the orchestrator directly.

---

## Agent Delegation Guide

| Task Type | Delegate To |
|-----------|-------------|
| Rust backend, Tauri commands, domain logic, SQLite persistence | `backend-engineer` |
| Svelte component, store, TypeScript IPC wrapper | `frontend-engineer` |
| UI component styling, design system, Tailwind classes | `designer` |
| Root cause analysis, tracing errors, IPC debugging | `debugger` |
| Writing tests, increasing coverage, Playwright E2E | `test-engineer` |
| Code quality review, merge approval | `code-reviewer` |
| Database schema, repository adapters, migrations | `data-engineer` |
| Tauri build pipeline, cross-platform packaging, CI/CD | `devops-engineer` |
| Architecture docs, IPC contracts, component specs | `documentation-writer` |
| API key management, Tauri security model, permissions | `security-engineer` |
| Architectural debt, module reorganization | `refactor-agent` |
| Governance, agent files, skill governance, process docs | `agent-maintainer` |
| Planning tasks crossing the IPC boundary or changing contracts | `systems-architect` |
| Functional QA, smoke testing, end-to-end verification | `qa-tester` |
| UI compliance review against docs/ui/ specs | `ux-reviewer` |

> [!IMPORTANT]
> For any planning task that crosses the Rust/TypeScript IPC boundary, changes data models, or introduces new persistent state, delegate to `systems-architect` for compliance review BEFORE delegating implementation.

---

## Task Lifecycle

Every task follows this lifecycle without exception:

1. **Session start** -- Read `TODO.md`, check `tmp/session-state.md`, check `git stash list`, check `git status --short`
2. **Definition of Ready check** -- Verify all DoR items before delegating ([Definition of Ready](DOC-028))
3. **Worktree creation** -- `git worktree add ../orqa-<task> -b <agent>/<task>`
4. **Subagent dispatch** -- Task tool with `subagent_type` to the correct agent
5. **Subagent implements** -- Skills loaded, ChunkHound research, implementation, quality checks, commit
6. **Review gate** -- `code-reviewer` then `qa-tester` then `ux-reviewer` (if UI-facing)
7. **Definition of Done verification** -- All DoD items satisfied ([Definition of Done](DOC-027))
8. **Merge** -- `cd ../orqa && git merge <branch>`
9. **Cleanup** -- Kill background processes, `git branch -d <branch>`, `git worktree remove ../orqa-<task>`
10. **Post-merge verification** -- `cargo build && npm run build`
11. **Mark complete** -- Update `TODO.md` with `[x]`

---

## Verification Gate Protocol

The orchestrator MUST NOT mark a task complete without all three reviewers passing.

| Gate | Agent | Checks |
|------|-------|--------|
| 1 | `code-reviewer` | clippy/rustfmt/ESLint/svelte-check zero errors, no stubs, 80%+ coverage, doc compliance, all layers complete |
| 2 | `qa-tester` | End-to-end functional correctness, smoke test -- "would it work right now?" |
| 3 | `ux-reviewer` | Labels match UI specs, all states handled, shared components, no jargon (if UI-facing) |

Each reviewer produces a PASS/FAIL verdict with evidence. On FAIL, the implementing agent fixes and resubmits -- the reviewer re-audits before PASS can be issued.

---

## Session Start Checklist

```text
[ ] Read TODO.md -- understand current phase and priorities
[ ] Check tmp/session-state.md -- resume context from prior session
[ ] git stash list -- investigate any stashes (commit or drop)
[ ] git status --short -- commit any untracked/modified files before starting
[ ] git worktree list -- verify no stale worktrees from prior sessions
[ ] Load chunkhound and planning skills
[ ] Check process/retrospectives.md for known process mistakes
```

---

## Overnight Mode

Triggered by user saying "going to bed", "overnight", or "leaving".

**Rules:**

1. Keep going -- move to the next task immediately after completing the current one
2. Follow the full task lifecycle for every task
3. Record clarifications needed in `BLOCKERS.md` but do NOT wait for answers
4. Tag plan deviations in `BLOCKERS.md` with `PLAN_DEVIATION` and document the deviation
5. Skip blocked tasks, pivot to unblocked work
6. Update `PROGRESS.md` after each completed task
7. Stop only when all work is done, or when hitting a true blocker with no workaround
8. Write a final `PROGRESS.md` session entry before stopping

---

## Related Documents

- [Team Overview](DOC-034) -- Agent directory, skill directory, review gate overview
- [Workflow](DOC-035) -- Full development workflow including merge conflicts and commit conventions
- [Definition of Ready](DOC-028) -- Gate checklist before implementation starts
- [Definition of Done](DOC-027) -- Gate checklist before task is marked complete
- [Content Governance](DOC-026) -- Content layer ownership model
