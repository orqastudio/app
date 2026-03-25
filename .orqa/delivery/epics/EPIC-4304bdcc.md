---
id: EPIC-4304bdcc
type: epic
title: "Stabilisation — dev environment, governance process, agent teams"
description: "Stabilise the foundations before feature work: dev environment (orqa dev + cargo tauri dev), governance process (artifact audit, state machine review), agent team design (composable system prompts, plugin architecture), and token efficiency."
status: active
priority: P1
created: 2026-03-25
updated: 2026-03-25
horizon: active
relationships:
  - target: MS-654badde
    type: fulfils
    rationale: "Stabilisation is prerequisite for reliable dogfooding and all future delivery"
  - target: PILLAR-569581e0
    type: grounded
    rationale: "Stable foundations enable clarity through structure"
  - target: TASK-c8aa52a6
    type: delivered-by
  - target: TASK-8870f959
    type: delivered-by
  - target: TASK-aee98da7
    type: delivered-by
  - target: TASK-74f5fcdf
    type: delivered-by
  - target: TASK-cc86ee65
    type: delivered-by
  - target: TASK-272b3d07
    type: delivered-by
  - target: TASK-d28b2909
    type: delivered-by
  - target: TASK-2cc041f3
    type: delivered-by
  - target: TASK-ff0a2460
    type: delivered-by
---

## Context

Session 3 (2026-03-24) delivered major infrastructure changes: app restructure to standard Tauri layout, dev environment overhaul (orqa dev with cargo tauri dev), daemon filesystem watching, CLI standalone commands, single source of truth enforcement, and plugin manifest fixes. It also produced critical product architecture decisions about the plugin system, agent team design, and governance model.

This epic stabilises those changes and implements the remaining design decisions before moving to feature work.

## Tasks

| ID | Title | Status | Depends On |
|----|-------|--------|------------|
| [TASK-c8aa52a6](TASK-c8aa52a6) | Test orqa dev end-to-end | completed | — |
| [TASK-8870f959](TASK-8870f959) | Re-research team design with new principles | todo | — |
| [TASK-aee98da7](TASK-aee98da7) | Token efficiency — lazy rule loading | todo | TASK-8870f959 |
| [TASK-74f5fcdf](TASK-74f5fcdf) | Artifact system review — state machine, definitions, audit | todo | — |
| [TASK-cc86ee65](TASK-cc86ee65) | Milestone dependency mapping | todo | TASK-74f5fcdf |
| [TASK-d28b2909](TASK-d28b2909) | Relationship vocabulary — confirm and document plugin types | captured | — |
| [TASK-2cc041f3](TASK-2cc041f3) | Artifact IDs must be title hashes — audit + migrate | captured | — |
| [TASK-ff0a2460](TASK-ff0a2460) | Forward-only relationship storage — remove stored inverses | captured | TASK-d28b2909 |
| [TASK-272b3d07](TASK-272b3d07) | Reconcile EPIC-4304bdcc | todo | all above |
