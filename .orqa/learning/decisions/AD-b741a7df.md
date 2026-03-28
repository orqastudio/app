---
id: AD-b741a7df
type: discovery-decision
title: "Connector architecture v2: dev process separation, graph-first enforcement, plugin specialist agents"
status: archived
created: 2026-03-20
updated: 2026-03-24
relationships:
  - target: EPIC-fb1822c2
    type: drives
  - target: AD-a44384d1
    type: evolves-into
    rationale: "Section 1 (Dev Process Separation) superseded — MCP and LSP are now CLI stdio modes, not separate managed processes. Other sections remain active."
---

# AD-061: Connector Architecture v2 — Dev Process Separation, Graph-First Enforcement, Plugin Specialist Agents

## Context

The connector architecture review (2026-03-20) audited how OrqaStudio's MCP server, native ONNX search engine, and LSP server map to Claude Code's plugin system. The audit found:

- OrqaStudio's MCP server already exposes 10 tools (`graph_query`, `search_semantic`, `search_regex`, `code_research`, etc.) but these are unreachable from Claude Code because the server is not running as an independent managed process.
- The ONNX-based native search engine exists in Rust but has no initialization path in the dev controller — it starts lazily inside the app process, not as a standalone service Claude Code can reach.
- The LSP server already validates artifact frontmatter and relationship integrity — but hook scripts (`validate-artifact.mjs`) duplicate this work using naive file-system parsing instead of calling the LSP.
- 50% of knowledge files are unreachable: the `INTENT_MAP` in the hooks covers only a subset of the 26+ knowledge files available in installed plugins.
- ChunkHound references remain in CLAUDE.md, rules, and hook scripts. ChunkHound is an external Python tool that was superseded by the native ONNX search implementation. All references must be removed.
- Hook execution produces no telemetry — there is no visibility into what hooks ran, what they checked, or what they skipped.
- No bash safety hook exists to intercept dangerous command patterns (`--no-verify`, `rm -rf`, `sudo`, force push).

## Decision

### 1. Dev Process Separation

Run the MCP server, ONNX search engine, and LSP server as independent managed processes launched by the dev controller. These processes are not app-internal — they are dev infrastructure that Claude Code (and other external tools) can reach at stable addresses.

- MCP server: started by dev controller at install time, registered in `.mcp.json` so Claude Code discovers it automatically
- ONNX search engine: initialized as a controller-managed process, not lazily inside the app
- LSP server: started by dev controller, advertised to editors and hook scripts via a well-known socket path

This enables full dogfooding: Claude Code uses the same MCP and search tools that the app's internal agents use.

### 2. Replace Hook Duplication with LSP/MCP Calls

`validate-artifact.mjs` currently parses YAML frontmatter with regex and checks relationships by reading files directly. The LSP server already does this correctly with schema awareness. Hooks must call the LSP (or MCP `graph_query`) instead of reimplementing validation.

This applies to all hooks that inspect artifact state — they delegate to the platform's existing capabilities rather than duplicating them.

### 3. Graph-First Enforcement

Agents must query `graph_query` before acting on any artifact. Reading a task file is not sufficient — the agent must understand the task's relationships (epic context, milestone, blocking dependencies) before planning work. This is enforced via:

- Orchestrator delegation prompt includes a mandatory `graph_query` step before any task breakdown
- `search_semantic` is part of the orchestrator's standard delegation flow — find similar prior work before starting
- A capability-to-tool mapping knowledge artifact documents which MCP tool to use for which governance operation

### 4. Plugin Specialist Agents

Core agent definitions (planner, implementer, reviewer, etc.) are generic. Plugins can now provide specialist agents via `provides.agents` in `orqa-plugin.json`. A specialist agent extends a core role with domain-specific prompting and knowledge injection.

Example: the `software` plugin provides a `rust-implementer` agent that extends the generic `implementer` with Rust coding standards, error handling patterns, and IPC boundary rules pre-injected. The `svelte` plugin provides a `svelte-implementer` with Svelte 5 runes patterns and component purity rules pre-loaded.

The orchestrator resolves specialist agents at delegation time: if the task domain matches a plugin's specialist, the orchestrator uses the specialist instead of the generic agent.

### 5. Knowledge Bundles

Related knowledge files are grouped into named bundles. Instead of injecting individual knowledge files, the orchestrator injects a bundle — a coherent set of knowledge appropriate for a task domain.

Bundles are defined in plugin manifests under `provides.knowledge_bundles`. Each bundle has a name, a list of knowledge file references, and a domain tag used for intent matching.

This makes INTENT_MAP maintenance tractable: instead of mapping 26+ individual files to intent keywords, the map points to 6-8 bundles.

### 6. Remove All ChunkHound References

ChunkHound is an external Python tool that predated OrqaStudio's native ONNX search engine. The native implementation is the only supported search path. All ChunkHound references — in CLAUDE.md, rules, hook scripts, documentation, and agent prompts — must be removed and replaced with the native search tool names (`search_semantic`, `search_regex`, `code_research`).

### 7. Hook Telemetry

Every hook execution logs a structured event to the dev controller's output stream: hook name, trigger event, files affected, checks performed, outcome (pass/warn/block), and duration. This gives the developer visibility into enforcement activity without reading individual hook scripts.

### 8. Bash Safety Hook

A `PostToolUse` hook on Bash tool calls intercepts dangerous command patterns before they execute:

- `--no-verify` on git commands (bypasses pre-commit hook)
- `rm -rf` without a path in the project directory
- `sudo` (not needed in dev context)
- `git push --force` or `git push -f` to main/master
- `make kill` or `make stop` without explicit user approval

The hook warns and requests confirmation rather than silently blocking, matching the existing tool approval UX.

## Consequences

- Claude Code gains access to all 10 MCP tools (graph_query, search_semantic, etc.) with no manual server startup
- validate-artifact.mjs becomes a thin adapter over LSP calls — no parsing logic of its own
- Hook duplication is eliminated — the platform's validation capabilities are used
- All 26+ knowledge files are reachable via knowledge bundles and an updated INTENT_MAP
- Plugin authors can provide specialist agents without forking core agent definitions
- Dangerous bash patterns are intercepted before execution
- Hook activity is observable without reading log files
- The codebase has no remaining ChunkHound references

## Alternatives Considered

1. **Keep processes app-internal** — rejected: Claude Code cannot reach app-internal processes; the MCP tools remain inaccessible
2. **Rewrite validate-artifact.mjs properly** — rejected: the LSP already does this correctly; rewriting duplicates rather than leverages
3. **Knowledge bundles as flat lists** — considered: bundles add a layer of indirection but the tractability gain for INTENT_MAP maintenance justifies the complexity
4. **Block dangerous bash commands entirely** — rejected: some commands (sudo for system deps) are legitimate; warn-and-confirm is safer than hard-block
