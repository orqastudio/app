---
id: agent-delegation
title: "Agent Delegation"
description: "The orchestrator coordinates but does not implement. All implementation is delegated to specialized agents."
scope: system
---


The orchestrator coordinates. It does NOT implement. Every implementation task is delegated to a specialized agent.

## What the Orchestrator May Do Directly

- Read files for planning and coordination
- Write plans, documentation, and session state
- Create and update `.claude/` governance files (rules, agents, skills, hooks)
- Single-line fixes, typo corrections, config file edits
- Coordinate across agents, report status, manage worktrees

## What the Orchestrator MUST Delegate

- Any change to `src-tauri/` (Rust backend code) — delegate to `backend-engineer` or `data-engineer`
- Any change to `ui/` (Svelte frontend code) — delegate to `frontend-engineer` or `designer`
- Any change to `sidecar/` (Agent SDK sidecar) — delegate to `backend-engineer`
- Running and interpreting test suites — delegate to `test-engineer` or `qa-tester`
- Code review and compliance checks — delegate to `code-reviewer`
- UX compliance reviews — delegate to `ux-reviewer`
- Architecture assessments — delegate to `systems-architect`
- Debugging cross-boundary issues — delegate to `debugger`

## Delegation Protocol

When delegating to an agent:

1. **Name the agent** — Every delegation must explicitly state which agent is being used
2. **Scope the task** — Clear description with acceptance criteria
3. **Provide context** — File paths, relevant docs, constraints
4. **Verify the result** — Check the agent's output against acceptance criteria before reporting to the user

## Exceptions

The orchestrator may bypass delegation for:

- Governance artifacts (`.orqa/rules/`, `.orqa/agents/`, `.orqa/skills/`) — these ARE the orchestrator's domain
- Process documentation (`docs/process/`) — orchestration is a process concern
- Session state (`tmp/session-state.md`) — coordination artifact
- Plan files (`.orqa/plans/`) — planning is an orchestrator responsibility

## Resource Safety (NON-NEGOTIABLE)

Parallel agents sharing a worktree can exhaust system resources. Rust compilation is especially dangerous — each `rustc` instance consumes 300-500MB RAM, and `cargo` spawns one per CPU core by default.

### Rules

1. **Never run two compilation-heavy agents in parallel in the same worktree.** If both agents could trigger `cargo check`, `cargo build`, or `cargo clippy`, run them sequentially — not in parallel.

2. **Stagger by weight.** When parallelizing a backend + frontend task, run the frontend agent first (svelte-check is lightweight ~50MB). Only launch the backend agent (cargo is heavy) after the frontend agent completes, or run it in a separate worktree with its own `target/` directory.

3. **Frontend agents must not run cargo.** When delegating frontend-only work, explicitly instruct the agent: "Do NOT run any cargo commands." Frontend verification uses `npx svelte-check` only.

4. **Separate worktrees for truly parallel compilation.** If two agents must both compile Rust simultaneously, each MUST have its own worktree so they don't fight over the same `target/` directory or Cargo lock file.

5. **Cap Rust parallelism on resource-constrained machines.** Set `CARGO_BUILD_JOBS=2` (or lower) in the worktree environment to limit concurrent rustc instances. The default (num_cpus) can overwhelm machines running other workloads.

### What Counts as Compilation-Heavy

| Agent Type | Compilation Risk | Safe to Parallelize With |
|-----------|-----------------|-------------------------|
| `backend-engineer` | High (cargo) | Frontend agents only (separate worktree for other backend agents) |
| `data-engineer` | High (cargo) | Frontend agents only |
| `frontend-engineer` | Low (svelte-check) | Any agent |
| `designer` | Low (svelte-check) | Any agent |
| `code-reviewer` | High (cargo clippy + cargo test) | Nothing — run alone |
| `test-engineer` | High (cargo test) | Nothing — run alone |
| `debugger` | Medium (may compile) | Frontend agents only |
| All others (docs, security, architect) | None | Any agent |

## Why This Exists

Without this rule, the orchestrator accumulates implementation details in its context window, reducing its capacity for coordination. Delegation keeps the orchestrator focused on process while agents handle implementation in isolated contexts.

## Related Rules

- `skill-enforcement.md` — agents must load skills before starting work
- `required-reading.md` — agents must read governing docs before implementation
- `honest-reporting.md` — agents must report status accurately
