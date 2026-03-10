---
id: IDEA-042
title: Reliable dev cycle commands (start, stop, restart)
description: The current dev server lifecycle is fragile — port conflicts, orphaned processes, and no Makefile. Agents and humans both struggle to reliably start, stop, and restart the app during development.
status: captured
created: 2026-03-10
updated: 2026-03-10
pillars:
  - PILLAR-001
research-needed:
  - Survey current pain points (port conflicts, orphan processes, taskkill timeouts on Windows)
  - Evaluate cross-platform process management (make vs just vs npm scripts vs custom)
  - Design idempotent start/stop/restart that work from CLI agents and human terminals
promoted-to: null
---

## Motivation

Development workflow repeatedly hits friction from unreliable dev server lifecycle:

- **Port conflicts**: Vite holds port 1420 after the Tauri app closes. `taskkill` often times out on Windows.
- **No Makefile**: [RULE-007](RULE-007) mandates `make` targets but no Makefile exists yet. Agents fall back to raw `cargo tauri dev` which lacks process cleanup.
- **Orphaned processes**: Node/Vite child processes survive parent kills, holding file locks and ports.
- **No restart command**: Restart requires manually killing processes, waiting for ports to release, then relaunching. This is error-prone and breaks agent automation.
- **Background task confusion**: `cargo tauri dev` completing as a background task means the app exited, not that it started successfully. Agents misinterpret this.

This is a foundational DX problem — every implementation session hits it, and it will get worse with dogfooding.

## Sketch

- Create a `Makefile` (or `justfile`) with `dev`, `stop`, `restart`, `check`, `test` targets
- `stop` must reliably kill all child processes (Vite, Node, cargo) and release ports
- `restart` = atomic stop + rebuild + start
- `dev` should be idempotent — if already running, no-op or warn
- Consider a PID file or port-based detection for process state
- Windows-specific: handle `taskkill` timeouts, `TIME_WAIT` sockets, MINGW path issues
