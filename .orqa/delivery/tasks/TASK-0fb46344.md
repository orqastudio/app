---
id: TASK-0fb46344
title: Decompose AppState into grouped sub-structs
description: "AppState has 11 Mutex-wrapped fields. Group related state into sub-structs (SearchState, StreamingState, GovernanceState) to reduce flat mutex surface."
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - AppState fields grouped into 3-4 logical sub-structs
  - All command handlers updated to use new state shape
  - No deadlock regressions (brief lock patterns maintained)
  - make check passes
relationships:
  - target: EPIC-5f9fcf48
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-389af55e
    type: depended-on-by
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
