---
id: TASK-254
title: Move backend to backend/src-tauri/
description: "Relocate src-tauri/ to backend/src-tauri/ and update Makefile, tauri.conf.json, and .gitignore."
status: completed
created: 2026-03-12
updated: 2026-03-12
acceptance:
  - src-tauri/ moved to backend/src-tauri/
  - Makefile CARGO_MANIFEST updated
  - tauri.conf.json frontendDist path updated
  - .gitignore target path updated
  - make lint-backend passes
  - make test-rust passes
relationships:
  - target: EPIC-056
    type: delivers
    rationale: Task belongs to this epic
  - target: TASK-253
    type: depends-on
  - target: TASK-255
    type: depended-on-by
  - target: TASK-346
    type: depended-on-by
---

## What

Move the Rust backend directory and update all config references.

## How

1. `git mv src-tauri backend/src-tauri`
2. Update Makefile: `CARGO_MANIFEST := backend/src-tauri/Cargo.toml`
3. Update `backend/src-tauri/tauri.conf.json`: `frontendDist` from `../build` to `../../build`
4. Update `.gitignore`: `/src-tauri/target/` to `/backend/src-tauri/target/`
5. Verify with `make lint-backend && make test-rust`

## Verification

- [ ] `cargo metadata --manifest-path backend/src-tauri/Cargo.toml` succeeds
- [ ] `make lint-backend` passes
- [ ] `make test-rust` passes
