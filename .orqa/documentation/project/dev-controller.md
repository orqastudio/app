---
id: DOC-063
title: Dev Controller and OrqaDev Dashboard
description: Architecture of the dev controller (debugger/dev.mjs) and the OrqaDev web dashboard for unified process management and log streaming during development.
created: 2026-03-12
updated: 2026-03-12
sort: 5
relationships:
  - target: RES-039
    type: informs
    rationale: Documentation page references RES-039
---

## Overview

The dev controller (`debugger/dev.mjs`) is a persistent Node.js process that owns the
entire development lifecycle. It spawns and manages Vite and Tauri processes, captures
their output, and serves the **OrqaDev web dashboard** — a real-time log viewer and
process control panel accessible at `http://localhost:3001`.

This replaces `cargo tauri dev` which has known issues with Vite process orphaning on
crash, `taskkill` hangs on MSYS2/Git Bash, and no unified build visibility. See
[RES-039](RES-039) for the original research.

## Architecture

```
┌─────────────────────────────────────────────┐
│  Dev Controller (debugger/dev.mjs)           │
│  Port 3001 — HTTP + SSE                     │
│                                             │
│  ┌──────────┐  ┌──────────┐                 │
│  │  Vite    │  │  Cargo   │  Child          │
│  │  :1420   │  │  tauri   │  processes      │
│  └────┬─────┘  └────┬─────┘                 │
│       │stdout/err    │stdout/err             │
│       └──────┬───────┘                       │
│              ▼                               │
│     Log broadcaster (SSE)                    │
│     { source, text, error }                  │
│              │                               │
│              ▼                               │
│     GET /events → all connected clients      │
└─────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────┐
│  OrqaDev Dashboard (dev-dashboard.html)      │
│  Browser tab — auto-opened on start          │
│                                             │
│  ┌─ Header ──────────────────────────────┐  │
│  │  ORQADEV    ● vite alive  ● rust alive│  │
│  ├─ Controls ────────────────────────────┤  │
│  │  [Start] [↻Tauri] [↻Vite] [↻All] [■] │  │
│  │  Filters: [ctrl] [vite] [rust]        │  │
│  ├─ Log Output ──────────────────────────┤  │
│  │  12:34:05 [vite] hmr update /App...   │  │
│  │  12:34:06 [rust] Compiling orqa...    │  │
│  │  12:34:07 [ctrl] Tauri ready on :1420 │  │
│  ├─ Footer ──────────────────────────────┤  │
│  │  142 lines                 ● connected│  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
```

## HTTP Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| `GET /` | Serves `dev-dashboard.html` |
| `GET /events` | SSE stream — real-time logs and status updates |
| `POST /command/start` | Start Vite + Tauri |
| `POST /command/restart-tauri` | Kill Tauri, recompile, relaunch (Vite stays) |
| `POST /command/restart-vite` | Restart Vite only |
| `POST /command/restart` | Restart both Vite + Tauri |
| `POST /command/stop` | Graceful shutdown of all processes |

## SSE Event Protocol

### `log` event

```json
{ "source": "vite", "text": "hmr update /src/App.svelte", "error": false }
```

Sources: `ctrl` (controller), `vite` (frontend dev server), `rust` (Cargo/Tauri)

### `status` event

```json
{ "vite": true, "rust": "building" }
```

Values: `true` (alive), `false` (dead), `"building"` (compiling)

## Process Management

The controller handles platform-specific process cleanup:

| Platform | Port detection | Process kill |
|----------|---------------|-------------|
| Windows (MSYS2) | `netstat -ano` | PowerShell `Get-Process` + tree traversal |
| macOS | `lsof -ti:port` | `kill` |
| Linux | `ss -tlnp` | `pgrep` + `kill` |

State is tracked in `tmp/dev-controller.json` with PID and child process info.

## Make Integration

| Command | What It Does |
|---------|-------------|
| `make dev` | Spawn controller detached, wait for ready, exit |
| `make start` | Run controller in foreground (long-running) |
| `make stop` | Graceful stop via controller |
| `make kill` | Force-kill all processes |
| `make restart-tauri` | Signal controller to rebuild Tauri only |
| `make restart-vite` | Signal controller to restart Vite only |
| `make restart` | Signal controller to restart everything |
| `make status` | Show controller and process status |

See [commands reference](commands.md) for full details.

## Files

| File | Purpose |
|------|---------|
| `debugger/dev.mjs` | Controller + HTTP/SSE server |
| `debugger/dev-dashboard.html` | Self-contained dashboard UI (HTML/CSS/JS) |
| `tmp/dev-controller.json` | Runtime state (PIDs, status) — gitignored |

## Why Not `cargo tauri dev`

1. **Vite orphaning** — `cargo tauri dev` orphans the Vite process on crash (Tauri issues #10023, #2794, #1626)
2. **taskkill hangs** — MSYS2/Git Bash path mangling breaks Windows process cleanup
3. **No unified visibility** — Standard approach splits output across multiple terminals
4. **No partial restart** — Can't keep Vite alive while restarting only the Tauri binary
5. **No remote control** — Agents and scripts can't trigger restarts programmatically
