---
id: TASK-04d5e6f7
type: task
name: "Set up Cargo workspace"
status: completed
description: "Create root Cargo.toml workspace with all 4 Rust crates + the Tauri app as members. Shared build cache, unified cargo check/clippy/test."
relationships:
  - target: EPIC-f2b9e7d3
    type: delivers
    rationale: Phase 1 — monorepo consolidation
  - target: TASK-02b3c4d5
    type: depends-on
    rationale: Repos must be imported first
acceptance:
  - "Root Cargo.toml defines workspace with all Rust crates"
  - "cargo check at root compiles all crates with shared build cache"
  - "cargo clippy at root lints all crates"
  - "cargo test at root runs all crate tests"
  - "Tauri app builds successfully as workspace member"
  - "Path dependencies use workspace-relative paths"
---

## Scope

### Root Cargo.toml

```toml
[workspace]
members = [
  "libs/validation",
  "libs/search",
  "libs/mcp-server",
  "libs/lsp-server",
  "app/backend/src-tauri",
]
resolver = "2"
```

### Update path dependencies

Each crate's `Cargo.toml` currently uses relative paths like `path = "../../../libs/search"`. These need updating to workspace-relative paths since the directory structure may shift.

After the monorepo merge, verify all path deps still resolve correctly. The paths should remain the same since repos are imported at their existing paths.

### Shared build cache

One `target/` directory at workspace root. Currently each crate has its own `target/`. This eliminates redundant compilation — shared dependencies compile once.

### Makefile updates

`make lint-backend`, `make test-rust`, `make format` should use workspace-level commands:
- `cargo clippy --workspace -- -D warnings`
- `cargo test --workspace`
- `cargo fmt --all`
