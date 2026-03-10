---
id: EPIC-036
title: "Rebrand: Forge → OrqaStudio"
description: 'Product rebrand from "Forge" to "Orqa Studio": name, directory structure (src/ → ui/), brand assets, documentation, and build configuration updates.'
status: done
priority: P1
created: 2026-03-04
updated: 2026-03-09
milestone: MS-001
pillars:
  - PILLAR-001
research-refs:
  - rebrand-forge-to-orqa
docs-required: []
docs-produced: []
scoring:
  user-value: 3
  pillar-alignment: 3
  dependency-weight: 5
  effort: 3
  risk: 2
  score: 16
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

AD-027 (Vision Evolution)

## Git Evidence

- `b20f9f8` — Core rebrand
- `4a1c88f` — Brand assets
- `8e20d5d` — Simplify scanner to Claude-only
