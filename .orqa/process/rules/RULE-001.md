---
id: RULE-532100d9
type: rule
title: Agent Delegation
description: The orchestrator coordinates but does not implement. All implementation is delegated to universal roles with appropriate knowledge loaded.
status: active
created: 2026-03-07
updated: 2026-03-14
enforcement:
  - "event: stop"
  - "event: stop"
  - governance-maintenance
relationships:
  - target: DOC-01ddd8aa
    type: documented-by
  - target: DOC-c158f4a6
    type: documented-by
  - target: DOC-b10607c0
    type: documented-by
---
The orchestrator coordinates. It does NOT implement. Every implementation task is delegated to a universal role with the appropriate knowledge loaded.

## Universal Roles

| Role | Purpose | When to Use |
|------|---------|-------------|
| **Researcher** | Investigate questions, gather information | Before planning, when understanding is needed |
| **Planner** | Design approaches, map dependencies | Before implementation, when a plan is needed |
| **Implementer** | Build things — code, deliverables | When work needs to be done |
| **Reviewer** | Check quality, compliance, correctness | After implementation, before merge |
| **Writer** | Create documentation, specifications | Before and alongside implementation |
| **Designer** | Design experiences, interfaces, structures | When UI/UX work is needed |
| **Governance Steward** | Create and maintain `.orqa/` artifacts with graph integrity | When any governance artifact needs creating or updating |

## What the Orchestrator May Do Directly

- Read files for planning and coordination
- Coordinate across agents, report status to the user
- Write session state (`tmp/session-state.md`)

**If the orchestrator is writing anything other than coordination output, the system has failed to delegate.** See the delegation reference (DOC-c158f4a6) for the full work-type-to-role mapping.

## What the Orchestrator MUST Delegate

- Any change to `backend/src-tauri/` (Rust backend code) — delegate to Implementer with backend knowledge
- Any change to `ui/` (Svelte frontend code) — delegate to Implementer with frontend knowledge, or Designer
- Any change to `sidecar/` (Agent SDK sidecar) — delegate to Implementer with backend knowledge
- Any change to `.orqa/` artifacts — delegate to Governance Steward
- Any documentation content — delegate to Writer
- Running and interpreting test suites — delegate to Reviewer with test-engineering knowledge
- Code review and compliance checks — delegate to Reviewer with code-quality-review knowledge
- UX compliance reviews — delegate to Reviewer with ux-compliance-review knowledge
- Architecture assessments — delegate to Planner or Researcher with architectural-evaluation knowledge
- Debugging cross-boundary issues — delegate to Implementer with diagnostic-methodology knowledge

## Delegation Protocol

When delegating to a role:

1. **Name the role** — Every delegation must explicitly state which universal role is being used
2. **Resolve capabilities** — Read the agent's `capabilities` field, determine the current context (CLI or App), and resolve to concrete tool names using [RULE-92dba0cb](RULE-92dba0cb) mapping tables. Include the resolved tool names in the delegation prompt.
3. **Specify knowledge** — List the knowledge to load (e.g., "Implementer with rust-async-patterns, tauri-v2, orqa-ipc-patterns")
4. **Scope the task** — Clear description with acceptance criteria
5. **Provide context** — File paths, relevant docs, constraints
6. **Verify the result** — Check the agent's output against acceptance criteria before reporting to the user

## Autonomous Continuation (NON-NEGOTIABLE)

When the orchestrator has approved tasks and no blocker exists, it MUST continue to the next task without asking the user for permission. The user's interrupt capability is the coordination mechanism — not permission-seeking questions.

**The orchestrator MUST only pause when:**
1. A dependency gate is not met (task depends on incomplete work)
2. A genuine user decision is needed (scope change, ambiguity, trade-off)
3. All work is complete (epic done)

**FORBIDDEN:**
- "Want me to continue?" when tasks are approved and unblocked
- "Shall I proceed with the next phase?" when no decision is needed
- "Ready for X?" when X is already planned and unblocked
- Any variation of asking permission to execute an approved plan

This was promoted from [IMPL-85add0f1](IMPL-85add0f1) after 3 recurrences.

## Exceptions

The orchestrator may bypass delegation for:

- Governance artifacts (`.orqa/process/rules/`, `.orqa/process/agents/`, `.orqa/process/knowledge/`) — these ARE the orchestrator's domain
- Process documentation (`.orqa/documentation/guide/`) — orchestration is a process concern
- Session state (`tmp/session-state.md`) — coordination artifact
- Planning artifact **structure** (`.orqa/delivery/`) — creating/updating frontmatter, status transitions, cross-references. Content authoring (research findings, documentation pages) is delegated to Writer.

## Resource Safety (NON-NEGOTIABLE)

Parallel agents sharing a worktree can exhaust system resources. Rust compilation is especially dangerous — each `rustc` instance consumes 300-500MB RAM, and `cargo` spawns one per CPU core by default.

### Rules

1. **Never run two compilation-heavy agents in parallel in the same worktree.** If both agents could trigger `cargo check`, `cargo build`, or `cargo clippy`, run them sequentially — not in parallel.

2. **Stagger by weight.** When parallelizing backend + frontend work, run the frontend agent first (svelte-check is lightweight ~50MB). Only launch the backend agent (cargo is heavy) after the frontend agent completes, or run it in a separate worktree.

3. **Frontend-only agents must not run cargo.** When delegating frontend-only work, explicitly instruct the agent: "Do NOT run any cargo commands." Frontend verification uses `npx svelte-check` only.

4. **Separate worktrees for truly parallel compilation.** If two agents must both compile Rust simultaneously, each MUST have its own worktree.

5. **Cap Rust parallelism on resource-constrained machines.** Set `CARGO_BUILD_JOBS=2` (or lower) to limit concurrent rustc instances.

### Compilation Risk by Knowledge Combination

| Role + Knowledge | Compilation Risk | Safe to Parallelize With |
|--------------|-----------------|-------------------------|
| Implementer + backend knowledge | High (cargo) | Frontend-only agents |
| Implementer + database knowledge | High (cargo) | Frontend-only agents |
| Implementer + frontend knowledge | Low (svelte-check) | Any agent |
| Designer | Low (svelte-check) | Any agent |
| Reviewer + code-quality-review | High (cargo clippy + cargo test) | Nothing — run alone |
| Reviewer + test-engineering | High (cargo test) | Nothing — run alone |
| Implementer + diagnostic-methodology | Medium (may compile) | Frontend-only agents |
| Researcher, Planner, Writer | None | Any agent |
| Governance Steward | None | Any agent |

## Why This Exists

Without this rule, the orchestrator accumulates implementation details in its context window, reducing its capacity for coordination. Delegation keeps the orchestrator focused on process while agents handle implementation in isolated contexts.

## Related Rules

- [RULE-deab6ea7](RULE-deab6ea7) (knowledge-enforcement) — agents must load knowledge before starting work
- [RULE-b2753bad](RULE-b2753bad) (required-reading) — agents must read governing docs before implementation
- [RULE-878e5422](RULE-878e5422) (honest-reporting) — agents must report status accurately
- [RULE-f809076f](RULE-f809076f) (tool-access-restrictions) — constrains which tools each role may use
- [RULE-92dba0cb](RULE-92dba0cb) (provider-agnostic-capabilities) — capability → tool resolution at delegation time
