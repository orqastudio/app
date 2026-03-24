---
id: DOC-7c1a3e51
type: doc
title: CLI Architecture
description: "How the orqa CLI works as the single developer interface: three protocol modes (MCP, LSP, direct commands), the daemon as the only long-running service, and why there are no separate MCP or LSP server processes."
category: architecture
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: KNOW-7c1a3e50
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
  - target: AD-2ce57da9
    type: implements
    rationale: "Documents the architecture decision that established CLI as single interface"
---

# CLI Architecture

## Overview

OrqaStudio's developer tooling is built around a single CLI binary: `orqa`. This binary is the **only developer interface** — there are no separate MCP servers, LSP servers, or search processes. The CLI supports three protocol modes, all communicating with a single daemon as their backend.

This design was formalised in [AD-2ce57da9](AD-2ce57da9).

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Developer Tools                    │
│                                                     │
│  Claude Code ──→ orqa mcp (stdio)                   │
│  VS Code    ──→ orqa lsp (stdio)                    │
│  Terminal   ──→ orqa <command>                       │
└──────────────────────┬──────────────────────────────┘
                       │
                       ▼
              ┌────────────────┐
              │   Daemon       │
              │  (port 10258)  │
              │                │
              │  Graph engine  │
              │  Search (ONNX) │
              │  Validation    │
              └────────────────┘
```

### The Daemon

The daemon is the only long-running service in the OrqaStudio development environment. It maintains:

- **Artifact graph** — in-memory graph of all `.orqa/` artifacts and their relationships
- **Search engine** — ONNX Runtime embeddings + DuckDB storage for semantic search
- **Validation engine** — schema-driven validation shared by all consumers

The daemon starts via the app or via `orqa daemon start` for headless/CI use. It listens on port 10258.

### CLI Protocol Modes

The CLI binary supports three modes. Each mode uses a different wire protocol but calls the same daemon backend:

| Mode | Command | Protocol | Consumer | Transport |
|------|---------|----------|----------|-----------|
| MCP | `orqa mcp` | Model Context Protocol | Claude Code, AI assistants | stdio (stdin/stdout) |
| LSP | `orqa lsp` | Language Server Protocol | VS Code, Cursor, Neovim | stdio (stdin/stdout) |
| Direct | `orqa search`, `orqa graph`, etc. | CLI output | Developer terminal | Human-readable text |

#### MCP Mode

Claude Code's `.mcp.json` configuration spawns `orqa mcp` as an stdio subprocess. The CLI translates MCP tool calls into daemon API requests and returns MCP-formatted responses. Available MCP tools include `graph_query`, `graph_resolve`, `search_semantic`, `search_regex`, and `search_research`.

#### LSP Mode

The editor spawns `orqa lsp` as an stdio subprocess via its LSP client configuration. The CLI implements the Language Server Protocol, providing:

- **Diagnostics** — real-time validation of artifact YAML frontmatter (red squiggles for invalid statuses, wrong relationship types, missing required fields)
- **Completions** — valid status values, relationship types, artifact IDs
- **Hover** — artifact previews and relationship information

#### Direct Commands

Standard CLI commands for terminal use:

| Command | Purpose |
|---------|---------|
| `orqa search <query>` | Semantic or regex search across artifacts and code |
| `orqa graph query` | Query the artifact graph |
| `orqa check` | Run the validation engine (same checks as LSP, different output format) |
| `orqa install` | Sync plugin content to `.orqa/` |
| `orqa version` | Version management and sync |

## What This Eliminates

The previous architecture ([AD-3f9a1c7b](AD-3f9a1c7b)) required separate MCP and LSP server processes with their own ports and lifecycle management. This design eliminates:

| Before | After |
|--------|-------|
| MCP server process with its own port | `orqa mcp` as stdio subprocess — no port |
| LSP server process with its own port | `orqa lsp` as stdio subprocess — no port |
| Port conflict resolution for 3+ services | Single daemon port (10258) |
| PID files and health checks per service | Single daemon with health endpoint |
| Complex startup dependency chains | `orqa daemon start` is the only service |

## Configuration

### Claude Code (MCP)

In `.mcp.json`:

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

### VS Code (LSP)

In editor settings, configure the LSP client to spawn `orqa lsp` as the server command.

### Daemon

Started automatically by the app, or manually:

```bash
orqa daemon start    # Start the daemon
orqa daemon status   # Check if running
orqa daemon stop     # Stop the daemon
```

## Related Documents

- [KNOW-7c1a3e50](KNOW-7c1a3e50) — Agent-facing knowledge pair for this documentation page
- [AD-2ce57da9](AD-2ce57da9) — Architecture decision: CLI as single interface, daemon as app runtime
