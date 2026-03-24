---
id: TASK-93c4f5a6
title: "Implement CLI process lifecycle commands (daemon, search, MCP)"
type: task
description: "Add orqa daemon|search|mcp start|stop|status commands to the CLI. CLI manages PID files, health checks, and graceful shutdown for all OrqaStudio services."
status: completed
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - orqa daemon start/stop/status commands exist and work
  - orqa search start/stop/status commands exist and work
  - orqa mcp start/stop/status commands exist and work
  - PID files are written and cleaned up on start/stop
  - Health check endpoint or mechanism exists for each service
  - Graceful shutdown sends proper signals before force-killing
  - orqa dev starts all services using these lifecycle commands
  - orqa help documents all new commands
  - make check passes
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Phase 3 of port allocation epic"
  - target: TASK-71a2d3e4
    type: depends-on
    rationale: "Needs canonical ports for service configuration"
  - target: TASK-a4d5e6b7
    type: depended-on-by
    rationale: "Auto-generated inverse of depends-on relationship from TASK-a4d5e6b7"
  - target: TASK-b5e6f7c8
    type: depended-on-by
    rationale: "Auto-generated inverse of depends-on relationship from TASK-b5e6f7c8"
---

## What

The CLI becomes the single entry point for managing all OrqaStudio service processes. Each service (daemon, search engine, MCP server) gets start/stop/status subcommands with proper process management.

## Design

```
orqa daemon start [--port N]   # Start daemon, write PID file
orqa daemon stop               # Graceful shutdown via PID
orqa daemon status             # Report running/stopped, port, PID

orqa search start [--port N]   # Start search engine process
orqa search stop
orqa search status

orqa mcp start [--port N]      # Start MCP server
orqa mcp stop
orqa mcp status

orqa dev                       # Start all services for development
```

### PID File Location

`tmp/pids/` (gitignored) — one file per service: `daemon.pid`, `search.pid`, `mcp.pid`

### Health Checks

Each service exposes a `/health` endpoint (HTTP) or equivalent mechanism. The `status` command checks this.

## Verification

1. Start each service individually, verify PID file exists
2. Stop each service, verify process is gone and PID file cleaned up
3. `orqa dev` starts all three services
4. `orqa dev` is idempotent (doesn't double-start already-running services)
