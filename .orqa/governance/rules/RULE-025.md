---
id: RULE-025
title: Root Directory Cleanliness
description: The project root must stay lean. Every file in root must have a justification.
status: active
created: "2026-03-07"
updated: "2026-03-07"
layer: core
scope: universal
---
The project root must stay lean. Every file in root must have a justification.

## What Belongs in Root

| File/Dir | Reason |
|----------|--------|
| `README.md` | Project documentation (only user-facing doc in root) |
| `AGENTS.md` | Cross-agent instructions |
| `Cargo.toml`, `Cargo.lock` | Rust project configuration (must be root) |
| `package.json`, `package-lock.json` | Node.js/frontend dependencies |
| `tsconfig.json` | TypeScript configuration |
| `svelte.config.js` | Svelte configuration |
| `vite.config.ts` | Vite build configuration |
| `tailwind.config.ts` | Tailwind CSS configuration |
| `postcss.config.js` | PostCSS configuration |
| `.gitignore`, `.gitattributes` | Git configuration (must be root) |
| `.mcp.json` | MCP server config (CLI tool integration) |
| `.chunkhound.json` | ChunkHound code search config (must be root for auto-discovery) |
| `components.json` | shadcn-svelte component registry config (must be root) |
| `skills-lock.json` | Skills CLI lock file for reproducible skill versions (must be root) |
| `src-tauri/` | Tauri/Rust backend source |
| `ui/` | Svelte/TypeScript frontend source |
| `tests/` | E2E tests |

## What Does NOT Belong in Root

- **Temporary output files** (`.txt`, debug logs, check results) -> `tmp/`
- **Documentation** (other than README.md) -> `.orqa/documentation/`
- **Tools and scripts** -> `tools/`
- **New config files** — only add to root if the tool absolutely requires it

## Rules

1. **Never write temporary files to root.** Use `tmp/` (gitignored) for all transient output.
2. **Never create new .md documentation files in root.** README.md is the only documentation file. AGENTS.md (cross-agent instructions) is the only other .md file permitted.
3. **Before adding any file to root**, verify the tool requires root placement. If it supports a config path option, use it to place the file elsewhere.
4. **Clean up after yourself.** If a task produces temporary files, delete them or ensure they are in `tmp/`.

## Related Rules

- [RULE-003](RULE-003) (artifact-config-integrity) — config paths in `project.json` must match actual disk structure; root directory discipline keeps that structure predictable
- [RULE-007](RULE-007) (development-commands) — `make` targets reference root-level config files (`Cargo.toml`, `package.json`, etc.) whose placement this rule governs
- [RULE-013](RULE-013) (git-workflow) — `.gitignore` and `.gitattributes` belong in root per this rule; `tmp/` is gitignored to keep temporary files out of commits

