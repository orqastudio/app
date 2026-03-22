---
id: TASK-0e401567
title: Move sidecar to sidecars/claude-agentsdk-sidecar/
description: Relocate sidecar/ to sidecars/claude-agentsdk-sidecar/ and update all references in Makefile and Rust source.
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - sidecar/ moved to sidecars/claude-agentsdk-sidecar/
  - Makefile sidecar targets updated
  - sidecar_commands.rs path references updated (5 paths)
  - make lint-backend passes
  - make test-rust passes
relationships:
  - target: EPIC-7b039d05
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-0fca4194
    type: depends-on
  - target: TASK-e5c3ae15
    type: depended-on-by
  - target: TASK-03741ecb
    type: depended-on-by
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
