---
id: "TASK-1af78c36"
type: task
title: "Research: cross-cutting concerns of directory restructure"
description: "Audit every config, import, and path reference affected by the proposed repository reorganisation into backend/, ui/src/, sidecars/, and debugger/ directories."
status: "completed"
created: "2026-03-12"
updated: "2026-03-12"
acceptance:
  - "Complete inventory of files containing paths to src-tauri/, ui/, sidecar/, scripts/"
  - "Per-file list of exact changes needed"
  - "Watcher configuration analysis (current vs proposed)"
  - "Proposed migration order with verification steps between each move"
  - "Risk assessment with rollback strategy"
  - "Findings written to RES-4dbf04d7"
relationships:
  - target: "EPIC-5adc6d0a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-43c0557f"
    type: "depended-on-by"
---
## What

Audit the entire repository for cross-cutting path references that would break when
directories are reorganised per [EPIC-5adc6d0a](EPIC-5adc6d0a). The goal is a complete map of every
change needed so that the implementation phase can proceed with confidence.

## How

1. **Config file audit** — Search all config files (Cargo.toml, package.json, vite.config.ts,
   tauri.conf.json, tsconfig.json, svelte.config.js, postcss.config.js, tailwind.config.ts,
   Makefile, .gitignore, .mcp.json, .chunkhound.json, components.json) for references to
   `src-tauri`, `ui/`, `sidecar/`, `scripts/`

2. **Import path audit** — Search TypeScript/Svelte files for relative imports that cross
   directory boundaries. Search Rust files for path references.

3. **Watcher analysis** — Read Vite config and Cargo/Tauri watch config to understand
   current watcher scope and document needed changes.

4. **Dev controller audit** — Read `debugger/dev.mjs` for hardcoded paths, process spawn
   commands, and dashboard serving paths.

5. **Documentation audit** — Search `.orqa/` for references to current directory paths.

6. **Migration order** — Based on dependency analysis, propose which directory to move
   first and what verification to run between each step.

7. **Write findings** — Update [RES-4dbf04d7](RES-4dbf04d7) with complete findings.

## Verification

- [ ] Every config file in the repository has been checked
- [ ] Import paths across all languages have been audited
- [ ] Watcher configuration is documented (current and proposed)
- [ ] Migration order is proposed with rationale
- [ ] Risk assessment covers rollback strategy
- [ ] [RES-4dbf04d7](RES-4dbf04d7) contains complete findings