---
id: EPIC-8d2e4f6a
type: epic
title: "Connector architecture v2: dev processes, graph-first enforcement, plugin specialists"
description: Improve connector reliability and agent performance by separating MCP/ONNX/LSP into independent dev processes, leveraging existing platform capabilities instead of duplicating them in hooks, enforcing artifact graph usage, enabling plugin-provided specialist agents, and adding knowledge bundles, hook telemetry, and bash safety checks.
status: ready
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: MS-654badde
    type: fulfils
  - target: AD-3f9a1c7b
    type: driven-by
---

# EPIC-099: Connector Architecture v2 — Dev Processes, Graph-First Enforcement, Plugin Specialists

## Problem

OrqaStudio's MCP server exposes 10 tools and the LSP server validates artifact integrity — but neither is usable from Claude Code because they aren't running as independent managed processes. Hook scripts duplicate work the LSP already does. 50% of knowledge files are unreachable via INTENT_MAP. ChunkHound references remain throughout the codebase despite native ONNX search being the only supported implementation. Hook execution is silent with no telemetry. No bash safety hook exists.

## Design

See AD-061 for the full architecture decision.

### Phase 1 — Dev Process Infrastructure (unblocks everything else)

1. **MCP server as managed process**: Add MCP server startup to the dev controller. Fix install-time `.mcp.json` registration so Claude Code discovers the server automatically on `orqa install`.
2. **ONNX search engine initialization**: Move ONNX search engine startup from lazy app-internal to dev controller concern. The search engine runs as a stable process Claude Code can reach.
3. **LSP server as managed process**: Add LSP server startup to the dev controller. Advertise socket path to hook scripts so they can delegate validation calls.
4. **MCP tool surface verification**: After fixing `.mcp.json` and process startup, verify all 10 MCP tools appear in Claude Code's tool list (`graph_query`, `search_semantic`, `search_regex`, `code_research`, and the 6 additional graph tools).
5. **Hook telemetry**: Implement structured event logging for every hook execution. Events stream to dev controller output: hook name, trigger, files affected, outcome, duration.
6. **Remove ChunkHound references**: Audit and remove all ChunkHound references from CLAUDE.md, rules, hook scripts, documentation, and agent prompts. Replace with native search tool names.

### Phase 2 — Leverage Existing Capabilities

1. **Replace validate-artifact.mjs**: Rewrite as a thin adapter over LSP calls. Remove all YAML parsing and file-system inspection logic. The LSP provides the validation; the hook script reports the result.
2. **Expand INTENT_MAP**: Cover all 26+ knowledge files currently installed across plugins. Map intent keywords to knowledge bundle names (not individual files).
3. **Knowledge bundles**: Define named knowledge bundles in plugin manifests (`provides.knowledge_bundles`). Each bundle groups related knowledge files under a domain tag. Update orchestrator delegation to inject bundles instead of individual files.
4. **Bash safety PostToolUse hook**: Implement a `PostToolUse` hook on Bash tool calls. Intercept: `--no-verify` on git, `rm -rf` without scoped path, `sudo`, force push to main/master, `make kill`/`make stop` without user approval. Warn and request confirmation.
5. **Plugin specialist agent pattern**: Define `provides.agents` in plugin manifests. A specialist agent specifies a base core agent, additional knowledge bundle references, and domain-specific prompt additions. Orchestrator resolves specialists at delegation time.

### Phase 3 — Graph-First Enforcement

1. **Mandatory graph_query in orchestrator flow**: Update orchestrator delegation prompt to require `graph_query` before any task breakdown. The task's relationships (epic, milestone, dependencies) must be loaded before planning begins.
2. **search_semantic in delegation flow**: Add `search_semantic` as a required step in the orchestrator's pre-delegation checklist. Find similar prior work (tasks, decisions, lessons) before starting new work.
3. **Capability-to-tool mapping artifact**: Create a knowledge artifact documenting which MCP tool to use for which governance operation (artifact lookup, relationship traversal, semantic search, code search).
4. **Hook execution semantics documentation**: Document when hooks run, what they check, how they signal outcomes, and how to add new hooks. This is the missing contract for plugin authors adding hooks.

## Acceptance Criteria

- [ ] MCP server running as independent process, accessible from Claude Code — `graph_query` works without manual server startup
- [ ] `search_semantic` works from CLI via `orqa mcp` or directly through Claude Code's MCP tool list
- [ ] Zero ChunkHound references in codebase (grep for "chunkhound" returns nothing)
- [ ] Hook telemetry visible in dev controller output — at minimum: hook name, outcome, duration per execution
- [ ] Bash safety hook blocks `--no-verify`, `rm -rf` (unscoped), `sudo`, force push to main, and `make kill`/`make stop` without confirmation
- [ ] All 26+ knowledge files reachable via INTENT_MAP (either directly or via bundles)
- [ ] At least 2 plugins provide specialist agents via `provides.agents` in their manifests
- [ ] `validate-artifact.mjs` delegates to LSP/MCP instead of parsing YAML directly — no file-system inspection logic in the hook script
- [ ] `make check` passes after all changes

## Risks

- Dev controller process management: adding three new managed processes increases startup complexity and failure surface
- LSP socket stability: hooks calling the LSP depend on the LSP being up; need graceful degradation if LSP is not running
- Knowledge bundle migration: existing orchestrator delegation logic references individual knowledge files; migration to bundles must not regress injection quality
- ChunkHound removal: some rules reference ChunkHound for search guidance; replacement text must be accurate and complete
