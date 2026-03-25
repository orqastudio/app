# OrqaStudio

Plugin-composed governance platform. Core provides engines (graph, workflow, state machine, prompt pipeline); plugins provide definitions (artifact types, state machines, knowledge, workflow stages). Nothing is hardcoded in core.

## Autonomous Execution (NON-NEGOTIABLE)

Work continuously without stopping. Do not ask "shall I proceed?" or "ready for the next task?". The user will interrupt if they want to steer. Silence means continue.

The ONLY acceptable reasons to pause:
1. A genuine blocker you cannot resolve from this file, the research, or task artifacts
2. A destructive/irreversible action that could lose user work

## Architecture

**Three-layer agent taxonomy:** `Universal Role + Stage Context + Domain Knowledge = Effective Agent Type`

- **Universal roles** (core): Implementer, Reviewer, Researcher, Writer, Governance Steward, Planner, Designer, Installer, Plugin Developer
- **Stage context** (workflow plugins): stage-specific instructions injected at delegation time
- **Domain knowledge** (knowledge plugins): composed into agents based on task scope, file paths, and subject matter

**Prompt generation pipeline:** Plugin Registry --> Schema Assembly --> Section Resolution --> Token Budgeting --> Prompt Output. Agents receive 1,500-4,000 token prompts, not monolithic context dumps.

**Knowledge injection tiers:** always (compressed summaries at spawn), stage-triggered (when workflow stage matches), on-demand (via semantic search).

**Conflict resolution priority:** project rules > project knowledge > plugin knowledge > core knowledge.

## Reference Documents

- **Research**: `.orqa/discovery/research/RES-d6e8ab11.md` -- Team Design v2
- **Decisions**: `.orqa/process/decisions/AD-1ef9f57c.md` -- Resolved open questions
- **Decisions**: `.orqa/process/decisions/AD-8727f99a.md` -- tmp to .state rename

## Team Discipline (NON-NEGOTIABLE)

### Always Use Agent Teams

ALL delegated work MUST use the team infrastructure:

1. **`TeamCreate`** -- create a named team before spawning any agents
2. **`TaskCreate`** -- create tasks within the team for tracking
3. **`Agent`** -- spawn agents with `run_in_background: true` and `team_name` set
4. **`TaskUpdate`** -- agents mark tasks complete, orchestrator verifies via findings file
5. **`TeamDelete`** -- clean up after committing work, before creating the next team

Never spawn a bare Agent without a team. Never run agents in the foreground. Even single-task work uses a team.

### Hub-Spoke Orchestration

- The orchestrator coordinates. It does NOT implement.
- Delegate ALL implementation to background agents via teams
- Read structured summaries from findings files -- do not accumulate agent output in your context
- Stay available for conversation with the user
- NEVER end a response with "shall I continue?" or "let me know if you'd like me to proceed"

### Ephemeral Task-Scoped Agents

- One fresh agent per task, one context window per task
- Agent receives: role assignment, task description, file paths, acceptance criteria, relevant knowledge
- Agent writes findings to `.state/team/<team-name>/task-<id>.md`
- Orchestrator reads findings and decides next step

### Role-Based Tool Constraints

| Role | Can Edit | Can Run Shell | Artifact Scope |
|------|----------|---------------|----------------|
| **Implementer** | Yes | Yes | Source code only |
| **Reviewer** | No | Yes (checks only) | Read-only, produces verdicts |
| **Researcher** | No | No | Creates research artifacts only |
| **Writer** | Yes | No | Documentation only |
| **Governance Steward** | Yes | No | `.orqa/` artifacts only |
| **Planner** | Yes | No | Delivery artifacts only |

### Completion Gate (STRICT — no silent deferrals)

Before creating a new team:
- Read all findings files from the current team
- Verify EVERY acceptance criterion is marked DONE or FAILED — not "deferred"
- If any criterion is FAILED: either fix it now or get explicit user approval to defer
- You may NOT mark an epic or task as done if any acceptance criterion is incomplete
- You may NOT create "follow-up" tasks to avoid completing current work
- Commit all changes
- `TeamDelete` the current team
- Only then proceed

Epics and tasks are the user-approved work list. Deferring work without user approval is a violation of the plan.

### Resource Safety

- Never run two Rust compilation agents in parallel in the same worktree
- Frontend agents (svelte-check) are lightweight -- safe to parallelise
- After Rust code changes: rebuild binaries and restart daemon before continuing

## Key Design Decisions

- **Forward-only relationship storage** -- task stores `delivers: epic`, epic does NOT store `delivered-by: task`. Graph computes inverses.
- **Daemon is business logic boundary** -- MCP/LSP are access protocols, not application boundaries. Prompt generation belongs in the daemon.
- **Plugin-composed everything** -- no governance patterns hardcoded in core. Plugins provide definitions, core provides engines.
- **.state/ not tmp/** -- session state and metrics are operational data, not disposable.
- **No backwards compatibility** -- pre-release, breaking changes expected, data migrated via `orqa migrate`.
- **30 relationship types** -- semantic precision creates structure. Each type is a unique bond.
- **Narrow from/to constraints** -- specificity is the point. Fix bugs, don't widen.

## Git Workflow

- Commit at natural boundaries (task completion, phase completion)
- No `--no-verify` -- fix errors, don't skip hooks
- After Rust changes: rebuild and restart daemon
- Agents do NOT commit -- the orchestrator commits after reviewing findings

## Session Protocol

1. Read this file
2. Check `.state/session-state.md` for previous session context
3. Check `git status` and `git stash list`
4. Resume from where the previous session left off
5. Begin working immediately -- do not summarise what you are about to do
6. When context compaction occurs, re-read this file to re-orient, then continue without asking
7. Write session state to `.state/session-state.md` periodically -- at minimum after each task completion

## Drift Prevention

Do NOT:
- Store inverse relationships on upstream artifacts
- Implement directly -- always delegate to background agents via teams
- Skip the completion gate between teams
- Add backwards compatibility shims
- Use `tmp/` for new state files (use `.state/`)
- Spawn agents without a team
- Run agents in the foreground
- Ask the user for permission to continue when tasks are unblocked
- Defer acceptance criteria without explicit user approval
- Mark tasks/epics as done when acceptance criteria are incomplete
- Create follow-up tasks as a substitute for completing current work
