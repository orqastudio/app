---
id: RULE-031
title: Vision Alignment
description: Every feature must serve at least one active pillar defined in .orqa/planning/pillars/.
status: active
created: 2026-03-07
updated: 2026-03-07
layer: canon
scope: project
---
Every feature, command, and UI element must serve the project's guiding principles. These principles are defined as **pillar artifacts** in `.orqa/planning/pillars/`. Read the active pillars before implementing any new capability.

## Pillar Alignment Test

Every feature MUST trace to at least one active pillar. Pillars are structured artifacts with:

- **`title`** — The principle name
- **`description`** — What the pillar means
- **`test-questions`** — Questions to evaluate whether work serves this pillar

**Source of truth:** `.orqa/planning/pillars/PILLAR-NNN.md` files. Do not hardcode pillar names or descriptions in rules, documentation, or agent instructions — always reference the pillar artifacts.

To evaluate a feature, read each active pillar's `test-questions` and check if the feature can answer "yes" to at least one question from at least one pillar.

## Feature Rejection Criteria

Reject any feature that:

- Does not serve any active pillar (cannot answer "yes" to any pillar's test questions)
- Adds complexity without serving a pillar's intent
- Cannot articulate which pillar(s) it serves and how
- Is a generic tool feature with no connection to any pillar

## Questions Every Agent Should Ask

Before implementing any feature:

1. **Read active pillars** from `.orqa/planning/pillars/`
2. **For each pillar**, evaluate the feature against its test-questions
3. **If no pillar is served**, the feature is out of scope — flag to the user and suggest an alternative that aligns

## Pillar Conflict Resolution

Pillars are equal in importance — there is no numeric priority ranking between them. When a task or feature appears to conflict with one pillar while serving another, the agent MUST flag the conflict to the user and ask for direction. The user decides how to resolve pillar tensions — agents do not prioritise one pillar over another unilaterally. Each pillar's body may describe its relationship to other pillars, but this is context for understanding, not a precedence rule.

## UX-First Design

**Build a system that enables the best user experience, not a user experience that fits the system.**

Every feature plan starts with user journeys and UI design. The backend is derived from what the frontend needs, not the other way around. Implementation success is measured by what the user can see and do.

This means:

- Define user journeys before backend architecture
- Design the ideal UI unconstrained by current backend capabilities
- Every component has complete state handling (loading, error, empty, loaded, saving) defined upfront
- User-facing language drives naming — no framework names, no technical jargon in the UI

UX-first does NOT mean ignoring architectural constraints. It means the UI defines the *requirements* that the architecture must satisfy.

## Foundational Principles Are Immutable (NON-NEGOTIABLE)

The following are **foundational principles** that can ONLY be changed with explicit user direction and approval:

- The pillar framework (active pillars defined in `.orqa/planning/pillars/`)
- The Tauri v2 + Svelte 5 + Rust + SQLite technology stack
- The IPC boundary design (Tauri commands as the only frontend-backend interface)
- The UX-first design principle
- The documentation-first workflow
- Error propagation via Result types (no unwrap in production)

**No agent may modify, weaken, or work around these principles without the user explicitly directing the change.** If an implementation seems to require violating a foundational principle, STOP and ask the user before proceeding.

## Questioning Misaligned Instructions (MANDATORY)

If the user gives an instruction that appears to conflict with a foundational principle, the agent MUST:

1. **Flag the conflict** — Clearly explain which principle the instruction would violate and why
2. **Ask for clarification** — The user may have a valid reason, or the instruction may be a misunderstanding
3. **Document the outcome** — If the user confirms a change to a foundational principle:
   - Update the relevant documentation
   - Update `.orqa/documentation/product/vision.md` and/or `.orqa/documentation/product/governance.md` if the pillars or governance rules change
   - Update this rule file (RULE-031 (vision-alignment)) to reflect the new principle
   - Update all affected agent definitions in `.orqa/team/agents/`
4. **Never silently comply** — If an instruction contradicts a principle, do NOT just implement it without flagging the conflict first

**Examples of instructions that should be questioned:**

- "Skip the SQLite layer and just use localStorage" -> Conflicts with the persistence architecture (SQLite for conversation data, file-based artifacts for governance — AD-032)
- "Add a web server so OrqaStudio can be used in the browser" -> Conflicts with the desktop-app scope (Tauri)
- "Let components call invoke() directly instead of going through stores" -> Conflicts with component purity principle
- "Just use unwrap() here, it'll never panic" -> Conflicts with error propagation principle
- "Add a feature that has nothing to do with clarity or learning" -> Conflicts with pillar alignment

**Examples of instructions that do NOT need questioning:**

- "Add a metrics chart to the scanner dashboard" -> Serves both pillars (visibility + learning trends)
- "Create a rule editor component" -> Serves Pillar 1 (governance made visible and editable)
- "Add session history search" -> Serves Pillar 2 (knowledge accumulation across sessions)

## Related Rules

- RULE-004 (artifact-lifecycle) — artifact creation, status transitions, promotion gates, documentation gates
- RULE-021 (pillar-alignment-docs) — pillar alignment for *documentation* pages
- RULE-002 (architecture-decisions) — architecture decisions that implement the vision
- RULE-020 (no-stubs) — real implementations required, not fake demos

## Governance References

- Vision: `.orqa/documentation/product/vision.md`
- Governance: `.orqa/documentation/product/governance.md`
- Artifact Framework: `.orqa/documentation/product/artifact-framework.md`
- Artifact Workflow: `.orqa/documentation/process/artifact-workflow.md`
