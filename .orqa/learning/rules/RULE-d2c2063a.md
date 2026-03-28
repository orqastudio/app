---
id: "RULE-d2c2063a"
type: rule
title: "Development Commands"
description: "Dev environment startup must use orqa CLI. Raw cargo test/clippy/fmt/build are allowed."
status: active
enforcement_type: mechanical
created: "2026-03-07"
updated: "2026-03-25"
enforcement:

  - mechanism: behavioral

    message: "Use 'orqa debug' to start the dev environment, not raw 'cargo tauri dev' or 'npm run dev'. Raw cargo test/clippy/fmt/build and npx commands are allowed for quality checks."

  - mechanism: hook

    type: PreToolUse
    event: bash
    action: block
    pattern: "cargo tauri dev|npm run dev"
summary: "Dev environment commands via orqa CLI — raw 'cargo tauri dev' and 'npm run dev' forbidden (use 'orqa debug' instead). Raw cargo test/clippy/fmt/build are allowed since the orqa CLI wraps them and agents use them for quality checks. Agents must only use restart commands during dev, never orqa debug/stop/kill unless user asks. Dev server required for any code-modifying session. Exceptions: cargo add, npm install, git, npx."
tier: stage-triggered
roles: [orchestrator, implementer]
priority: P1
tags: [development-commands, make-targets, dev-server, restart-protocol]
relationships:

  - target: "AD-e8a0f910"

    type: "enforces"
---
The dev environment MUST be started via `orqa debug`, not raw `cargo tauri dev` or `npm run dev`. These are the only commands blocked by the enforcement hook.

Raw `cargo test`, `cargo clippy`, `cargo fmt`, `cargo build`, `npx tsc`, `npx vitest`, and `npm run test` are **allowed** -- agents use them for quality checks and the `orqa` CLI wraps them internally.

## Command Mapping

| Action | Use This | Also Allowed |
| --- | --- | --- |
| Start dev environment | `orqa debug` | NOT `cargo tauri dev` or `npm run dev` |
| Stop gracefully | `orqa debug stop` | |
| Force kill everything | `orqa debug kill` | |
| Restart Tauri app | `orqa debug restart` | |
| Run all checks | `orqa check` | `cargo clippy`, `cargo fmt --check`, `npx tsc` |
| Run all linters | `orqa check lint` | `cargo clippy -- -D warnings` |
| Run all tests | `orqa test` | `cargo test`, `npx vitest run` |
| Run Rust tests | `orqa test rust` | `cargo test -p <crate>` |
| Run frontend tests | `orqa test app` | `npx vitest run` |
| Build production | `orqa build` | `cargo build` |
| Install deps | `make install` | `npm install`, `cargo add` |
| Index code search | `orqa mcp index` | |

## Why

- `orqa debug` manages the full dev environment lifecycle (Vite + Tauri + controller)
- Raw `cargo tauri dev` bypasses the controller and can cause port conflicts
- Raw `npm run dev` starts Vite without the controller orchestration
- Other raw commands are fine because they are stateless and idempotent

## Dev Server (NON-NEGOTIABLE)

The dev environment must be running during any session that modifies code (Rust, Svelte, TypeScript, CSS). This provides:

- **Frontend**: Vite HMR — instant reload, window stays open
- **Rust**: Changes require manual restart (see below)

**Dogfooding context:** OrqaStudio is developed using itself. The controller uses `--no-watch` so that editing `.rs` files does not kill the running app mid-conversation. Vite HMR still works for frontend changes.

### Agent Restart Behaviour (NON-NEGOTIABLE)

**During development or dogfooding, agents MUST ONLY use the restart commands** (`orqa debug restart`) to manage the dev environment. Agents MUST NOT use `orqa debug` (start) or `orqa debug stop/kill` unless the user explicitly asks.

The assumption is that the dev environment is already running. If an agent needs to apply changes:

- **Rust changes**: `orqa debug restart` — recompiles and relaunches the Tauri app

**Rules:**

1. Only sessions that are purely docs/planning are exempt from needing the dev environment
2. After Rust changes: commit all work, then run `orqa debug restart`
3. **The orchestrator manages its own dev lifecycle.** Do not expect the user to run restart commands.

## Exceptions

These raw commands are allowed:

- `cargo test`, `cargo clippy`, `cargo fmt`, `cargo build` — build/quality commands
- `npx tsc`, `npx vitest`, `npx eslint` — frontend quality commands
- `cargo add <crate>` — adding new dependencies
- `npm install <package>` — adding new packages
- `git` commands — version control operations
- `bun add <package>` — adding sidecar dependencies

## Forward Compatibility

When adding a new recurring command to the project:

1. Add an `orqa` CLI subcommand if it involves orchestration
2. Update this rule's command mapping table
3. Only then start using the command

## Related Rules

- [RULE-f609242f](RULE-f609242f) (git-workflow) — git commands remain raw (no wrapper needed)
