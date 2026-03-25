---
id: KNOW-e89753ad
type: knowledge
title: OrqaStudio CLI Commands
description: "Complete reference for the orqa CLI: every command, its subcommands, and when to use each. Agents must use orqa commands instead of raw cargo/npm/make."
status: active
created: 2026-03-24
updated: 2026-03-24
category: tooling
version: 1
user-invocable: true
relationships:
  - target: DOC-e89753ad
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
---

# OrqaStudio CLI Commands

The `orqa` CLI is the single developer interface. All development commands go through it. There are no separate MCP servers, LSP servers, or build scripts to manage independently.

## Command Reference

### orqa install — Dev Environment Setup

```
orqa install              Full setup (prereqs + deps + build + plugin sync + verify)
orqa install prereqs      Check prerequisites (node 22+, rust, git)
orqa install deps         Install npm workspace + cargo dependencies
orqa install build        Build all libs in dependency order
orqa install publish      Publish libs to package registry
```

**When to use:** First-time setup, after cloning, after dependency changes.

### orqa dev — Dev Environment Management

```
orqa dev                  Start full dev environment (Vite + Tauri)
orqa dev stop             Stop gracefully
orqa dev kill             Force-kill all processes
orqa dev restart          Restart Vite + Tauri (not the controller)
orqa dev restart-tauri    Restart Tauri only (after Rust changes)
orqa dev restart-vite     Restart Vite only
orqa dev status           Show process status
orqa dev icons            Generate brand icons from SVG sources
```

**When to use:** Starting and managing the dev environment. Use `orqa dev restart-tauri` after Rust backend changes. Use `orqa dev restart-vite` if Vite gets stuck.

### orqa check — Code Quality

```
orqa check               Run all quality checks from installed plugins
orqa check <tool>        Run a specific tool (eslint, clippy, svelte-check)
orqa check configure     Generate linter config files from coding standards rules
```

**When to use:** Before every commit. The pre-commit hook calls this automatically on staged files. Tools are discovered from plugin manifests — no hardcoded tool list.

### orqa test — Test Suites

```
orqa test                Run all test suites
orqa test rust           Rust backend tests (cargo test)
orqa test app            Frontend tests (vitest)
```

**When to use:** After implementation changes, before marking tasks complete.

### orqa enforce — Enforcement + Integrity

```
orqa enforce [path]                   Run ALL checks on all artifacts
orqa enforce --mechanism json-schema  Run specific mechanism only
orqa enforce --rule RULE-xxx          Run all mechanisms for one rule
orqa enforce --file path/to/file.md   Run all mechanisms for one file
orqa enforce --report                 Enforcement coverage report
orqa enforce --fix                    Auto-fix fixable errors
orqa enforce --json                   JSON output
orqa enforce schema                   Validate project.json and plugin manifests
orqa enforce response                 Log enforcement response events
```

**When to use:** Validating artifact integrity, checking enforcement coverage, auto-fixing schema violations.

### orqa audit — Governance Audit

```
orqa audit               Full governance audit (integrity, version, license, readme)
orqa audit --fix         Full audit with auto-fix
orqa audit escalation    Scan lessons for escalation candidates (recurrence >= 3)
orqa audit escalation --create-tasks   Create CRITICAL task artifacts for findings
```

**When to use:** Session-end governance health check. The stop hook runs `orqa audit escalation --create-tasks` automatically.

### orqa graph — Artifact Graph

```
orqa graph                             List all artifacts
orqa graph --type <type>               Filter by type (epic, task, rule, etc.)
orqa graph --status <status>           Filter by status (active, done, etc.)
orqa graph --related-to <id>           Show related artifacts
orqa graph --rel-type <type>           Filter by relationship type
orqa graph --search <query>            Text search in titles
orqa graph --id <id>                   Show details for one artifact
orqa graph --stats                     Aggregate statistics
orqa graph --tree                      Delivery tree (hierarchy view)
orqa graph --json                      JSON output
```

**When to use:** Discovering artifacts, checking relationships, understanding project state. Always query the graph before starting work.

### orqa plugin — Plugin Management

```
orqa plugin list                       List installed plugins
orqa plugin install <owner/repo|path>  Install a plugin
orqa plugin uninstall <name>           Remove a plugin
orqa plugin update [name]              Update one or all plugins
orqa plugin enable <name>              Enable (copy content to .orqa/)
orqa plugin disable <name>             Disable (remove content from .orqa/)
orqa plugin refresh [name]             Re-sync content for enabled plugins
orqa plugin diff [name]                Show content differences
orqa plugin registry                   Browse the plugin registry
orqa plugin create                     Scaffold a new plugin
```

**When to use:** Managing plugin lifecycle. `orqa plugin refresh` after editing plugin source content.

### orqa version — Version Management

```
orqa version show        Show current canonical version
orqa version sync        Sync VERSION file to all package.json, Cargo.toml, orqa-plugin.json
orqa version bump <ver>  Set new version and sync (e.g. 0.2.0-dev)
orqa version check       Check for version drift across packages
```

**When to use:** Before releases, after version bumps. Always use `-dev` suffix for pre-release versions.

### orqa id — Artifact ID Management

```
orqa id generate <TYPE>       Generate a new hex ID (e.g. orqa id generate TASK)
orqa id check                 Scan graph for duplicate IDs
orqa id check --fix           Auto-regenerate duplicate IDs
orqa id migrate <old> <new>   Rename an ID across the entire graph
```

**When to use:** Creating new artifacts, detecting ID collisions, renaming artifacts.

### orqa git — Monorepo Git Operations

```
orqa git status    Show which components have changes
orqa git pr        Create a pull request on the local git server
orqa git sync      Push to all remotes
orqa git audit     Check git infrastructure health
```

**When to use:** Multi-remote push, component-aware status, local PR creation.

### orqa mcp — MCP Server

```
orqa mcp [project-path]   Start MCP server (stdio)
```

Bridges stdin/stdout to the running OrqaStudio app via IPC. Falls back to standalone MCP server crate if the app is not running. Configured in `.mcp.json`.

**When to use:** Automatically spawned by Claude Code. Not invoked manually.

### orqa lsp — LSP Server

```
orqa lsp [project-path]   Start LSP server (stdio)
```

Provides real-time diagnostics for `.orqa/` artifacts: frontmatter schema validation, hex ID format, relationship integrity, status validation. Connects to the app via IPC or spawns standalone.

**When to use:** Automatically spawned by the editor's LSP client. Not invoked manually.

### orqa daemon — Validation Daemon

```
orqa daemon start [--port <port>]   Start daemon (default port: 10258)
orqa daemon stop                    Stop the daemon
orqa daemon status                  Show daemon health
```

Keeps the artifact graph in memory for low-latency calls from hooks, LSP, MCP, and CLI.

**When to use:** Started automatically by the app. Manual start for headless/CI use.

### orqa index — Search Indexing

```
orqa index [project-path]              Download model, index, embed
orqa index --download-only             Download model only
orqa index --skip-download             Use existing model
orqa index --model-dir <path>          Custom model directory
orqa index --db <path>                 Custom DuckDB path
```

Downloads the ONNX embedding model, indexes the codebase, and generates embeddings for semantic search.

**When to use:** First-time setup, after significant codebase changes, to refresh the search index.

### orqa debug — Debug Tool

```
orqa debug [command]    Run the OrqaStudio debug tool
```

Wraps the debug-tool submodule for diagnostic operations.

### orqa hosting — Local Git Server

```
orqa hosting up        Start local git server
orqa hosting down      Stop local git server
orqa hosting setup     Initialize git server configuration
orqa hosting status    Show server status
orqa hosting push      Push to local server
```

Manages the local git hosting server for self-hosted development.

### orqa repo — Repo Maintenance

```
orqa repo license    License audit across all packages
orqa repo readme     README completeness audit
```

**When to use:** Before releases, during governance audits.

## FORBIDDEN

- Using raw `cargo clippy`, `cargo test`, `cargo fmt` — use `orqa check` and `orqa test`
- Using raw `npm run lint`, `npm run check` — use `orqa check`
- Using `make` targets for anything except `make install` (bootstrap only)
- Bypassing the pre-commit hook with `--no-verify`
- Running `orqa dev` when the dev environment is already running — use `orqa dev restart-*` instead
