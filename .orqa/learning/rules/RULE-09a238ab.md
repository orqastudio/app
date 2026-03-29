---
id: "RULE-09a238ab"
type: rule
title: "Data Persistence Boundaries"
description: "Defines which data belongs in SQLite, which in file-based artifacts, and which is ephemeral. Prevents misplaced persistence."
status: active
enforcement_type: advisory
created: "2026-03-11"
updated: "2026-03-13"
enforcement:

  - engine: behavioral

    message: "SQLite stores only conversation data; file-based artifacts store governance data; ephemeral state must not be relied upon for correctness"
summary: "Three persistence channels with strict boundaries: SQLite for conversation data only (sessions, messages, metrics), file-based artifacts (.orqa/) for governance data (rules, knowledge, agents, docs), ephemeral state (.state/) for session data lost on restart. SQLite must not store governance. Files must not store conversations. Ephemeral must not be relied upon for correctness."
tier: stage-triggered
roles: [implementer, reviewer, planner]
priority: P1
tags: [persistence, sqlite, artifacts, ephemeral, data-boundaries]
relationships:

  - target: "PD-859ed163"

    type: "enforces"
---
Data persistence in OrqaStudio follows three channels, each with clear boundaries.

## Persistence Channels

| Channel | What Belongs | Why |
| --- | --- | --- |
| **SQLite** | Conversation data: sessions, messages, stream metrics, project settings | Structured, queryable, transactional |
| **File-based artifacts** | Governance data: rules, knowledge, agents, docs, planning artifacts | Version-controlled, human-readable, scannable |
| **Ephemeral** | Session state (WorkflowTracker), injected knowledge cache, dev server PIDs | Lost on restart, reconstructible |

## Boundaries

### SQLite (conversation persistence only)

Per [PD-859ed163](PD-859ed163), SQLite is scoped to conversation persistence:

- Sessions, messages, tool calls, stream events
- Project metadata (path, stack detection, settings)
- Search index (DuckDB, separate from SQLite)

SQLite MUST NOT store:

- Governance artifacts (rules, knowledge, agents)
- Planning artifacts (epics, tasks, ideas)
- User preferences that should be file-based

### File-based artifacts (.orqa/)

The `.orqa/` directory is the single source of truth for governance:

- Rules, knowledge, agents, hooks
- Documentation, research, decisions
- Planning (milestones, epics, tasks, ideas)
- Project configuration (`project.json`)

File-based artifacts MUST NOT:

- Duplicate data that's in SQLite
- Store conversation content
- Store ephemeral state

### Ephemeral state

Temporary data that doesn't survive app restart:

- `WorkflowTracker` — session event history for process gates
- Injected knowledge dedup cache — which knowledge artifacts were already injected this session
- `.state/` directory — session state, script output, dev artifacts

Ephemeral state MUST NOT:

- Be relied upon for correctness (graceful degradation if missing)
- Be committed to git (.state/ is gitignored)

## FORBIDDEN

- Storing governance artifacts in SQLite
- Storing conversation data in .orqa/ files
- Relying on ephemeral state for data integrity
- Using localStorage for any application state ([PD-859ed163](PD-859ed163))

## Related Rules

- [RULE-63cc16ad](RULE-63cc16ad) (artifact-config-integrity) — file-based artifact scanning
- RULE-b03009da (end-to-end-completeness) — all layers must agree on persistence strategy
