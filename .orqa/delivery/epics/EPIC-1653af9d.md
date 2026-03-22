---
id: EPIC-1653af9d
title: "CLI as consumer — bridge to Rust engine"
description: "Refactor CLI from containing enforcement logic to being a thin bridge from command-line/Node to the Rust engine. Remove TS validator fallback. All orqa commands delegate to the daemon/binary. Dev environment commands (build, watch, kill, dev) managed by CLI."
status: captured
priority: P1
relationships:
  - target: EPIC-81c336c1
    type: depends-on
    rationale: "Needs the Rust engine to handle all validation/enforcement"
---
# CLI as Consumer

- Remove `libs/cli/src/validator/` (TS validator fallback) — Rust binary is the engine
- `orqa enforce` calls Rust binary, formats output
- `orqa dev` starts daemon + watchers + Vite + Tauri in watch mode
- `orqa daemon start|stop|status` manages daemon lifecycle
- `orqa build` builds all ecosystem components (including itself)
- `orqa watch` rebuilds on change
- `orqa kill` stops all running instances
- `make install` is the ONLY Makefile target — bootstraps the CLI
- All other commands go through `orqa`
