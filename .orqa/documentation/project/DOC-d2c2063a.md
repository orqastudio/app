---
id: DOC-d2c2063a
type: doc
status: active
title: Development Commands
domain: reference
category: reference
description: Reference for all development commands available via make targets.
created: 2026-03-05
updated: 2026-03-24
sort: 4
relationships: []
---

**Date:** 2026-03-05

Single source of truth for all Makefile targets and the underlying commands they run. Use `make <target>` unless you have a specific reason to use the raw command directly.

---

## Setup

### `make install`

Install all project dependencies: frontend Node.js packages, sidecar Bun packages, and Rust crate dependencies.

**Underlying command:**

```bash
npm install
cd sidecar && bun install
cargo fetch --manifest-path app/src-tauri/Cargo.toml
```

**When to use:** After cloning the repository for the first time, or after pulling changes that modify `package.json`, `sidecar/package.json`, or `Cargo.toml`.

---

### `make install-sidecar`

Build the Agent SDK sidecar binary.

**Underlying command:**

```bash
bun build sidecar/index.ts --compile --outfile app/src-tauri/binaries/sidecar
```

**When to use:** After cloning the repository for the first time, after pulling changes to the sidecar source, or when the sidecar binary is missing or stale. Requires Bun 1.0+.

---

## Development

### `make dev`

Start the full dev environment. Spawns the dev controller as a detached background process, polls until Vite and Tauri are both ready, then exits. The controller continues running independently with its own terminal window and web dashboard.

**Underlying command:**

```bash
node tools/debug/dev.mjs dev
```

**What it does:**

1. Spawns the controller as a detached process
2. Polls the control file until state is `running` (Vite + Tauri both up)
3. Exits cleanly — no hanging process

**When to use:** Primary command for starting local development. Partner to `make kill` — use `make dev` to start everything, `make kill` to tear it all down.

---

### `make start`

Start the dev controller in the foreground — a persistent Node process that owns the entire dev lifecycle. Spawns Vite, compiles and runs the Tauri app, opens a web-based dev dashboard, and streams unified output from all processes.

**Underlying command:**

```bash
node tools/debug/dev.mjs start
```

**What it does:**

1. Kills any orphaned processes from previous runs
2. Starts the dashboard server at `http://localhost:10401` and opens it in the browser
3. Spawns Vite dev server, waits for port 10420
4. Compiles and runs the Tauri app via `cargo run`
5. Watches for IPC signals from restart/stop commands
6. Streams all output with colour-coded prefixes: `[ctrl]`, `[vite]`, `[rust]`

**When to use:** When you want the controller in the foreground (e.g., for debugging). Most users should use `make dev` instead.

---

### `make restart-tauri`

Signal the dev controller to kill the Tauri app binary, recompile, and relaunch. Vite and the controller stay alive.

**Underlying command:**

```bash
node tools/debug/dev.mjs restart-tauri
```

**When to use:** After Rust backend changes that require a recompile. The dashboard shows rebuild progress live.

---

### `make restart-vite`

Signal the dev controller to restart only the Vite dev server. The Tauri app stays alive.

**Underlying command:**

```bash
node tools/debug/dev.mjs restart-vite
```

**When to use:** When the Vite dev server gets stuck, HMR stops working, or port 10420 needs to be recycled.

---

### `make restart`

Signal the dev controller to restart everything — kills Vite and the Tauri app, then relaunches both. The controller stays alive.

**Underlying command:**

```bash
node tools/debug/dev.mjs restart
```

**When to use:** When both Vite and Rust need a clean restart.

---

### `make stop`

Gracefully signal the dev controller to stop all processes and exit. The dashboard auto-closes. **After stopping, a manual `make dev` is required to bring the environment back up.** During dogfooding, this means the app goes down and must be relaunched.

**Underlying command:**

```bash
node tools/debug/dev.mjs stop
```

**When to use:** When you intentionally want to shut everything down and don't need it running. Prefer `make restart` commands during active development — they keep the controller alive.

---

### `make kill`

Force-kill all OrqaStudio processes regardless of controller state. Does not wait for graceful shutdown.

**Underlying command:**

```bash
node tools/debug/dev.mjs kill
```

**When to use:** When `make stop` doesn't work, processes are stuck, or you need to tear down the entire dev environment. Partner to `make dev`.

---

### `make status`

Show the dev controller's state: controller PID, child process PIDs, and whether they are alive.

**Underlying command:**

```bash
node tools/debug/dev.mjs status
```

**When to use:** To check what's running without opening the dashboard.

---

### `make dev-frontend`

Run the Vite frontend dev server alone, without the Tauri backend. Opens the app in a browser tab.

**Underlying command:**

```bash
npm run dev
```

**When to use:** When working exclusively on Svelte component layout, styling, or static UI states that do not require live IPC calls. Faster startup than `make dev`.

---

## Quality

### `make check`

Run all quality checks in sequence. This is the standard pre-commit gate. All checks must pass before any commit.

**Underlying commands (in order):**

```bash
cargo fmt --check
cargo clippy --manifest-path app/src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path app/src-tauri/Cargo.toml
npm run check
npm run lint
npm run test
```

**When to use:** Before every commit. Also run after pulling changes that touch source files to verify the working tree is clean.

---

### `make format`

Auto-format all Rust source files with `rustfmt`.

**Underlying command:**

```bash
cargo fmt --manifest-path app/src-tauri/Cargo.toml
```

**When to use:** Before committing Rust changes. Run once to apply formatting, then `make format-check` to verify.

---

### `make format-check`

Check Rust formatting without making changes. Fails if any file would be reformatted.

**Underlying command:**

```bash
cargo fmt --check
```

**When to use:** In `make check` (already included). Run standalone to confirm formatting is clean before pushing.

---

### `make lint`

Run all linters: backend (clippy) and frontend (ESLint).

**Underlying commands:**

```bash
cargo clippy --manifest-path app/src-tauri/Cargo.toml --all-targets -- -D warnings
npm run lint
```

**When to use:** After any code change. Part of `make check`. Runs both `lint-backend` and `lint-frontend`.

---

### `make lint-backend`

Run the Rust linter (clippy) with all warnings promoted to errors.

**Underlying command:**

```bash
cargo clippy --manifest-path app/src-tauri/Cargo.toml --all-targets -- -D warnings
```

**When to use:** After any Rust change. Zero-warning policy is enforced — this command must exit cleanly.

---

### `make lint-frontend`

Run ESLint across all frontend TypeScript and Svelte files.

**Underlying command:**

```bash
npm run lint
```

**When to use:** After any TypeScript or Svelte change. Part of `make lint` and `make check`.

---

### `make typecheck`

Run `svelte-check` and TypeScript type checking for the frontend.

**Underlying command:**

```bash
npm run check
```

**When to use:** After any Svelte component or TypeScript change to catch type errors and Svelte-specific issues before running full `make check`.

---

## Testing

### `make test`

Run all tests: Rust backend tests and frontend Vitest tests.

**Underlying commands:**

```bash
cargo test --manifest-path app/src-tauri/Cargo.toml
npm run test
```

**When to use:** Before committing. Also part of `make check`.

---

### `make test-rust`

Run only the Rust backend tests.

**Underlying command:**

```bash
cargo test --manifest-path app/src-tauri/Cargo.toml
```

**When to use:** When iterating on backend changes and you want fast feedback without running frontend tests.

---

### `make test-frontend`

Run only the frontend Vitest tests.

**Underlying command:**

```bash
npm run test
```

**When to use:** When iterating on Svelte components or stores and you want fast feedback without running Rust tests.

---

### `make coverage-rust`

Run Rust library tests with LLVM source-based code coverage via `cargo-llvm-cov`. Prints a per-file coverage summary (regions, functions, lines) to the terminal. Uses `--lib` to skip the Tauri binary entry point which requires a frontend build to compile.

**Underlying command:**

```bash
cargo llvm-cov --manifest-path app/src-tauri/Cargo.toml --lib
```

**When to use:** To check current Rust backend test coverage. Target is 80% per module. Requires `cargo-llvm-cov` (`cargo install cargo-llvm-cov`) and the `llvm-tools-preview` rustup component (`rustup component add llvm-tools-preview`). A minimal `ui/build/index.html` must exist for Tauri's `generate_context!()` macro to compile.

---

### `make coverage-frontend`

Run frontend Vitest tests with v8 code coverage reporting. Prints a coverage summary to the terminal showing line, function, branch, and statement coverage per file.

**Underlying command:**

```bash
npm run test:coverage
```

**When to use:** To check current frontend test coverage. Target is 80% per module. Thresholds are not yet enforced — the report is informational until coverage reaches the target.

---

### `make test-watch`

Run Vitest in watch mode. Re-runs affected tests on file save.

**Underlying command:**

```bash
npm run test:watch
```

**When to use:** During active frontend development when you want continuous test feedback without manually re-running tests.

---

### `make test-e2e`

Run Playwright end-to-end tests against the running Tauri application.

**Underlying command:**

```bash
npx playwright test
```

**When to use:** After completing a user-facing feature to verify the full user journey works end-to-end. Requires the Tauri application to be running. See `tests/` for test files.

---

## Build

### `make build`

Build a production-ready distributable application for the current platform.

**Underlying command:**

```bash
cargo tauri build
```

**When to use:** When preparing a release artifact. Produces a platform-appropriate installer or executable in `app/src-tauri/target/release/`.

---

### `make build-frontend`

Build only the Svelte/Vite frontend. Does not compile Rust or bundle the Tauri app.

**Underlying command:**

```bash
npm run build
```

**When to use:** To verify the frontend builds cleanly without running the full Tauri build. Faster than `make build` for frontend-only changes.

---

### `make build-sidecar`

Compile the Agent SDK sidecar TypeScript into a standalone binary.

**Underlying command:**

```bash
bun build sidecar/index.ts --compile --outfile app/src-tauri/binaries/sidecar
```

**When to use:** Before `make build` if sidecar source has changed, or to update the sidecar binary independently of a full release build.

---

## Code Search

### `make index`

Build or update the native code search index.

**Underlying command:**

```bash
orqa index
```

**When to use:** After adding new files or making significant structural changes that should be discoverable via semantic search. Run on first setup to build the initial index. Indexing is also triggered automatically by the search engine when files change.

---

### `make reindex`

Force a full rebuild of the native code search index, discarding the existing index.

**Underlying command:**

```bash
orqa index --force
```

**When to use:** When the index appears stale, after large refactors that move many files, or when search results seem incomplete.

---

## Skills

### `make skills-list`

List all currently installed skills with their versions.

**Underlying command:**

```bash
npx skills list
```

**When to use:** To audit which skills are active and verify versions match `skills-lock.json`.

---

### `make skills-update`

Update all skills to their latest compatible versions and refresh `skills-lock.json`.

**Underlying command:**

```bash
npx skills update
```

**When to use:** Periodically to pull in improvements to skills. Review the diff in `skills-lock.json` before committing.

---

## Utilities

### `make clean`

Remove all build artifacts: Rust target directory and frontend build output.

**Underlying commands:**

```bash
cargo clean
rm -rf node_modules/.vite ui/.svelte-kit
```

**When to use:** When debugging mysterious build failures, or to reclaim disk space. After cleaning, `make install` and `make dev` will trigger full recompilation.

---

### `make help`

Print a summary of all available `make` targets with one-line descriptions.

**Underlying command:**

```bash
@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'
```

**When to use:** Quick reference when you cannot remember a target name.

---

## For Agents

`make` targets are the standard interface for all development operations in OrqaStudio™. Agents MUST use `make` targets rather than raw `cargo`, `npm`, or `bun` commands.

**Why:** Makefile targets encode the correct flags, manifest paths, and command sequences for this project. Raw commands omit project-specific flags (e.g., `--manifest-path`, `-D warnings`) and silently produce incomplete results.

| Do this | Not this |
| --------- | ---------- |
| `make check` | `cargo clippy` or `npm run lint` separately |
| `make test` | `cargo test` alone |
| `make format` | `rustfmt src/main.rs` |
| `make build` | `cargo build --release` |
| `make test-rust` | `cargo test --manifest-path app/src-tauri/Cargo.toml` |

The only exception is when a target does not yet exist for a specific operation. In that case, use the raw command and note in the task summary that a Makefile target should be added.

---

## Related Documents

- [Getting Started](getting-started.md) — Prerequisites and project setup
- [Coding Standards](coding-standards.md) — Code quality rules and patterns
- Agentic Workflow — Task lifecycle and agent coordination
