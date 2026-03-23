---
id: EPIC-b434f0ac
type: epic
title: "Clean up tools, scripts, and githooks"
description: "Migrate all dev tools and scripts to use the CLI (which bridges to the Rust engine) instead of hand-rolling frontmatter parsers and validation logic. Shell scripts call orqa commands."
status: captured
priority: P3
relationships:
  - target: EPIC-1653af9d
    type: depends-on
    rationale: "Needs CLI commands available for tools to call"
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
# Tools and Scripts Cleanup

- 20+ `.mjs` files in `app/tools/` each with own `parseFrontmatter` — refactor to call `orqa` CLI
- `app/.githooks/*.mjs` — refactor to call `orqa enforce`
- `app/scripts/*.mjs` — refactor to use CLI
- Shell scripts in connector — call CLI
- Assess which scripts are one-off migration tools (can be archived) vs ongoing utilities (must use CLI)