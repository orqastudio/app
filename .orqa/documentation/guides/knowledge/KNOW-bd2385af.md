---
id: KNOW-bd2385af
type: knowledge
title: "Agent-optimized: Documentation Placement Guide"
description: "Condensed placement rules — plugin vs .orqa/, pairing rule, editing rules, drift detection."
status: active
tier: on-demand
relationships:
  - type: synchronised-with
    target: DOC-7068f40a
---

# Documentation Placement — Agent Reference

## Core Rule

Plugins are the canonical source for all content that ships with the product. `.orqa/` holds installed copies (synced by `orqa install`) plus dev-only artifacts.

## Where to Write

### Plugin directory (`plugins/<name>/`)
- Content that ships with the product
- Would exist in a fresh project after `orqa install`
- Framework-level: `plugins/core/`
- Domain-specific: `plugins/svelte/`, `plugins/tauri/`
- Project-type defaults: `plugins/software-kanban/`

### Project directory (`.orqa/`)
- Architecture decisions: `.orqa/process/decisions/`
- Lessons learned: `.orqa/process/lessons/`
- Planning artifacts: `.orqa/implementation/`
- Dev documentation: `.orqa/documentation/`
- Project-specific rules: `.orqa/process/rules/`

## Pairing Rule (NON-NEGOTIABLE)

Every piece of content requires TWO artifacts:
1. **Doc** (human-facing) in `docs/` or `documentation/`
2. **Knowledge** (agent-facing) in `knowledge/`

Link with `synchronised-with` relationship. Creating one without the other is incomplete.

## Editing Rules

- Plugin-canonical content: edit in `plugins/<name>/`, then `orqa install`
- Dev-only content: edit directly in `.orqa/`
- Installed copies: DO NOT edit — overwritten on next install

## Drift Detection

Three-way diff: plugin source vs installed baseline vs project copy. Both local edits and plugin updates surface during `orqa install`.
