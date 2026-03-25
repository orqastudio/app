---
id: "EPIC-4cec22ea"
type: epic
title: "Native Tool UX & First-Run Setup"
description: "Two related UX improvements: (1) friendly tool call display with names, icons, and grouping; (2) first-run setup wizard for project creation and AI provider configuration."
status: "completed"
priority: "P1"
created: "2026-03-04"
updated: "2026-03-09"
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: "RES-bb4d4ae3"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-bb4d4ae3"
  - target: "RES-a2a77d0c"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-a2a77d0c"
  - target: "TASK-e79a1581"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-e328d953"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-81911380"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "IDEA-22f1345b"
    type: "realised-by"
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
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