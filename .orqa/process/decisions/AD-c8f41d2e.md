---
id: "AD-c8f41d2e"
type: discovery-decision
title: "Service dependency checks and orqa dev orchestration"
description: "Design for how the connector verifies service availability (daemon, MCP, LSP), how orqa dev orchestrates all services, and port allocation."
status: active
created: 2026-03-26T00:00:00.000Z
updated: 2026-03-26T00:00:00.000Z
relationships:
  - target: "EPIC-2451d1a9"
    type: "related"
    rationale: "Phase 2 — Connector Thinning service dependency design"
  - target: "AD-a44384d1"
    type: "refines"
    rationale: "Builds on CLI-as-single-interface decision to define the daemon-only dependency model"
  - target: "AD-1ef9f57c"
    type: "aligned-with"
    rationale: "Consistent with resolution Q8: daemon is the business logic boundary, MCP/LSP are access protocols"
  - target: "TASK-5a858776"
    type: "related"
    rationale: "Design artifact for the service dependency check task"
---

## Decision

The connector depends on exactly **one service**: the daemon. MCP and LSP are stdio subprocesses managed by their consumers (Claude Code, editors), not services the connector checks. The `orqa dev` command orchestrates all development processes but the connector's service gate checks only the daemon.

---

## 1. Service Architecture (Current State)

Per AD-a44384d1, the architecture has converged to:

| Component | Transport | Lifecycle | Port |
|-----------|-----------|-----------|------|
| **Daemon** | HTTP | Long-running process (PID file in `tmp/daemon.pid`) | `ORQA_PORT_BASE + 58` (default: 10258) |
| **MCP** | stdio | Subprocess of Claude Code (spawned on demand) | None |
| **LSP** | stdio | Subprocess of editor (spawned on demand) | None |
| **Search** | Embedded in daemon | Same as daemon | Same as daemon |

**Key insight:** The connector only needs to gate on the daemon. MCP and LSP are not services — they are protocol adapters that the connector's consumer (Claude Code) spawns as needed. The connector never connects to MCP or LSP; it IS the thing that uses them.

---

## 2. Daemon Gate (Existing — Verified)

### Mechanism

The daemon gate is implemented in two places, both functional:

1. **`daemon-gate.sh`** — Runs on `UserPromptSubmit` hook. Blocks interaction if daemon is unreachable.
2. **`session-start.sh`** — Runs on `SessionStart` hook. Blocks session if daemon is unreachable.

Both use the same pattern:

```bash
PORT_BASE="${ORQA_PORT_BASE:-10200}"
DAEMON_PORT=$((PORT_BASE + 58))
curl -sf --max-time 2 "http://127.0.0.1:${DAEMON_PORT}/health"
```

### Health Endpoint Contract

The daemon exposes `GET /health` which returns JSON:

```json
{
  "artifacts": 142,
  "rules": 23
}
```

- **Timeout:** 2 seconds (hook scripts), 500ms (CLI commands)
- **Failure behavior:** Hook outputs a deny decision via stderr JSON, exits with code 2
- **Recovery guidance:** Message includes `orqa daemon start` command and port information

### TypeScript Equivalent

The connector's TypeScript hooks (`shared.ts`) also have daemon connectivity:

```typescript
const DAEMON_BASE = `http://localhost:${getDaemonPort()}`;
// callDaemon() — POST to daemon with 8s timeout
// Falls back to spawning orqa-validation binary if daemon is unreachable
```

The binary fallback (`callBinary`) provides degraded operation when the daemon is down.

### Verified Behavior

- Session start: **BLOCKS** if daemon is unreachable (exit 2 + deny decision)
- Prompt submit: **BLOCKS** if daemon is unreachable (exit 2 + deny decision)
- Hook calls: **DEGRADES** to binary fallback (spawns `orqa-validation` per-call)

---

## 3. MCP Check — NOT NEEDED

### Rationale

Per AD-a44384d1, MCP is a CLI protocol mode (`orqa mcp`), not a separate service. Claude Code spawns it as an stdio subprocess via `.mcp.json`:

```json
{
  "mcpServers": {
    "orqastudio": {
      "command": "orqa",
      "args": ["mcp"]
    }
  }
}
```

The connector does not connect to MCP. Claude Code manages the MCP subprocess lifecycle. If `orqa mcp` cannot reach the daemon, it prints a warning to stderr — Claude Code sees this and reports it.

### Migration Note

The current codebase still has a standalone `orqa-mcp-server` binary (Rust) that `orqa mcp` spawns. The target architecture (AD-a44384d1) is to make `orqa mcp` a native CLI mode that speaks MCP protocol directly. Either way, MCP remains stdio — no port, no health check needed from the connector.

During development (`orqa dev`), the dev controller spawns the MCP binary as a managed process for live reload purposes. This is a development convenience, not a production dependency.

---

## 4. LSP Check — NOT NEEDED

### Rationale

Same as MCP. LSP is an stdio subprocess spawned by the editor:

```
Editor → orqa lsp → daemon (HTTP)
```

The connector has no relationship with LSP. The editor manages the LSP subprocess. If the daemon is down, `orqa lsp` degrades gracefully (no diagnostics until daemon is available).

---

## 5. Port Allocation Scheme

Current port map (from `libs/cli/src/lib/ports.ts`):

| Service | Offset | Default Port | Notes |
|---------|--------|-------------|-------|
| daemon | +58 | 10258 | Only port the connector checks |
| vite | +220 | 10420 | Dev server only |
| dashboard | +201 | 10401 | Dev metrics only |
| sync | +202 | 10402 | Dev sync only |

**No additional ports needed.** MCP and LSP are stdio. Search is embedded in the daemon. The connector checks exactly one port: the daemon port.

Environment override: `ORQA_PORT_BASE` (default: 10200) shifts all ports.

---

## 6. `orqa dev` Command Design

### Current State (Verified)

`orqa dev` (`libs/cli/src/commands/dev.ts`) already orchestrates all development services:

#### Spawn Order

1. **Daemon** — `orqa daemon start` (detached, PID file at `tmp/daemon.pid`)
2. **Search server** — `orqa-search-server` binary (managed process)
3. **Wait 2 seconds** — let search index load
4. **MCP server** — `orqa-mcp-server` binary (managed process)
5. **LSP server** — `orqa-lsp-server` binary (managed process)
6. **TypeScript watch builds** — `tsc --watch` for libs/sdk, libs/graph-visualiser, libs/logger
7. **Tauri app** — `cargo tauri dev` (compiles + launches app with Vite HMR)

#### Health Check Sequence

- Daemon: HTTP `GET /health` with 3s timeout and 150ms poll interval
- Search/MCP/LSP: process-alive check only (managed processes, no health endpoints)
- Vite: port check (is port listening?)
- Tauri app: process name check (`orqa-studio`)

#### Watch Mode

File watchers auto-rebuild and restart services on source changes:

- **MCP server**: watches `libs/mcp-server/src/**/*.rs` → cargo build → restart
- **LSP server**: watches `libs/lsp-server/src/**/*.rs` → cargo build → restart
- **Plugin sources**: watches `plugins/*/src/` → npm build → plugin refresh
- **Tauri app + daemon**: handled by `cargo tauri dev` (built-in Rust watch)

Debounce: 500ms between file change detection and rebuild trigger.

#### Signal-Based Restart

The dev controller uses a signal file (`tmp/dev-signal`) for remote control:

| Signal | Action |
|--------|--------|
| `restart` | Full restart of all services |
| `restart-daemon` | Daemon only |
| `restart-search` | Search + MCP (MCP depends on search) |
| `restart-mcp` | MCP server only |
| `restart-lsp` | LSP server only |
| `restart-app` | Tauri app only |
| `stop` | Graceful shutdown of everything |

CLI subcommands (`orqa dev restart mcp`, etc.) write to this signal file; the running controller picks up the signal via `fs.watchFile`.

#### Graceful Shutdown

On Ctrl+C, SIGINT, or SIGTERM:
1. Close all file watchers
2. Kill managed processes (search, MCP, LSP, app) via process tree kill
3. Remove control file (`tmp/dev-controller.json`)
4. Remove signal file (`tmp/dev-signal`)

On app window close (exit code 0):
1. Kill search, MCP, LSP
2. Remove control/signal files
3. Exit

#### Control File (IPC State)

`tmp/dev-controller.json` tracks running state:

```json
{
  "pid": 12345,
  "state": "running",
  "app": 12346,
  "search": 12347,
  "mcp": 12348,
  "lsp": 12349
}
```

### Target State (Post-Migration)

When AD-a44384d1 is fully implemented (MCP/LSP as native CLI modes, search embedded in daemon):

1. **Daemon** — `orqa daemon start` (same as today)
2. **TypeScript watch builds** — same as today
3. **Tauri app** — `cargo tauri dev` (same as today)

That is it. No separate search, MCP, or LSP processes. The daemon is the only backend. MCP and LSP are spawned by their consumers (Claude Code, editor) on demand.

The `orqa dev` command simplifies to: start daemon, start TypeScript watches, start Tauri app.

---

## 7. Connector Blocking Behavior Summary

| Event | Service Check | Failure Mode |
|-------|--------------|-------------|
| SessionStart | Daemon `/health` (2s timeout) | **BLOCK** — deny decision, exit 2 |
| UserPromptSubmit | Daemon `/health` (2s timeout) | **BLOCK** — deny decision, exit 2 |
| Hook calls (PreToolUse, etc.) | Daemon POST (8s timeout) | **DEGRADE** — binary fallback |
| MCP tool calls | N/A (Claude Code manages) | MCP subprocess fails → Claude Code reports |
| LSP diagnostics | N/A (Editor manages) | LSP subprocess fails → editor shows no diagnostics |

---

## Rationale

This design follows directly from two prior decisions:

1. **AD-a44384d1** — CLI as single interface, daemon as only service
2. **AD-1ef9f57c Q8** — Daemon is the business logic boundary, MCP/LSP are access protocols

The connector should not check services it does not consume. It consumes the daemon (via HTTP). MCP and LSP are consumed by Claude Code and the editor respectively — those consumers handle their own subprocess lifecycle.

Adding MCP/LSP health checks to the connector would:
- Create a dependency on services that might not be running (LSP only runs when an editor is open)
- Conflate development-time process management with production service architecture
- Add complexity for zero benefit (the connector never calls MCP or LSP)

## Consequences

### Positive

- Connector startup is fast — one HTTP health check, not three
- No false positives from LSP/MCP being "down" (they are demand-started, not always-on)
- Clear separation: connector checks its own dependency (daemon), not other tools' dependencies
- `orqa dev` already handles development process orchestration completely

### Negative

- If the daemon is down but MCP appears to work (because of stale cached responses), the connector will still block — this is correct behavior since rules cannot be enforced
- Developers must run `orqa dev` or `orqa daemon start` before using the connector

### Migration Path

No code changes needed for the connector's daemon gate — it already works correctly. The existing `daemon-gate.sh` and `session-start.sh` scripts implement the design documented here. When MCP/LSP transition from separate binaries to native CLI modes, `orqa dev` will simplify but the connector's behavior remains unchanged.
