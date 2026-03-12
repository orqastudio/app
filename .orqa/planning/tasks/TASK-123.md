---
id: TASK-123
title: "Initialize Tauri v2 + Svelte 5 project"
description: "Set up the initial Tauri v2 project with Svelte 5 frontend, configured plugins, build tooling, and development scripts."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-030
depends-on: []
acceptance:
  - make dev launches the app successfully
  - Hot module replacement works for Svelte changes
  - Tauri plugins are configured and accessible
---
## What

Initialized the Tauri v2 project with a Svelte 5 frontend template and configured all required plugins, build tooling, and development scripts.

## How

Used the Tauri v2 CLI to scaffold the project, then wired up Svelte 5, Tailwind CSS, PostCSS, and TypeScript. Created the Makefile with `dev`, `build`, `check`, and related targets.

## Verification

`make dev` launches the app, HMR reloads on Svelte file changes, and Tauri plugins are accessible from the frontend.
