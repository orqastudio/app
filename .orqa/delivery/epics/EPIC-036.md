---
id: EPIC-036
title: "Rebrand: Forge → OrqaStudio"
description: "Product rebrand from \"Forge\" to \"Orqa Studio\": name, directory structure (src/ → ui/), brand assets, documentation, and build configuration updates."
status: completed
priority: P1
created: 2026-03-04
updated: 2026-03-09
horizon: null
scoring: null
relationships:
  - target: RES-015
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-015
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-018
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-326
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
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

[AD-027](AD-027) (Vision Evolution)

## Git Evidence

- `b20f9f8` — Core rebrand
- `4a1c88f` — Brand assets
- `8e20d5d` — Simplify scanner to Claude-only

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
