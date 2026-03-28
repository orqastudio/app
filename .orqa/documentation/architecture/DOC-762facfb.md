---
id: DOC-762facfb
type: doc
status: active
title: Codebase Structure
domain: architecture
description: Directory layout for the orqastudio-dev codebase, organized to make architectural purposes self-evident
created: 2026-03-28T00:00:00.000Z
---

# Codebase Structure

> This is part of the OrqaStudio Architecture Reference.

---

## 12. Codebase Structure

The directory layout makes architectural purposes self-evident:

```text
orqastudio-dev/
  engine/                       # Rust engine crates (functional domains)
    types/                      # Shared types, errors, traits, config, paths (foundation)
    artifact/                   # Artifact reading, parsing, frontmatter extraction
    graph/                      # Artifact relationships, traceability
    workflow/                   # State machine evaluation, guards, actions
    prompt/                     # Prompt generation pipeline
    search/                     # Semantic search, ONNX embeddings
    enforcement/                # Rule evaluation, artifact validation, config generation
    plugin/                     # Plugin system, composition, installation
    agent/                      # Base roles, task-specific agent generation
    streaming/                  # Stream loop, tool execution, LLM interaction
    lesson/                     # Lesson store, promotion pipeline
    project/                    # Project scanning, settings, governance counts
    validation/                 # Integrity checks, graph construction
    core/                       # Thin facade -- re-exports all engine crates

  libs/                         # TypeScript libraries (non-engine)
    sdk/                        # TypeScript SDK and core JSON schema (@orqastudio/sdk)
    cli/                        # TypeScript CLI library (@orqastudio/cli)
    types/                      # TypeScript shared types (@orqastudio/types)
    logger/                     # Shared logging
    graph-visualiser/           # Graph visualization component
    svelte-components/          # Shared Svelte UI components
    mcp-server/                 # MCP protocol server
    lsp-server/                 # LSP protocol server
    brand/                      # Brand assets, icons, design tokens

  daemon/                       # Persistent Rust process
    src/                        # File watchers, health endpoint, system tray

  app/                          # Desktop application (engine consumer)
    src/                        # SvelteKit frontend (TypeScript)
    src-tauri/                  # Tauri backend (Rust, thin wrapper around engine crates)

  connectors/                   # Connector plugins (generation pipelines)
    claude-code/                # Generates Claude Code Plugin to .claude/

  plugins/                      # OrqaStudio plugins organized by taxonomy
    methodology/                # Methodology plugins (one active at a time)
      agile-methodology/        # Agile methodology definition
    workflows/                  # Workflow plugins (one per methodology stage)
      agile-discovery/          # Workflow: discovery stage
      agile-planning/           # Workflow: planning stage
      agile-documentation/      # Workflow: documentation stage
      agile-review/             # Workflow: review stage
      software-kanban/          # Workflow: implementation stage
      core/                     # Workflow: learning stage + framework schemas
    knowledge/                  # Domain knowledge plugins
      cli/                      # Domain knowledge: CLI
      rust/                     # Domain knowledge: Rust
      svelte/                   # Domain knowledge: Svelte
      tauri/                    # Domain knowledge: Tauri
      typescript/               # Domain knowledge: TypeScript
      systems-thinking/         # Domain knowledge: systems thinking
      plugin-dev/               # Domain knowledge: plugin development
    infrastructure/             # Infrastructure plugins
      coding-standards/         # Infrastructure: linting config generation
      githooks/                 # Infrastructure: git hook generation

  integrations/                 # LLM provider integrations
    claude-agent-sdk/

  models/                       # ONNX models for local semantic search
  scripts/                      # Maintenance scripts
  infrastructure/               # Deployment tooling (Forgejo setup)
  .orqa/                        # Governance artifacts (stage-first organization)
  .state/                       # Session state, runtime metrics (not committed)
  .githooks/                    # Git hook scripts (committed)
  .claude/                      # Claude Code configuration, task lists, agent definitions
  targets/                      # Hand-written target states (removed after Phase 10)
  tools/                        # Dev tools (debug dashboard)
```text
