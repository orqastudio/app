---
id: "MS-b1ac0a20"
type: "milestone"
title: "Dogfooding"
description: "OrqaStudio is usable as a daily workspace for building OrqaStudio itself. The app runs in dogfood mode, editing its own source code. This milestone covers the gap between \"pieces exist\" and \"it actually works end-to-end."
status: "active"
created: 2026-03-07T00:00:00.000Z
updated: 2026-03-07T00:00:00.000Z
gate: "Can we use this app instead of the terminal for governance management, conversation debugging, and structured thinking about the project?\nAre recent lessons observations rather than corrections? If most lessons captured in the current phase are pure observations — not requiring promotion to rules with enforcement — the infrastructure layer is mature enough to begin app dogfooding. (See IMPL-4eed88ef)\n"
relationships: []
---

## Context

OrqaStudio is developed using itself (`.orqa/project.json` has `dogfood: true`). The app runs with `--no-watch` so editing Rust files doesn't kill the active session. Frontend changes hot-reload via Vite HMR. Rust changes require manual restart.

Most core infrastructure is in place (40+ IPC commands, streaming pipeline, session persistence, tool approval, governance scanning). This milestone focuses on wiring existing pieces together and filling critical gaps that prevent daily use.

## Epics

### P1 — Critical Path

| Epic | Title | Status |
| ------ | ------- | -------- |
| [EPIC-797972a7](EPIC-797972a7) | AI Transparency Wiring | done |
| [EPIC-096fed18](EPIC-096fed18) | Settings UI for Thinking & Custom Prompt | draft |
| [EPIC-b8dc200d](EPIC-b8dc200d) | Context Injection on Failed Resume | draft |
| [EPIC-320d1a2f](EPIC-320d1a2f) | Artifact Editing UI | draft |
| [EPIC-9ddef7f9](EPIC-9ddef7f9) | Artifact Browser Enhancements | draft |
| [EPIC-2f1efbd5](EPIC-2f1efbd5) | Artifact System Migration | done |
| [EPIC-57dd7d4c](EPIC-57dd7d4c) | Vision Alignment & Schema Simplification | done |
| [EPIC-42a5330b](EPIC-42a5330b) | Three-Tier Skill Loading | done |
| [EPIC-489c0a47](EPIC-489c0a47) | Dogfood Readiness Verification | done |
| [EPIC-4ce64ab0](EPIC-4ce64ab0) | Three-Layer Governance Classification | done |
| [EPIC-0e8860dd](EPIC-0e8860dd) | Pillars as First-Class Artifacts | done |
| [EPIC-31a26b85](EPIC-31a26b85) | Git Workflow Enforcement Review | done |
| [EPIC-d45b4dfd](EPIC-d45b4dfd) | Artifact Graph SDK and Structural Integrity | done |
| [EPIC-82dd0bd2](EPIC-82dd0bd2) | Pipeline Health Dashboard | draft |
| [EPIC-a60f5b6b](EPIC-a60f5b6b) | Prioritization System and Process Enforcement | draft |

### P1 — Critical Path (Retroactive — Completed)

| Epic | Title | Status |
| ------ | ------- | -------- |
| [EPIC-7f3119b1](EPIC-7f3119b1) | Native Search Engine | done |
| [EPIC-4cec22ea](EPIC-4cec22ea) | Native Tool UX & First-Run Setup | done |
| [EPIC-0bbae4c4](EPIC-0bbae4c4) | Rebrand: Forge → OrqaStudio | done |
| [EPIC-63ff87da](EPIC-63ff87da) | Dogfood Readiness | done |
| [EPIC-f72b1a22](EPIC-f72b1a22) | UX Polish Sprint | done |
| [EPIC-c1833545](EPIC-c1833545) | Composability Refactoring | done |
| [EPIC-2f1648f5](EPIC-2f1648f5) | Provider Abstraction Layer | done |
| [EPIC-4fb5e9e1](EPIC-4fb5e9e1) | Vision & Brand Identity | done |

### P2 — Enablers

| Epic | Title | Status |
| ------ | ------- | -------- |
| [EPIC-80e3bf71](EPIC-80e3bf71) | File Watcher for External Changes | draft |
| [EPIC-c1833545](EPIC-c1833545) | Composability Refactoring | draft |
| [EPIC-e24086ed](EPIC-e24086ed) | Code Quality Audit | draft |
| [EPIC-e7deeac7](EPIC-e7deeac7) | Frontend Test Suite | draft |

### P3 — Polish

| Epic | Title | Status |
| ------ | ------- | -------- |
| [EPIC-560cf78c](EPIC-560cf78c) | Developer Experience Polish | draft |

## Completion Criteria

- [ ] All P1 epics are done
- [ ] App is used daily for OrqaStudio development alongside the CLI
- [ ] Governance artifacts are browsable and editable in the UI
- [ ] `.orqa/` artifacts (milestones, epics, ideas, research, lessons) are visible in the UI
- [ ] Conversation debugging is possible through AI transparency features
- [ ] Session context survives app restarts
- [ ] Basic git status visibility (branch, uncommitted changes, worktrees) — see [IDEA-8cad4236](IDEA-8cad4236)
- [ ] Historical artifact backfill complete (decision chains, surpassed artifacts, lesson history) — see [TASK-bf4b1013](TASK-bf4b1013)
