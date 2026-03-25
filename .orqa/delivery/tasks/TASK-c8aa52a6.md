---
id: TASK-c8aa52a6
type: task
title: "Test orqa dev end-to-end"
status: completed
priority: P1
created: 2026-03-25
updated: 2026-03-25
acceptance:
  - "orqa dev starts all processes (daemon, search, MCP, LSP, Vite, Tauri app)"
  - "All processes confirmed running via orqa dev status"
  - "File watchers active for Rust sources, TS libraries, and plugins"
  - "cargo tauri dev handles app compilation and Vite"
relationships:
  - target: EPIC-4304bdcc
    type: delivers
  - target: TASK-272b3d07
    type: depended-on-by
    rationale: "Auto-generated inverse of depended-on-by relationship from TASK-272b3d07"
---
## Done

Tested 2026-03-24. All 6 processes running, all watchers active. Fixed: cargo tauri dev (not npx), log file for controller output, lock contention between watchers resolved.
