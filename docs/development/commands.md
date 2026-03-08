---
title: "Development Commands"
description: "Reference for all development commands available via make targets."
category: development
tags: []
created: 2026-03-05
updated: 2026-03-05
---

**Date:** 2026-03-05

Single source of truth for all Makefile targets and the underlying commands they run. Use `make <target>` unless you have a specific reason to use the raw command directly.

---

## Setup

### `make install`

Install all frontend Node.js dependencies.

**Underlying command:**

```bash
npm install
```

**When to use:** After cloning the repository for the first time, or after pulling changes that modify `package.json`.

---

### `make install-sidecar`

Build the Agent SDK sidecar binary.

**Underlying command:**

```bash
bun build sidecar/index.ts --compile --outfile src-tauri/binaries/sidecar
```

**When to use:** After cloning the repository for the first time, after pulling changes to the sidecar source, or when the sidecar binary is missing or stale. Requires Bun 1.0+.

---

## Development

### `make dev`

Run the full Tauri application in development mode without the Rust file watcher. Starts both the Vite frontend dev server and the Tauri backend. Frontend hot-reload (HMR) is active; Rust changes require a manual restart.

**Underlying command:**

```bash
cargo tauri dev --no-watch
```

**When to use:** Primary command for local development. Safe default during dogfooding — agents can edit `.rs` files without triggering app restarts mid-conversation. Restart manually after Rust changes.

---

### `make dev-watch`

Run the full Tauri application with auto-rebuild on Rust file changes. The app window will close, recompile, and reopen whenever a `.rs` file is saved.

**Underlying command:**

```bash
cargo tauri dev
```

**When to use:** When you are not dogfooding (i.e., the app is not being used for its own development) and want automatic Rust rebuilds.

---

### `make dev-frontend`

Run the Vite frontend dev server alone, without the Tauri backend. Opens the app in a browser tab.

**Underlying command:**

```bash
npm run dev
```

**When to use:** When working exclusively on Svelte component layout, styling, or static UI states that do not require live IPC calls. Faster startup than `make dev`.

---

### `make dev-sidecar`

Run the Agent SDK sidecar in development mode with Bun's hot reload.

**Underlying command:**

```bash
bun run --hot sidecar/index.ts
```

**When to use:** When iterating on sidecar logic (streaming pipeline, NDJSON message format, tool output handling) without rebuilding the full binary.

---

## Quality

### `make check`

Run all quality checks in sequence. This is the standard pre-commit gate. All checks must pass before any commit.

**Underlying commands (in order):**

```bash
cargo fmt --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
npm run check
npm run lint
npm run test
```

**When to use:** Before every commit. Also run after pulling changes that touch source files to verify the working tree is clean.

---

### `make fmt`

Auto-format all Rust source files with `rustfmt`.

**Underlying command:**

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml
```

**When to use:** Before committing Rust changes. Run once to apply formatting, then `make fmt-check` to verify.

---

### `make fmt-check`

Check Rust formatting without making changes. Fails if any file would be reformatted.

**Underlying command:**

```bash
cargo fmt --check
```

**When to use:** In `make check` (already included). Run standalone to confirm formatting is clean before pushing.

---

### `make clippy`

Run the Rust linter with all warnings promoted to errors.

**Underlying command:**

```bash
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

**When to use:** After any Rust change. Zero-warning policy is enforced — this command must exit cleanly.

---

### `make lint`

Run ESLint across all frontend TypeScript and Svelte files.

**Underlying command:**

```bash
npm run lint
```

**When to use:** After any TypeScript or Svelte change. Part of `make check`.

---

### `make check-frontend`

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
cargo test --manifest-path src-tauri/Cargo.toml
npm run test
```

**When to use:** Before committing. Also part of `make check`.

---

### `make test-rust`

Run only the Rust backend tests.

**Underlying command:**

```bash
cargo test --manifest-path src-tauri/Cargo.toml
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

**When to use:** When preparing a release artifact. Produces a platform-appropriate installer or executable in `src-tauri/target/release/`.

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
bun build sidecar/index.ts --compile --outfile src-tauri/binaries/sidecar
```

**When to use:** Before `make build` if sidecar source has changed, or to update the sidecar binary independently of a full release build.

---

## Documentation

### `make docs`

Serve the project documentation locally via Docsify.

**Underlying command:**

```bash
npx docsify serve docs/
```

**When to use:** When reading or reviewing project documentation. Opens a local server (default: `http://localhost:3000`) with the rendered docs site.

---

## Code Search

### `make index`

Build or update the ChunkHound code search index.

**Underlying command:**

```bash
uvx chunkhound index
```

**When to use:** After adding new files or making significant structural changes that should be discoverable via semantic search. Run on first setup to build the initial index.

---

### `make reindex`

Force a full rebuild of the ChunkHound code search index, discarding the existing index.

**Underlying command:**

```bash
uvx chunkhound index --force
```

**When to use:** When the index appears stale, after large refactors that move many files, or when search results seem incomplete.

---

### `make calibrate`

Calibrate the ChunkHound similarity thresholds for the current codebase.

**Underlying command:**

```bash
uvx chunkhound calibrate
```

**When to use:** After significant codebase growth or after a reindex, if semantic search results feel too broad or too narrow.

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
|---------|----------|
| `make check` | `cargo clippy` or `npm run lint` separately |
| `make test` | `cargo test` alone |
| `make fmt` | `rustfmt src/main.rs` |
| `make build` | `cargo build --release` |
| `make test-rust` | `cargo test --manifest-path src-tauri/Cargo.toml` |

The only exception is when a target does not yet exist for a specific operation. In that case, use the raw command and note in the task summary that a Makefile target should be added.

---

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Learning Through Reflection | N/A |
| Clarity Through Structure | Standardizes development commands into a single discoverable interface, reducing process friction and ensuring consistent quality checks across all contributors and agents. |

---

## Related Documents

- [Getting Started](getting-started.md) — Prerequisites and project setup
- [Coding Standards](coding-standards.md) — Code quality rules and patterns
- [Agentic Workflow](/process/workflow) — Task lifecycle and agent coordination
