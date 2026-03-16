---
id: EPIC-035
title: "Native Tool UX & First-Run Setup"
description: "Two related UX improvements: (1) friendly tool call display with names, icons, and grouping; (2) first-run setup wizard for project creation and AI provider configuration."
status: completed
priority: P1
created: 2026-03-04
updated: 2026-03-09
horizon: null
scoring: null
relationships:
  - target: RES-012
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-012
  - target: RES-013
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-013
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-016
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-017
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-325
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
  - target: IDEA-003
    type: evolves-from
  - target: DOC-058
    type: informs
  - target: DOC-061
    type: informs
---
## Implementation Design

### Native Tool UX
- Friendly names for tool types: Read → "Reading file", Bash → "Running command"
- Lucide icons per tool type
- Parameter extraction for summary display
- Consecutive call de-duplication ("Read 3 files")
- Collapsible detail view

### First-Run Setup Wizard
- Claude CLI detection (binary on PATH)
- Auth status verification
- Project configuration (name, icon, model)
- Custom project icon upload via Tauri dialog plugin
- Settings decomposition into focused sub-components

## Git Evidence

- `b0ee670` — Phase 1: Native tool UX
- `1ccf304` — Phase 2a: First-run setup wizard
- `5156a6e` — CLI version and auth status
- `34ec185` — Custom project icon
- `1193abb` — File-based project settings

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
