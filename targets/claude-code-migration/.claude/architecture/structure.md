# Proposed Codebase Structure

> This is part of the OrqaStudio Architecture Reference. See ARCHITECTURE.md for the complete document.

---

## 12. Proposed Codebase Structure

The directory layout should make architectural purposes self-evident:

```
orqastudio-dev/
  engine/                       # NOT a directory — engine crates live in libs/

  libs/                         # Rust library crates (engine functional domains)
    graph/                      # Artifact relationships, traceability
    workflow/                   # State machine evaluation, guards, actions
    prompt/                     # Prompt generation pipeline
    search/                     # Semantic search, ONNX embeddings
    enforcement/                # Rule evaluation, artifact validation, config generation
    plugin/                     # Plugin system, composition, installation
    agent/                      # Base roles, task-specific agent generation
    brand/                      # Brand assets, icons, design tokens

  daemon/                       # Persistent Rust process
    src/                        # File watchers, MCP/LSP servers, system tray

  app/                          # Desktop application (engine consumer)
    src/                        # SvelteKit frontend (TypeScript)
    src-tauri/                  # Tauri backend (Rust, thin wrapper around engine crates)

  cli/                          # Rust CLI tool (thin wrapper around engine crates)

  connectors/                   # Connector plugins (generation pipelines)
    claude-code/                # Generates Claude Code Plugin to .claude/

  plugins/                      # OrqaStudio plugins organized by type
    methodology/                # Methodology plugins (one at a time)
      agile-workflow/
    workflows/                  # Workflow plugins (one per stage)
      agile-discovery/
      agile-planning/
      agile-documentation/
      agile-review/
      software-kanban/
      core/                     # Learning stage + framework schemas + enforcement
    knowledge/                  # Domain knowledge plugins
      cli/
      rust/
      svelte/
      tauri/
      typescript/
      coding-standards/
      systems-thinking/
      plugin-dev/
  sidecars/                     # LLM provider integrations (top-level — unique purpose)
    claude-agent-sdk/

  models/                       # ONNX models for local semantic search
  templates/                    # Project scaffolding templates for orqa init
  scripts/                      # Migration and maintenance scripts
  infrastructure/               # Deployment tooling (Forgejo setup)
```
