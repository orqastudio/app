---
id: RULE-001
title: Agent Delegation
description: The orchestrator coordinates but does not implement. All implementation is delegated to universal roles with appropriate skills.
status: active
created: 2026-03-07
updated: 2026-03-07
layer: core
scope:
  - AGENT-003
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Agent delegation provides clarity through structured roles and boundaries
  - target: RULE-026
    type: informs
    rationale: Listed in Related Rules section
  - target: RULE-023
    type: informs
    rationale: Listed in Related Rules section
  - target: RULE-015
    type: informs
    rationale: Listed in Related Rules section
  - target: RULE-037
    type: informs
    rationale: Listed in Related Rules section
  - target: RULE-040
    type: informs
    rationale: Listed in Related Rules section
  - type: informed-by
    target: RULE-026
    rationale: Inverse of informs relationship from RULE-026
  - type: informed-by
    target: RULE-036
    rationale: Inverse of informs relationship from RULE-036
  - type: informed-by
    target: RULE-037
    rationale: Inverse of informs relationship from RULE-037
  - type: informed-by
    target: RULE-040
    rationale: Inverse of informs relationship from RULE-040
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
- Create and update artifact structure (frontmatter, status, references) in `.orqa/`
- Create and update governance files (rules, agents, skills, hooks)
- Single-line fixes, typo corrections, config file edits
- Coordinate across agents, report status, manage worktrees

## What the Orchestrator MUST Delegate

- Any change to `backend/src-tauri/` (Rust backend code) — delegate to Implementer with backend skills
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
2. **Resolve capabilities** — Read the agent's `capabilities` field, determine the current context (CLI or App), and resolve to concrete tool names using [RULE-040](RULE-040) mapping tables. Include the resolved tool names in the delegation prompt.
3. **Specify skills** — List the skills to load (e.g., "Implementer with rust-async-patterns, tauri-v2, orqa-ipc-patterns")
4. **Scope the task** — Clear description with acceptance criteria
5. **Provide context** — File paths, relevant docs, constraints
6. **Verify the result** — Check the agent's output against acceptance criteria before reporting to the user

## Exceptions

The orchestrator may bypass delegation for:

- Governance artifacts (`.orqa/process/rules/`, `.orqa/process/agents/`, `.orqa/process/skills/`) — these ARE the orchestrator's domain
- Process documentation (`.orqa/documentation/process/`) — orchestration is a process concern
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

- [RULE-026](RULE-026) (skill-enforcement) — agents must load skills before starting work
- [RULE-023](RULE-023) (required-reading) — agents must read governing docs before implementation
- [RULE-015](RULE-015) (honest-reporting) — agents must report status accurately
- [RULE-037](RULE-037) (tool-access-restrictions) — constrains which tools each role may use
- [RULE-040](RULE-040) (provider-agnostic-capabilities) — capability → tool resolution at delegation time
