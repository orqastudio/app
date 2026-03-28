---
id: "TASK-d5d62fa5"
type: "task"
title: "Test orqa dev end-to-end"
status: archived
priority: "P1"
created: 2026-03-25T00:00:00.000Z
updated: 2026-03-25T00:00:00.000Z
acceptance:
  - "orqa dev starts all processes (daemon, search, MCP, LSP, Vite, Tauri app)"
  - "All processes confirmed running via orqa dev status"
  - "File watchers active for Rust sources, TS libraries, and plugins"
  - "cargo tauri dev handles app compilation and Vite"
relationships:
  - target: "EPIC-7fd25812"
    type: "delivers"
---

## Done

Tested 2026-03-24. All 6 processes running, all watchers active. Fixed: cargo tauri dev (not npx), log file for controller output, lock contention between watchers resolved.
