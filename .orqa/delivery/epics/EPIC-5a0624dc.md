---
id: "EPIC-5a0624dc"
type: "epic"
title: "Tech Stack Research"
description: "Pre-build investigation that determined the technology choices underpinning the entire OrqaStudio platform."
status: "completed"
priority: "P1"
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-07T00:00:00.000Z
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 3
  dependencies: 5
relationships:
  - target: "MS-063c15b9"
    type: "fulfils"
    rationale: "Epic belongs to this milestone"
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

All research findings documented in `.orqa/discovery/research/`.

## Notes

Retroactively captured. Work preceded the artifact framework.

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.

## Tasks

- [TASK-87ff401c](TASK-87ff401c): Claude Agent SDK sidecar research
- [TASK-16b6ff1e](TASK-16b6ff1e): Tauri v2 capability audit
- [TASK-449bdcd2](TASK-449bdcd2): Frontend library selection
- [TASK-13d2bf63](TASK-13d2bf63): SQLite persistence design
- [TASK-e8c74e51](TASK-e8c74e51): Onboarding strategy definition