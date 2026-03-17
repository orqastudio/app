---
id: EPIC-025
title: Tech Stack Research
description: Pre-build investigation that determined the technology choices underpinning the entire OrqaStudio platform.
status: completed
priority: P1
created: 2026-03-02
updated: 2026-03-07
horizon: null
scoring:
  impact: 5
  urgency: 5
  complexity: 3
  dependencies: 5
relationships:
  - target: MS-000
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-094
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-095
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-096
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-097
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-098
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-315
    type: delivered-by
    rationale: Epic contains this task
  - target: EPIC-026
    type: depended-on-by
  - target: EPIC-027
    type: depended-on-by
  - target: EPIC-028
    type: depended-on-by
  - target: EPIC-029
    type: depended-on-by
  - target: EPIC-030
    type: depended-on-by
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-001
    type: grounded-by
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
