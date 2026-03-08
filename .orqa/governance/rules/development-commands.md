---
id: development-commands
title: "Development Commands"
description: "All development commands must be invoked via make targets. Raw cargo and npm commands are forbidden."
---


All development commands MUST be invoked via `make` targets. Raw `cargo` and `npm run` commands are forbidden for tasks that have a `make` equivalent.

## Command Mapping

| Action | Use This | NOT This |
|--------|----------|----------|
| Run app | `make dev` | `cargo tauri dev` |
| Stop app | `make stop` | `taskkill` / manual process hunting |
| Restart app | `make restart` | killing processes then `make dev` |
| Run all checks | `make check` | `cargo clippy && npm run check && ...` |
| Format Rust | `make fmt` | `cargo fmt` |
| Check formatting | `make fmt-check` | `cargo fmt --check` |
| Run Rust linter | `make clippy` | `cargo clippy -- -D warnings` |
| Run all tests | `make test` | `cargo test && npm run test` |
| Run Rust tests | `make test-rust` | `cargo test` |
| Run frontend tests | `make test-frontend` | `npm run test` |
| Run frontend checks | `make check-frontend` | `npm run check` |
| Run ESLint | `make lint` | `npm run lint` |
| Build production | `make build` | `cargo tauri build` |
| Install deps | `make install` | `npm install && cargo fetch` |
| Index code search | `make index` | `uvx chunkhound index` |

## Why

- Single source of truth for how commands are invoked
- Ensures `--manifest-path` and other flags are always correct
- Consistent across agents, humans, and CI
- Documented in `docs/development/commands.md`

## Dev Server (NON-NEGOTIABLE)

Any session that modifies code (Rust, Svelte, TypeScript, CSS) MUST have `make dev` running as a background task. This provides:

- **Frontend**: Vite HMR — instant reload, window stays open
- **Rust**: Changes require manual restart (see below)

**Dogfooding context:** OrqaStudio is developed using itself. The app you are running inside IS the codebase you are editing. `make dev` uses `--no-watch` so that editing `.rs` files does not kill the running app mid-conversation. Vite HMR still works for frontend changes.

After Rust backend changes, use `make restart` to cleanly stop all processes and restart. This kills the Tauri app, Vite dev server, and any cargo builds, waits for ports to release, then starts fresh. **NEVER use `make dev-watch`** — it causes the app to restart on every Rust file save, which destroys the active session.

**Rules:**

1. Start `make dev` as a background task at the beginning of any implementation session
2. After completing changes, verify the dev server is still running and the app is open
3. If `make dev` dies during work (compile error, crash), fix the issue and restart it
4. Only sessions that are purely docs/planning are exempt
5. After Rust changes: write session state, commit all work, then **offer to run `make restart`**. `make restart` is atomic — it stops, rebuilds, and relaunches in one command. The session ends when the app restarts; the next session resumes from `tmp/session-state.md`.
6. **NEVER run `make dev-watch`** — it is for human use only, outside of dogfooding
7. **NEVER break restart into multiple steps** (e.g., `make stop` then `make dev`). The app closes on stop, killing the session mid-sequence. Always use `make restart` as one atomic operation.
8. **The orchestrator manages its own dev lifecycle.** Do not expect the user to run `make dev`, `make restart`, or `make stop`. Offer to run them and execute on approval.

## Exceptions

These raw commands are still allowed because they have no `make` equivalent:

- `cargo add <crate>` — adding new dependencies
- `npm install <package>` — adding new packages
- `git` commands — version control operations
- `npx` one-off commands not covered by make targets
- `bun add <package>` — adding sidecar dependencies

## Forward Compatibility

When adding a new recurring command to the project:
1. Add a `make` target first
2. Document it in `docs/development/commands.md`
3. Update this rule's command mapping table
4. Only then start using the command

## Related Rules

- `coding-standards.md` — references `make check` for pre-commit verification
- `testing-standards.md` — references `make test` variants for running tests
- `git-workflow.md` — git commands remain raw (no make wrapper needed)
