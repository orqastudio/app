---
id: EPIC-025
title: "Phase 0a — Tech Stack Research"
status: done
priority: P1
milestone: MS-000
created: 2026-03-02
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: [EPIC-026, EPIC-027, EPIC-028, EPIC-029, EPIC-030]
assignee: null
pillar:
  - clarity-through-structure
scoring:
  pillar: 3
  impact: 5
  dependency: 3
  effort: 1
score: 22.0
roadmap-ref: "Phase 0a"
docs-required: []
docs-produced:
  - .orqa/research/mvp/
description: >
  Pre-build investigation that determined the technology choices
  underpinning the entire OrqaStudio platform.
tags: [foundation, research, tech-stack, planning]
---

## Why P1

All subsequent phases depend on these decisions. No architecture decisions, product definition, or scaffold can begin without knowing the tech stack.

## What Was Done

- Claude integration research — evaluated Agent SDK sidecar architecture for conversation management
- Tauri v2 capability audit — confirmed Tauri v2 meets desktop app requirements (security model, IPC, plugin ecosystem)
- Frontend library selection — evaluated and selected Svelte 5 with shadcn-svelte
- Persistence design — evaluated SQLite via rusqlite for structured local storage
- Onboarding strategy — defined approach for first-run project setup and Claude authentication

## Output

All research findings documented in `.orqa/research/mvp/`.

## Notes

Retroactively captured. Work preceded the artifact framework.
