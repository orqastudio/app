---
id: KNOW-c6265cb8
type: knowledge
title: "Agent-optimized: Dev Environment Setup Guide"
description: "Condensed dev environment setup — submodules, npm linking, repo structure, dev workflow."
status: active
tier: on-demand
relationships:
  - type: synchronised-with
    target: DOC-8cf6ef38
---

# Dev Environment Setup — Agent Reference

## First-Time Setup

```bash
git clone --recurse-submodules git@github.com:orqastudio/orqastudio-dev.git
cd orqastudio-dev
bash scripts/link-all.sh   # builds libs, creates npm links, adds orqa to PATH
orqa --version              # verify
```

## Repository Structure

- `app/` — Tauri v2 desktop app (Rust + Svelte 5 + SQLite)
- `libs/types/` — @orqastudio/types (shared TS types + core.json)
- `libs/sdk/` — @orqastudio/sdk (Svelte 5 stores, graph SDK)
- `libs/cli/` — @orqastudio/cli (orqa command)
- `libs/svelte-components/` — shared UI components
- `libs/graph-visualiser/` — Cytoscape visualization
- `plugins/software/` — software delivery plugin
- `plugins/cli/` — CLI tools plugin
- `connectors/claude-code/` — Claude Code connector
- `.orqa/` — project governance artifacts

## npm Link Dependency Order

1. `libs/types` — no deps, build first
2. `libs/sdk` — depends on types
3. `libs/cli` — depends on types
4. `app/ui` — depends on types, sdk, other libs

## After Changing a Library

```bash
cd libs/<name> && npx tsc          # rebuild
cd ../../app/ui && npm link @orqastudio/<name>  # re-link
```

## Running

- `make dev` — start full dev environment (Vite + Tauri)
- `make verify` — run all checks

## Submodule Workflow

Commit in submodule first, then commit updated pointer in dev repo.
