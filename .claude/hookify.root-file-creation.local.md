---
name: warn-root-file-creation
enabled: true
event: file
action: warn
conditions:
  - field: file_path
    operator: regex_match
    pattern: ^(C:\\\\Users\\\\Bobbi\\\\code\\\\forge|/c/Users/Bobbi/code/forge)/[^/]+\.(txt|log|json|md|yaml|yml)$
  - field: file_path
    operator: not_contains
    pattern: README.md
---

**Root directory file creation detected.**

Only these files belong in root: `README.md`, `TODO.md`, `BLOCKERS.md`, `AGENTS.md`, `Makefile`, `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`, `tsconfig.json`, `svelte.config.js`, `vite.config.ts`, `tailwind.config.ts`, `postcss.config.js`, `.gitignore`, `.gitattributes`, `.pre-commit-config.yaml`, `.mcp.json`, `.chunkhound.json`

- Temporary output -> `tmp/`
- Documentation -> `docs/`
- Audit reports -> `docs/audits/`
- Config files -> use tool's path option to place elsewhere

See: `.claude/rules/root-cleanliness.md`
