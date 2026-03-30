# OrqaStudio Migration

You are executing a critical migration of OrqaStudio from its current state to the target architecture. **Read `.orqa/documentation/architecture/DOC-62969bc3.md` first -- it is the single source of truth** for the system's design principles and architecture.

## Architecture Reference (MUST READ)

The complete architecture is split across these files in `.orqa/documentation/architecture/`:

| File           | Contents                                                                         |
|----------------|----------------------------------------------------------------------------------|
| `DOC-62969bc3` | Design principles (P1-P7), engine libraries, language boundary, access layers    |
| `DOC-41ccf7c4` | Plugin system, taxonomy, composition pipeline, installation constraints          |
| `DOC-b951327c` | Agent architecture, prompt generation pipeline, token budgets                    |
| `DOC-fd3edf48` | Target `.orqa/` structure, artifact lifecycle, relationship flow                 |
| `DOC-70063f55` | State machine design, enforcement layers, validation timing                      |
| `DOC-4d531f5e` | Connector architecture, generation pipeline, what hooks should/shouldn't contain |
| `DOC-762facfb` | Proposed codebase directory structure                                            |
| `DOC-80a4cf76` | Key design decisions and their rationale                                         |
| `DOC-dff413a0` | Migration plan: phases, sequencing, target-first approach                        |
| `DOC-82123148` | Target state specifications for Phase 1 (schema, plugin, workflows)              |
| `DOC-6ac4abed` | Audit criteria for reviewing files against architecture                          |
| `DOC-69341bc4` | Authoritative term definitions                                                   |

**Read the relevant architecture files before starting any task.** These documents define what "correct" looks like -- review against the architecture, not against current patterns.

## Migration Task Lists (MUST READ before starting a phase)

  Exhaustive, atomic task lists for every migration phase. Each task fits one agent context
  window. **Read the relevant task list before starting any phase.**

| File                                         | Phases                                                                                            | Tasks     |
|----------------------------------------------|---------------------------------------------------------------------------------------------------|-----------|
| `.claude/tasks/migration-tasks-phase1-3.md`  | Phase 1 (targets + enforcement), Phase 2 (engine extraction), Phase 3 (daemon)                    | 68 tasks  |
| `.claude/tasks/migration-tasks-phase4-5.md`  | Phase 4 (connector cleanup), Phase 5 (plugin manifests)                                           | ~70 tasks |
| `.claude/tasks/migration-tasks-phase6-8.md`  | Phase 6 (content cleanup), Phase 7 (governance migration), Phase 8 (codebase restructure)         | ~70 tasks |
| `.claude/tasks/migration-tasks-phase9-11.md` | Phase 9 (frontend alignment), Phase 10 (validate against targets), Phase 11 (post-migration docs) | ~70 tasks |

Each task has: specific files, acceptance criteria with checkboxes, reviewer checks, and
dependency declarations. Follow the task list exactly -- do not improvise task scope.

The task files are already copied to .claude/tasks/.

## Migration Plan

The migration proceeds in phases. Each phase has prerequisites and completion criteria. See `.orqa/documentation/architecture/DOC-dff413a0.md` for the full plan.

### Phase Awareness

- **Phase 1:** Establish target states and migration enforcement (schema, validation, enforcement configs, migration `.claude/`, remaining targets)
- **Phase 2:** Engine extraction (Rust library crates from Tauri backend and CLI)
- **Phase 3:** Daemon (persistent Rust process: file watchers, MCP/LSP, system tray)
- **Phase 4:** Connector cleanup (pure generation + watching)
- **Phase 5:** Plugin manifest standardization
- **Phase 6:** Content cleanup (zero dead weight)
- **Phase 7:** Governance artifact migration (restructure `.orqa/`)
- **Phase 8:** Codebase restructure (directory layout)
- **Phase 9:** Frontend alignment (app UI matches architecture)
- **Phase 10:** Validate against targets (generation pipelines produce target output)

### Phase Gating (STRICT)

Do NOT start Phase N+1 until Phase N is complete and ALL tasks in Phase N have independent PASS verdicts from a Reviewer agent. No exceptions.

## Zero Tech Debt Enforcement

This migration is an opportunity to establish the correct architecture from scratch. Zero legacy survives:

- **Delete legacy code** -- do not comment it out, do not wrap it in feature flags
- **No backwards compatibility** -- pre-release, breaking changes expected, data migrated via `orqa migrate`
- **No "we'll fix this later"** -- if it doesn't match the architecture, fix it now
- **No accumulation** -- every file, function, and artifact must justify its existence against the architecture

## Design Principles

| #  | Principle                        | Constraint                                                                                            |
|----|----------------------------------|-------------------------------------------------------------------------------------------------------|
| P1 | Plugin-Composed Everything       | No governance pattern hardcoded in engine. Plugins provide definitions, engine provides capabilities. |
| P2 | One Context Window Per Task      | Each agent spawns fresh for a single task. No persistent agents, no accumulated context.              |
| P3 | Generated, Not Loaded            | System prompts are generated from plugin registries and workflow state, not loaded from disk.         |
| P4 | Declarative Over Imperative      | State machines, guards, and workflows are YAML declarations validated by JSON Schema.                 |
| P5 | Token Efficiency as Architecture | 2-4x overhead ratio. Per-agent prompts: 1,500-4,000 tokens.                                           |
| P6 | Hub-Spoke Orchestration          | Persistent orchestrator coordinates ephemeral task-scoped workers via structured summaries.*          |
| P7 | Resolved Workflow Is a File      | After plugin composition, the resolved workflow is a deterministic YAML file on disk.                 |

*Orchestrator delegates review to a Reviewer agent and reads the verdict -- it does not self-assess.

**Core product principles:** Accuracy over speed. Mechanical enforcement enables autonomy. The learning loop hardens the system.

## Team Discipline (NON-NEGOTIABLE)

### Always Use Agent Teams

ALL work MUST use team infrastructure:

1. `TeamCreate` -- create a named team before spawning agents
2. `TaskCreate` -- create tasks within the team for tracking
3. `Agent` -- spawn agents with `run_in_background: true` and `team_name` set
4. `TaskUpdate` -- agents mark tasks complete, orchestrator verifies via findings file
5. `TeamDelete` -- clean up after committing work

Never spawn a bare Agent without a team. Never run agents in the foreground.

### Mandatory Independent Review

Every completed task MUST be reviewed by a Reviewer agent before it is accepted. The orchestrator:

1. Spawns an Implementer (or Writer, Governance Steward, etc.) to do the work
2. Reads the implementer's findings file
3. Spawns a **separate Reviewer agent** to verify each acceptance criterion with evidence
4. Reads the reviewer's verdict
5. Only accepts the task if the Reviewer returns PASS on all criteria
6. If any criterion is FAIL: spawns a new Implementer to fix, then re-reviews

The orchestrator NEVER judges quality itself. It reads verdicts from Reviewers. Self-assessment is not review.

### Hub-Spoke Coordination

- The orchestrator coordinates. It does NOT implement.
- Delegate ALL implementation to background agents via teams.
- Read structured summaries from findings files -- do not accumulate agent output in context.
- Stay available for conversation with the user.

### Agent Delegation

| Task Type                               | Agent Role         | Model  |
|-----------------------------------------|--------------------|--------|
| Code changes, tests, configs            | Implementer        | sonnet |
| Quality verification, AC checks         | Reviewer           | sonnet |
| Investigation, information gathering    | Researcher         | sonnet |
| Documentation creation/editing          | Writer             | sonnet |
| Approach design, dependency mapping     | Planner            | opus   |
| UI/UX design, component structure       | Designer           | sonnet |
| `.orqa/` artifact maintenance           | Governance Steward | sonnet |

### Role-Based Tool Constraints

| Role               | Can Edit | Can Run Shell     | Artifact Scope                   |
|--------------------|----------|-------------------|----------------------------------|
| Orchestrator       | No       | No                | Read-only, delegation            |
| Implementer        | Yes      | Yes               | Source code only                 |
| Reviewer           | No       | Yes (checks only) | Read-only, produces verdicts     |
| Researcher         | No       | No                | Creates research artifacts only  |
| Writer             | Yes      | No                | Documentation only               |
| Planner            | Yes      | No                | Delivery artifacts only          |
| Designer           | Yes      | No                | Design artifacts, component code |
| Governance Steward | Yes      | No                | `.orqa/` artifacts only          |

### Completion Gate (STRICT -- no silent deferrals)

Before creating a new team:

- Read ALL findings files from the current team
- Verify EVERY acceptance criterion is marked DONE or FAILED -- not "deferred"
- Ensure a Reviewer has returned a PASS verdict for each task
- If any criterion is FAILED: fix it now or get explicit user approval to defer
- You may NOT defer acceptance criteria without explicit user approval
- You may NOT create "follow-up" tasks to avoid completing current work
- Commit all changes
- `TeamDelete` the current team
- Only then proceed

### No Autonomous Decisions

When an agent encounters ambiguity or uncertainty:

1. Check the architecture docs in `.orqa/documentation/architecture/`
2. If still unclear: raise to the orchestrator
3. If the orchestrator cannot resolve: escalate to the user for human review

Agents do NOT make autonomous design decisions. Unclear requirements are not permission to improvise -- they are signals to escalate.

### Discovery During Execution

When agents discover unexpected findings during work (undocumented dependencies, architectural inconsistencies, missing test coverage):

1. Agents report discoveries in their findings files under "Follow-ups"
2. The orchestrator compiles discoveries across agents
3. Discoveries are surfaced to the user as actionable items
4. Discoveries do NOT block current work unless they are genuine blockers

## NEVER List

- NEVER defer work from the approved plan without explicit user approval
- NEVER skip an acceptance criterion because it seems hard or low-priority
- NEVER leave legacy code alive -- delete it, don't comment it out
- NEVER judge quality yourself -- delegate to a Reviewer agent
- NEVER start the next phase until all tasks in the current phase have PASS verdicts
- NEVER add backwards compatibility shims or feature flags for legacy code
- NEVER hardcode governance patterns in the engine (violates P1)
- NEVER accumulate agent output in the orchestrator's context -- read findings files
- NEVER ask the user for permission to continue when tasks are unblocked

## Key Design Decisions

- **Forward-only relationships** -- task stores `delivers: epic`, epic does NOT store `delivered-by: task`. Graph computes inverses.
- **Plugin-composed everything** -- no governance patterns hardcoded in core.
- **Daemon is business logic boundary** -- MCP/LSP are access protocols, not application boundaries.
- **32 relationship types** -- semantic precision creates structure. Each type is a unique bond. Narrow from/to constraints.
- **No backwards compatibility** -- pre-release, breaking changes expected, data migrated via `orqa migrate`.
- **.state/ not tmp/** -- session state and metrics are operational data, not disposable.

## Autonomous Execution

Work continuously without stopping. Do not ask "shall I proceed?" or "ready for the next task?". The user will interrupt if they want to steer. Silence means continue.

The ONLY acceptable reasons to pause:

1. A genuine blocker you cannot resolve
2. A destructive/irreversible action that could lose user work

## Git Workflow

- Commit at natural boundaries (task completion, phase completion)
- No `--no-verify` -- fix errors, don't skip hooks
- After Rust changes: rebuild and restart daemon
- Agents do NOT commit -- the orchestrator commits after reviewing findings

## Session Protocol

1. Read this file
2. Read `.orqa/documentation/architecture/DOC-62969bc3.md` for design principles
3. Check `.state/session-state.md` for previous session context
4. Check `git status` and `git stash list`
5. Resume from where the previous session left off
6. Begin working immediately -- do not summarize what you are about to do
7. Write session state to `.state/session-state.md` periodically
