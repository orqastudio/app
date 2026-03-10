---
id: RULE-027
slug: structure-before-work
layer: canon
status: active
scope: system
title: Structure Before Work
description: No implementation work may begin without the full artifact structure in place first.
created: 2026-03-07
updated: 2026-03-07
---

## The Rule (NON-NEGOTIABLE)

**No implementation work may begin until the artifact structure for that work exists.**

Before writing ANY code, the following artifacts MUST exist:

1. **Epic** — An `EPIC-NNN.md` in `.orqa/planning/epics/` with status, milestone reference, research-refs, and implementation design in the body
2. **Tasks** — One or more `TASK-NNN.md` in `.orqa/planning/tasks/` with epic reference, scope, and acceptance criteria
3. **Research** (if investigation was needed) — Research docs in `.orqa/planning/research/` referenced by the epic's `research-refs`
4. **Decision** (if an architectural choice was made) — An `AD-NNN.md` in `.orqa/governance/decisions/` with the decision index updated

The orchestrator MUST verify these artifacts exist and are complete before delegating any implementation task to an agent.

## Why

Without structure first:
- Work happens without traceability — there's no record of what was decided, why, or what was delivered
- Scope creeps silently — without defined acceptance criteria, "done" is ambiguous
- The artifact system becomes a retroactive paperwork exercise instead of a planning tool
- Historical backfill is expensive and lossy — decisions reconstructed from git history are less accurate than decisions captured in the moment

## The Sequence (MANDATORY)

```
1. Identify the work (user request, bug, idea)
2. Create or update the epic with implementation design
3. Create tasks with scope and acceptance criteria
4. Create research docs if investigation is needed
5. Create decision artifacts if architectural choices are made
6. Get user approval of the structure
7. THEN delegate implementation to agents
```

Steps 2-5 may happen in parallel. Step 6 is a gate — no implementation without approval.

## What Counts as "Structure in Place"

| Artifact | Minimum Required Fields |
|----------|----------------------|
| Epic | `id`, `title`, `status`, `milestone`, `description`, implementation design in body |
| Task | `id`, `title`, `status`, `epic`, `scope`, `acceptance` |
| Research | `title`, `type`, `status`, `category`, `description` |
| Decision | `id`, `title`, `status`, `category`, `description`, Decision + Rationale + Consequences sections |

## Exceptions

- **Single-line fixes** (typos, config tweaks) — no epic/task needed, but still commit with descriptive message
- **Governance artifact updates** (rules, skills, agent definitions) — these ARE the structure, they don't need their own structure
- **Session state and memory updates** — coordination artifacts, not implementation
- **Emergency hotfixes** — fix first, create structure immediately after (within the same session)

For emergency hotfixes: the structure MUST be created before the session ends. "I'll backfill later" is not acceptable.

## Orchestrator Responsibility

The orchestrator MUST:

1. **Check for existing structure** before starting any work — search epics and tasks for related artifacts
2. **Create missing structure** before delegating — if a user request doesn't have an epic, create one first
3. **Refuse to delegate** if the structure is incomplete — agents must not start work without a task that has an epic reference and acceptance criteria
4. **Verify structure accuracy** after work completes — the epic's implementation design should match what was actually built

## Agent Responsibility

Agents receiving tasks MUST:

1. **Read the task** — verify scope and acceptance criteria exist
2. **Read the epic** — understand the broader context and implementation design
3. **Flag missing structure** — if the task lacks acceptance criteria or the epic lacks design, stop and report to the orchestrator
4. **Never start work on an unstructured request** — "just fix this" without a task is a process violation

## FORBIDDEN

- Implementing features without an epic
- Creating tasks without an epic reference
- Starting implementation before the structure is approved
- Treating artifact creation as an afterthought ("we'll document it later")
- Delegating to agents without tasks that have scope and acceptance criteria
- Backfilling structure more than one session after the work was done

## Related Rules

- RULE-008 (documentation-first) — documentation before code (this rule extends it to ALL artifacts)
- RULE-004 (artifact-lifecycle) — artifact creation standards and status transitions
- RULE-011 (enforcement-before-code) — enforcement artifacts before implementation
- RULE-019 (no-deferred-deliverables) — if it's in scope, it ships
- RULE-015 (honest-reporting) — structure prevents ambiguous completion claims
