---
id: RULE-99abcea1
type: rule
title: Use agent teams for implementation
description: The orchestrator must delegate implementation, review, and research tasks to subagents via the Agent tool rather than performing them directly.
status: active
enforcement:
  - mechanism: behavioral
    message: "Delegate implementation to subagents via the Agent tool. Do not write code directly."
relationships: []
---

The orchestrator MUST use Claude Code's Agent tool to spawn subagents for all implementation, review, research, and documentation writing work. The Agent tool creates isolated subagent contexts with their own tool access and context windows, keeping the orchestrator's context clean for coordination.

## When to Use the Agent Tool

The orchestrator MUST delegate via the Agent tool for:

| Work Type | Role to Assign | What the Subagent Does |
|-----------|---------------|----------------------|
| **Implementation** | Implementer | Write, edit, and test code. Run build and lint commands. |
| **Code review** | Reviewer | Read code, run quality checks (`make check`, `make test`), produce PASS/FAIL verdicts. |
| **Research** | Researcher | Investigate codebases, read documentation, analyse patterns, report findings. |
| **Documentation writing** | Writer | Create and edit documentation pages, update cross-references. |
| **UI/UX design** | Designer | Create and edit Svelte components, design layouts and interactions. |
| **Planning** | Planner | Analyse dependencies, map system boundaries, produce implementation plans. |

Every delegation MUST name the role explicitly in the Agent tool prompt so the subagent understands its boundaries.

## Agent Teams (NON-NEGOTIABLE)

The orchestrator MUST use agent teams (via `TeamCreate`) for ALL delegated work, even when only a single task is at hand. Running agents in the background via teams keeps the orchestrator available for conversation with the user.

### Why Teams Even for Single Tasks

- The orchestrator's primary job is conversation with the user — not waiting for agents to finish
- Background agents allow the user to steer, ask questions, or add context while work is in progress
- Even a single-task team provides the structured findings-to-disk pattern ([RULE-04684a16](RULE-04684a16))

### Protocol

1. **Always use `run_in_background: true`** when spawning agents — foreground agents block the orchestrator
2. **Name agents** meaningfully so status updates are clear
3. **Read findings files** from `.state/team/<team-name>/` when agents complete
4. **Report results** to the user in conversation

## When NOT to Use the Agent Tool

The orchestrator may act directly for lightweight coordination tasks that do not constitute implementation:

- **Quick file reads** for planning and coordination decisions (reading an epic, checking a task status, scanning a rule).
- **Session state** (`.state/session-state.md`) — the orchestrator MUST write this directly. It has the full conversation context; delegating to an agent that must be re-told everything is wasteful and lossy.
- **Artifact status transitions** (updating frontmatter fields like `status`, `updated`, `depends-on`).
- **Coordination messages** to the user (summaries, status reports, scope questions).

All other work — including governance artifact edits, source code changes, tests, code review, research, and documentation — MUST be delegated to agents via teams.

## How to Delegate Properly

Every Agent tool invocation MUST include:

1. **Role assignment.** State which universal role the subagent is performing (e.g., "You are an Implementer").
2. **Task description.** A clear statement of what the subagent must accomplish.
3. **File paths.** Specific files or directories the subagent should read or modify.
4. **Acceptance criteria.** Concrete, verifiable conditions that define when the task is done.
5. **Relevant context.** Epic references, architecture decisions, knowledge artifacts, or documentation pages the subagent needs.
6. **Capability constraints.** Which tools the subagent may use, resolved from [RULE-8abcbfd5](RULE-8abcbfd5).

### Example Delegation

```
You are an Implementer.

Task: Add a `last_modified` field to the session metadata struct and update
the SQLite migration.

Files:
- backend/src-tauri/src/domain/sessions.rs
- backend/src-tauri/src/repo/session_repo.rs
- backend/src-tauri/migrations/

Acceptance criteria:
- [ ] `SessionMetadata` struct has a `last_modified: DateTime<Utc>` field
- [ ] Migration adds the column to the sessions table
- [ ] All existing tests pass (`make test-rust`)
- [ ] No clippy warnings (`make lint-backend`)

Context: See EPIC-XXXXXXXX for the full design. Follow error handling
patterns in RULE-05ae2ce7.
```

## Subagent Independence

Subagents operate in their own context windows. The orchestrator MUST NOT:

- Micromanage subagent tool calls or intermediate steps.
- Re-read files the subagent already read just to "verify" (delegate verification to a Reviewer subagent instead).
- Accumulate subagent output in the orchestrator context (summarise key findings only).

The orchestrator's job is to define the task clearly, launch the subagent, and evaluate the result against acceptance criteria.

## Parallel and Sequential Execution

- **Lightweight subagents** (Researcher, Planner, Writer) may run in parallel when their tasks are independent.
- **Compilation-heavy subagents** (Implementer with Rust, Reviewer running `make check`) MUST run sequentially in the same worktree per [RULE-87ba1b81](RULE-87ba1b81) resource safety rules.
- **Frontend-only subagents** (Implementer with Svelte/TypeScript) are lightweight and safe to parallelise with other lightweight agents.

## FORBIDDEN

- The orchestrator writing or editing implementation source code (any file outside `.orqa/` and `.state/`) directly instead of delegating to a subagent.
- The orchestrator running test suites (`make test`, `make check`, `cargo test`) directly instead of delegating to a Reviewer subagent.
- The orchestrator performing code review directly instead of delegating to a Reviewer subagent.
- The orchestrator writing documentation content directly instead of delegating to a Writer subagent.
- Delegating without specifying the role, acceptance criteria, or relevant file paths.
- Spawning multiple compilation-heavy subagents in the same worktree simultaneously.
- Running agents in the foreground (blocking the orchestrator) when background execution is possible.
- Delegating session state writing to an agent — the orchestrator has the full context and must write it directly.

## Related Rules

- [RULE-87ba1b81](RULE-87ba1b81) (agent-delegation) — defines the universal roles, delegation protocol, and resource safety constraints that this rule operationalises through the Agent tool.
- [RULE-b723ea53](RULE-b723ea53) (tool-access-restrictions) — defines which capabilities each role may use; the orchestrator must respect these when briefing subagents.
- [RULE-8abcbfd5](RULE-8abcbfd5) (provider-agnostic-capabilities) — capability-to-tool resolution used when specifying subagent tool access.
- [RULE-ef822519](RULE-ef822519) (context-management) — delegating to subagents keeps the orchestrator's context window lean.
- [RULE-dd5b69e6](RULE-dd5b69e6) (skill-enforcement) — subagents must load their required skills before starting work.
