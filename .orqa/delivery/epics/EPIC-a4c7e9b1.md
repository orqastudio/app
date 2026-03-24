---
id: EPIC-a4c7e9b1
type: epic
title: "Port allocation standardisation and CLI process ownership"
description: "Standardise all service ports above 10000, fix the daemon port mismatch between CLI and MCP server, move process lifecycle management from the dev controller to the CLI, extract the search engine from the MCP server into a standalone process, and demote the dev controller to debug-only tooling."
status: captured
priority: P1
scoring:
  impact: 4
  urgency: 5
  complexity: 3
  dependencies: 2
created: 2026-03-24
updated: 2026-03-24
horizon: active
relationships:
  - target: MS-654badde
    type: fulfils
    rationale: "Port standardisation and CLI process ownership are infrastructure prerequisites for reliable dogfooding"
  - target: PILLAR-569581e0
    type: grounded
    rationale: "Makes infrastructure configuration visible, structured, and consistently managed through the CLI"
---

## Context

OrqaStudio runs multiple services with inconsistent port allocation. The daemon port is mismatched between CLI (`libs/cli/src/commands/daemon.ts`: DEFAULT_PORT = 3002) and MCP server (`libs/mcp-server/src/daemon.rs`: DEFAULT_DAEMON_PORT = 9258). Current workaround: `orqa daemon start --port 9258`.

User decisions from 2026-03-24 session:
- All ports move above 10000 to avoid conflicts with common development tools
- CLI owns all process lifecycle (daemon, search, MCP)
- Dev controller (`dev.mjs`) becomes debug-only
- `make` is only for bootstrapping fresh dev environments
- Search engine should be extracted from MCP server into separate process

## Implementation Design

### Port Allocation Table

| Service | Current Port | New Port | Config Location |
|---------|-------------|----------|-----------------|
| Daemon (CLI) | 3002 | 10258 | `libs/cli/src/commands/daemon.ts` |
| Daemon (MCP expects) | 9258 | 10258 | `libs/mcp-server/src/daemon.rs` |
| MCP Server | varies | 10259 | MCP server config |
| Search Engine | (embedded in MCP) | 10260 | New standalone config |
| Vite Dev Server | 1420 | 10420 | `ui/vite.config.ts` |
| Tauri Dev Port | 1420 | 10420 | `backend/src-tauri/tauri.conf.json` |

All ports in the 10200-10499 range reserved for OrqaStudio services.

### Architecture

```
CLI (orqa)
  ├── orqa daemon start/stop/status     → manages daemon process
  ├── orqa search start/stop/status     → manages search engine process
  ├── orqa mcp start/stop/status        → manages MCP server process
  └── orqa dev                          → starts all services for development

Dev Controller (dev.mjs)
  └── debug-only — runs services with verbose logging, inspector ports
```

### Phases

**Phase 1: Port remapping** — Update all port constants and config files to use 10000+ range. Single canonical port table in one config location.

**Phase 2: Fix daemon port mismatch** — Align CLI and MCP server to use the same daemon port constant. Remove the `--port 9258` workaround.

**Phase 3: CLI process lifecycle** — Implement `orqa daemon|search|mcp start|stop|status` commands. CLI manages PID files, health checks, and graceful shutdown.

**Phase 4: Extract search engine** — Move the ONNX embedding + DuckDB search from the MCP server into a standalone process with its own port. MCP server becomes a thin protocol adapter.

**Phase 5: Demote dev controller** — Move `dev.mjs` to `tools/debug-controller.mjs`. Remove `make dev` dependency on it. `orqa dev` becomes the primary entry point.

## Tasks

- [ ] [TASK-71a2d3e4](TASK-71a2d3e4): Remap all service ports to 10000+ range
- [ ] [TASK-82b3e4f5](TASK-82b3e4f5): Fix daemon port mismatch between CLI and MCP server
- [ ] [TASK-93c4f5a6](TASK-93c4f5a6): Implement CLI process lifecycle commands (daemon, search, MCP)
- [ ] [TASK-a4d5e6b7](TASK-a4d5e6b7): Extract search engine from MCP server into standalone process
- [ ] [TASK-b5e6f7c8](TASK-b5e6f7c8): Demote dev controller to debug-only tooling
- [ ] [TASK-c6f7a8d9](TASK-c6f7a8d9): Update documentation and commands reference
- [ ] [TASK-d7a8b9e0](TASK-d7a8b9e0): Reconcile EPIC-a4c7e9b1

## Out of Scope

To be confirmed with user:
- Changing the Tauri IPC mechanism (stays as `invoke()`)
- Docker/container port mapping (no Docker in current dev flow)
- CI/CD port configuration (no CI yet)
