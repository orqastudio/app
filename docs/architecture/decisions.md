---
title: "Architecture Decisions"
description: "Summary index of numbered architecture decisions (AD-NNN) that govern OrqaStudio's technical direction."
category: architecture
tags: []
created: 2026-03-02
updated: 2026-03-08
---

> Individual decision artifacts are in `.orqa/decisions/`. This page is a summary index.
> To add a new decision, create `AD-NNN.md` in `.orqa/decisions/` following the schema in
> `docs/product/artifact-framework.md`.

**Date:** 2026-03-02

Architecture decisions for OrqaStudio™. Each decision is numbered AD-NNN and is immutable once recorded — it can only be superseded by a new decision. Superseded decisions remain in the log for historical context.

## Decision Log

| ID | Title | Status | Category | Artifact |
|----|-------|--------|----------|----------|
| AD-001 | Thick Backend Architecture | accepted | architecture | `.orqa/decisions/AD-001.md` |
| AD-002 | IPC Boundary Design | accepted | architecture | `.orqa/decisions/AD-002.md` |
| AD-003 | Error Propagation via Result Types | accepted | architecture | `.orqa/decisions/AD-003.md` |
| AD-004 | Svelte 5 Runes Only | accepted | frontend | `.orqa/decisions/AD-004.md` |
| AD-005 | SQLite for All Structured Persistence | accepted | persistence | `.orqa/decisions/AD-005.md` |
| AD-006 | Component Purity | accepted | frontend | `.orqa/decisions/AD-006.md` |
| AD-007 | Agent SDK Sidecar Integration | accepted | integration | `.orqa/decisions/AD-007.md` |
| AD-008 | Max Subscription Authentication | accepted | integration | `.orqa/decisions/AD-008.md` |
| AD-009 | Streaming Pipeline | accepted | architecture | `.orqa/decisions/AD-009.md` |
| AD-010 | Tool Implementation as MCP | accepted | integration | `.orqa/decisions/AD-010.md` |
| AD-011 | Security Model | accepted | security | `.orqa/decisions/AD-011.md` |
| AD-012 | Tauri Plugin Selections | accepted | dependencies | `.orqa/decisions/AD-012.md` |
| AD-013 | Frontend Library Selections | accepted | dependencies | `.orqa/decisions/AD-013.md` |
| AD-014 | Persistence Architecture | accepted | persistence | `.orqa/decisions/AD-014.md` |
| AD-015 | Governance Artifact Format | accepted | architecture | `.orqa/decisions/AD-015.md` |
| AD-016 | Onboarding Strategy | accepted | ux | `.orqa/decisions/AD-016.md` |
| AD-017 | Composability Principle | accepted | architecture | `.orqa/decisions/AD-017.md` |
| AD-018 | Four-Zone VS Code-Style Layout | superseded | ux | `.orqa/decisions/AD-018.md` |
| AD-019 | Three-Zone + Nav Sub-Panel Layout | accepted | ux | `.orqa/decisions/AD-019.md` |
| AD-020 | Documentation Browsing Is Project-Scoped and Filesystem-Driven | accepted | ux | `.orqa/decisions/AD-020.md` |

## How to Read This Table

- **Status: accepted** — the decision is active and must be followed
- **Status: superseded** — the decision has been replaced by a newer AD; see the artifact for which AD supersedes it
- **Artifact** — the canonical source of the full decision text: context, rationale, consequences, and implementation notes

## Adding a New Decision

1. Determine the next available ID by checking the highest existing AD number in `.orqa/decisions/`
2. Create `.orqa/decisions/AD-NNN.md` following the schema in `docs/product/artifact-framework.md`
3. Add a row to the Decision Log table above
4. If the new decision supersedes an existing one, update the superseded row's status to `superseded` and add a note in the superseded artifact linking to the new one

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | Architecture decisions make technical constraints visible and explicit — each decision is a structured artifact that communicates why the system is built the way it is and what it rules out. |
| Learning Through Reflection | N/A |

## Related Documents

- `docs/product/artifact-framework.md` — artifact schemas, including the AD schema
- `docs/architecture/` — architecture documentation for individual subsystems
- `.orqa/rules/architecture-decisions.md` — enforcement rule that agents must read decisions before implementing
