---
id: TASK-254
title: "Move backend to backend/src-tauri/"
description: "Relocate src-tauri/ to backend/src-tauri/ and update Makefile, tauri.conf.json, and .gitignore."
status: done
created: "2026-03-12"
updated: "2026-03-12"
epic: EPIC-056
depends-on:
  - TASK-253
acceptance:
  - "src-tauri/ moved to backend/src-tauri/"
  - "Makefile CARGO_MANIFEST updated"
  - "tauri.conf.json frontendDist path updated"
  - ".gitignore target path updated"
  - "make lint-backend passes"
  - "make test-rust passes"
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
