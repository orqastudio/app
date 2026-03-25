---
id: IDEA-028dace7
type: idea
title: "File-opening protocol for .orqa artifacts"
description: "Register a custom file association or URI scheme so .orqa artifact files (.md files in .orqa/ directories) open directly in OrqaStudio from the OS file explorer. Cross-platform (Windows, macOS, Linux). Works in production installs and during development when the app is running."
status: captured
priority: P3
created: 2026-03-24
updated: 2026-03-24
horizon: later
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Makes the artifact graph tangible — clicking a file opens it in its proper context, not a text editor"
  - target: PERSONA-477971bf
    type: benefits
    rationale: "Practitioners can navigate to artifacts from any file explorer"
---

## What

Double-clicking an artifact file (e.g., `RULE-9814ec3c.md`) in Windows Explorer, Finder, or Nautilus should open it in OrqaStudio's artifact viewer — not a text editor.

## Approach options
- Custom URI scheme (`orqa://artifact/.orqa/process/rules/RULE-9814ec3c.md`)
- File association for `.md` files inside `.orqa/` directories
- Tauri deep linking support
