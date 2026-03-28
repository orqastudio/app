---
id: KNOW-41ccf7c4
type: knowledge
status: active
title: Plugin Architecture
domain: architecture
description: Plugin taxonomy, purposes, composition pipeline, installation constraints — essential for understanding what plugins provide and how they compose
tier: core
relationships:
  synchronised-with: DOC-41ccf7c4
---

# Plugin Architecture

## Plugin Purposes

| Purpose | Effect at Install Time |
| --------- | ---------------------- |
| **Methodology definition** | Triggers full schema/workflow recomposition |
| **Workflow definition** | Triggers full schema/workflow recomposition |
| **Knowledge/rules** | Assets installed. Rule changes trigger enforcement regeneration. |
| **App extension** | Assets installed, no recomposition |
| **Sidecar** | Provides LLM inference capability |
| **Connector** | Generates + watches for regeneration |
| **Infrastructure** | Generates enforcement configs from engine rules |

## Methodology vs Workflow (Critical Distinction)

- **Methodology** = overarching approach (e.g., Agile). One per project. Provides named contribution points (slots).
- **Workflow** = self-contained sub-workflow for one methodology stage. Owns its own state machine. No inheritance. One per stage.

## Plugin Taxonomy

**Methodology:** `agile-methodology`

**Workflow plugins (one per stage):**

| Plugin | Stage |
| -------- | ------- |
| `agile-discovery` | Discovery |
| `agile-planning` | Planning |
| `agile-documentation` | Documentation |
| `software-kanban` | Implementation |
| `agile-review` | Review |
| `core` | Learning (uninstallable, also provides framework schemas + git hooks) |

**Domain knowledge:** `cli`, `rust`, `svelte`, `tauri`, `typescript`, `coding-standards`, `systems-thinking`, `plugin-dev`

**Connector:** `claude-code` → generates `.claude/` directory

## Installation Constraints

1. **One methodology plugin** per project — error if second installed
2. **One workflow plugin per stage** — error if two claim same stage
3. **Definition plugins** (methodology, workflow) → full recomposition
4. **Non-definition plugins** → assets only, no recomposition

## Composition Pipeline

Runs on every definition plugin install:

1. Read methodology plugin's workflow skeleton
2. Read each workflow plugin's contribution manifest (from plugin dirs, NOT copies)
3. Merge contributions into stage slots
4. Compose full JSON schema from all plugin-provided artifact types
5. Validate composed result
6. Write resolved workflows to `.orqa/workflows/<stage>.resolved.yaml`
7. Write `schema.composed.json` for LSP/MCP validation
8. Write prompt registry for prompt pipeline

**Source workflow definitions stay in plugin directories — only resolved output goes to `.orqa/`.**

## Content Installation

Plugin authors determine destination in `.orqa/` hierarchy. `manifest.json` tracks source hash + installed hash enabling **three-way diff**: plugin source vs installed baseline vs project copy.
