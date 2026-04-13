---
id: RULE-1b238fc8
type: rule
title: Vision Alignment
description: Every feature must serve at least one active pillar defined in .orqa/discovery/pillars/.
status: active
enforcement_type: advisory
created: 2026-03-07
updated: 2026-04-13
enforcement:

  - mechanism: behavioral

    message: "Every feature must serve at least one active pillar; orchestrator reads active pillars and evaluates gate questions before delegating any task; features failing pillar alignment are rejected"
relationships:

  - target: PD-e711446e

    type: enforces

  - target: DOC-06224bf6

    type: documented-by
---
Every feature, command, and UI element must serve the project's guiding principles. These principles are defined as **pillar artifacts** in `.orqa/discovery/pillars/`. Read the active pillars before implementing any new capability.

## Pillar Alignment Test

Every feature MUST trace to at least one active pillar. Pillars are structured artifacts with:

- **`title`** — The principle name
- **`description`** — What the pillar means
- **`gate`** — Questions to evaluate whether work serves this pillar

**Source of truth:** `.orqa/discovery/pillars/PILLAR-NNN.md` files. Do not hardcode pillar names or descriptions in rules, documentation, or agent instructions — always reference the pillar artifacts.

To evaluate a feature, read each active pillar's `gate` questions and check if the feature can answer "yes" to at least one question from at least one pillar.

## Feature Rejection Criteria

Reject any feature that:

- Does not serve any active pillar (cannot answer "yes" to any pillar's gate questions)
- Adds complexity without serving a pillar's intent
- Cannot articulate which pillar(s) it serves and how
- Is a generic tool feature with no connection to any pillar

## Questions Every Agent Should Ask

Before implementing any feature:

1. **Read active pillars** from `.orqa/discovery/pillars/`
2. **For each pillar**, evaluate the feature against its gate questions
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

- The pillar framework (active pillars defined in `.orqa/discovery/pillars/`)
- The project's technology stack as defined in architecture decisions
- The service boundary design (defined in architecture decisions)
- The UX-first design principle
- The documentation-first workflow
- Error propagation via result types (no silent failures in production)

**No agent may modify, weaken, or work around these principles without the user explicitly directing the change.** If an implementation seems to require violating a foundational principle, STOP and ask the user before proceeding.

## Questioning Misaligned Instructions (MANDATORY)

If the user gives an instruction that appears to conflict with a foundational principle, the agent MUST:

1. **Flag the conflict** — Clearly explain which principle the instruction would violate and why
2. **Ask for clarification** — The user may have a valid reason, or the instruction may be a misunderstanding
3. **Document the outcome** — If the user confirms a change to a foundational principle:
   - Update the relevant documentation
   - Update `.orqa/documentation/about/vision.md` and/or `.orqa/documentation/about/governance.md` if the pillars or governance rules change
   - Update this rule file ([RULE-1b238fc8](RULE-1b238fc8) (vision-alignment)) to reflect the new principle
   - Update all affected agent definitions in `.orqa/process/agents/`
4. **Never silently comply** — If an instruction contradicts a principle, do NOT just implement it without flagging the conflict first

## Related Rules

- [RULE-b10fe6d1](RULE-b10fe6d1) (artifact-lifecycle) — artifact creation, status transitions, promotion gates, documentation gates
- [RULE-05562ed4](RULE-05562ed4) (pillar-alignment-docs) — pillar alignment for *documentation* pages
- [RULE-05ae2ce7](RULE-05ae2ce7) (architecture-decisions) — architecture decisions that implement the vision
- [RULE-af5771e3](RULE-af5771e3) (no-stubs) — real implementations required, not fake demos

## Governance References

- Vision: `.orqa/documentation/about/vision.md`
- Governance: `.orqa/documentation/about/governance.md`
- Artifact Framework: `.orqa/documentation/about/artifact-framework.md`
- Artifact Workflow: `.orqa/documentation/guide/artifact-workflow.md`
