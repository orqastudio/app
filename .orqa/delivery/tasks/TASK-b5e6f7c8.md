---
id: TASK-b5e6f7c8
title: "Demote dev controller to debug-only tooling"
type: task
description: "Move dev.mjs from the primary development workflow to a debug-only tool. orqa dev becomes the primary entry point for starting the development environment. make dev calls orqa dev."
status: completed
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - dev.mjs is moved to tools/debug-controller.mjs (or similar debug-only location)
  - orqa dev is the primary command for starting the development environment
  - make dev delegates to orqa dev (make is bootstrapping only)
  - Debug controller retains verbose logging, inspector ports, and diagnostic features
  - Existing make restart-tauri, make restart-vite continue to work (via orqa commands)
  - make check passes
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Phase 5 of port allocation epic"
  - target: TASK-93c4f5a6
    type: depends-on
    rationale: "Needs CLI process lifecycle commands as replacement"
---

## What

The dev controller (`dev.mjs`) currently manages all service processes during development. With CLI process lifecycle commands in place, the dev controller is demoted to a debug-only tool that provides extra diagnostic features (verbose logging, Node inspector ports, process trace output).

## Changes

1. Move `dev.mjs` to `tools/debug-controller.mjs`
2. Update `Makefile`: `make dev` calls `orqa dev`
3. Update `make restart-tauri` to use `orqa daemon restart` (or equivalent)
4. Update `make restart-vite` to use `orqa vite restart` (or equivalent)
5. Update RULE-c71f1c3f (development-commands) to reflect new command mapping
6. Keep debug controller functional for troubleshooting — it's not deleted, just demoted

## Verification

1. `make dev` works and starts all services via `orqa dev`
2. `make restart-tauri` works via CLI
3. Debug controller still works when invoked directly for troubleshooting
