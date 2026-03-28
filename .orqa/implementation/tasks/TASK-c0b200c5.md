---
id: "TASK-c0b200c5"
type: "task"
title: "Nest frontend source into ui/src/"
description: "Move ui/ contents into ui/src/ and update svelte.config.js, components.json, and vite.config.ts."
status: archived
created: 2026-03-12T00:00:00.000Z
updated: 2026-03-12T00:00:00.000Z
acceptance:
  - "Frontend source files moved to ui/src/"
  - "svelte.config.js src field updated"
  - "components.json css path updated"
  - "vite.config.ts allow list updated"
  - "make typecheck passes"
  - "make test-frontend passes"
relationships:
  - target: "EPIC-5adc6d0a"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-1a134716"
    type: "depends-on"
---

## What

Nest frontend source one level deeper so Vite watches only `ui/src/`.

## How

1. Move contents: `ui/lib/`, `ui/routes/`, `ui/app.html`, `ui/app.css` → `ui/src/`
2. Update `svelte.config.js`: `src: 'ui'` → `src: 'ui/src'`
3. Update `components.json`: `"css": "ui/app.css"` → `"css": "ui/src/app.css"`
4. Update `vite.config.ts`: `allow: ['ui', ...]` → `allow: ['ui/src', ...]`
5. Verify with `make typecheck && make test-frontend`

## Verification

- [ ] `make typecheck` passes
- [ ] `make test-frontend` passes
- [ ] `$lib` imports still resolve correctly
