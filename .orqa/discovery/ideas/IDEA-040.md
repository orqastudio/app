---
id: IDEA-040
title: Identifiable app processes in task manager
description: "Make OrqaStudio's spawned processes (Tauri app, Vite dev server, sidecar, cargo builds) easily identifiable in the OS task manager so developers can quickly find and kill them when needed."
status: captured
created: 2026-03-10
updated: 2026-03-13
horizon: next
research-needed:
  - "Windows process naming — can we rename spawned child processes or set window titles?"
  - "Tauri process tree — which processes does cargo tauri dev spawn, and which can we control?"
  - Vite/Node process identification — custom process title or environment variable marker
  - Sidecar process naming — Bun process identification in task manager
  - "make stop reliability — why do zombie handles persist after taskkill on Windows?"
relationships:
  - target: MS-001
    type: delivers
  - target: PILLAR-002
    type: informed-by
  - target: PILLAR-002
    type: grounded-by
---

## Motivation

During development, OrqaStudio spawns multiple processes (Tauri app, Vite dev server, sidecar, cargo builds) that appear in the Windows task manager as generic `node.exe`, `cargo.exe`, or `orqa-studio.exe` entries. When a process gets stuck (as happened with a zombie Vite process holding port 1420), it is difficult to identify which `node.exe` belongs to OrqaStudio vs. other applications.

Making these processes identifiable would let developers quickly find and kill the right processes without guessing PIDs or grepping netstat output.

## Sketch

Possible approaches:

- **Window titles**: Set descriptive window titles on spawned processes (e.g., "OrqaStudio Vite Dev Server")
- **Process grouping**: Use Windows job objects to group all OrqaStudio child processes under one killable group
- **PID tracking**: `make dev` writes spawned PIDs to a `.dev-pids` file; `make stop` reads and kills them
- **Named markers**: Set environment variables (e.g., `ORQA_PROCESS=vite`) that show up in process details
- **Improved `make stop`**: More aggressive cleanup that handles zombie handles gracefully
