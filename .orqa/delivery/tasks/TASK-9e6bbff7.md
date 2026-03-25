---
id: "TASK-9e6bbff7"
type: "task"
title: "Decompose AppState into grouped sub-structs"
description: "AppState has 11 Mutex-wrapped fields. Group related state into sub-structs (SearchState, StreamingState, GovernanceState) to reduce flat mutex surface."
status: "completed"
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "AppState fields grouped into 3-4 logical sub-structs"
  - "All command handlers updated to use new state shape"
  - "No deadlock regressions (brief lock patterns maintained)"
  - "make check passes"
relationships:
  - target: "EPIC-a1555708"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## What

AppState has 11 Mutex-wrapped fields. Group related state into sub-structs (SearchState, StreamingState, GovernanceState) to reduce flat mutex surface.

## How

To be determined during implementation.

## Verification

- [ ] AppState fields grouped into 3-4 logical sub-structs
- [ ] All command handlers updated to use new state shape
- [ ] No deadlock regressions (brief lock patterns maintained)
- [ ] make check passes