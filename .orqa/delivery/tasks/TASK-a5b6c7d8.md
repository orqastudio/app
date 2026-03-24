---
id: TASK-a5b6c7d8
title: "Resolve dashboard/sync-bridge port conflict and update all 3001 consumers"
type: task
description: "The OrqaDev Dashboard and Sync Bridge both default to port 3001. Multiple consumers hardcode localhost:3001/log. Separate these services to distinct ports (dashboard: 10401, sync bridge: 10402) and update all hardcoded references."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - OrqaDev Dashboard uses port 10401 instead of 3001
  - Sync Bridge uses port 10402 instead of 3001
  - libs/logger/src/index.ts updated to use new dashboard port
  - app/ui/src/lib/utils/dev-console.ts updated to use new dashboard port
  - connectors/claude-code/src/hooks/telemetry.ts updated to use new dashboard port
  - tools/debug/dev.mjs updated to use new dashboard port
  - infrastructure/sync-bridge/src/config.ts updated to use new port
  - search_regex for localhost:3001 returns zero results in application code
  - make check passes
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Resolves port conflict discovered during port audit"
  - target: TASK-71a2d3e4
    type: depends-on
    rationale: "Canonical port allocation must be established first"
---

## What

The port audit discovered that both the OrqaDev Dashboard (`tools/debug/dev.mjs:54`) and Sync Bridge (`infrastructure/sync-bridge/src/config.ts:20`) default to port 3001. Additionally, four consumers hardcode `localhost:3001/log`:

- `tools/debug/dev.mjs:54` — dashboard server binds here
- `libs/logger/src/index.ts:40` — logger lib forwards here
- `app/ui/src/lib/utils/dev-console.ts:13` — frontend dev console forwards here
- `connectors/claude-code/src/hooks/telemetry.ts:8` — hook telemetry forwards here

These must be separated to avoid latent port conflicts and updated to the new 10000+ range.

## Files to Modify

- `tools/debug/dev.mjs` — change `DASHBOARD_PORT` from 3001 to 10401
- `libs/logger/src/index.ts` — change hardcoded `localhost:3001` to `localhost:10401`
- `app/ui/src/lib/utils/dev-console.ts` — change hardcoded `localhost:3001` to `localhost:10401`
- `connectors/claude-code/src/hooks/telemetry.ts` — change hardcoded `localhost:3001` to `localhost:10401`
- `infrastructure/sync-bridge/src/config.ts` — change default from `3001` to `10402`

## Verification

1. `search_regex` for `3001` in application code returns zero results
2. Dashboard starts on 10401 and receives log events
3. Sync bridge starts on 10402 without conflicting with dashboard
4. `make check` passes
