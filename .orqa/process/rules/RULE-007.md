---
id: RULE-007
title: Development Commands
description: All development commands must be invoked via make targets. Raw cargo and npm commands are forbidden.
status: active
created: 2026-03-07
updated: 2026-03-07
layer: project
enforcement:
  - "event: bash"
  - "event: bash"
  - "event: bash"
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Make targets provide a single structured interface for development commands
  - target: RULE-006
    type: informs
    rationale: Make targets are the required way to invoke coding standard checks (make check)
  - target: RULE-029
    type: informs
    rationale: Make targets are the required way to run all test variants
  - target: RULE-013
    type: informs
    rationale: Git commands remain raw — make does not wrap version control operations
  - target: EPIC-037
    type: informs
  - target: RULE-009
    type: informed-by
  - target: RULE-013
    type: informed-by
  - target: RULE-025
    type: informed-by
  - target: RULE-039
    type: informed-by
---
All development commands MUST be invoked via `make` targets. Raw `cargo` and `npm run` commands are forbidden for tasks that have a `make` equivalent.

## Command Mapping

| Action | Use This | NOT This |
|--------|----------|----------|
| Start dev environment | `make dev` | `cargo tauri dev` |
| Start controller (foreground) | `make start` | running dev.mjs directly |
| Stop gracefully | `make stop` | `taskkill` / manual process hunting |
| Force kill everything | `make kill` | `taskkill` / manual process hunting |
| Restart Tauri app | `make restart-tauri` | killing processes then `make dev` |
| Restart Vite | `make restart-vite` | killing Vite manually |
| Restart Vite + Tauri | `make restart` | killing everything then `make dev` |
| Run all checks | `make check` | `cargo clippy && npm run check && ...` |
| Format Rust | `make format` | `cargo fmt` |
| Check formatting | `make format-check` | `cargo fmt --check` |
| Run all linters | `make lint` | `cargo clippy && npm run lint` |
| Run Rust linter | `make lint-backend` | `cargo clippy -- -D warnings` |
| Run ESLint | `make lint-frontend` | `npm run lint` |
| Run all tests | `make test` | `cargo test && npm run test` |
| Run Rust tests | `make test-rust` | `cargo test` |
| Run frontend tests | `make test-frontend` | `npm run test` |
| Run frontend type checks | `make typecheck` | `npm run check` |
| Build production | `make build` | `cargo tauri build` |
| Install deps | `make install` | `npm install && cargo fetch` |
| Index code search | `make index` | `uvx chunkhound index` |

## Why

- Single source of truth for how commands are invoked
- Ensures `--manifest-path` and other flags are always correct
- Consistent across agents, humans, and CI
- Documented in `.orqa/documentation/development/commands.md`

## Dev Server (NON-NEGOTIABLE)

The dev environment must be running during any session that modifies code (Rust, Svelte, TypeScript, CSS). This provides:

- **Frontend**: Vite HMR — instant reload, window stays open
- **Rust**: Changes require manual restart (see below)

**`make dev` vs `make start`:** `make dev` spawns the controller as a detached process, waits for everything to be ready, then exits cleanly. `make start` runs the controller in the foreground. Agents should use `make dev` to start and `make kill` to tear down. `make start` is for humans who want the controller in their terminal.

**Dogfooding context:** OrqaStudio is developed using itself. The controller uses `--no-watch` so that editing `.rs` files does not kill the running app mid-conversation. Vite HMR still works for frontend changes.

### Agent Restart Behaviour (NON-NEGOTIABLE)

**During development or dogfooding, agents MUST ONLY use the restart commands** (`make restart-tauri`, `make restart-vite`, `make restart`) to manage the dev environment. Agents MUST NOT use `make dev`, `make start`, `make stop`, or `make kill` unless the user explicitly asks them to start or stop the controller.

The assumption is that the dev environment is already running. If an agent needs to apply changes:

- **Rust changes**: `make restart-tauri` — recompiles and relaunches the Tauri app, Vite stays alive
- **Vite stuck**: `make restart-vite` — restarts the Vite dev server only
- **Both need restart**: `make restart` — restarts Vite + Tauri, controller stays alive

If the controller is not running, `make restart-tauri` and `make restart` will automatically start it.

**Rules:**

1. Only sessions that are purely docs/planning are exempt from needing the dev environment
2. After Rust changes: commit all work, then **offer to run `make restart-tauri`**. The controller stays alive, Vite stays alive — only the Tauri app binary is killed, recompiled, and relaunched.
3. **NEVER run `make dev-watch`** — it causes the app to restart on every Rust file save
4. **The orchestrator manages its own dev lifecycle.** Do not expect the user to run restart commands. Offer to run them and execute on approval.

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
2. Document it in `.orqa/documentation/development/commands.md`
3. Update this rule's command mapping table
4. Only then start using the command

## Related Rules

- [RULE-006](RULE-006) (coding-standards) — references `make check` for pre-commit verification
- [RULE-029](RULE-029) (testing-standards) — references `make test` variants for running tests
- [RULE-013](RULE-013) (git-workflow) — git commands remain raw (no make wrapper needed)
