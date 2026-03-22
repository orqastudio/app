---
id: EPIC-cdb03816
title: Pre-connector switch — native search MCP, skill consolidation, connector cleanup
description: System architecture work needed before switching to the Claude Code connector plugin. Consolidates search skills, exposes native search as MCP, refactors skill sync to proactive-only, and ensures the connector is production-ready.
status: captured
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: MS-654badde
    type: fulfils
  - target: TASK-9fb85edf
    type: delivered-by
  - target: TASK-6827dc2b
    type: delivered-by
  - target: TASK-945e12fa
    type: delivered-by
  - target: TASK-15a03760
    type: delivered-by
  - target: TASK-1b4f95d3
    type: delivered-by
  - target: TASK-52a123bd
    type: delivered-by
---

# EPIC-cdb03816: Pre-Connector Switch — System Architecture

## Goal

Get the system architecture right before switching to the Claude Code connector. This avoids switching then immediately refactoring under stricter governance conditions.

## Deliverables

1. **Native search as MCP** — expose search_regex, search_semantic as MCP tools from the app binary
2. **code_research implementation** — compound search tool built on regex + semantic
3. **Search skill consolidation** — merge chunkhound + orqa-code-search + orqa-native-search into one `search` skill
4. **Skill sync refactor** — sync only proactive skills (coding standards, agent preloads); rest available via MCP on demand
5. **ONNX model download** — download BGE-small model as part of this epic for dev use (NOT bundled in app by default)
6. **Connector cleanup** — final audit, remove stale references, verify all hooks/agents/skills work end-to-end
