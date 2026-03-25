---
id: "RULE-aff3c5db"
type: rule
title: "Rebuild After Changes"
description: "After any commit touching compiled Rust source, binaries must be rebuilt and the daemon restarted immediately. Never leave stale binaries running."
status: "active"
created: "2026-03-24"
updated: "2026-03-24"
enforcement:
  - mechanism: behavioral
    message: "After committing Rust source changes, stop the daemon, rebuild affected binaries, and restart. Stale binaries waste tokens by serving outdated validation, graph data, and diagnostics."
  - mechanism: hook
    type: PostToolUse
    event: bash
    action: warn
    pattern: "git commit"
    message: "If this commit includes Rust source files (libs/validation/, libs/mcp-server/, libs/lsp-server/, app/backend/), rebuild binaries and restart the daemon before continuing."
relationships:
  - target: "RULE-d2c2063a"
    type: "extends"
  - target: "RULE-998da8ea"
    type: "extends"
  - target: "RULE-f609242f"
    type: "extends"
---

After any commit that touches compiled Rust source code, the affected binaries MUST be rebuilt and the daemon restarted immediately. Stale binaries serve outdated results — validation errors that no longer exist, graph data that doesn't reflect recent changes, and diagnostics based on old code. This wastes significant tokens because agents work against broken assumptions.

## What Requires a Rebuild

| Source Path | Binary Affected | Rebuild Command |
|-------------|----------------|-----------------|
| `libs/validation/` | Validation engine (daemon) | `cargo build -p orqa-validation` |
| `libs/mcp-server/` | MCP server (daemon) | `cargo build -p orqa-mcp-server` |
| `libs/lsp-server/` | LSP server | `cargo build -p orqa-lsp-server` |
| `app/backend/` | Tauri app backend | `make restart-tauri` |

If multiple paths are affected in a single commit, rebuild all affected binaries in one pass.

## The Rebuild Sequence (MANDATORY)

After committing Rust source changes:

```
1. Stop the daemon         → orqa daemon stop (or make stop)
2. Rebuild affected binary → cargo build -p <crate>
3. Restart the daemon      → orqa daemon start (or make dev)
4. Verify                  → orqa daemon status
```

**The rebuild is part of the commit workflow, not a follow-up task.** A commit that changes Rust source without rebuilding leaves the running environment in an inconsistent state.

## Watch Mode Exception

If the dev environment is running in watch mode (e.g., `cargo watch` or an equivalent file-watching rebuild), rebuilds happen automatically on file save. In this case:

- Manual rebuild is not required after each commit
- The agent MUST still verify the rebuild completed successfully before continuing work that depends on the changed code
- If watch mode is not active, the manual rebuild sequence applies

## When This Applies

- Any `git commit` that includes files matching `libs/validation/**/*.rs`, `libs/mcp-server/**/*.rs`, `libs/lsp-server/**/*.rs`, or `app/backend/**/*.rs`
- Any `git commit` that includes `Cargo.toml` changes affecting these crates
- After `git merge` or `git rebase` that brings in Rust source changes

## When This Does NOT Apply

- Commits that only touch frontend files (`ui/`, `.svelte`, `.ts`, `.css`)
- Commits that only touch governance artifacts (`.orqa/`)
- Commits that only touch documentation or configuration
- Commits that only touch TypeScript/JavaScript tooling (`libs/cli/`, `connectors/`)

## Why Stale Binaries Are Dangerous

In a dogfooding environment, agents rely on daemon services for real-time feedback:

| Service | What Goes Stale | Impact |
|---------|----------------|--------|
| **Validation engine** | Schema validation, frontmatter checks | Agents see errors for already-fixed issues, or miss new violations |
| **MCP server** | Graph queries, artifact resolution | Agents get outdated graph state, make decisions on wrong data |
| **LSP server** | Diagnostics, completions | Editor shows phantom errors or misses real ones |

Each stale response costs tokens — the agent reads it, reasons about it, and potentially acts on false information. A 30-second rebuild prevents minutes of wasted agent work.

## Enforcement

Two enforcement layers:

1. **PostToolUse hook on Bash** — When a `git commit` command is detected, the hook checks if any staged files match Rust source paths. If so, it emits a warning reminding the agent to rebuild and restart the daemon before continuing.

2. **Agent behavioral constraint** — Agents committing Rust changes must include the rebuild step in their commit workflow. Completion reports for tasks that modify Rust source must include evidence that binaries were rebuilt (e.g., build output or daemon status check).

## FORBIDDEN

- Committing Rust source changes and continuing to use the daemon without rebuilding
- Assuming the daemon will "pick up" changes without a restart (it won't — compiled binaries don't hot-reload)
- Skipping the rebuild because "the changes are small" — any Rust source change requires a rebuild
- Deferring the rebuild to "after the next few commits" — rebuild after EVERY commit that touches Rust source

## Related Rules

- [RULE-d2c2063a](RULE-d2c2063a) (development-commands) — defines the make targets used for rebuilding and restarting
- [RULE-998da8ea](RULE-998da8ea) (dogfood-mode) — stale binaries are especially dangerous in dogfood mode where the running app IS the development environment
- [RULE-f609242f](RULE-f609242f) (git-workflow) — the rebuild step integrates into the commit workflow defined here
