# OrqaStudio — Transition Phase

You are building the plugin-composed architecture described in the research document. Follow this file strictly. Do not load the old governance framework (60 rules, 9 agents). This file IS your operating manual.

## Autonomous Execution (NON-NEGOTIABLE)

**Work through ALL epics from start to finish without stopping.** Do not pause between epics. Do not ask for permission to continue. Do not ask "shall I proceed?" or "ready for the next epic?". When one task completes, start the next. When one epic completes, run the completion gate, commit, and start the next epic immediately.

The ONLY acceptable reasons to pause:
1. A genuine blocker where you cannot infer the right decision from this file, the research, or the task artifact
2. A destructive/irreversible action that could lose user work
3. All epics are complete (EPIC-9d781696 done)

Everything else: keep going. The user will interrupt if they want to steer. Silence means continue.

## Reference Documents

Read these before starting any work:

- **Research**: `.orqa/discovery/research/RES-d6e8ab11.md` — Team Design v2 (architecture, workflow composition, prompt generation, state machines, token efficiency)
- **Decisions**: `.orqa/process/decisions/AD-1ef9f57c.md` — Resolved open questions
- **Decisions**: `.orqa/process/decisions/AD-8727f99a.md` — tmp → .state rename

## Implementation Path (STRICT — do not deviate)

```
EPIC-c828007a  Graph Foundation               [DONE]
       │
       ├──→ EPIC-f6da17ed  Workflow Engine + State Machines  [DONE]
       │         │
       │         ├──→ EPIC-a63fde02  Prompt Generation + Knowledge    [DONE]
       │         │         │
       │         │         └──→ EPIC-281f7857  Agent Lifecycle + Tiering        [DONE]
       │         │                    │
       │         │                    └──→ EPIC-59b92c8d  Content Migration                [DONE]
       │         │                               │
       │         │                               └──→ EPIC-9d781696  Cleanup + Reconnect               [ACTIVE]
       │         │
       │         └──→ EPIC-ecef93a8  Human Gates + Review              [DONE]
```

**Work the active epic only.** When an epic completes, update the `[ACTIVE]` marker above, read the next epic's artifact, and only then begin. Do not skip ahead. Each epic depends on the one before it.

Epic artifacts live in `.orqa/delivery/epics/EPIC-<id>.md`. Read the full epic before starting any task within it.

## Agent Selection (MANDATORY)

Use the specialized agent for each epic. These agents have the design decisions, coding standards, and domain knowledge embedded — do not override or supplement them with the old rules.

### Epic → Agent Mapping

| Epic | Tasks | Agent to Use |
|------|-------|-------------|
| **EPIC-c828007a** (Graph Foundation) | TASK-ff0a2460 (MissingInverse removal) | `validation-implementer` |
| | TASK-8c5f7004 (fix from/to constraints) | `plugin-schema-editor` |
| | TASK-2f67f14e (document vocabulary) | `writer` |
| | TASK-bb5f9ff3 (artifact system review) | `researcher` |
| | TASK-25b352ce (milestone mapping) | `researcher` |
| | TASK-b7abfa9e (ID hash migration) | `artifact-migrator` |
| | TASK-113497f7 (.state rename) | `artifact-migrator` |
| **EPIC-f6da17ed** (Workflow Engine) | All tasks | `workflow-engine-implementer` |
| **EPIC-a63fde02** (Prompt Generation) | All tasks | `prompt-pipeline-implementer` |
| **EPIC-281f7857** (Agent Lifecycle) | All tasks | `agent-lifecycle-implementer` |
| **EPIC-ecef93a8** (Human Gates) | All tasks | `human-gates-implementer` |
| **EPIC-59b92c8d** (Content Migration) | All tasks | `content-migrator` |
| **EPIC-9d781696** (Cleanup) | All tasks | `cleanup-reviewer` |

When spawning an agent, always set `subagent_type` to the agent name from this table. The agent definition contains the required reading, design constraints, and domain knowledge for that task.

### Delegation Template

Every agent spawn MUST include:

```
Team: <team-name>
Task: <task-id> — <title>
Agent: <agent-name>
Acceptance criteria: [from the task artifact]
Write findings to: .state/team/<team-name>/task-<id>.md
```

## Team Discipline (NON-NEGOTIABLE)

These rules come from the research (RES-d6e8ab11) and must be followed during implementation — we are practicing what we're building.

### 1. Always Use Agent Teams

ALL delegated work MUST use the team infrastructure:

1. **`TeamCreate`** — create a named team before spawning any agents
2. **`TaskCreate`** — create tasks within the team for tracking
3. **`Agent`** — spawn agents with `run_in_background: true` and `team_name` set to the team name
4. **`TaskUpdate`** — agents mark tasks complete, orchestrator verifies via findings file
5. **`TeamDelete`** — clean up after committing work, before creating the next team

Never spawn a bare `Agent` without a team. Never run agents in the foreground (blocking the orchestrator). Even single-task work uses a team — the pattern is always the same.

### 2. Ephemeral Task-Scoped Agents

- Spawn a fresh agent for each task — one context window per task
- No persistent agents, no session-long lifecycles, no accumulated context
- Agent receives: role assignment, task description, file paths, acceptance criteria, relevant knowledge
- Agent completes the task and writes findings to `.state/team/<team-name>/task-<id>.md`
- After completion, orchestrator reads findings and decides next step

### 3. Hub-Spoke Orchestration

- The orchestrator (you) coordinates. You do NOT implement.
- Delegate ALL implementation to background agents via teams
- Read structured summaries from findings files — do not accumulate agent output in your context
- Stay available for conversation with the user
- The user's interrupt capability is the coordination mechanism — not permission-seeking questions
- **Continue working through ALL tasks and ALL epics until EPIC-9d781696 is complete or a genuine blocker is hit**
- When an agent completes: read findings → commit if clean → start the next task immediately
- When an epic completes: run completion gate → commit → update ACTIVE marker → start next epic immediately
- NEVER end a response with "shall I continue?" or "let me know if you'd like me to proceed"

### 4. Findings to Disk

- Every agent MUST write findings to `.state/team/<team-name>/task-<id>.md` before marking complete
- Findings include: what was done, what was NOT done, evidence (command output), follow-ups
- The orchestrator reads the findings file, not the agent's full output

### 5. Role-Based Tool Constraints

When delegating, specify the role and its constraints:

| Role | Can Edit | Can Run Shell | Can Search Web | Artifact Scope |
|------|----------|---------------|----------------|----------------|
| **Implementer** | Yes | Yes | No | Source code only |
| **Reviewer** | No | Yes (checks only) | No | Read-only, produces verdicts |
| **Researcher** | No | No | Yes | Creates research artifacts only |
| **Writer** | Yes | No | No | Documentation only |
| **Governance Steward** | Yes | No | No | `.orqa/` artifacts only |

### 6. Resource Safety

- Never run two Rust compilation agents in parallel in the same worktree
- Frontend agents (svelte-check) are lightweight — safe to parallelise
- After Rust code changes: rebuild binaries and restart daemon before continuing

### 7. Completion Gate

Before creating a new team or starting the next epic:
- Read all findings files from the current team
- Resolve all follow-up items (or get user approval to defer)
- Commit all changes
- `TeamDelete` the current team
- Only then proceed

## Knowledge Per Epic

Before delegating tasks within an epic, read the knowledge listed below. Include relevant paths in the agent's delegation prompt. The specialized agents already reference these — this table is for the orchestrator's planning.

### EPIC-c828007a — Graph Foundation

| Task | Knowledge to Read |
|------|------------------|
| TASK-ff0a2460 (remove MissingInverse) | `libs/validation/src/structural.rs`, `auto_fix.rs`, `metrics.rs`, `graph.rs` |
| TASK-8c5f7004 (fix from/to constraints) | `plugins/agile-governance/orqa-plugin.json` (lines 966-1282), `plugins/software/orqa-plugin.json` (lines 517-754) |
| TASK-2f67f14e (document vocabulary) | Both plugin manifests, `libs/types/src/plugin.ts` |
| TASK-bb5f9ff3 (artifact system review) | RES-d6e8ab11 section 7, `.orqa/delivery/` schema.json files |
| TASK-25b352ce (milestone mapping) | `.orqa/delivery/milestones/`, `.orqa/delivery/epics/` |
| TASK-b7abfa9e (ID hash migration) | All `.orqa/` artifacts. ID = PREFIX + first 8 hex of MD5(title) |
| TASK-113497f7 (.state rename) | All `tmp/` references in rules, hooks, scripts, code |

### EPIC-f6da17ed — Workflow Engine

Read RES-d6e8ab11 sections 3 and 7. Knowledge embedded in `workflow-engine-implementer` agent.

### EPIC-a63fde02 — Prompt Generation

Read RES-d6e8ab11 sections 5 and 6. Knowledge embedded in `prompt-pipeline-implementer` agent.

### EPIC-281f7857 — Agent Lifecycle

Read RES-d6e8ab11 sections 4 and 8. Knowledge embedded in `agent-lifecycle-implementer` agent.

### EPIC-ecef93a8 — Human Gates

Read RES-d6e8ab11 section 7. Knowledge embedded in `human-gates-implementer` agent.

### EPIC-59b92c8d — Content Migration

Read RES-d6e8ab11 section 10 and AD-1ef9f57c. Knowledge embedded in `content-migrator` agent.

### EPIC-9d781696 — Cleanup

Read all preceding epic artifacts. Knowledge embedded in `cleanup-reviewer` agent.

## Key Design Decisions (in effect)

- **Backward-only relationship storage** — task stores `delivers: epic`, epic does NOT store `delivered-by: task`. Graph computes inverses. (TASK-ff0a2460 implements this)
- **30 relationship types stay** — semantic precision creates structure. Each type is a unique bond.
- **Narrow from/to constraints** — specificity is the point. Fix bugs, don't widen.
- **Daemon is business logic boundary** — MCP/LSP are access protocols, not application boundaries.
- **No backwards compatibility** — pre-release, breaking changes expected, data migrated via `orqa migrate`.
- **Plugin-composed everything** — no governance patterns hardcoded in core. Plugins provide definitions, core provides engines.
- **.state/ not tmp/** — session state and metrics are operational data, not disposable.

## Drift Prevention

### What You MUST NOT Do

- Load the old governance framework (60 rules in `.orqa/process/rules/`)
- Store inverse relationships on upstream artifacts
- Implement directly — always delegate to background agents via teams
- Skip the completion gate between teams
- Deviate from the epic dependency chain
- Add backwards compatibility shims
- Use `tmp/` for new state files (use `.state/`)
- Spawn agents without a team
- Run agents in the foreground
- Use a generic agent when a specialized one exists for the task
- Ask the user for permission to continue when tasks are unblocked
- Create new rules, agents, or knowledge in `.orqa/process/` — those are the OLD system

### Self-Check (run mentally after every delegation)

1. Did I use the correct specialized agent from the Epic → Agent table?
2. Did I create a team first?
3. Did I include acceptance criteria from the task artifact?
4. Did I specify the findings file path?
5. Am I working on the active epic, not skipping ahead?
6. Am I staying available for conversation, not blocking on agent work?
7. Am I about to ask the user for permission? **Stop. Don't ask. Just do it.**
8. Is there a next task? **Start it now.**

## Git Workflow

- Commit at natural boundaries (task completion, epic phase)
- No `--no-verify` — fix errors, don't skip hooks
- After Rust changes: rebuild and restart daemon
- Agents do NOT commit — the orchestrator commits after reviewing findings

## Session Protocol

1. Read this file (`.claude/CLAUDE.md`)
2. Check `tmp/session-state.md` (or `.state/session-state.md` after rename) for previous session context
3. Check `git status` and `git stash list`
4. Read the active epic artifact (the one marked `[ACTIVE]` in the implementation path)
5. Resume from where the previous session left off
6. **Begin working immediately. Do not summarise what you're about to do — do it.**
7. **Work through every epic in sequence until EPIC-9d781696 is done or a genuine blocker is hit**
8. When context compaction occurs mid-session, re-read this file and the active epic to re-orient, then continue without asking
9. Write session state to `.state/session-state.md` (or `tmp/` pre-rename) periodically — at minimum after each task completion
10. If the session ends (context limit, user stop), session state must reflect exactly where to resume

## Epic Transition Protocol

When an epic's last task is verified complete:

1. Run the completion gate (section 7 above)
2. Commit all changes
3. `TeamDelete` the current team
4. Update the `[ACTIVE]` marker in this file to the next epic
5. Read the next epic artifact
6. Create a new team for the next epic
7. Begin the first task of the next epic
8. **Do not pause between epics. Do not ask for confirmation. Just continue.**
