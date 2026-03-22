---
id: TASK-6675ad7c
title: LSP server — real-time frontmatter validation
status: active
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-6967c7dc
    type: delivers
  - target: TASK-c9880303
    type: depends-on
  - target: TASK-bd4e7250
    type: depended-on-by
---

# TASK-6675ad7c: LSP Server

## Acceptance Criteria

1. LSP server module added to Tauri app (`src/servers/lsp.rs`)
2. tower-lsp and lsp-types dependencies added to Cargo.toml
3. Frontmatter schema validation (required fields, valid types per core.json)
4. Relationship type validation (only keys from core.json + plugins)
5. Relationship target existence (targets must resolve in the graph)
6. Status validation (12 canonical statuses only)
7. Bidirectional relationship enforcement (missing inverse = diagnostic error)
8. Serves over stdio, registered for .md files in .orqa/ directories
