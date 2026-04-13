---
id: "EPIC-fb1822c2"
type: epic
title: "Connector architecture v2: service extraction, semantic knowledge injection, graph-first enforcement"
description: "Extract MCP/LSP/ONNX into standalone libraries that run independently in dev mode. Replace keyword-based INTENT_MAP with semantic search for knowledge injection. Enforce artifact graph usage. Enable plugin specialist agents. Add hook telemetry and bash safety."
status: archived
created: 2026-03-20T00:00:00.000Z
updated: 2026-04-13T00:00:00.000Z
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

# EPIC-099: Connector Architecture v2 — Service Extraction, Semantic Knowledge Injection, Graph-First Enforcement

## Problem

OrqaStudio's MCP server (10 tools), LSP server (artifact validation), and ONNX search engine (semantic + regex) are all embedded inside the Tauri app binary. In dev mode, none of these services are available until the app compiles — which means Claude Code can't use graph queries, semantic search, or artifact validation to help build the app itself. This is a critical dogfooding gap.

Additionally, knowledge injection relies on a hand-maintained INTENT_MAP of keyword→knowledge mappings. This is fragile, incomplete (only covered 12 of 46 files before expansion), and fundamentally the wrong approach when we already have a semantic search engine that can determine relevance automatically.

## Design

See AD-061 for the architecture decision.

### Phase 1 — Service Extraction (unblocks everything else)

Extract three services from the Tauri app into standalone Rust crate libraries:

| Library | Current Location | Standalone Binary | App Integration |
| --------- | ----------------- | ------------------- | ----------------- |
| `libs/search` | `backend/src-tauri/src/search/` | `cargo run -p orqa-search-server` | Compiled into app as dependency |
| `libs/mcp-server` | `backend/src-tauri/src/servers/mcp.rs` | `cargo run -p orqa-mcp-server` | Compiled into app as dependency |
| `libs/lsp-server` | `backend/src-tauri/src/servers/lsp.rs` | `cargo run -p orqa-lsp-server` | Compiled into app as dependency |

Each becomes a Rust workspace member with its own `Cargo.toml`. In production, the app imports them as library crates. In dev mode, the dev controller spawns them as standalone processes — available before the app compiles.

1. **Extract search engine** into `libs/search` crate — ONNX embedder, DuckDB store, chunker, regex/semantic search
2. **Extract MCP server** into `libs/mcp-server` crate — graph query, search tools, artifact resolution. Depends on search crate.
3. **Extract LSP server** into `libs/lsp-server` crate — frontmatter validation, relationship verification, status checks
4. **Dev controller integration** — spawn all three as managed processes alongside Vite/Tauri. Log to controller dashboard. Support individual restart.
5. **Verify MCP tools surface in Claude Code** — after extraction, confirm all tools appear in the deferred tools list

### Phase 2 — Semantic Knowledge Injection

Replace the keyword-based INTENT_MAP with semantic search against the knowledge corpus:

1. **Semantic injection in prompt-injector** — on UserPromptSubmit, embed the user's prompt via the ONNX search engine, search against knowledge files (scope: artifacts), inject top-N relevant matches. No keywords, no maintenance. New knowledge files are automatically discoverable.
2. **Remove INTENT_MAP** — the keyword mapping becomes unnecessary once semantic injection works. Remove it and the associated maintenance burden.
3. **Knowledge bundles** (optional optimisation) — if semantic search returns closely related files (e.g., store-patterns + store-orchestration), inject them as a coherent bundle with cross-references. May emerge naturally from semantic similarity.
4. **Replace validate-artifact.mjs** with LSP/MCP calls — the LSP already validates schema, relationships, and status. The hook becomes a thin adapter.

### Phase 3 — Enforcement and Agents

1. **Graph-first enforcement** — enforce agents query graph_query before writing/editing .orqa/ artifacts. PreToolUse check: verify graph context was loaded before artifact modification.
2. **Semantic search in delegation** — orchestrator uses search_semantic as standard pre-delegation step. Find similar prior work before starting new tasks.
3. **Plugin specialist agents** — plugins provide specialist agents via `provides.agents` in manifest. Core agents stay generic. Specialists have specialised system prompts and domain knowledge. Connector install symlinks them alongside core agents.
4. **Hook telemetry** — structured logging for every hook execution. Events stream to dev controller: hook name, trigger, outcome, duration.
5. **Hook execution semantics docs** — document ordering, error handling, result merging, hook-to-rule mapping.

### Already Complete

- ChunkHound purge (all references removed from active files)
- INTENT_MAP expanded to 47 entries covering all 46 knowledge files (temporary bridge until semantic injection)
- Bash safety PostToolUse hook (14 rules: blocks force push, --no-verify, rm -rf, sudo, eval; warns on force-with-lease, branch -D, DROP TABLE)

## Acceptance Criteria

- [ ] Search engine runs as standalone process in dev mode (`cargo run -p orqa-search-server`)
- [ ] MCP server runs as standalone process in dev mode (`cargo run -p orqa-mcp-server`)
- [ ] LSP server runs as standalone process in dev mode (`cargo run -p orqa-lsp-server`)
- [ ] All three packaged into Tauri app in production build (library crate dependencies)
- [ ] Dev controller spawns and manages all three processes
- [ ] MCP tools (graph_query, search_semantic, etc.) available in Claude Code without app running
- [ ] Knowledge injection uses semantic search instead of INTENT_MAP
- [ ] INTENT_MAP removed from prompt-injector
- [ ] validate-artifact.mjs delegates to LSP instead of file-system parsing
- [ ] At least 2 plugins provide specialist agents via `provides.agents`
- [ ] Hook telemetry visible in dev controller output
- [ ] Hook execution semantics documented
- [ ] `make check` passes after all changes

## Risks

- Crate extraction: shared types between search/MCP/LSP need careful interface design
- ONNX model availability: standalone search needs model path resolution outside the app's data directory
- Semantic injection quality: depends on embedding model quality for short prompts against knowledge file content
- Dev controller complexity: 5 managed processes (Vite, Tauri, MCP, LSP, Search) increases startup surface
- Knowledge injection latency: semantic search adds ~100-500ms per prompt vs instant keyword matching

## Dependencies

- ONNX model (BGE-small-en-v1.5) must be available in dev environment
- Rust workspace must support multiple binary crates
- Dev controller must handle process dependencies (search must start before MCP)
