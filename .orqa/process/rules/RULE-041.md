---
id: RULE-041
title: Data Persistence Boundaries
description: "Defines which data belongs in SQLite, which in file-based artifacts, and which is ephemeral. Prevents misplaced persistence."
status: active
created: 2026-03-11
updated: 2026-03-13
layer: project
enforcement: []
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Data persistence boundaries create clear structural separation of concerns
  - target: RULE-003
    type: informs
    rationale: File-based artifact scanning depends on correct config paths — persistence boundaries define which data lives in scanned .orqa/ files vs SQLite
  - target: RULE-010
    type: informs
    rationale: End-to-end completeness requires all layers to agree on which persistence channel stores each type of data
  - target: EPIC-052
    type: informs
  - target: AD-032
    type: enforces
---

Data persistence in OrqaStudio follows three channels, each with clear boundaries.

## Persistence Channels

| Channel | What Belongs | Why |
|---------|-------------|-----|
| **SQLite** | Conversation data: sessions, messages, stream metrics, project settings | Structured, queryable, transactional |
| **File-based artifacts** | Governance data: rules, skills, agents, docs, planning artifacts | Version-controlled, human-readable, scannable |
| **Ephemeral** | Session state (WorkflowTracker), injected skill cache, dev server PIDs | Lost on restart, reconstructible |

## Boundaries

### SQLite (conversation persistence only)

Per [AD-032](AD-032), SQLite is scoped to conversation persistence:
- Sessions, messages, tool calls, stream events
- Project metadata (path, stack detection, settings)
- Search index (DuckDB, separate from SQLite)

SQLite MUST NOT store:
- Governance artifacts (rules, skills, agents)
- Planning artifacts (epics, tasks, ideas)
- User preferences that should be file-based

### File-based artifacts (.orqa/)

The `.orqa/` directory is the single source of truth for governance:
- Rules, skills, agents, hooks
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
- Injected skill dedup cache — which skills were already injected this session
- `tmp/` directory — session state, script output, dev artifacts

Ephemeral state MUST NOT:
- Be relied upon for correctness (graceful degradation if missing)
- Be committed to git (tmp/ is gitignored)

## FORBIDDEN

- Storing governance artifacts in SQLite
- Storing conversation data in .orqa/ files
- Relying on ephemeral state for data integrity
- Using localStorage for any application state ([AD-032](AD-032))

## Related Rules

- [RULE-003](RULE-003) (artifact-config-integrity) — file-based artifact scanning
- [RULE-010](RULE-010) (end-to-end-completeness) — all layers must agree on persistence strategy
