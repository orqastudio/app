---
id: TASK-424c6e2c
type: task
title: "Move domain skills to their respective plugins"
status: archived
created: 2026-03-19
updated: 2026-03-21
relationships:
  - target: EPIC-9b58fdcb
    type: delivers
---

# TASK-424c6e2c: Move Domain Skills to Plugins

## Context

Several domain-specific skills currently live in `app/.orqa/process/skills/` (core platform) when they should be shipped by their respective plugins:

- `svelte5-best-practices.md` + 7 reference skills (`svelte5-runes`, `svelte5-events`, etc.) → `plugins/svelte/skills/`
- `tauri-v2.md` + 2 reference skills (`tauri-v2-capabilities-reference`, `tauri-v2-ipc-patterns`) → `plugins/tauri/skills/`
- `rust-async-patterns.md` → `plugins/rust/skills/`
- `tailwind-design-system.md` → `plugins/svelte/skills/` or a future design plugin
- `typescript-advanced-types.md` → `plugins/typescript/skills/`

## Acceptance Criteria

1. Skills moved from `app/.orqa/process/skills/` to their plugin's `skills/` directory
2. Plugin manifests (`orqa-plugin.json`) updated with new skill entries
3. Relationships updated on moved skills (employed-by agents)
4. Inverse relationships updated on agent definitions
5. No broken references after move
