---
id: KNOW-72d39a1b
type: knowledge
name: plugin-canonical-architecture
title: "Plugin-Canonical Architecture — Where Content Lives and Why"
description: "Plugins are the canonical source of truth for all app-functional content. .orqa/ contains installed copies (via orqa install) plus development-only artifacts. Explains the content flow, when to write in plugins vs .orqa/, and the docs-knowledge pairing rule."
layer: project
user-invocable: false
thinking-mode: governance
status: active
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: DOC-ffee5406
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
  - target: AGENT-1dab5ebe
    type: employed-by
    rationale: "Auto-generated inverse of employed-by relationship from AGENT-1dab5ebe"
  - target: AGENT-34a6e2cd
    type: employed-by
    rationale: "Auto-generated inverse of employed-by relationship from AGENT-34a6e2cd"
  - target: AGENT-fbdfa9ef
    type: employed-by
    rationale: "Auto-generated inverse of employed-by relationship from AGENT-fbdfa9ef"
---
# Plugin-Canonical Architecture

## Core Principle

**Plugins are the canonical source of truth** for all content the app needs to function. The `.orqa/` directory contains **installed copies** (synced by `orqa install`) plus **development-only artifacts**.

## Content Flow

```
Plugin (canonical)           orqa install           .orqa/ (installed copy)
─────────────────           ────────────           ─────────────────────
plugins/core/knowledge/  ──────────────>  .orqa/process/knowledge/
plugins/core/rules/      ──────────────>  .orqa/process/rules/
plugins/core/agents/     ──────────────>  .orqa/process/agents/
plugins/<name>/knowledge/ ─────────────>  .orqa/process/knowledge/
plugins/<name>/rules/    ──────────────>  .orqa/process/rules/
```

Each plugin's `orqa-plugin.json` declares `content` entries with `source` and `target` paths. `orqa install` reads these manifests and syncs.

## Where to Write

### Write in the PLUGIN directory when:

- The content describes a feature that ships with the product
- The content would exist in a fresh project after `orqa install`
- The content is framework-level infrastructure (port allocation, service architecture, orchestrator definition)
- The content is domain-specific to a plugin (Svelte patterns → svelte plugin, Tauri patterns → tauri plugin)

**Examples:** `plugins/core/knowledge/`, `plugins/core/rules/`, `plugins/software/knowledge/`

### Write in .orqa/ when:

- The content is specific to THIS project's development process
- The content is a development-only artifact: decisions (AD-*), lessons (IMPL-*), planning (EPIC-*, TASK-*, IDEA-*)
- The content is project-specific documentation: coding standards, workflow guides, architecture docs
- The content is a project-specific rule that other OrqaStudio projects wouldn't need

**Examples:** `.orqa/process/decisions/`, `.orqa/process/lessons/`, `.orqa/delivery/`, `.orqa/documentation/`

## The Pairing Rule

Documentation and knowledge ALWAYS come in pairs, at whatever level they live:

| Artifact | Audience | Purpose |
|----------|----------|---------|
| **Documentation** (`docs/` or `documentation/`) | Humans | Readable narrative — what and why |
| **Knowledge** (`knowledge/`) | Agents | Structured content for context injection — what and how |

Creating one without the other is incomplete work. Use the `synchronised-with` relationship type to link paired artifacts.

## Common Mistakes

| Mistake | Correction |
|---------|-----------|
| Writing knowledge in `.orqa/process/knowledge/` for a feature that ships with the app | Write in the plugin's `knowledge/` directory — `.orqa/` gets the installed copy via `orqa install` |
| Editing an installed copy in `.orqa/` directly | Edit the canonical source in the plugin directory, then run `orqa install` |
| Creating a knowledge artifact without a matching doc page | Always create both — docs + knowledge are a pair |
| Creating a doc page without matching knowledge | Always create both — docs + knowledge are a pair |

## Drift Detection

The three-way diff model detects drift between:
1. **Plugin source** (canonical)
2. **Installed baseline** (what `orqa install` last synced)
3. **Project copy** (what's currently in `.orqa/`)

If a project copy diverges from the installed baseline, the system detects a local edit. If the plugin source diverges from the installed baseline, the system detects a plugin update. Both require reconciliation.

## Related Artifacts

- [AD-c4d5e6f7](AD-c4d5e6f7) — architecture decision formalising this model
- [IMPL-d7e9f1a3](IMPL-d7e9f1a3) — lesson that prompted this decision
- [DOC-ffee5406](DOC-ffee5406) — user-facing documentation pair for this knowledge
