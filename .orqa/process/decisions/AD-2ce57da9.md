---
id: AD-2ce57da9
type: decision
title: "CLI as single developer interface, daemon as app runtime only"
description: "Eliminate separate MCP server and LSP server processes. The CLI becomes the single developer interface with multiple protocol modes (MCP, LSP, direct commands). The daemon is for app runtime only."
status: active
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: EPIC-a4c7e9b1
    type: drives
    rationale: "Simplifies the port table and process list in the epic — no MCP server port, no LSP server port (both are stdio via CLI)"
  - target: AD-3f9a1c7b
    type: evolves-from
    rationale: "AD-3f9a1c7b called for MCP and LSP as separate managed processes; this decision consolidates them into CLI protocol modes"
  - target: AD-48593e40
    type: evolves-from
    rationale: "AD-48593e40 described central LSP/MCP registration via plugin manifests as separate servers; this decision makes them CLI modes instead"
  - target: AD-99c2a969
    type: evolves-from
    rationale: "Search engine remains standalone but is reached via the daemon, not a separate search process"
  - target: PILLAR-569581e0
    type: aligned-with
    rationale: "Simplifies developer infrastructure to a single CLI entry point — clearer, more structured, less to manage"
  - target: EPIC-3ecc76ff
    type: implemented-by
  - target: DOC-0102aeb8
    type: aligned-with-by
    rationale: "Auto-generated inverse of aligned-with-by relationship from DOC-0102aeb8"
  - target: KNOW-7c1a3e50
    type: aligned-with-by
    rationale: "Auto-generated inverse of aligned-with-by relationship from KNOW-7c1a3e50"
  - target: DOC-7c1a3e51
    type: aligned-with-by
    rationale: "Auto-generated inverse of aligned-with-by relationship from DOC-7c1a3e51"
---
## Decision

Eliminate separate MCP server and LSP server processes. The CLI (`orqa`) becomes the **single developer interface** with multiple protocol modes. The daemon is for **app runtime only**.

### Architecture

| Component | Role | Lifecycle |
|-----------|------|-----------|
| **Daemon** | App runtime service — graph queries, validation, search engine | Long-running. Started by the app or by `orqa daemon start` for headless use. |
| **CLI** | Single developer interface | Invoked on demand. Not a long-running process. |

### CLI Protocol Modes

The CLI supports multiple protocol modes, all communicating with the daemon as their backend:

| CLI Mode | Protocol | Consumer | How It Runs |
|----------|----------|----------|-------------|
| `orqa mcp` | MCP (stdio) | Claude Code, AI coding assistants | Spawned by the AI tool as an stdio subprocess |
| `orqa lsp` | LSP (stdio) | IDEs (VS Code, Cursor, Neovim) | Spawned by the editor as an stdio subprocess |
| `orqa search`, `orqa graph`, `orqa validate`, etc. | Direct CLI | Developer terminal | Interactive CLI commands |

### What This Eliminates

- **No separate MCP server process** — `orqa mcp` speaks MCP protocol over stdio, launched by Claude Code as needed
- **No separate LSP server process** — `orqa lsp` speaks LSP protocol over stdio, launched by the editor as needed
- **No MCP server port** — stdio, not TCP
- **No LSP server port** — stdio, not TCP (debug TCP mode optional)
- **No MCP server lifecycle management** — the AI tool manages the subprocess
- **No LSP server lifecycle management** — the editor manages the subprocess

### What Remains

- **Daemon** — the app runtime service. Handles graph queries, search, validation. Listens on a single port (10258).
- **Search engine** — embedded within the daemon (ONNX + DuckDB). Not a separate process.
- **CLI** — talks to the daemon for all backend operations. Protocol mode determines the wire format (MCP, LSP, or human-readable CLI output).

### LSP Enforcement Capability

The LSP mode (`orqa lsp`) enforces artifact schemas in real-time within editors:

- Red squiggles on invalid status values in YAML frontmatter
- Warnings for wrong relationship types
- Errors for missing required fields
- Completions for valid status transitions and relationship types

This replaces the file-level validation that was done by hooks and scripts.

## Rationale

The previous architecture (AD-3f9a1c7b) called for MCP and LSP as separate managed processes launched by the dev controller. This created unnecessary complexity:

1. **Port proliferation** — MCP server needed its own port, LSP server needed its own port, each with port conflict resolution logic
2. **Process lifecycle overhead** — PID files, health checks, graceful shutdown for each process
3. **Startup dependency chains** — the dev controller had to start daemon, then MCP, then LSP, then search, in order
4. **Redundant functionality** — both MCP and LSP needed their own daemon client code, their own health checks, their own error handling

The stdio model is how MCP and LSP are designed to work. Claude Code spawns `orqa mcp` as a subprocess and communicates over stdin/stdout. VS Code spawns `orqa lsp` as a subprocess. Neither needs a port. Neither needs lifecycle management beyond the subprocess lifetime.

The daemon remains the single long-running service because it maintains state (graph, search index) that is expensive to rebuild on every CLI invocation.

## Alternatives Considered

### Keep MCP and LSP as separate TCP servers (REJECTED — AD-3f9a1c7b original)

Separate processes with their own ports. Rejected because:
- Port management complexity for services designed for stdio
- Unnecessary lifecycle management for services that are naturally subprocess-scoped
- MCP spec and LSP spec both prefer stdio transport

### Embed everything in the daemon (REJECTED)

Run MCP and LSP as daemon sub-services on different ports. Rejected because:
- Daemon becomes a monolith
- Editor and AI tool expect to spawn a subprocess, not connect to a port
- Mixing app runtime concerns with developer tooling concerns

### CLI wraps separate binaries (REJECTED)

`orqa mcp` shells out to a separate `orqa-mcp-server` binary. Rejected because:
- Extra build artifacts and distribution complexity
- The CLI already has all the daemon client code; adding protocol adapters is minimal

## Consequences

### Positive

- Port table simplifies dramatically — only the daemon port matters for service infrastructure
- Process list simplifies — daemon is the only long-running service (plus the app itself)
- Developer setup is simpler — `orqa daemon start` is the only service to manage
- MCP and LSP get the transport their protocols were designed for (stdio)
- No port conflicts possible for MCP and LSP (no ports used)

### Negative

- Each `orqa mcp` invocation must connect to the daemon — slight latency on first tool call
- IDE must be configured to spawn `orqa lsp` (standard LSP client config)
- Daemon must be running for CLI modes to function (graceful error if not)

### Constraints

- The daemon must expose a stable internal API that CLI protocol modes can call
- CLI must detect whether the daemon is running and provide helpful errors if not
- `orqa mcp` must be fully compatible with the MCP stdio transport spec
- `orqa lsp` must be fully compatible with the LSP stdio transport spec
