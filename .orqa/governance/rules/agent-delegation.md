---
id: agent-delegation
layer: canon
status: active
title: "Agent Delegation"
description: "The orchestrator coordinates but does not implement. All implementation is delegated to universal roles with appropriate skills."
scope: system
---


The orchestrator coordinates. It does NOT implement. Every implementation task is delegated to a universal role with the appropriate skills loaded.

## Universal Roles

| Role | Purpose | When to Use |
|------|---------|-------------|
| **Researcher** | Investigate questions, gather information | Before planning, when understanding is needed |
| **Planner** | Design approaches, map dependencies | Before implementation, when a plan is needed |
| **Implementer** | Build things — code, deliverables | When work needs to be done |
| **Reviewer** | Check quality, compliance, correctness | After implementation, before merge |
| **Writer** | Create documentation, specifications | Before and alongside implementation |
| **Designer** | Design experiences, interfaces, structures | When UI/UX work is needed |

## What the Orchestrator May Do Directly

- Read files for planning and coordination
- Write plans, documentation, and session state
- Create and update governance files in `.orqa/` (rules, agents, skills, hooks)
- Single-line fixes, typo corrections, config file edits
- Coordinate across agents, report status, manage worktrees

## What the Orchestrator MUST Delegate

- Any change to `src-tauri/` (Rust backend code) — delegate to Implementer with backend skills
- Any change to `ui/` (Svelte frontend code) — delegate to Implementer with frontend skills, or Designer
- Any change to `sidecar/` (Agent SDK sidecar) — delegate to Implementer with backend skills
- Running and interpreting test suites — delegate to Reviewer with test-engineering skills
- Code review and compliance checks — delegate to Reviewer with code-quality-review skills
- UX compliance reviews — delegate to Reviewer with ux-compliance-review skills
- Architecture assessments — delegate to Planner or Researcher with architectural-evaluation skills
- Debugging cross-boundary issues — delegate to Implementer with diagnostic-methodology skills

## Delegation Protocol

When delegating to a role:

1. **Name the role** — Every delegation must explicitly state which universal role is being used
2. **Specify skills** — List the skills to load (e.g., "Implementer with rust-async-patterns, tauri-v2, orqa-ipc-patterns")
3. **Scope the task** — Clear description with acceptance criteria
4. **Provide context** — File paths, relevant docs, constraints
5. **Verify the result** — Check the agent's output against acceptance criteria before reporting to the user

## Exceptions

The orchestrator may bypass delegation for:

- Governance artifacts (`.orqa/governance/rules/`, `.orqa/team/agents/`, `.orqa/team/skills/`) — these ARE the orchestrator's domain
- Process documentation (`.orqa/documentation/process/`) — orchestration is a process concern
- Session state (`tmp/session-state.md`) — coordination artifact
- Plan files (`.orqa/planning/plans/`) — planning is an orchestrator responsibility

## Resource Safety (NON-NEGOTIABLE)

Parallel agents sharing a worktree can exhaust system resources. Rust compilation is especially dangerous — each `rustc` instance consumes 300-500MB RAM, and `cargo` spawns one per CPU core by default.

### Rules

1. **Never run two compilation-heavy agents in parallel in the same worktree.** If both agents could trigger `cargo check`, `cargo build`, or `cargo clippy`, run them sequentially — not in parallel.

2. **Stagger by weight.** When parallelizing backend + frontend work, run the frontend agent first (svelte-check is lightweight ~50MB). Only launch the backend agent (cargo is heavy) after the frontend agent completes, or run it in a separate worktree.

3. **Frontend-only agents must not run cargo.** When delegating frontend-only work, explicitly instruct the agent: "Do NOT run any cargo commands." Frontend verification uses `npx svelte-check` only.

4. **Separate worktrees for truly parallel compilation.** If two agents must both compile Rust simultaneously, each MUST have its own worktree.

5. **Cap Rust parallelism on resource-constrained machines.** Set `CARGO_BUILD_JOBS=2` (or lower) to limit concurrent rustc instances.

### Compilation Risk by Skill Combination

| Role + Skills | Compilation Risk | Safe to Parallelize With |
|--------------|-----------------|-------------------------|
| Implementer + backend skills | High (cargo) | Frontend-only agents |
| Implementer + database skills | High (cargo) | Frontend-only agents |
| Implementer + frontend skills | Low (svelte-check) | Any agent |
| Designer | Low (svelte-check) | Any agent |
| Reviewer + code-quality-review | High (cargo clippy + cargo test) | Nothing — run alone |
| Reviewer + test-engineering | High (cargo test) | Nothing — run alone |
| Implementer + diagnostic-methodology | Medium (may compile) | Frontend-only agents |
| Researcher, Planner, Writer | None | Any agent |

## Why This Exists

Without this rule, the orchestrator accumulates implementation details in its context window, reducing its capacity for coordination. Delegation keeps the orchestrator focused on process while agents handle implementation in isolated contexts.

## Related Rules

- `skill-enforcement.md` — agents must load skills before starting work
- `required-reading.md` — agents must read governing docs before implementation
- `honest-reporting.md` — agents must report status accurately
