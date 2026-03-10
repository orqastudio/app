---
id: RULE-025
slug: root-cleanliness
layer: canon
status: active
scope: system
title: Root Directory Cleanliness
description: The project root must stay lean. Every file in root must have a justification.
created: 2026-03-07
updated: 2026-03-07
---


The project root must stay lean. Every file in root must have a justification.

## What Belongs in Root

| File/Dir | Reason |
|----------|--------|
| `README.md` | Project documentation (only user-facing doc in root) |
| `TODO.md`, `BLOCKERS.md` | Project tracking |
| `AGENTS.md` | Cross-agent instructions |
| `Cargo.toml`, `Cargo.lock` | Rust project configuration (must be root) |
| `package.json`, `package-lock.json` | Node.js/frontend dependencies |
| `tsconfig.json` | TypeScript configuration |
| `svelte.config.js` | Svelte configuration |
| `vite.config.ts` | Vite build configuration |
| `tailwind.config.ts` | Tailwind CSS configuration |
| `postcss.config.js` | PostCSS configuration |
| `.gitignore`, `.gitattributes` | Git configuration (must be root) |
| `.pre-commit-config.yaml` | Pre-commit hooks (must be root) |
| `.mcp.json` | MCP server config (CLI tool integration) |
| `.chunkhound.json` | ChunkHound code search config (must be root for auto-discovery) |
| `components.json` | shadcn-svelte component registry config (must be root) |
| `skills-lock.json` | Skills CLI lock file for reproducible skill versions (must be root) |
| `src-tauri/` | Tauri/Rust backend source |
| `ui/` | Svelte/TypeScript frontend source |
| `tests/` | E2E tests |
| `docs/` | Project documentation |

## What Does NOT Belong in Root

- **Temporary output files** (`.txt`, debug logs, check results) -> `tmp/`
- **Audit reports** -> `docs/audits/`
- **Documentation** (other than README.md) -> `docs/`
- **Tools and scripts** -> `tools/`
- **New config files** — only add to root if the tool absolutely requires it

## Rules

1. **Never write temporary files to root.** Use `tmp/` (gitignored) for all transient output.
2. **Never create new .md documentation files in root.** README.md is the only documentation file. Project tracking files (TODO, BLOCKERS) and agent instructions (AGENTS) are the only other .md files permitted.
3. **Before adding any file to root**, verify the tool requires root placement. If it supports a config path option, use it to place the file elsewhere.
4. **Clean up after yourself.** If a task produces temporary files, delete them or ensure they are in `tmp/`.
