---
id: EPIC-025
title: Tech Stack Research
description: Pre-build investigation that determined the technology choices underpinning the entire OrqaStudio platform.
status: done
priority: P1
created: "2026-03-02"
updated: "2026-03-07"
milestone: MS-000
pillars:
  - PILLAR-001
depends-on: []
blocks:
  - EPIC-026
  - EPIC-027
  - EPIC-028
  - EPIC-029
  - EPIC-030
docs-required: []
docs-produced: []
scoring:
  pillar: 3
  impact: 5
  dependency: 3
  effort: 1
  score: 28
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

All research findings documented in `.orqa/delivery/research/`.

## Notes

Retroactively captured. Work preceded the artifact framework.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

- [TASK-094](TASK-094): Claude Agent SDK sidecar research
- [TASK-095](TASK-095): Tauri v2 capability audit
- [TASK-096](TASK-096): Frontend library selection
- [TASK-097](TASK-097): SQLite persistence design
- [TASK-098](TASK-098): Onboarding strategy definition
