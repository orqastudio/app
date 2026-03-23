---
id: EPIC-fba4debd
type: epic
title: "System tray for daemon lifecycle"
description: "A system tray application for managing the OrqaStudio daemon. Shows daemon status, launches the app and debugger in dev mode. Lightweight — runs independently of the app."
status: captured
priority: P1
relationships:
  - target: EPIC-81c336c1
    type: depends-on
    rationale: "Requires the daemon to be running"
---
# System Tray for Daemon Lifecycle

## Problem

The daemon runs as a headless process. Users have no visual indicator of its status and must use CLI commands (`orqa daemon status/start/stop`) to manage it. There's no quick way to launch the app or debugger in dev mode without opening a terminal.

## Target State

A lightweight system tray application that:

1. **Shows daemon status** — icon changes based on running/stopped/error
2. **Start/stop daemon** — right-click menu to start or stop
3. **Launch app in dev mode** — opens OrqaStudio app (Vite + Tauri)
4. **Launch debugger in dev mode** — opens the debug dashboard tool
5. **Show daemon logs** — quick access to recent log output
6. **Auto-start option** — optionally start daemon on login

## Implementation Options

### Option A: Tauri tray app (Rust + minimal UI)
- Small standalone Tauri binary with system tray only (no window)
- Uses Tauri's built-in tray API
- Calls `orqa daemon start/stop/status` via CLI
- Consistent with the app's tech stack

### Option B: Native tray (Rust only)
- Use `tray-item` or `tao` crate directly
- No web runtime overhead
- Lighter than Tauri but less polished

### Option C: Node tray
- Use `systray2` or `electron-tray` (without full Electron)
- Consistent with CLI toolchain
- But adds Node runtime requirement

## Tasks

1. Choose implementation approach
2. Create tray binary (start/stop daemon, show status)
3. Add "Launch App" menu item (runs `orqa dev` or starts Vite + Tauri)
4. Add "Launch Debugger" menu item (opens debug dashboard)
5. Add status indicator (green=running, red=stopped, yellow=starting)
6. Add log viewer (last N lines from dashboard)
7. Package for Windows/macOS/Linux
8. Wire into `orqa dev` startup (tray appears automatically)

## Notes

- Production mode (bundling daemon into app) is out of scope for now
- The tray should work even without the app installed (CLI + daemon only)
- Dev mode is the primary use case