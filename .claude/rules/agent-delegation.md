---
scope: system
---

# Agent Delegation (NON-NEGOTIABLE)

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

- Governance artifacts (`.claude/rules/`, `.claude/agents/`, `.claude/skills/`) — these ARE the orchestrator's domain
- Process documentation (`docs/process/`) — orchestration is a process concern
- Session state (`tmp/session-state.md`) — coordination artifact
- Plan files (`.orqa/plans/`) — planning is an orchestrator responsibility

## Why This Exists

Without this rule, the orchestrator accumulates implementation details in its context window, reducing its capacity for coordination. Delegation keeps the orchestrator focused on process while agents handle implementation in isolated contexts.

## Related Rules

- `skill-enforcement.md` — agents must load skills before starting work
- `required-reading.md` — agents must read governing docs before implementation
- `honest-reporting.md` — agents must report status accurately
