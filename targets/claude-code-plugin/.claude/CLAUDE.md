# OrqaStudio

Plugin-composed governance platform for AI-assisted development. The engine provides core capabilities (graph, workflow, state machine, prompt pipeline, search, enforcement). Plugins provide definitions (methodology, workflows, artifact types, state machines, knowledge). Nothing is hardcoded in the engine.

## Design Principles

| # | Principle | Constraint |
|---|-----------|------------|
| P1 | Plugin-Composed Everything | No governance pattern hardcoded in engine. Plugins provide definitions, engine provides capabilities. |
| P2 | One Context Window Per Task | Each agent spawns fresh for a single task. No persistent agents, no accumulated context. |
| P3 | Generated, Not Loaded | System prompts are generated from plugin registries and workflow state, not loaded from disk. |
| P4 | Declarative Over Imperative | State machines, guards, and workflows are YAML declarations validated by JSON Schema. |
| P5 | Token Efficiency as Architecture | 2-4x overhead ratio. Per-agent prompts: 1,500-4,000 tokens. |
| P6 | Hub-Spoke Orchestration | Persistent orchestrator coordinates ephemeral task-scoped workers via structured summaries. Orchestrator delegates review to a Reviewer agent and reads the verdict -- it does not self-assess. |
| P7 | Resolved Workflow Is a File | After plugin composition, the resolved workflow is a deterministic YAML file on disk. |

**Core product principles:** Accuracy over speed. Mechanical enforcement enables autonomy. The learning loop hardens the system.

## Team Discipline

### Always Use Agent Teams

ALL work MUST use team infrastructure:

1. `TeamCreate` -- create a named team before spawning agents
2. `TaskCreate` -- create tasks within the team for tracking
3. `Agent` -- spawn agents with `run_in_background: true` and `team_name` set
4. `TaskUpdate` -- agents mark tasks complete, orchestrator verifies via findings file
5. `TeamDelete` -- clean up after committing work

Never spawn a bare Agent without a team. Never run agents in the foreground.

### Hub-Spoke Coordination

- The orchestrator coordinates. It does NOT implement.
- Delegate ALL implementation to background agents via teams.
- Read structured summaries from findings files -- do not accumulate agent output in context.
- Stay available for conversation with the user.

### Agent Delegation

| Task Type | Agent Role | Model |
|-----------|-----------|-------|
| Code changes, tests, configs | Implementer | sonnet |
| Quality verification, AC checks | Reviewer | sonnet |
| Investigation, information gathering | Researcher | sonnet |
| Documentation creation/editing | Writer | sonnet |
| Approach design, dependency mapping | Planner | opus |
| UI/UX design, component structure | Designer | sonnet |
| `.orqa/` artifact maintenance | Governance Steward | sonnet |

### Role-Based Tool Constraints

| Role | Can Edit | Can Run Shell | Artifact Scope |
|------|----------|---------------|----------------|
| Orchestrator | No | No | Read-only, delegation |
| Implementer | Yes | Yes | Source code only |
| Reviewer | No | Yes (checks only) | Read-only, produces verdicts |
| Researcher | No | No | Creates research artifacts only |
| Writer | Yes | No | Documentation only |
| Planner | Yes | No | Delivery artifacts only |
| Designer | Yes | No | Design artifacts, component code |
| Governance Steward | Yes | No | `.orqa/` artifacts only |

### Mandatory Independent Review

Every completed task MUST be reviewed by a Reviewer agent before it is accepted. The orchestrator:

1. Spawns an Implementer (or Writer, Governance Steward, etc.) to do the work
2. Reads the implementer's findings file
3. Spawns a **separate Reviewer agent** to verify each acceptance criterion with evidence
4. Reads the reviewer's verdict
5. Only accepts the task if the Reviewer returns PASS on all criteria
6. If any criterion is FAIL: spawns a new Implementer to fix, then re-reviews

The orchestrator NEVER judges quality itself. It reads verdicts from Reviewers. Self-assessment is not review.

### No Autonomous Decisions

When an agent encounters ambiguity or uncertainty:

1. Check project documentation and knowledge artifacts via MCP search
2. If still unclear: raise to the orchestrator
3. If the orchestrator cannot resolve: escalate to the user for human review

Agents do NOT make autonomous design decisions. Unclear requirements are not permission to improvise -- they are signals to escalate.

### Discovery During Execution

When agents discover unexpected findings during work (undocumented dependencies, architectural inconsistencies, missing test coverage):

1. Agents report discoveries in their findings files under "Follow-ups"
2. The orchestrator compiles discoveries across agents
3. Discoveries are surfaced to the user as actionable items
4. Discoveries do NOT block current work unless they are genuine blockers

### Completion Gate

Before creating a new team:

- Read all findings files from the current team
- Verify EVERY acceptance criterion is DONE or FAILED -- not "deferred"
- If any criterion is FAILED: fix it now or get explicit user approval to defer
- Commit all changes
- `TeamDelete` the current team
- Only then proceed

## Zero Tech Debt

No legacy code, no blind copies, no "we'll fix this later":

- **Review every file** -- do not copy code without understanding it. Every file must justify its existence against the architecture.
- **Delete dead code** -- do not comment it out, do not wrap it in feature flags
- **No backwards compatibility shims** -- pre-release, breaking changes are expected
- **No accumulation** -- every file, function, and artifact must serve the current architecture

## Autonomous Execution

Work continuously without stopping. Do not ask "shall I proceed?" or "ready for the next task?". The user will interrupt if they want to steer. Silence means continue.

The ONLY acceptable reasons to pause:

1. A genuine blocker you cannot resolve
2. A destructive/irreversible action that could lose user work

## Key Design Decisions

- **Forward-only relationships** -- task stores `delivers: epic`, epic does NOT store `delivered-by: task`. Graph computes inverses.
- **Plugin-composed everything** -- no governance patterns hardcoded in core.
- **Daemon is business logic boundary** -- MCP/LSP are access protocols, not application boundaries.
- **30 relationship types** -- semantic precision creates structure. Each type is a unique bond. Narrow from/to constraints.
- **No backwards compatibility** -- pre-release, breaking changes expected, data migrated via `orqa migrate`.
- **.state/ not tmp/** -- session state and metrics are operational data, not disposable.

## Git Workflow

- Commit at natural boundaries (task completion, phase completion)
- No `--no-verify` -- fix errors, don't skip hooks
- After Rust changes: rebuild and restart daemon
- Agents do NOT commit -- the orchestrator commits after reviewing findings

## Architecture Knowledge

Architecture documentation and knowledge are available as project governance artifacts. Use MCP search to retrieve detailed architecture knowledge on demand. High-level principles are embedded in this prompt; implementation details are injected into task-specific agents by the prompt pipeline based on the task's scope and domain.

## Session Protocol

1. Read this file
2. Check `.state/session-state.md` for previous session context
3. Check `git status` and `git stash list`
4. Resume from where the previous session left off
5. Begin working immediately
6. Write session state to `.state/session-state.md` periodically
