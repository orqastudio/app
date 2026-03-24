---
id: DOC-ffee5406
type: doc
title: Plugin-Canonical Architecture Guide
description: "How content is organised in OrqaStudio: plugins are the canonical source of truth, .orqa/ holds installed copies and dev-only artifacts. Covers the content flow, placement decisions, and the docs-knowledge pairing rule."
category: architecture
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: KNOW-72d39a1b
    type: synchronised-with
    rationale: "Agent-facing knowledge pair for this user-facing documentation page"
---

# Plugin-Canonical Architecture Guide

## Overview

OrqaStudio uses a **plugin-canonical** architecture for content management. This means:

- **Plugins own the canonical source** of all content that ships with the product — knowledge, rules, agents, documentation, skills.
- **`.orqa/` holds installed copies** of plugin content (synced by `orqa install`) **plus development-only artifacts** that exist only in the project.

Understanding this distinction is essential for knowing where to create, edit, and find content.

## How Content Flows

When you run `orqa install`, the CLI reads each plugin's `orqa-plugin.json` manifest and syncs content from the plugin's source directories into `.orqa/`:

```
plugins/core/knowledge/    -->  .orqa/process/knowledge/
plugins/core/rules/        -->  .orqa/process/rules/
plugins/core/agents/       -->  .orqa/process/agents/
plugins/software/knowledge/ --> .orqa/process/knowledge/
plugins/agile-governance/   --> .orqa/process/ (various)
```

Each plugin manifest declares content entries like:

```json
{
  "content": {
    "knowledge": { "source": "knowledge", "target": ".orqa/process/knowledge" },
    "rules": { "source": "rules", "target": ".orqa/process/rules" }
  }
}
```

The plugin directory is the **canonical source**. The `.orqa/` copy is the **installed version**.

## Where Does My Content Belong?

### In a plugin directory (canonical, ships with the product)

Write content in a plugin when it describes something that is part of the product itself:

| Content Type | Example | Plugin |
|-------------|---------|--------|
| Core framework knowledge | Port allocation, service architecture | `plugins/core/` |
| Orchestrator and agent definitions | Agent roles, delegation rules | `plugins/core/` |
| Domain-specific patterns | Svelte component patterns | `plugins/svelte/` |
| Technology-specific guidance | Tauri IPC patterns | `plugins/tauri/` |
| CLI command documentation | CLI usage and commands | `plugins/cli/` |
| Project type templates | Software project defaults | `plugins/software/` |

**Test:** Would this content exist in a brand new project that runs `orqa install`? If yes, it belongs in a plugin.

### In .orqa/ (dev-only, project-specific)

Write content in `.orqa/` when it is specific to this project's development:

| Content Type | Example | Location |
|-------------|---------|----------|
| Architecture decisions | AD-c4d5e6f7, AD-2aa4d6db | `.orqa/process/decisions/` |
| Lessons learned | IMPL-d7e9f1a3 | `.orqa/process/lessons/` |
| Project-specific rules | Rules unique to this project | `.orqa/process/rules/` |
| Planning artifacts | Epics, tasks, ideas, research | `.orqa/delivery/` |
| Development documentation | Coding standards, workflow guides | `.orqa/documentation/` |

**Test:** Is this specific to THIS project's development process? If yes, it belongs in `.orqa/`.

## The Docs-Knowledge Pairing Rule

Every piece of documented content must exist as **two paired artifacts**:

1. **Documentation page** — user-facing. Written for human developers who want to understand a feature, configuration, or concept. Readable, contextual, narrative.
2. **Knowledge artifact** — agent-facing. Written to be injected into agent context when working in a related code area. Structured, concise, action-oriented.

This pairing applies at both levels:
- **Plugin level:** `plugins/core/docs/` paired with `plugins/core/knowledge/`
- **Project level:** `.orqa/documentation/` paired with `.orqa/process/knowledge/`

Use the `synchronised-with` relationship type to link paired artifacts in their frontmatter.

### Why Pairs?

- **Missing docs:** Agents have context but humans can't find explanations — confusing for new contributors.
- **Missing knowledge:** Humans have docs but agents lack context injection — agents make mistakes in documented areas.
- **Both present:** Humans can read the guide, agents get structured context injected automatically. Full coverage.

## Editing Content

### Editing plugin-canonical content

1. Find the canonical source in `plugins/<name>/`
2. Edit the file in the plugin directory
3. Run `orqa install` to sync the change into `.orqa/`

**Do not edit the installed copy in `.orqa/` directly** — your changes will be overwritten on the next `orqa install`.

### Editing dev-only content

Edit directly in `.orqa/`. These files are not managed by `orqa install` and won't be overwritten.

### Drift detection

OrqaStudio uses a three-way diff model to detect when:
- A **local edit** was made to an installed copy (project copy differs from installed baseline)
- A **plugin update** changed the canonical source (plugin source differs from installed baseline)

Both cases surface during `orqa install` for reconciliation.

## Quick Reference

| Question | Answer |
|----------|--------|
| Where is the canonical source for core rules? | `plugins/core/rules/` |
| Where do I find installed rules at runtime? | `.orqa/process/rules/` |
| Where do I write a new architecture decision? | `.orqa/process/decisions/` |
| Where do I write production feature docs? | In the relevant plugin's `docs/` or `knowledge/` directory |
| Where do I write development process docs? | `.orqa/documentation/` |
| Do I need both a doc page and knowledge artifact? | Yes, always — they come in pairs |

## Related Documents

- [AD-c4d5e6f7](AD-c4d5e6f7) — Architecture decision formalising the plugin-canonical model
- [IMPL-d7e9f1a3](IMPL-d7e9f1a3) — Lesson that prompted this architectural clarification
- [DOC-65eb8303](DOC-65eb8303) — Dev environment setup (covers `orqa install` in the setup workflow)
- [KNOW-72d39a1b](KNOW-72d39a1b) — Agent-facing knowledge pair for this documentation page
