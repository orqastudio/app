---
id: EPIC-a4c7e9b1
type: epic
title: "Port allocation standardisation and CLI process ownership"
description: "Standardise all service ports above 10000, fix the daemon port mismatch between CLI and MCP server, establish the CLI as the single developer interface with MCP/LSP as stdio protocol modes, embed the search engine in the daemon, and demote the dev controller to debug-only tooling."
status: active
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
  - target: TASK-14b5c6d7
    type: delivered-by
  - target: TASK-82b3e4f5
    type: delivered-by
  - target: TASK-f3a4b5c6
    type: delivered-by
  - target: TASK-c8d9e0f1
    type: delivered-by
  - target: TASK-b5e6f7c8
    type: delivered-by
  - target: TASK-c6f7a8d9
    type: delivered-by
  - target: TASK-71a2d3e4
    type: delivered-by
  - target: TASK-b7c8d9e0
    type: delivered-by
  - target: AD-2ce57da9
    type: driven-by
  - target: TASK-93c4f5a6
    type: delivered-by
  - target: TASK-a4d5e6b7
    type: delivered-by
  - target: TASK-a5b6c7d8
    type: delivered-by
  - target: TASK-d7a8b9e0
    type: delivered-by
---

## Context

OrqaStudio runs multiple services with inconsistent port allocation. The daemon port is mismatched between CLI (`libs/cli/src/commands/daemon.ts`: DEFAULT_PORT = 3002) and MCP server (`libs/mcp-server/src/daemon.rs`: DEFAULT_DAEMON_PORT = 9258). Current workaround: `orqa daemon start --port 9258`.

User decisions from 2026-03-24 session:
- All ports move above 10000 to avoid conflicts with common development tools
- CLI is the single developer interface with multiple protocol modes ([AD-2ce57da9](AD-2ce57da9))
- `orqa mcp` = MCP protocol mode (stdio) for Claude Code — NOT a separate server process
- `orqa lsp` = LSP protocol mode (stdio) for IDEs — NOT a separate server process
- Daemon is for app runtime only (graph, search, validation)
- Search engine is embedded in the daemon, not a separate process
- Dev controller (`dev.mjs`) becomes debug-only
- `make` is only for bootstrapping fresh dev environments

## Implementation Design

### Port Allocation Table (Complete Inventory)

| Service | Current Port | New Port | Config Locations | Notes |
|---------|-------------|----------|-----------------|-------|
| Daemon (graph, search, validation) | 3002 (CLI) / 9258 (MCP/LSP) | 10258 | `libs/cli/src/commands/daemon.ts`, `libs/mcp-server/src/daemon.rs`, `libs/validation/src/bin/server.rs`, `libs/lsp-server/src/bin/server.rs`, `app/backend/src-tauri/src/servers/lsp.rs` | Only long-running service |
| Vite Dev Server | 1420 | 10420 | `tools/debug/dev.mjs`, `app/backend/src-tauri/tauri.conf.json` | Dev only |
| OrqaDev Dashboard | 3001 | 10401 | `tools/debug/dev.mjs`, `libs/logger/src/index.ts`, `app/ui/src/lib/utils/dev-console.ts`, `connectors/claude-code/src/hooks/telemetry.ts` | Debug only |
| Sync Bridge | 3001 | 10402 | `infrastructure/sync-bridge/src/config.ts` | |
| Forgejo HTTP | 3030 | 10030 | `infrastructure/orqastudio-git/docker-compose.yml` | |
| Forgejo SSH | 222 | 10222 | `infrastructure/orqastudio-git/docker-compose.yml` | |
| App IPC Socket | random (port 0) | Keep as-is | `app/backend/src-tauri/src/servers/ipc_socket.rs` | |

**Eliminated ports (per [AD-2ce57da9](AD-2ce57da9)):**
- ~~MCP Server (was 10259)~~ — now `orqa mcp` over stdio, no port needed
- ~~Search Engine (was 10260)~~ — now embedded in the daemon
- ~~LSP Server TCP (was 10261)~~ — now `orqa lsp` over stdio, no port needed

All ports in the 10200-10499 range reserved for OrqaStudio services.

### Consumers of Daemon Port (all must be updated)

**Port 3002 consumers:**
- `connectors/claude-code/src/hooks/shared.ts:11` — `DAEMON_BASE = "http://localhost:3002"`
- `libs/cli/src/commands/enforce.ts:115` — `const port = 3002`
- `libs/cli/src/commands/daemon.ts:14` — `DEFAULT_PORT = 3002`
- `libs/validation/src/bin/server.rs:562` — fallback default `3002`
- `app/backend/src-tauri/src/commands/daemon_commands.rs:32` — hardcoded `127.0.0.1:3002`

**Port 9258 consumers:**
- `libs/mcp-server/src/daemon.rs:22` — `DEFAULT_DAEMON_PORT: u16 = 9258`
- `libs/mcp-server/src/bin/server.rs:50` — defaults to 9258
- `libs/lsp-server/src/bin/server.rs:28` — `DEFAULT_DAEMON_PORT: u16 = 9258`

### Consumers of Dashboard Port 3001 (all must be updated)

- `tools/debug/dev.mjs:54` — dashboard server binds here
- `libs/logger/src/index.ts:40` — logger lib forwards here
- `app/ui/src/lib/utils/dev-console.ts:13` — frontend dev console forwards here
- `connectors/claude-code/src/hooks/telemetry.ts:8` — hook telemetry forwards here

### Architecture Decisions (NON-NEGOTIABLE)

1. **Daemon is the only long-running service** — The daemon (graph, search, validation) is the single backend service. MCP and LSP are CLI protocol modes over stdio, not separate processes. See [AD-2ce57da9](AD-2ce57da9).

2. **Port conflict resolution** — If the daemon port is busy when launching, the existing process on that port MUST be killed first. Do not fail with "port in use". Do not silently pick another port.

3. **PID file lifecycle** — The daemon gets a PID file in `tmp/pids/` for lifecycle management. PID file is written on start and cleaned up on stop.

4. **Health check endpoint** — The daemon exposes a health check endpoint (`/health`). CLI modes (MCP, LSP) inherit liveness from being stdio subprocesses.

5. **Configurable port base** — Port base MUST be configurable via `ORQA_PORT_BASE` environment variable (default: 10200) so multiple OrqaStudio instances can coexist. All port offsets are relative to this base.

### Architecture (per [AD-2ce57da9](AD-2ce57da9))

```
Daemon (long-running service)
  ├── Graph queries, validation
  ├── Search engine (ONNX + DuckDB, embedded)
  └── Listens on port 10258

CLI (orqa) — single developer interface
  ├── orqa daemon start/stop/status     → manages daemon process
  ├── orqa mcp                          → MCP protocol mode (stdio) — spawned by Claude Code
  ├── orqa lsp                          → LSP protocol mode (stdio) — spawned by editors
  ├── orqa search/graph/validate        → direct CLI commands (talk to daemon)
  └── orqa dev                          → starts daemon + Vite for development

Dev Controller (dev.mjs)
  └── debug-only — runs services with verbose logging, inspector ports
```

**Key simplification:** MCP and LSP are not managed processes. They are CLI modes that Claude Code and editors spawn as stdio subprocesses. The daemon is the only process the CLI needs to manage.

### Issues Discovered During Audit

1. **App LSP hardcodes daemon port 3002** — `app/backend/src-tauri/src/servers/lsp.rs:15` passes `3002` to `orqa_lsp_server::run_stdio()`. This conflicts with the LSP server's own default of 9258. Must be updated to use canonical port.

2. **Sync bridge and dashboard both default to 3001** — `infrastructure/sync-bridge/src/config.ts:20` defaults to 3001, same as the dev dashboard at `tools/debug/dev.mjs:54`. Latent conflict that must be resolved with separate ports.

3. **Logger, dev-console, and hooks all hardcode localhost:3001/log** — Multiple consumers hardcode the dashboard URL. Must be updated to the new dashboard port or made configurable.

### Phases

**Phase 1: Port remapping** — Update all port constants and config files to use 10000+ range. Single canonical port table in one config location. Includes all newly discovered config locations from the audit.

**Phase 2: Fix daemon port mismatch** — Align CLI, MCP server, LSP server, and app backend to use the same daemon port constant. Remove the `--port 9258` workaround. Fix the app LSP hardcoded port.

**Phase 3: CLI daemon lifecycle** — Implement `orqa daemon start|stop|status` command. CLI manages daemon PID file, health check, and graceful shutdown. Implement `ORQA_PORT_BASE` configuration. (Note: `orqa mcp` and `orqa lsp` are stdio modes, not managed processes — no start/stop/status needed.)

**Phase 4: CLI protocol modes** — Implement `orqa mcp` (MCP stdio mode) and `orqa lsp` (LSP stdio mode). Both connect to the daemon as their backend. Search engine is embedded in the daemon (no separate extraction needed).

**Phase 5: Demote dev controller** — Move `dev.mjs` to `tools/debug-controller.mjs`. Remove `make dev` dependency on it. `orqa dev` becomes the primary entry point (starts daemon + Vite).

## Tasks

- [ ] [TASK-71a2d3e4](TASK-71a2d3e4): Remap all service ports to 10000+ range
- [ ] [TASK-82b3e4f5](TASK-82b3e4f5): Fix daemon port mismatch between CLI and MCP server
- [ ] [TASK-93c4f5a6](TASK-93c4f5a6): Implement CLI daemon lifecycle commands (start/stop/status, PID file, health check)
- [ ] ~~[TASK-a4d5e6b7](TASK-a4d5e6b7): Extract search engine from MCP server into standalone process~~ — **REMOVED per [AD-2ce57da9](AD-2ce57da9)**: search engine stays embedded in daemon
- [ ] [TASK-b5e6f7c8](TASK-b5e6f7c8): Demote dev controller to debug-only tooling
- [ ] [TASK-c6f7a8d9](TASK-c6f7a8d9): Update documentation and commands reference
- [ ] [TASK-b7c8d9e0](TASK-b7c8d9e0): Create canonical port allocation reference doc
- [ ] [TASK-f3a4b5c6](TASK-f3a4b5c6): Fix app LSP hardcoded daemon port
- [ ] [TASK-a5b6c7d8](TASK-a5b6c7d8): Resolve dashboard/sync-bridge port conflict and update all 3001 consumers
- [ ] NEW: Implement `orqa mcp` CLI protocol mode (MCP over stdio, connects to daemon)
- [ ] NEW: Implement `orqa lsp` CLI protocol mode (LSP over stdio, connects to daemon, schema enforcement)
- [ ] [TASK-d7a8b9e0](TASK-d7a8b9e0): Reconcile EPIC-a4c7e9b1

## Out of Scope

To be confirmed with user:
- Changing the Tauri IPC mechanism (stays as `invoke()`)
- Docker/container port mapping (no Docker in current dev flow)
- CI/CD port configuration (no CI yet)
