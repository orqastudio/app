---
id: KNOW-6d80cf39
type: knowledge
title: "Documentation Placement — Where to Write Docs and Knowledge"
description: "Decision guide for placing documentation and knowledge artifacts. Plugin directories for production content, .orqa/ for development content. Includes the pairing rule, placement flowchart, and common mistakes."
summary: "Decision guide for placing documentation and knowledge artifacts. Plugin directories for production content, .orqa/ for development content. Includes the pairing rule, placement flowchart, and common mistakes."
status: active
created: 2026-03-24
updated: 2026-03-24
category: governance
version: 1.0.0
user-invocable: false
thinking-mode: governance
relationships:
  - target: DOC-7068f40a
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
---

# Documentation Placement

## Core Rule

**Plugins are the canonical source of truth** for all content that ships with the product. `.orqa/` contains installed copies (synced by `orqa install`) plus development-only artifacts.

## Placement Decision

### Write in PLUGIN directory when

- The content describes a feature that ships with the product
- The content would exist in a fresh project after `orqa install`
- The content is framework-level (agent roles, delegation, search methodology)
- The content is domain-specific to a plugin (Svelte patterns, Tauri IPC)

**Locations:** `plugins/core/knowledge/`, `plugins/core/docs/`, `plugins/<name>/knowledge/`, `plugins/<name>/docs/`

### Write in .orqa/ when

- The content is specific to THIS project's development process
- The content is a development artifact: decisions (AD-*), lessons (IMPL-*), planning (EPIC-*, TASK-*)
- The content is project documentation: coding standards, workflow guides
- The content is a project-specific rule

**Locations:** `.orqa/process/decisions/`, `.orqa/learning/lessons/`, `.orqa/delivery/`, `.orqa/documentation/`

## The Pairing Rule (NON-NEGOTIABLE)

Documentation and knowledge ALWAYS come in pairs:

| Artifact | Audience | Purpose | Location Pattern |
| ---------- | ---------- | --------- | ----------------- |
| Documentation | Humans | Readable narrative — what and why | `docs/` or `documentation/` |
| Knowledge | Agents | Structured context injection — what and how | `knowledge/` |

Link pairs with `synchronised-with` relationship in frontmatter. Creating one without the other is incomplete work.

## Content Flow

```text
Plugin (canonical)           orqa install           .orqa/ (installed copy)
─────────────────           ────────────           ─────────────────────
plugins/core/knowledge/  ──────────────>  .orqa/documentation/knowledge/
plugins/core/rules/      ──────────────>  .orqa/learning/rules/
plugins/core/agents/     ──────────────>  .claude/agents/
plugins/core/docs/       ──────────────>  .orqa/documentation/
```text

Each plugin's `orqa-plugin.json` declares `content` entries with `source` and `target` paths.

## Editing Rules

| Content Type | How to Edit |
| ------------- | ------------- |
| Plugin-canonical content | Edit in `plugins/<name>/`, then `orqa install` |
| Dev-only content in `.orqa/` | Edit directly — not managed by `orqa install` |
| Installed copies in `.orqa/` | DO NOT edit — changes overwritten on next install |

## Common Mistakes

| Mistake | Correction |
| --------- | ----------- |
| Writing product knowledge in `.orqa/documentation/knowledge/` | Write in the plugin's `knowledge/` directory |
| Editing installed copy in `.orqa/` directly | Edit canonical source in plugin, then `orqa install` |
| Creating doc without knowledge pair | Always create both |
| Creating knowledge without doc pair | Always create both |
| Putting dev content in a plugin | Dev content stays in `.orqa/` |

## Drift Detection

Three-way diff model detects drift between:

1. **Plugin source** (canonical)
2. **Installed baseline** (what `orqa install` last synced)
3. **Project copy** (what's currently in `.orqa/`)

Local edits and plugin updates both require reconciliation during `orqa install`.

## Related Artifacts

- [DOC-7068f40a](DOC-7068f40a) — user-facing documentation pair for this knowledge
- [KNOW-e3432947](KNOW-e3432947) — project-level knowledge about plugin-canonical architecture
- [AD-26d8d45d](AD-26d8d45d) — architecture decision formalising the plugin-canonical model
