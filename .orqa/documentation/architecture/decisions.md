---
title: "Architecture Decisions"
description: "Summary index of numbered architecture decisions (AD-NNN) that govern OrqaStudio's technical direction."
created: 2026-03-02
updated: 2026-03-09
---

> Individual decision artifacts are in `.orqa/governance/decisions/`. This page is a summary index.
> To add a new decision, create `AD-NNN.md` in `.orqa/governance/decisions/` following the schema in
> `.orqa/documentation/product/artifact-framework.md`.

**Date:** 2026-03-02

Architecture decisions for OrqaStudio™. Each decision is numbered AD-NNN and is immutable once recorded — it can only be superseded by a new decision. Superseded decisions remain in the log for historical context.

## Decision Log

| ID | Title | Status | Category | Artifact |
|----|-------|--------|----------|----------|
| AD-001 | Thick Backend Architecture | accepted | architecture | `.orqa/governance/decisions/AD-001.md` |
| AD-002 | IPC Boundary Design | accepted | architecture | `.orqa/governance/decisions/AD-002.md` |
| AD-003 | Error Propagation via Result Types | accepted | architecture | `.orqa/governance/decisions/AD-003.md` |
| AD-004 | Svelte 5 Runes Only | accepted | frontend | `.orqa/governance/decisions/AD-004.md` |
| AD-005 | SQLite for All Structured Persistence | accepted | persistence | `.orqa/governance/decisions/AD-005.md` |
| AD-006 | Component Purity | accepted | frontend | `.orqa/governance/decisions/AD-006.md` |
| AD-007 | Agent SDK Sidecar Integration | accepted | integration | `.orqa/governance/decisions/AD-007.md` |
| AD-008 | Max Subscription Authentication | accepted | integration | `.orqa/governance/decisions/AD-008.md` |
| AD-009 | Streaming Pipeline | accepted | architecture | `.orqa/governance/decisions/AD-009.md` |
| AD-010 | Tool Implementation as MCP | accepted | integration | `.orqa/governance/decisions/AD-010.md` |
| AD-011 | Security Model | accepted | security | `.orqa/governance/decisions/AD-011.md` |
| AD-012 | Tauri Plugin Selections | accepted | dependencies | `.orqa/governance/decisions/AD-012.md` |
| AD-013 | Frontend Library Selections | accepted | dependencies | `.orqa/governance/decisions/AD-013.md` |
| AD-014 | Persistence Architecture | accepted | persistence | `.orqa/governance/decisions/AD-014.md` |
| AD-015 | Governance Artifact Format | superseded | architecture | `.orqa/governance/decisions/AD-015.md` |
| AD-016 | Onboarding Strategy | accepted | ux | `.orqa/governance/decisions/AD-016.md` |
| AD-017 | Composability Principle | accepted | architecture | `.orqa/governance/decisions/AD-017.md` |
| AD-018 | Four-Zone VS Code-Style Layout | superseded | ux | `.orqa/governance/decisions/AD-018.md` |
| AD-019 | Three-Zone + Nav Sub-Panel Layout | accepted | ux | `.orqa/governance/decisions/AD-019.md` |
| AD-020 | Documentation Browsing Is Project-Scoped and Filesystem-Driven | accepted | ux | `.orqa/governance/decisions/AD-020.md` |
| AD-021 | .orqa/ as Single Source of Truth | accepted | governance | `.orqa/governance/decisions/AD-021.md` |
| AD-022 | Config-Driven Artifact Scanning | accepted | governance | `.orqa/governance/decisions/AD-022.md` |
| AD-023 | Artifact Schema Simplification — Plans Merged Into Research | accepted | governance | `.orqa/governance/decisions/AD-023.md` |
| AD-024 | Native Search Engine (DuckDB + ONNX Embeddings + DirectML) | accepted | architecture | `.orqa/governance/decisions/AD-024.md` |
| AD-025 | Provider-Agnostic AI Integration | accepted | architecture | `.orqa/governance/decisions/AD-025.md` |
| AD-026 | Domain Service Extraction Pattern | accepted | architecture | `.orqa/governance/decisions/AD-026.md` |
| AD-027 | Vision Evolution — Domain-Agnostic Clarity Engine | accepted | product | `.orqa/governance/decisions/AD-027.md` |
| AD-028 | Three-Tier Skill Loading — Agent, Orchestrator, Wrapper | accepted | governance | `.orqa/governance/decisions/AD-028.md` |
| AD-029 | Universal Roles, Domain-Specific Skills | accepted | governance | `.orqa/governance/decisions/AD-029.md` |
| AD-030 | Skill-Driven Project Initialisation | accepted | governance | `.orqa/governance/decisions/AD-030.md` |
| AD-031 | Pillars as First-Class Planning Artifacts | accepted | governance | `.orqa/governance/decisions/AD-031.md` |

## How to Read This Table

- **Status: accepted** — the decision is active and must be followed
- **Status: superseded** — the decision has been replaced by a newer AD; see the artifact for which AD supersedes it
- **Artifact** — the canonical source of the full decision text: context, rationale, consequences, and implementation notes

## Adding a New Decision

1. Determine the next available ID by checking the highest existing AD number in `.orqa/governance/decisions/`
2. Create `.orqa/governance/decisions/AD-NNN.md` following the schema in `.orqa/documentation/product/artifact-framework.md`
3. Add a row to the Decision Log table above
4. If the new decision supersedes an existing one, update the superseded row's status to `superseded` and add a note in the superseded artifact linking to the new one

## Pillar Alignment

| Pillar | Alignment |
|--------|-----------|
| Clarity Through Structure | Architecture decisions make technical constraints visible and explicit — each decision is a structured artifact that communicates why the system is built the way it is and what it rules out. |
| Learning Through Reflection | N/A |

## Related Documents

- `.orqa/documentation/product/artifact-framework.md` — artifact schemas, including the AD schema
- `.orqa/documentation/architecture/` — architecture documentation for individual subsystems
- `.orqa/governance/rules/architecture-decisions.md` — enforcement rule that agents must read decisions before implementing
