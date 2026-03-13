---
id: TASK-249
title: "Decompose AppState into grouped sub-structs"
description: "AppState has 11 Mutex-wrapped fields. Group related state into sub-structs (SearchState, StreamingState, GovernanceState) to reduce flat mutex surface."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-055
acceptance:
  - "AppState fields grouped into 3-4 logical sub-structs"
  - "All command handlers updated to use new state shape"
  - "No deadlock regressions (brief lock patterns maintained)"
  - "make check passes"
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
