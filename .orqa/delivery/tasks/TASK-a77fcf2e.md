---
id: "TASK-a77fcf2e"
type: "task"
title: "Move sidecar to sidecars/claude-agentsdk-sidecar/"
description: "Relocate sidecar/ to sidecars/claude-agentsdk-sidecar/ and update all references in Makefile and Rust source."
status: "completed"
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "sidecar/ moved to sidecars/claude-agentsdk-sidecar/"
  - "Makefile sidecar targets updated"
  - "sidecar_commands.rs path references updated (5 paths)"
  - "make lint-backend passes"
  - "make test-rust passes"
relationships:
  - target: "EPIC-5adc6d0a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-2a557489"
    type: "depends-on"
---

## What

Move the sidecar directory and update all references.

## How

1. `git mv sidecar sidecars/claude-agentsdk-sidecar`
2. Update Makefile: `cd sidecar` → `cd sidecars/claude-agentsdk-sidecar` (3 changes)
3. Update `src-tauri/src/commands/sidecar_commands.rs` (5 path references)
4. Verify with `make lint-backend && make test-rust`

## Verification

- [ ] `ls sidecars/claude-agentsdk-sidecar/dist/sidecar.js` succeeds after build
- [ ] `make lint-backend` passes
- [ ] `make test-rust` passes