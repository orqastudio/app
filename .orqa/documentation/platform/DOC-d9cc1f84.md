---
id: DOC-d9cc1f84
type: doc
status: active
title: Orchestration
domain: architecture
category: architecture
description: How the orchestrator coordinates work across specialized agents using delegation and verification.
created: 2026-03-02
updated: 2026-04-13
sort: 20
relationships:
  - target: PD-48b310f9
    type: documents
  - target: RULE-dd5b69e6
    type: documents
    rationale: Documentation page references RULE-dd5b69e6
  - target: RULE-8abcbfd5
    type: documents
    rationale: Documentation page references RULE-8abcbfd5
  - target: RULE-87ba1b81
    type: documents
    rationale: This document is the source-of-truth for orchestrator behaviour and agent delegation that RULE-87ba1b81 enforces
---

## Purpose

The orchestrator exists to maintain coherence between what the user intends and what the codebase becomes. Without coordination, agentic work drifts: agents duplicate effort, bypass gates, and accumulate debt that future agents can't reason about. The orchestrator is the thread that holds it together.

Delegation is not a convenience — it is a structural requirement. When the orchestrator implements code directly, it accumulates context that crowds out coordination capacity, and it loses the independent perspective that makes review meaningful. An orchestrator that implements is a system that has lost its quality gate. Every implementation task goes to an agent precisely so the orchestrator can verify the result from the outside.

This matters most for continuity. OrqaStudio is developed across many sessions, by agents that each start fresh. The orchestrator is what carries intent across that boundary — it reads task artifacts, checks session state, and ensures that the next session picks up where the last left off without losing coherence. That continuity is what [PILLAR-a6a4bbbb](PILLAR-a6a4bbbb) (Purpose Through Continuity) means in practice: not just preserving work, but preserving the understanding of why the work was done and what comes next.

This page is the source of truth for orchestrator behaviour. Agent instruction files reference this page rather than duplicating its content.

---

## The Orchestrator's Role

The orchestrator is the **process coordinator** of the agentic team. It:

- Coordinates, delegates, and gates -- it does NOT implement
- Reads task artifacts in `.orqa/implementation/tasks/` at session start to understand current priorities
- Checks session state from `.state/session-state.md` if resuming
- Creates a named team (TeamCreate) before spawning any agents
- Creates tasks within the team (TaskCreate) for tracking
- Delegates all implementation to background agents (Agent, `run_in_background: true`)
- Reads findings from `.state/` files — does NOT accumulate agent output in context
- Verifies the Definition of Ready before delegating any task
- Spawns a separate Reviewer agent after each implementation is complete
- Accepts task completion only on Reviewer PASS verdict
- Commits completed work and cleans up the team (TeamDelete)
- Verifies the Definition of Done before reporting task complete
- Runs `cargo build && npm run build` after commits to verify integration

```mermaid
graph TD
    User --> Orch["Orchestrator<br/>(Main session)"]
    Orch -->|"TeamCreate + TaskCreate"| Team["Named Team"]
    Team -->|"Agent (background)"| Impl["Implementer Agent"]
    Impl -->|"writes findings to .state/"| Orch
    Orch -->|"Agent (background)"| Rev["Reviewer Agent"]
    Rev -->|"PASS/FAIL verdict"| Orch
    Orch -->|"PASS: commit + TeamDelete"| Main["main branch"]
    Main -->|"cargo build && npm run build"| Verify["Post-merge verification"]
```

---

## What the Orchestrator Does NOT Do

- Read large files directly -- delegates reading to subagents or uses context-aware search (`orqa-code-search`)
- Run verbose commands whose output fills context -- uses `--short`/`--oneline` flags
- Iterate on implementation -- the full edit-test-fix cycle belongs in the subagent
- Implement code, write files, or edit source directly -- all work goes to background agents
- Self-certify task completion -- always spawns a Reviewer agent for independent verification
- Accumulate agent output in context -- reads structured summaries from `.state/` findings files only

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
7. **Commit and clear** -- After the Reviewer returns a PASS verdict, commit all changes and run TeamDelete to clean up the team. Summarise completed work in 1-2 sentences in session state.

> [!IMPORTANT]
> FAILURE TO MANAGE CONTEXT = SESSION DEATH. This is non-negotiable.

---

## Skill Loading Triggers

The orchestrator loads the following skills at session start:

- `orqa-code-search` -- ALWAYS loaded (context-aware search wrapper; provides `search_regex`, `search_semantic`, `search_research` via orqastudio MCP server in CLI or native ONNX engine in App)
- `composability` -- ALWAYS loaded (composability philosophy)
- `planning` -- ALWAYS loaded (task planning methodology)

Additional skills are injected by the orchestrator based on task scope per [RULE-dd5b69e6](RULE-dd5b69e6). The orchestrator reads the task's `skills` field and includes them in the delegation prompt. See the Tier 2 injection table in [RULE-dd5b69e6](RULE-dd5b69e6) for the full mapping.

---

## Agent Delegation Guide

All agents are universal roles (see [PD-48b310f9](PD-48b310f9)). Agent definitions declare **capabilities** (not tools); these are resolved to provider-specific tool names at delegation time per [RULE-8abcbfd5](RULE-8abcbfd5). Domain expertise is loaded via skills — the role + skills combination determines capability.

| Task Type | Role | Skills to Load |
| ----------- | ------ | ---------------- |
| Rust backend, Tauri commands, domain logic, SQLite persistence | Implementer | `rust-async-patterns`, `tauri-v2`, `orqa-ipc-patterns`, `orqa-error-composition` |
| Svelte component, store, TypeScript IPC wrapper | Implementer | `svelte5-best-practices`, `typescript-advanced-types`, `orqa-store-patterns` |
| UI component styling, design system, Tailwind classes | Designer | `svelte5-best-practices`, `tailwind-design-system` |
| Root cause analysis, tracing errors, IPC debugging | Implementer | `diagnostic-methodology`, `rust-async-patterns`, `tauri-v2` |
| Writing tests, increasing coverage, Playwright E2E | Reviewer | `test-engineering`, `rust-async-patterns` |
| Code quality review, merge approval | Reviewer | `code-quality-review`, `rust-async-patterns`, `svelte5-best-practices` |
| Database schema, repository adapters, migrations | Implementer | `rust-async-patterns`, `orqa-repository-pattern` |
| Tauri build pipeline, cross-platform packaging, CI/CD | Implementer | `tauri-v2` |
| Architecture docs, IPC contracts, component specs | Writer | `architecture` |
| API key management, Tauri security model, permissions | Reviewer | `security-audit`, `tauri-v2` |
| Architectural debt, module reorganization | Implementer | `restructuring-methodology`, `rust-async-patterns` |
| Governance, agent files, skill governance, process docs | Orchestrator | `governance-maintenance`, `skills-maintenance` |
| Planning tasks crossing the IPC boundary or changing contracts | Planner | `architecture`, `planning`, `tauri-v2` |
| Functional QA, smoke testing, end-to-end verification | Reviewer | `qa-verification`, `svelte5-best-practices` |
| UI compliance review against `.orqa/documentation/reference/` specs | Reviewer | `ux-compliance-review`, `svelte5-best-practices` |

> [!IMPORTANT]
> For any planning task that crosses the Rust/TypeScript IPC boundary, changes data models, or introduces new persistent state, delegate to a Planner with `architecture` skills for compliance review BEFORE delegating implementation.

---

## Task Lifecycle

Every task follows this lifecycle without exception:

1. **Session start** -- Read task artifacts in `.orqa/implementation/tasks/`, check `.state/session-state.md`, check `git stash list`, check `git status --short`
2. **Team setup** -- `TeamCreate` to create a named team; `TaskCreate` for each task to track
3. **Definition of Ready check** -- Verify all DoR items before delegating (Definition of Ready)
4. **Agent dispatch** -- `Agent` tool with `run_in_background: true`, `team_name`, and `subagent_type`; provide structured task prompt with findings file path
5. **Agent implements** -- Skills loaded, code search research, implementation, quality checks; writes findings to `.state/team/<name>/task-<id>.md`
6. **TaskUpdate** -- Agent marks task complete (or failed) via TaskUpdate
7. **Orchestrator reads findings** -- Reads the findings file; does NOT read agent messages directly
8. **Reviewer agent** -- Orchestrator spawns a separate Reviewer agent to verify acceptance criteria with evidence; Reviewer writes PASS/FAIL verdict to its own findings file
9. **Definition of Done verification** -- All DoD items satisfied (Definition of Done)
10. **Accept or fix** -- On PASS: proceed to commit. On FAIL: delegate fix to implementer, re-review
11. **Commit** -- Orchestrator commits all changes with a meaningful commit message
12. **Cleanup** -- `TeamDelete` to remove the team
13. **Post-commit verification** -- `cargo build && npm run build`
14. **Mark complete** -- Update task artifact in `.orqa/implementation/tasks/` to `status: done`

---

## Verification Gate Protocol

The orchestrator MUST NOT mark a task complete without a Reviewer PASS verdict. Self-assessment is not review.

| Step | Who | What |
| ------ | ------- | -------- |
| 1 | Implementer | Writes findings to `.state/team/<name>/task-<id>.md` with work done and evidence |
| 2 | Orchestrator | Reads findings file; spawns a Reviewer agent with the task ACs and the findings file path |
| 3 | Reviewer | Verifies each AC with evidence from the actual code/artifacts; produces PASS or FAIL verdict |
| 4 | Orchestrator | Reads Reviewer verdict; accepts on PASS, delegates fix on FAIL |

The Reviewer does NOT fix issues. It reports specifically which ACs passed and which failed with evidence. On FAIL, the orchestrator delegates the fix back to the implementer, then spawns a fresh Reviewer to re-verify.

---

## Session Start Checklist

```text
[ ] Read task artifacts in `.orqa/implementation/tasks/` -- understand current tasks and priorities
[ ] Check .state/session-state.md -- resume context from prior session
[ ] git stash list -- investigate any stashes (commit or drop)
[ ] git status --short -- commit any untracked/modified files before starting
[ ] Load orqa-code-search, composability, and planning skills
[ ] Check .orqa/learning/lessons/ for known patterns and recurring issues
```

---

## Overnight Mode

Triggered by user saying "going to bed", "overnight", or "leaving".

**Rules:**

1. Keep going -- move to the next task immediately after completing the current one
2. Follow the full task lifecycle for every task
3. Record clarifications needed as blocked task artifacts in `.orqa/implementation/tasks/` but do NOT wait for answers
4. Tag plan deviations with `PLAN_DEVIATION` and document the deviation in the relevant task artifact
5. Skip blocked tasks, pivot to unblocked work
6. Update `PROGRESS.md` after each completed task
7. Stop only when all work is done, or when hitting a true blocker with no workaround
8. Write a final `PROGRESS.md` session entry before stopping

---

## Related Documents

- Team Overview -- Agent directory, skill directory, review gate overview
- Workflow -- Full development workflow including merge conflicts and commit conventions
- Definition of Ready -- Gate checklist before implementation starts
- Definition of Done -- Gate checklist before task is marked complete
- Content Governance -- Content layer ownership model
