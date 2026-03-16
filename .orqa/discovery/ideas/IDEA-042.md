---
id: IDEA-042
title: Dev Controller — unified development lifecycle orchestrator
description: "A persistent Node process that owns the entire dev lifecycle — spawning, monitoring, and restarting all app processes (Vite, Rust, sidecar) with unified output, IPC-based control, and dogfood session persistence."
status: exploring
created: 2026-03-10
updated: 2026-03-11
horizon: next
research-needed:
  - How to spawn Vite and cargo independently (bypass cargo tauri dev entirely)
  - IPC mechanism for make stop/restart to signal the running controller
  - Session context injection for dogfood persistence across restarts
  - "How to pipe controller output into OrqaStudio's own UI (dev panel)"
  - "Hot reload strategy — Vite HMR for frontend, controlled restart for Rust"
relationships:
  - target: MS-001
    type: delivers
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-002
    type: grounded-by
---


## Motivation

Development workflow hits friction from three independent problems that share one root cause: **no single process owns the dev lifecycle**.

1. **Orphaned processes** — `cargo tauri dev` spawns Vite as a grandchild. When cargo dies, Vite survives, holding ports. This is a [known Tauri bug](https://github.com/tauri-apps/tauri/issues/10023) with no upstream fix. See [RES-016](RES-016).
2. **No build visibility** — Rust compilation output, Vite HMR status, and sidecar logs are scattered across processes. Developers (human and agent) can't see what's happening.
3. **Session loss on restart** — Dogfooding means the app you're editing IS the app you're running. Restarting it kills the active session. There's no mechanism to persist and restore context across restarts.

All three are symptoms of not owning the process tree. A dev controller that IS the parent process solves all of them.

## Vision

A single persistent Node process (`debugger/dev.mjs`) that replaces `cargo tauri dev` entirely during development:

```
┌─────────────────────────────────────────────────────┐
│              Dev Controller (Node)                   │
│   Persistent — survives app crashes and restarts     │
│                                                      │
│   ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│   │  [vite]  │  │  [rust]  │  │    [sidecar]     │  │
│   │  HMR     │  │  compile │  │    bun build     │  │
│   │  :1420   │  │  + run   │  │    + spawn       │  │
│   └──────────┘  └──────────┘  └──────────────────┘  │
│                                                      │
│   Unified output stream (colour-coded prefixes)      │
│   Process health + crash detection                   │
│   IPC: make stop / make restart / make status        │
│   Session context capture + injection on restart     │
│   Future: pipe output to OrqaStudio dev panel        │
└─────────────────────────────────────────────────────┘
```

### Key Insight

Instead of `cargo tauri dev` being the parent (making Vite an uncontrollable grandchild), **the controller is the parent of everything**. It spawns Vite and `cargo run` separately, owns their stdio, monitors their health, and can kill/restart any child independently.

This sidesteps the entire Tauri CLI orphan problem — we don't use `cargo tauri dev` at all.

### Capabilities

| Capability | What It Does |
|-----------|-------------|
| **Unified output** | Streams stdout/stderr from all children with colour-coded prefixes: `[vite]`, `[rust]`, `[sidecar]` |
| **Process ownership** | Controller is the parent — killing it kills everything. No orphans. |
| **Independent restart** | Restart only Rust (after backend changes) without touching Vite. Restart only sidecar without touching either. |
| **Health monitoring** | Detect crashes, port conflicts, compile failures. Report status. Auto-restart on crash (configurable). |
| **IPC control** | `make stop` / `make restart` signal the running controller via a lockfile, named pipe, or HTTP. Controller stays alive during restart. |
| **Session persistence** | Before killing the app for restart, capture session context (conversation state, active artifact, panel layout) and inject it back after relaunch. |
| **Hot reload awareness** | Frontend changes → Vite HMR (no restart needed). Rust changes → controller rebuilds and restarts only the app binary. Sidecar changes → rebuild sidecar, restart sidecar process. |
| **Dev panel integration** | Future: pipe controller output into an OrqaStudio panel so the app shows its own build status, compilation errors, and process health. |

### Session Persistence (Dogfood Game-Changer)

The controller can solve the "restart kills my session" problem:

1. **Before restart**: Controller asks the running app for its state via IPC (or reads from SQLite/temp file)
   - Active conversation ID
   - Active artifact path
   - Panel layout state
   - Unsaved input text
2. **Kill and rebuild**: Controller stops the app binary, recompiles, relaunches
3. **After restart**: Controller injects the captured state into the new app instance
   - App opens to the same conversation
   - Same artifact selected
   - Same panel layout
   - Draft text restored

This means `make restart` becomes seamless — the app comes back exactly where it was.

### Commands

| Command | Behaviour |
|---------|-----------|
| `make dev` | Starts the controller. Controller spawns Vite, compiles Rust, launches app. Output streams to terminal. |
| `make stop` | Signals the controller to kill all children and exit. |
| `make restart` | Signals the controller to kill the app + recompile + relaunch. Controller stays alive. Vite stays alive. |
| `make restart-all` | Kill everything including Vite, rebuild all, relaunch. |
| `make status` | Query the controller for process status (PIDs, ports, health). |

## Interim Solution (Shipped)

`debugger/dev.mjs` currently provides reliable `stop` and `restart` using Node's `process.kill()` and port verification. This works today and can evolve into the full controller. See [RES-016](RES-016) for the research that informed this approach.

## Research Needed

1. **Bypassing `cargo tauri dev`** — Can we run `npm run dev` (Vite) and `cargo run --manifest-path backend/src-tauri/Cargo.toml` independently and have the Tauri app connect to the existing Vite server? What Tauri config is needed?
2. **IPC mechanism** — Named pipes, Unix domain sockets, HTTP on a local port, or a simple lockfile+signal approach? Must work on Windows + macOS + Linux.
3. **Session state capture** — What state needs persisting? How to extract it from the running app before kill? Tauri IPC? SQLite read? Temp file?
4. **Dev panel integration** — How to pipe controller output into the app's UI. WebSocket? Tauri event? File tail?
5. **Hot reload granularity** — Can we restart just the Rust binary without rebuilding (if only runtime config changed)? Can we detect which changes need a full recompile vs. incremental?
