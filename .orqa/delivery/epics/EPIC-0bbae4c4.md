---
id: "EPIC-0bbae4c4"
type: epic
title: "Rebrand: Forge → OrqaStudio"
description: "Product rebrand from \"Forge\" to \"Orqa Studio\": name, directory structure (src/ → ui/), brand assets, documentation, and build configuration updates."
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
  - target: "RES-0bbae4c4"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-0bbae4c4"
  - target: "TASK-dd51ff93"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-2195c722"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
## Implementation Design

### Scope
- Product name: Forge → Orqa Studio (OrqaStudio in code)
- Source directory: `src/` → `ui/`
- Config directory: `.forge/` → project-level config
- Brand assets: Anvil icon → custom Orqa assets and banner
- Build configs: Cargo.toml, package.json, tauri.conf.json
- 16 agent definitions updated
- All documentation and rules updated
- WelcomeScreen redesigned

## Produced Decision

[AD-e711446e](AD-e711446e) (Vision Evolution)

## Git Evidence

- `b20f9f8` — Core rebrand
- `4a1c88f` — Brand assets
- `8e20d5d` — Simplify scanner to Claude-only

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.